use core::cmp::min;

use crate::patch::*;
use crate::rig::*;
use crate::rig_type::*;

// A patch containing its own input data and output data. It ignores the data coming in,
// pushes its own data through, checks the output against the expected data, and
// sets a result flag if the data is identical or not. The output slice is not written to.

pub struct Override<'a> {
    patch: Box<dyn Patch>,
    canned_input: &'a [f32],
    expected_output: &'a [f32];
    sofar: usize,
    done: bool,
    mismatches: usize,
}

impl Override {
    pub fn new(patch: &Patch, canned_input: &[f32], expected_output: &[f32]) {
        assert!(canned_input.len() == expected_output.len());
        Override {
            patch: patch,
            canned_input: canned_input,
            expected_output: expected_output,
            sofar: 0,
            done: false,
            mismatches: 0;
        }
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn passed(&self) -> bool {
        self.done && (!self.mismatches == 0)
    }
}

impl Patch for Override {
    fn rust_process_audio(
        &mut self,
        input_slice: &[f32],
        _output_slice: &mut [f32],
        mut playhead: Playhead,
    ) {
        assert!(sofar <= self.canned_input);
        assert!(sofar == self.canned_input || !self.done);
        let live_input_length = input_slice.len();
        let left_to_process = canned_input.len() - sofar;
        let process_this_round = min(left_to_process, input_slice.len());
        let sub_canned_input: &[f32] = &canned_input[sofar..(sofar+process_this_round)];
        let sub_expected_output: &[f32] = &expected_output[sofar..(sofar+process_this_round)];
        let actual_output: Vec<f32> = vec![0.0, process_this_round];
        patch.rust_process_audio(sub_canned_input, actual_output, playhead);
        for it in sub_expected_output.iter().zip(actual_output.iter()) {
            let (expected_sample, actual_sample) = it;
            if expected_sammple != actual_sample {
                self.mistmatches += 1;
            }
        }
        sofar += process_this_round;
    }
}

// This does not check that there isn't a patch installed already.
fn run_override(patch: &Patch, canned_input: &[f32], expected_output: &[f32]) {
}

pub fn run_override_test() {
    let override = Override::new(patch, canned_input, expected_output);
    spew!("hi override");

    rig_install_patch(Box::new(override));
    rig_install_callback();

    let mut done: bool = false;
    let mut passed: bool = false;
    loop {
        rig_use(|rig| {
            if rig.done()
        });
        hw_delay(500);
    }

}
