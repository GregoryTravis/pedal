extern crate alloc;

use alloc::boxed::Box;
use alloc::sync::Arc;
use alloc::vec::Vec;

use crate::filter::reso::*;
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

pub fn test_reso() {
    let siner = PostCompose { signal: Arc::new(Sin {}), ff: scale_range(0.3, 0.9) };
    let q = Const { x: 0.95 };
    let reso = ResoFilter::new(Arc::new(siner), Arc::new(q));
    let reso_box = Box::new(reso);
    let mut output: Vec<f32> = vec![0.0; TEST_INPUT.len()];
    rig_run_patch_on_buffer(reso_box, &TEST_INPUT, &mut output);

    let _same: bool = same(&output, &RESO_OUTPUT);
    assert!(same(&output, &RESO_OUTPUT));
    let chk = sum(&output);
    spew!("reso: ok", chk, chk.to_bits());
}
