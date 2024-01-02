extern crate alloc;

use alloc::boxed::Box;
use core::cell::RefCell;
use core::ops::DerefMut;
use core::slice;

use cortex_m::interrupt::{self, Mutex};

use crate::load::*;
use crate::spew::*;
use shared::patch::*;

use crate::glep;

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

pub struct Rig {
    patch: Box<dyn Patch>,
    inl: f32,
    inr: f32,
    outl: f32,
    outr: f32,
    framesize: usize,
    time_in_samples: u64,
}

pub fn gogogo(box_patch: Box<dyn Patch>) -> i32 {
    // The audio handler must be installed AFTER this line.
    // TODO is this use of get_patch() an unnecessary copy?
    let rig = Rig {
        patch: box_patch,
        time_in_samples: 0,
        inl: 0.0,
        inr: 0.0,
        outl: 0.0,
        outr: 0.0,
        framesize: 0,
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
            //let ilen = len as isize;
            let time_in_seconds: f64 = (rig.time_in_samples as f64) / 48000.0;

            // Mono pedal, so left_input_slice  is unused, except that we dump a value
            let left_input_slice = unsafe { slice::from_raw_parts(*(in_ptr.wrapping_add(0)), len) };
            let right_input_slice =
                unsafe { slice::from_raw_parts(*(in_ptr.wrapping_add(1)), len) };
            let left_output_slice =
                unsafe { slice::from_raw_parts_mut(*(out_ptr.wrapping_add(0)), len) };
            let right_output_slice =
                unsafe { slice::from_raw_parts_mut(*(out_ptr.wrapping_add(1)), len) };

            rig.patch
                .rust_process_audio(right_input_slice, right_output_slice, time_in_seconds);
            left_output_slice.copy_from_slice(right_output_slice);

            // Mono pedal, so copy right to left

            rig.inl = left_input_slice[0];
            rig.inr = right_input_slice[0];
            rig.outl = left_output_slice[0];
            rig.outr = right_output_slice[0];
            rig.framesize = len;
            rig.time_in_samples += len as u64;
        }
    });
}

#[no_mangle]
pub fn patch_main() {
    loop {
        let mut time_in_samples = 0;
        let mut inl: f32 = 0.0;
        let mut inr: f32 = 0.0;
        let mut outl: f32 = 0.0;
        let mut outr: f32 = 0.0;
        let mut framesize: usize = 0;

        interrupt::free(|cs| {
            if let Some(ref mut rig) = THE_PATCH.borrow(cs).borrow_mut().deref_mut().as_mut() {
                time_in_samples = rig.time_in_samples;
                inl = rig.inl;
                inr = rig.inr;
                outl = rig.outl;
                outr = rig.outr;
                framesize = rig.framesize;
            }
        });

        glep!(inl, inr, outl, outr, framesize, time_in_samples);

        show_load();
        delay(500);
    }
}
