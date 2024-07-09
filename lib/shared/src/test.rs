extern crate alloc;

use alloc::boxed::Box;
use alloc::sync::Arc;
use alloc::vec::Vec;

use crate::filter::reso::*;
use crate::patch::Patch;
use crate::rig_util::*;
use crate::signal::base::*;
use crate::signal::combinators::*;
use crate::spew::*;
use crate::testdata::*;

/*
 * Unit tests for patches.
 */

fn same(a: &[f32], b: &[f32]) -> bool {
    assert!(a.len() == b.len());
    for i in 0..a.len() {
        if a[i] != b[i] {
            return false;
        }
    }
    return true;
}

fn sum(a: &[f32]) -> f32 {
    let mut sum: f32 = 0.0;
    for i in 0..a.len() {
        sum += a[i];
    }
    sum
}

fn test_patch(patch: Box<dyn Patch>, expected_output: &[f32]) {
    let mut output: Vec<f32> = vec![0.0; TEST_INPUT.len()];
    rig_run_patch_on_buffer(patch, &TEST_INPUT, &mut output);

    assert!(same(&output, expected_output));
    let chk = sum(&output);
    spew!("reso: ok", chk, chk.to_bits());
}

pub fn test_reso() {
    let siner = PostCompose { signal: Arc::new(Sin {}), ff: scale_range(0.3, 0.9) };
    let q = Const { x: 0.95 };
    let reso = Box::new(ResoFilter::new(Arc::new(siner), Arc::new(q)));
    test_patch(reso, &RESO_OUTPUT);
}
