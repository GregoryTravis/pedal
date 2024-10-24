extern crate alloc;

use alloc::boxed::Box;
//use core::ops::DerefMut;
use core::slice;

use crate::knob::Knobs;
#[cfg(not(feature = "for_host"))]
use crate::load_board::*;
#[cfg(feature = "for_host")]
use crate::load_host::*;
use crate::spew::*;
use crate::patch::*;
use crate::playhead::*;
#[cfg(not(feature = "for_host"))]
use crate::rig_board::*;
#[cfg(feature = "for_host")]
use crate::rig_host::*;
use crate::rig_type::Rig;
use crate::switch::Switches;

pub fn rig_install_patch(box_patch: Box<dyn Patch>, knobs: Box<dyn Knobs>, switches: Box<dyn Switches>) {
    // The audio handler must be installed AFTER this line.
    // TODO is this use of get_patch() an unnecessary copy?
    let rig = Rig {
        patch: box_patch,
        knobs: knobs,
        switches: switches,
        inl: 0.0,
        inr: 0.0,
        outl: 0.0,
        outr: 0.0,
        framesize: 0,
        playhead : Playhead::new(),
    };
    THE_PATCH.set(rig);
}

pub fn rig_deinstall_patch() {
    THE_PATCH.clear();
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
    // TODO why are we taking a len here
    len: usize) {
    // Create dummy left channel arrays
    let right_in_array = vec![0.0f32; len];
    let mut right_out_array = vec![0.0f32; len];
    let right_in_array_slice: &[f32] = &right_in_array;
    let right_out_array_slice: &mut [f32] = &mut right_out_array;
    let right_in_ptr: *const f32 = right_in_array_slice.as_ptr();
    let right_out_ptr: *mut f32 = right_out_array_slice.as_mut_ptr();

    let left_in_ptr: *const f32 = input_slice.as_ptr();
    let left_out_ptr: *mut f32 = output_slice.as_mut_ptr();

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
    load_before();
    THE_PATCH.use_it(|rig| {
        // Mono pedal, so left_input_slice  is unused, except that we dump a value
        let left_input_slice = unsafe { slice::from_raw_parts(*(in_ptr.wrapping_add(0)), len) };
        let right_input_slice =
            unsafe { slice::from_raw_parts(*(in_ptr.wrapping_add(1)), len) };
        let left_output_slice =
            unsafe { slice::from_raw_parts_mut(*(out_ptr.wrapping_add(0)), len) };
        let right_output_slice =
            unsafe { slice::from_raw_parts_mut(*(out_ptr.wrapping_add(1)), len) };

        rig.knobs.process();
        rig.switches.process();

        // I don't know what the convention is, but to get this to work in mono, I have to process
        // the left channel, and copy to the right channel (except on the original purple pedal?)
        // Left and right outputs are mixed to the analog mono out, so I'm told.
        rig.patch
            .rust_process_audio(left_input_slice, left_output_slice, &rig.knobs, &rig.switches, rig.playhead);
        right_output_slice.copy_from_slice(left_output_slice);

        rig.inl = left_input_slice[0];
        rig.inr = right_input_slice[0];
        rig.outl = left_output_slice[0];
        rig.outr = right_output_slice[0];
        rig.framesize = len;
        rig.playhead.increment_samples(len as u32);
    });
    load_after();
}

pub fn rig_log() {
    let mut inl: f32 = 0.0;
    let mut inr: f32 = 0.0;
    let mut outl: f32 = 0.0;
    let mut outr: f32 = 0.0;
    let mut framesize: usize = 0;
    let mut playhead: Playhead = Playhead::new();

    THE_PATCH.use_it(|rig| {
        inl = rig.inl;
        inr = rig.inr;
        outl = rig.outl;
        outr = rig.outr;
        framesize = rig.framesize;
        playhead = rig.playhead;
    });

    spew!(inl, inr, outl, outr, framesize, playhead.time_in_samples(), playhead.time_in_seconds());
}
