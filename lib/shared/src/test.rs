extern crate alloc;

use alloc::boxed::Box;
use alloc::vec::Vec;

use crate::patch::*;
use crate::rig_util::*;
use crate::spew::*;
use crate::testdata::*;
use crate::test_cases::*;
#[cfg(feature = "for_host")]
use crate::testdump::*;
use crate::testutil::*;

/*
 * Unit tests for patches.
 */

const DO_DUMP: bool = false;

#[cfg(feature = "for_host")]
fn local_test_dump_as_source(var: &str, a: &[f32]) {
    test_dump_as_source(var, a);
}

#[cfg(not(feature = "for_host"))]
fn local_test_dump_as_source(_var: &str, _a: &[f32]) {
}

fn test_patch(name: &str, patch: Box<dyn Patch>, canned_input: &[f32], expected_output: &[f32]) {
    let mut output: Vec<f32> = vec![0.0; canned_input.len()];
    rig_run_patch_on_buffer(patch, &canned_input, &mut output);

    if DO_DUMP {
        local_test_dump_as_source(&(name.to_ascii_uppercase().clone() + "_OUTPUT"), &output);
    } else {
        assert!(same(&output, expected_output));
        let chk = sum(&output);
        spew!("ok", name, chk, chk.to_bits());
    }
}

pub fn test_direct() {
    if DO_DUMP {
        local_test_dump_as_source("TEST_INPUT", &TEST_INPUT);
        local_test_dump_as_source("LONG_TEST_INPUT", &LONG_TEST_INPUT);
    }

    for test_case in get_test_cases() {
        let patch = test_case.patch;
        let canned_input = test_case.canned_input;
        let expected_output = test_case.expected_output;
        //spew!(test_case.name);
        test_patch(test_case.name, patch, canned_input, expected_output);
    }
}
