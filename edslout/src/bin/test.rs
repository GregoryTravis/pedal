extern crate alloc;

use alloc::boxed::Box;
use alloc::vec::Vec;

use edslout::edsl_nodey::*;
use edslout::edsl_const::*;
use edslout::edsl_high_pass::*;
use edslout::edsl_low_pass::*;
use edslout::edsl_low_pass_6::*;
use edslout::edsl_pass_thru::*;
use edslout::edsl_linear_vibrato::*;
use edslout::testdata::*;

use shared::constants::*;
use shared::filter::low_pass::*;
use shared::filter::seq::*;
use shared::test::*;

pub fn get_test_cases() -> Vec<Box<TestCase>> {
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
        Box::new(TestCase {
            name: "const",
            patch: Box::new(EdslConst::new()),
            canned_input: CONST_TEST_INPUT,
            expected_output: CONST_TEST_OUTPUT,
        }),
        Box::new(TestCase {
            name: "linear_vibrato",
            patch: Box::new(EdslLinearVibrato::new()),
            canned_input: TEST_INPUT,
            expected_output: LINEAR_VIBRATO_OUTPUT,
        }),
        ]
}

/*
 * Unit tests for patches.
 */

pub fn main() {
    if DO_DUMP {
        local_test_dump_as_source("TEST_INPUT", &TEST_INPUT);
        local_test_dump_as_source("EDSL_NODEY_INPUT", EDSL_NODEY_INPUT);
    }

    for test_case in get_test_cases() {
        let patch = test_case.patch;
        let canned_input = test_case.canned_input;
        let expected_output = test_case.expected_output;
        //spew!(test_case.name);
        test_patch(test_case.name, patch, canned_input, expected_output);
    }
}
