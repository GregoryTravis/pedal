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

pub struct Rig {
    patch: Box<dyn Patch>,
    dilly: Option<Box<Dilly>>,
    inl: f32,
    inr: f32,
    outl: f32,
    outr: f32,
    framesize: usize,
    playhead: Playhead,
}

impl Rig {
    /*
    fn dump_dilly_maybe(&mut self) {
        match &mut self.dilly {
            Some(dilly) => {
                dilly.dump_maybe();
            },
            None => {},
        }
    }
    */
}

pub fn gogogo(box_patch: Box<dyn Patch>) -> i32 {
    // TODO
    // The audio handler must be installed AFTER this line.

    let dilly_maybe = if DO_DILLY { Some(Box::new(Dilly::new(Box::new(CANNED_SOUND_0)))) } else { None };

    let rig = Rig {
        patch: box_patch,
        dilly: dilly_maybe,
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
            match &mut rig.dilly {
                Some(dilly) => {
                    dilly.rust_process_audio(&mut rig.patch, right_input_slice, right_output_slice, rig.playhead);
                },
                None => {
                    rig.patch.rust_process_audio(right_input_slice, right_output_slice, rig.playhead);
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
        let mut dilly_maybe: Option<Box<Dilly>> = None;

        interrupt::free(|cs| {
            if let Some(ref mut rig) = THE_PATCH.borrow(cs).borrow_mut().deref_mut().as_mut() {
                inl = rig.inl;
                inr = rig.inr;
                outl = rig.outl;
                outr = rig.outr;
                framesize = rig.framesize;
                playhead = rig.playhead;
                match &mut rig.dilly {
                    Some(dilly) => {
                        if dilly.is_done() {
                            dilly_maybe = rig.dilly.take();
                        }
                    },
                    None => {},
                }
            }
        });

        glep!(inl, inr, outl, outr, framesize, playhead.time_in_samples(), playhead.time_in_seconds());
        
        match &mut dilly_maybe {
            Some(dilly) => {
                assert!(dilly.is_done());
                dilly.dump_maybe();
            },
            None => {},
        }

        show_load();
        delay(500);
    }
}
