//extern crate std;
extern crate alloc;
extern crate lazy_static;

use alloc::boxed::Box;
use alloc::sync::Arc;
//use alloc::vec::Vec;
//use core::f32::consts::PI;
//use lazy_static::lazy_static;
//use std::println;
//use std::vec;

//use crate::constants::*;
use crate::filter::reso::*;
//use crate::patch::Patch;
//use crate::rig::*;
use crate::signal::base::*;
use crate::signal::combinators::*;
use crate::sim::sim_run_patch_on_buffer;
use crate::testdata::*;
#[allow(unused_imports)]
use crate::testdump::*;

/*
 * Unit tests for patches.
 */

//const TEST_LEN: usize = 100;

/*
pub struct TestData {
    input: Vec<f32>,
    output: Vec<f32>,
}
*/

/*
lazy_static! {
    static ref RESO_OUTPUT: Vec<f32> = vec![0.0; TEST_LEN];
}
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

pub fn test_reso() {
    /*
    let mut input = vec![0.0; TEST_LEN];

    for i in 0..TEST_LEN {
        let freq: f32 = 440.0;
        let t = (i as f32) / (SAMPLE_RATE as f32);
        let ph = t * 2.0 * PI * freq;
        input[i] = libm::sinf(ph);
    }

    test_dump_as_source("TEST_INPUT", &input);
    */

    let siner = PostCompose { signal: Arc::new(Sin {}), ff: scale_range(0.3, 0.9) };
    let q = Const { x: 0.95 };
    let reso = ResoFilter::new(Arc::new(siner), Arc::new(q));
    let reso_box = Box::new(reso);
    let output = sim_run_patch_on_buffer(reso_box, &TEST_INPUT);

    //test_dump_as_source("RESO_OUTPUT", &output);

    assert!(same(&output, &RESO_OUTPUT));
}
