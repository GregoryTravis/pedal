extern crate alloc;

use alloc::boxed::Box;
use core::cmp::min;

use crate::constants::*;
use crate::knob_dummy::DummyKnobs;
use crate::patch::Patch;
use crate::rig::*;
use crate::switch::Toggle;
use crate::switch_dummy::DummySwitches;

pub fn rig_run_patch_on_buffer(patch: Box<dyn Patch>, input: &[f32], output: &mut [f32]) {
    let len = input.len();

    rig_install_patch(patch, Box::new(DummyKnobs { }), Toggle::new(Box::new(DummySwitches { }), 0));

    let mut sofar = 0;
    while sofar < len {
        let start = sofar;
        let end = min(sofar + BLOCK_SIZE, len);
        let block_length = end-start;
        let sub_input = &input[start..end];
        let mut sub_output: &mut [f32] = &mut output[start..end];

        rust_process_audio_soft(&sub_input, &mut sub_output, block_length);

        sofar += BLOCK_SIZE;
    }

    rig_deinstall_patch();
}
