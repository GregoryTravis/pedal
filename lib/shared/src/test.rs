extern crate alloc;

use alloc::boxed::Box;
use alloc::vec::Vec;

use crate::patch::*;
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
    }

    assert!(same(expected_output, &output));
    let chk = sum(&output);
    spew!("ok", name, chk, chk.to_bits());
}
