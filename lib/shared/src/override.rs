use alloc::boxed::Box;
//use alloc::vec::Vec;
use core::any::Any;
use core::cmp::min;

#[cfg(not(feature = "for_host"))]
use crate::daisy_seed_board::hw_delay;
#[cfg(feature = "for_host")]
use crate::hw_host::hw_delay;
use crate::knob::Knobs;
use crate::knob_dummy::DummyKnobs;
use crate::patch::*;
use crate::playhead::Playhead;
use crate::rig::*;
#[cfg(not(feature = "for_host"))]
use crate::rig_board::*;
#[cfg(feature = "for_host")]
use crate::rig_host::*;
//use crate::rig_type::*;
use crate::spew::*;
use crate::switch::Toggle;
use crate::switch_dummy::DummySwitches;
use crate::test_cases::*;

// A patch containing its own input data and output data. It ignores the data coming in,
// pushes its own data through, checks the output against the expected data, and
// sets a result flag if the data is identical or not. The output slice is not written to.

pub struct Override {
    patch: Box<dyn Patch>,
    canned_input: &'static [f32],
    expected_output: &'static [f32],
    sofar: usize,
    done: bool,
    mismatches: usize,
}

impl Override {
    pub fn new(patch: Box<dyn Patch>, canned_input: &'static [f32], expected_output: &'static [f32]) -> Override {
        assert!(canned_input.len() == expected_output.len());
        Override {
            patch: patch,
            canned_input: canned_input,
            expected_output: expected_output,
            sofar: 0,
            done: false,
            mismatches: 0,
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
        output_slice: &mut [f32],
        knobs: &Box<dyn Knobs>,
        playhead: Playhead,
    ) {
        assert!(self.sofar <= self.canned_input.len());
        assert!(self.sofar == self.canned_input.len() || !self.done);
        let left_to_process = self.canned_input.len() - self.sofar;

        assert!(left_to_process <= self.canned_input.len());

        if left_to_process == 0 {
            self.done = true;
            return;
        }

        let process_this_round = min(left_to_process, input_slice.len());
        let sub_canned_input: &[f32] = &self.canned_input[self.sofar..(self.sofar+process_this_round)];
        let sub_expected_output: &[f32] = &self.expected_output[self.sofar..(self.sofar+process_this_round)];
        let actual_output: &mut [f32] = &mut output_slice[0..process_this_round];
        self.patch.rust_process_audio(sub_canned_input, actual_output, knobs, playhead);
        for it in sub_expected_output.iter().zip(actual_output.iter()) {
            let (expected_sample, actual_sample) = it;
            if expected_sample != actual_sample {
                self.mismatches += 1;
            }
        }
        self.sofar += process_this_round;
    }

    fn done(&self) -> bool {
        self.done
    }

    fn passed(&self) -> bool {
        self.mismatches == 0
    }

    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}

pub fn run_override_test() {
    for test_case in get_test_cases() {
        let patch = test_case.patch;
        let canned_input = test_case.canned_input;
        let expected_output = test_case.expected_output;
        let r#override = Override::new(patch, canned_input, expected_output);

        rig_install_patch(Box::new(r#override), Box::new(DummyKnobs { }), Toggle::new(Box::new(DummySwitches { }), 0));
        rig_install_callback();

        let mut done: bool = false;
        let mut passed: bool = false;
        while !done {
            THE_PATCH.use_it(|rig| {
                let patch: &Box<dyn Patch + '_> = &rig.patch;
                if patch.done() {
                    done = true;
                    passed = patch.passed();
                }
            });
            hw_delay(5);
        }
        spew!("override", test_case.name, passed);
    }
}
