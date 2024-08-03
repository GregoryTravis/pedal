use alloc::boxed::Box;
use alloc::sync::Arc;
use alloc::vec::Vec;

use crate::filter::chorus::*;
use crate::filter::high_pass::*;
use crate::filter::linear_vibrato::*;
use crate::filter::low_pass::*;
use crate::filter::pass_thru::*;
use crate::filter::reso::*;
use crate::filter::sine::*;
use crate::filter::sweep::*;
use crate::filter::vibrato::*;
use crate::patch::Patch;
use crate::signal::base::*;
use crate::signal::combinators::*;
use crate::testdata::*;

pub struct TestCase {
    pub name: &'static str,
    pub patch: Box<dyn Patch>,
    pub canned_input: &'static [f32],
    pub expected_output: &'static [f32],
}

pub fn get_test_cases() -> Vec<Box<TestCase>> {
    let siner = PostCompose { signal: Arc::new(Sin {}), ff: scale_range(0.3, 0.9) };
    let q = Const { x: 0.95 };
    let sweep = Box::new(SweepFilter::new(Arc::new(siner), Arc::new(q)));

    vec![Box::new(TestCase {
            name: "low_pass",
            patch: Box::new(LowPassFilter::new()),
            canned_input: TEST_INPUT,
            expected_output: LOW_PASS_OUTPUT,
        }),
        Box::new(TestCase {
            name: "high_pass",
            patch: Box::new(HighPassFilter::new()),
            canned_input: TEST_INPUT,
            expected_output: HIGH_PASS_OUTPUT,
        }),
        Box::new(TestCase {
            name: "pass_thru",
            patch: Box::new(PassThruFilter::new()),
            canned_input: TEST_INPUT,
            expected_output: PASS_THRU_OUTPUT,
        }),
        Box::new(TestCase {
            name: "vibrato",
            patch: Box::new(Vibrato::new(10, 1.0)),
            canned_input: TEST_INPUT,
            expected_output: VIBRATO_OUTPUT,
        }),
        Box::new(TestCase {
            name: "linear_vibrato",
            patch: Box::new(LinearVibrato::new(10, 1.0)),
            canned_input: TEST_INPUT,
            expected_output: LINEAR_VIBRATO_OUTPUT,
        }),
        Box::new(TestCase {
            name: "chorus",
            patch: Box::new(Chorus::new()),
            canned_input: TEST_INPUT,
            expected_output: CHORUS_OUTPUT,
        }),
        Box::new(TestCase {
            name: "sine",
            patch: Box::new(SineGenerator::new(440.0)),
            canned_input: TEST_INPUT,
            expected_output: SINE_OUTPUT,
        }),
        Box::new(TestCase {
            name: "reso",
            patch: Box::new(ResoFilter::new()),
            canned_input: TEST_INPUT,
            expected_output: RESO_OUTPUT,
        }),
        Box::new(TestCase {
            name: "sweep",
            patch: sweep,
            canned_input: TEST_INPUT,
            expected_output: SWEEP_OUTPUT,
        }),
    ]
}
