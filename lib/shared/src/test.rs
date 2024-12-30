extern crate alloc;

use alloc::boxed::Box;
use alloc::vec::Vec;
use core::cmp::min;

use crate::constants::*;
use crate::knob::Knobs;
use crate::knob_dummy::DummyKnobs;
use crate::patch::*;
use crate::playhead::Playhead;
use crate::rig_util::*;
use crate::spew::*;
#[cfg(feature = "for_host")]
use crate::testdump::*;
use crate::testutil::*;

/*
 * Unit tests for patches.
 */

pub const DO_DUMP: bool = false;

pub struct TestCase {
    pub name: &'static str,
    pub patch: Box<dyn Patch>,
    pub canned_input: &'static [f32],
    pub expected_output: &'static [f32],
}

#[cfg(feature = "for_host")]
pub fn local_test_dump_as_source(var: &str, a: &[f32]) {
    test_dump_as_source(var, a);
}

#[cfg(not(feature = "for_host"))]
pub fn local_test_dump_as_source(_var: &str, _a: &[f32]) {
}

pub fn test_patch(name: &str, patch: Box<dyn Patch>, canned_input: &[f32], expected_output: &[f32]) {
    let mut output: Vec<f32> = vec![0.0; canned_input.len()];
    rig_run_patch_on_buffer(patch, &canned_input, &mut output);

    if DO_DUMP {
        local_test_dump_as_source(&(name.to_ascii_uppercase().clone() + "_OUTPUT"), &output);
    } else {
        assert!(same(expected_output, &output));
        let chk = sum(&output);
        spew!("ok", name, chk, chk.to_bits());
    }
}

// This is for benchmarking. It does not use rig because that requires taking ownership of the
// patch, and that can't be done in a loop, it can only be done once. This is similar logic to
// rig_run_patch_on_buffer.
pub fn run_patch_direct(patch: &mut Box<dyn Patch>, canned_input: &[f32]) {
    let mut output: Vec<f32> = vec![0.0; canned_input.len()];
    let len = canned_input.len();

    let mut sofar = 0;
    let playhead: Playhead = Playhead::new();
    let knobs: Box<dyn Knobs> = Box::new(DummyKnobs { });

    while sofar < len {
        let start = sofar;
        let end = min(sofar + BLOCK_SIZE, len);
        //let block_length = end-start;
        let sub_input = &canned_input[start..end];
        let mut sub_output: &mut [f32] = &mut output[start..end];
        //spew!("oy", sofar, len, start, end, end-start, sub_input.len(), sub_output.len());

        patch.rust_process_audio(&sub_input, &mut sub_output, &knobs, playhead);

        sofar += BLOCK_SIZE;
    }
}
