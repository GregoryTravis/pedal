extern crate alloc;

use alloc::boxed::Box;
//use core::ops::DerefMut;
use core::slice;

//use crate::load::*;
use crate::spew::*;
use crate::patch::*;
use crate::playhead::*;
#[cfg(not(feature = "for_host"))]
use crate::rig_board::*;
#[cfg(feature = "for_host")]
use crate::rig_host::*;
use crate::rig_type::Rig;

use crate::spew;

extern "C" {
    pub fn cpp_rig_install_callback();
}

#[global_allocator]
static ALLOCATOR: emballoc::Allocator<32768> = emballoc::Allocator::new();

pub fn rig_install_patch(box_patch: Box<dyn Patch>) {
    // The audio handler must be installed AFTER this line.
    // TODO is this use of get_patch() an unnecessary copy?
    let rig = Rig {
        patch: box_patch,
        inl: 0.0,
        inr: 0.0,
        outl: 0.0,
        outr: 0.0,
        framesize: 0,
        playhead : Playhead::new(),
    };
    rig_set(rig);
}

pub fn rig_deinstall_patch() {
    rig_clear();
}

pub fn rig_install_callback() {
    unsafe {
        cpp_rig_install_callback();
    }
}

// This simulates what the Daisy Seed passes to the audio callback.
//
// (libDaisy/src/hid/audio.h)
// Non-Interleaving output buffer
// Arranged by float[chn][sample]
// Left 0, Right 1
// The mono pedal is right only
// typedef const float* const* InputBuffer;
// typedef float** OutputBuffer;
pub fn rust_process_audio_soft(
    input_slice: &[f32],
    output_slice: &mut [f32],
    len: usize) {
    // Create dummy left channel arrays
    let left_in_array = vec![0.0f32; len];
    let mut left_out_array = vec![0.0f32; len];
    let left_in_array_slice: &[f32] = &left_in_array;
    let left_out_array_slice: &mut [f32] = &mut left_out_array;
    let left_in_ptr: *const f32 = left_in_array_slice.as_ptr();
    let left_out_ptr: *mut f32 = left_out_array_slice.as_mut_ptr();

    let right_in_ptr: *const f32 = input_slice.as_ptr();
    let right_out_ptr: *mut f32 = output_slice.as_mut_ptr();

    let in_pointer_array: [*const f32; 2] = [left_in_ptr, right_in_ptr];
    let out_pointer_array: [*mut f32; 2] = [left_out_ptr, right_out_ptr];

    let in_ptr: *const *const f32 = in_pointer_array.as_ptr();
    let out_ptr: *const *mut f32 = out_pointer_array.as_ptr();

    rig_process_audio_callback(in_ptr, out_ptr, len);
}

#[no_mangle]
pub extern "C" fn rig_process_audio_callback(
    in_ptr: *const *const f32,
    out_ptr: *const *mut f32,
    len: usize) {
    //load_before();
    rig_use(|rig| {
        // Mono pedal, so left_input_slice  is unused, except that we dump a value
        let left_input_slice = unsafe { slice::from_raw_parts(*(in_ptr.wrapping_add(0)), len) };
        let right_input_slice =
            unsafe { slice::from_raw_parts(*(in_ptr.wrapping_add(1)), len) };
        let left_output_slice =
            unsafe { slice::from_raw_parts_mut(*(out_ptr.wrapping_add(0)), len) };
        let right_output_slice =
            unsafe { slice::from_raw_parts_mut(*(out_ptr.wrapping_add(1)), len) };

        rig.patch
            .rust_process_audio(right_input_slice, right_output_slice, rig.playhead);

        // Mono pedal, so copy right output to left output. Left and right outputs are mixed to
        // the analog mono out, so I'm told.
        left_output_slice.copy_from_slice(right_output_slice);

        rig.inl = left_input_slice[0];
        rig.inr = right_input_slice[0];
        rig.outl = left_output_slice[0];
        rig.outr = right_output_slice[0];
        rig.framesize = len;
        rig.playhead.increment_samples(len as u32);
    });
    //load_after();
}

pub fn rig_log() {
    let mut inl: f32 = 0.0;
    let mut inr: f32 = 0.0;
    let mut outl: f32 = 0.0;
    let mut outr: f32 = 0.0;
    let mut framesize: usize = 0;
    let mut playhead: Playhead = Playhead::new();

    rig_use(|rig| {
        inl = rig.inl;
        inr = rig.inr;
        outl = rig.outl;
        outr = rig.outr;
        framesize = rig.framesize;
        playhead = rig.playhead;
    });

    spew!(inl, inr, outl, outr, framesize, playhead.time_in_samples(), playhead.time_in_seconds());
}
