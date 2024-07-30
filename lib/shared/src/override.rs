use alloc::boxed::Box;
//use alloc::vec::Vec;
use core::any::Any;
use core::cmp::min;

#[cfg(not(feature = "for_host"))]
use crate::daisy_seed::hw_delay;
use crate::filter::low_pass::*;
#[cfg(feature = "for_host")]
use crate::hw_host::hw_delay;
use crate::patch::*;
use crate::playhead::Playhead;
use crate::rig::*;
#[cfg(not(feature = "for_host"))]
use crate::rig_board::*;
#[cfg(feature = "for_host")]
use crate::rig_host::*;
//use crate::rig_type::*;
use crate::spew::*;
use crate::testdata::*;

// A patch containing its own input data and output data. It ignores the data coming in,
// pushes its own data through, checks the output against the expected data, and
// sets a result flag if the data is identical or not. The output slice is not written to.

pub struct Override<'a> {
    patch: Box<dyn Patch>,
    canned_input: &'a [f32],
    expected_output: &'a [f32],
    sofar: usize,
    done: bool,
    mismatches: usize,
}

impl <'a> Override<'a> {
    pub fn new(patch: Box<dyn Patch>, canned_input: &'a [f32], expected_output: &'a [f32]) -> Override<'a> {
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

impl <'a> Patch for Override<'a> {
    fn rust_process_audio(
        &mut self,
        input_slice: &[f32],
        output_slice: &mut [f32],
        playhead: Playhead,
    ) {
        assert!(self.sofar <= self.canned_input.len());
        assert!(self.sofar == self.canned_input.len() || !self.done);
        //let live_input_length = input_slice.len();
        let left_to_process = self.canned_input.len() - self.sofar;
        let process_this_round = min(left_to_process, input_slice.len());
        let sub_canned_input: &[f32] = &self.canned_input[self.sofar..(self.sofar+process_this_round)];
        let sub_expected_output: &[f32] = &self.expected_output[self.sofar..(self.sofar+process_this_round)];
        //let mut actual_output_vec: Vec<f32> = vec![0.0; process_this_round];
        //let mut actual_output: &mut [f32] = &mut actual_output_vec[..];
        let actual_output: &mut [f32] = &mut output_slice[0..process_this_round];
        self.patch.rust_process_audio(sub_canned_input, actual_output, playhead);
        for it in sub_expected_output.iter().zip(actual_output.iter()) {
            let (expected_sample, actual_sample) = it;
            if expected_sample != actual_sample {
                self.mismatches += 1;
            }
        }
        self.sofar += process_this_round;
    }

    //fn as_any<'a>(&self) -> &(dyn Any + 'a) { self }
    //fn as_any(&'a self) -> &'a dyn Any { self }
    fn as_any(&self) -> &'a (dyn Any + '_) { self }
    //fn as_any<'b>(&'b self) -> &'b dyn Any { self as &'b dyn Any }
}

// This does not check that there isn't a patch installed already.
// fn run_override(patch: &Patch, canned_input: &[f32], expected_output: &[f32]) {
// }

pub fn run_override_test() {
    //let patch = Box::new(LowPassFilter::new());
    //let canned_input = TEST_INPUT;
    //let expected_output = LOW_PASS_OUTPUT;
    //let overrid = Override::new(patch, canned_input, expected_output);
    /*
    let overrid = Override { sofar: 3 };
    rig_install_patch(Box::new(overrid));
    rig_use(|rig| {
        let patch: &Box<dyn Patch> = &rig.patch;
        let ov = patch.as_any().downcast_ref::<Override>();
        match ov {
            Some(ov) => {
                spew!(13, ov.sofar);
            }
            None => {
                spew!(14);
            }
        }
        spew!(12);
    });
    */

    let patch = Box::new(LowPassFilter::new());
    let canned_input = TEST_INPUT;
    let expected_output = LOW_PASS_OUTPUT;
    let r#override = Override::new(patch, canned_input, expected_output);
    spew!("hi override");

    rig_install_patch(Box::new(r#override));
    rig_install_callback();

    let /*mut*/ done: bool = false;
    let /*mut*/ passed: bool = false;
    while !done {
        rig_use(|rig| {
            let _patch: &Box<dyn Patch + '_> = &rig.patch;
            /*
            match patch.as_any().downcast_ref::<Override>() {
                Some(ov) => {
                    spew!("ov");
                    if ov.is_done() {
                        done = true;
                        passed = ov.passed();
                        spew!("done");
                    }
                }
                None => {
                    spew!("uhuh");
                }
            }
            */
        });
        //hw_delay(500);
        hw_delay(50);
    }
    spew!("final", passed);
}
