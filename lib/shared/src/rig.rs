extern crate alloc;

use alloc::boxed::Box;
use core::cell::RefCell;
use core::ops::DerefMut;
use core::slice;

use cortex_m::interrupt::{self, Mutex};

use crate::canned_sound::*;
use crate::dilly::*;
use crate::glep;
use crate::load::*;
use crate::spew::*;
use crate::patch::*;
use crate::playhead::*;

extern "C" {
    pub fn UnsafeDelay(delay_ms: u32);
}

pub fn delay(delay_ms: u32) {
    unsafe {
        UnsafeDelay(delay_ms);
    }
}

extern "C" {
    pub fn cpp_main() -> i32;
}

static THE_PATCH: Mutex<RefCell<Option<Rig>>> = Mutex::new(RefCell::new(None));

#[global_allocator]
static ALLOCATOR: emballoc::Allocator<32768> = emballoc::Allocator::new();

const DO_DILLY: bool = true;

enum PatchOrDilly {
    PODPatch(Box<dyn Patch>),
    PODDilly(Box<Dilly>),
}

pub struct Rig {
    //patch_or_dilly: Either<Box<Dilly>, Box<dyn Patch>>,
    patch_or_dilly: PatchOrDilly,
    inl: f32,
    inr: f32,
    outl: f32,
    outr: f32,
    framesize: usize,
    playhead: Playhead,
}

impl Rig {
    /*
    fn get_patch(&self) -> &Box<dyn Patch> {
        match self.patch_or_dilly {
            PatchOrDilly::PODPatch(dynPatch) => &dynPatch,
            PatchOrDilly::PODDilly(dilly) => &(dilly as Box<dyn Patch>),
        }
    }
    */

    fn dump_dilly_maybe(&mut self) {
        match &mut self.patch_or_dilly {
            PatchOrDilly::PODPatch(_patch) => {},
            PatchOrDilly::PODDilly(dilly) => {
                dilly.dump_maybe()
            }
        }
        /*
        if self.patch_or_dilly.is_left() {
            self.patch_or_dilly.left().dump_maybe();
        }
        */
    }
}

pub fn gogogo(box_patch: Box<dyn Patch>) -> i32 {
    // TODO
    // The audio handler must be installed AFTER this line.

    let patch_or_dilly: PatchOrDilly;
    if DO_DILLY {
        patch_or_dilly = PatchOrDilly::PODDilly(Box::new(Dilly::new(box_patch, Box::new(CANNED_SOUND_0))));
        //patch_or_dilly = Left(Box::new(Dilly::new(box_patch, canned_sound_0)));
    } else {
        patch_or_dilly = PatchOrDilly::PODPatch(box_patch);
        //patch_or_dilly = Right(box_patch);
    }

    let rig = Rig {
        patch_or_dilly: patch_or_dilly,
        inl: 0.0,
        inr: 0.0,
        outl: 0.0,
        outr: 0.0,
        framesize: 0,
        playhead : Playhead::new(),
    };
    interrupt::free(|cs| THE_PATCH.borrow(cs).replace(Some(rig)));
    unsafe { cpp_main() }
}

#[no_mangle]
pub extern "C" fn rust_process_audio_stub(
    in_ptr: *const *const f32,
    out_ptr: *const *mut f32,
    len: usize,
) {
    interrupt::free(|cs| {
        if let Some(ref mut rig) = THE_PATCH.borrow(cs).borrow_mut().deref_mut().as_mut() {
            // Mono pedal, so left_input_slice  is unused, except that we dump a value
            let left_input_slice = unsafe { slice::from_raw_parts(*(in_ptr.wrapping_add(0)), len) };
            let right_input_slice =
                unsafe { slice::from_raw_parts(*(in_ptr.wrapping_add(1)), len) };
            let left_output_slice =
                unsafe { slice::from_raw_parts_mut(*(out_ptr.wrapping_add(0)), len) };
            let right_output_slice =
                unsafe { slice::from_raw_parts_mut(*(out_ptr.wrapping_add(1)), len) };

            // TODO: Factor this into a helper.
            match &mut rig.patch_or_dilly {
                PatchOrDilly::PODPatch(patch) => {
                    patch
                        .rust_process_audio(right_input_slice, right_output_slice, rig.playhead);
                },
                PatchOrDilly::PODDilly(dilly) => {
                    //(*dilly as Box<dyn Patch>)
                    dilly
                        .rust_process_audio(right_input_slice, right_output_slice, rig.playhead);
                },
            }

            // Mono pedal, so copy right output to left output. Left and right outputs are mixed to
            // the analog mono out, so I'm told.
            left_output_slice.copy_from_slice(right_output_slice);

            rig.inl = left_input_slice[0];
            rig.inr = right_input_slice[0];
            rig.outl = left_output_slice[0];
            rig.outr = right_output_slice[0];
            rig.framesize = len;
            rig.playhead.increment_samples(len as u64);
        }
    });
}

#[no_mangle]
pub fn patch_main() {
    loop {
        let mut inl: f32 = 0.0;
        let mut inr: f32 = 0.0;
        let mut outl: f32 = 0.0;
        let mut outr: f32 = 0.0;
        let mut framesize: usize = 0;
        let mut playhead: Playhead = Playhead::new();

        interrupt::free(|cs| {
            if let Some(ref mut rig) = THE_PATCH.borrow(cs).borrow_mut().deref_mut().as_mut() {
                inl = rig.inl;
                inr = rig.inr;
                outl = rig.outl;
                outr = rig.outr;
                framesize = rig.framesize;
                playhead = rig.playhead;
                //rig.dump_dilly_maybe();
            }
        });

        glep!(inl, inr, outl, outr, framesize, playhead.time_in_samples(), playhead.time_in_seconds());

        show_load();
        delay(500);
    }
}
