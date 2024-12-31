use alloc::boxed::Box;
use alloc::sync::Arc;
use alloc::vec::Vec;

use crate::constants::*;
use crate::filter::chorus::*;
use crate::filter::delay::*;
use crate::filter::edsl_nodey::*;
use crate::filter::edsl_high_pass::*;
use crate::filter::edsl_low_pass::*;
use crate::filter::edsl_low_pass_6::*;
use crate::filter::edsl_pass_thru::*;
use crate::filter::harmoneer::*;
use crate::filter::high_pass::*;
use crate::filter::linear_vibrato::*;
use crate::filter::low_pass::*;
use crate::filter::pass_thru::*;
use crate::filter::reso::*;
use crate::filter::seq::*;
use crate::filter::sine::*;
use crate::filter::sweep::*;
use crate::filter::vibrato::*;
use crate::rubin::*;
use crate::sdram::*;
use crate::signal::base::*;
use crate::signal::combinators::*;
use crate::test::*;
use crate::testdata::*;

pub fn get_test_cases() -> Vec<Box<TestCase>> {
    let siner = PostCompose { signal: Arc::new(Sin {}), ff: scale_range(0.3, 0.9) };
    let q = Const { x: 0.95 };
    let sweep = Box::new(SweepFilter::new(Arc::new(siner), Arc::new(q)));

    let mut sdram = SDRAM::new();

    let lp6 = {
        let lp = Box::new(LowPassFilter::new());
        let lp2 = Box::new(LowPassFilter::new());
        let lp3 = Box::new(LowPassFilter::new());
        let lp4 = Box::new(LowPassFilter::new());
        let lp5 = Box::new(LowPassFilter::new());
        let lp6 = Box::new(LowPassFilter::new());
        let seq0 = Box::new(Seq::new(BLOCK_SIZE, lp, lp2));
        let seq1 = Box::new(Seq::new(BLOCK_SIZE, seq0, lp3));
        let seq2 = Box::new(Seq::new(BLOCK_SIZE, seq1, lp4));
        let seq3 = Box::new(Seq::new(BLOCK_SIZE, seq2, lp5));
        let seq4 = Box::new(Seq::new(BLOCK_SIZE, seq3, lp6));
        seq4
    };

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
            patch: Box::new(LinearVibrato::new(10, 1.0, 0)),
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
            patch: Box::new(ResoFilter::new(0, 0)),
            canned_input: TEST_INPUT,
            expected_output: RESO_OUTPUT,
        }),
        Box::new(TestCase {
            name: "sweep",
            patch: sweep,
            canned_input: TEST_INPUT,
            expected_output: SWEEP_OUTPUT,
        }),
        Box::new(TestCase {
            name: "delay",
            patch: Box::new(Delay::new()),
            canned_input: TEST_INPUT,
            expected_output: DELAY_OUTPUT,
        }),
        Box::new(TestCase {
            name: "harmoneer",
            patch: Box::new(Harmoneer::new(1.74, &mut sdram)),
            canned_input: LONG_TEST_INPUT,
            expected_output: HARMONEER_OUTPUT,
        }),
        Box::new(TestCase {
            name: "rubin",
            patch: rubin(&mut sdram),
            canned_input: TEST_INPUT,
            expected_output: RUBIN_OUTPUT,
        }),
        Box::new(TestCase {
            name: "edsl_nodey",
            patch: Box::new(EdslNodey::new()),
            canned_input: EDSL_NODEY_INPUT,
            expected_output: EDSL_NODEY_OUTPUT,
        }),
        Box::new(TestCase {
            name: "edsl_high_pass",
            patch: Box::new(EdslHighPass::new()),
            canned_input: TEST_INPUT,
            expected_output: HIGH_PASS_OUTPUT,
        }),
        Box::new(TestCase {
            name: "edsl_low_pass",
            patch: Box::new(EdslLowPass::new()),
            canned_input: TEST_INPUT,
            expected_output: LOW_PASS_OUTPUT,
        }),
        Box::new(TestCase {
            name: "edsl_pass_thru",
            patch: Box::new(EdslPassThru::new()),
            canned_input: TEST_INPUT,
            expected_output: PASS_THRU_OUTPUT,
        }),
        Box::new(TestCase {
            name: "low_pass_6",
            patch: lp6,
            canned_input: TEST_INPUT,
            expected_output: EDSL_LOW_PASS_6_OUTPUT,
        }),
        Box::new(TestCase {
            name: "edsl_low_pass_6",
            patch: Box::new(EdslLowPass6::new()),
            canned_input: TEST_INPUT,
            expected_output: EDSL_LOW_PASS_6_OUTPUT,
        }),
    ]
}
