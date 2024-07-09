extern crate alloc;

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
//use crate::filter::speed_test::*;
use crate::filter::vibrato::*;
use crate::patch::*;
use crate::rig_util::*;
use crate::signal::base::*;
use crate::signal::combinators::*;
use crate::spew::*;
use crate::testdata::*;
use crate::testdump::*;
use crate::testutil::*;

/*
 * Unit tests for patches.
 */

const DO_DUMP: bool = false;

fn test_patch(name: &str, patch: Box<dyn Patch>, expected_output: &[f32]) {
    let mut output: Vec<f32> = vec![0.0; TEST_INPUT.len()];
    rig_run_patch_on_buffer(patch, &TEST_INPUT, &mut output);

    if DO_DUMP {
        test_dump_as_source(&(name.to_ascii_uppercase().clone() + "_OUTPUT"), &output);
    } else {
        assert!(same(&output, expected_output));
        let chk = sum(&output);
        spew!("ok", name, chk, chk.to_bits());
    }
}

pub fn test_reso() {
    let siner = PostCompose { signal: Arc::new(Sin {}), ff: scale_range(0.3, 0.9) };
    let q = Const { x: 0.95 };
    let reso = Box::new(ResoFilter::new(Arc::new(siner), Arc::new(q)));

    if DO_DUMP {
        test_dump_as_source("TEST_INPUT", &TEST_INPUT);
    }

    test_patch("low_pass", Box::new(LowPassFilter::new()), LOW_PASS_OUTPUT);
    test_patch("high_pass", Box::new(HighPassFilter::new()), HIGH_PASS_OUTPUT);
    test_patch("pass_thru", Box::new(PassThruFilter::new()), PASS_THRU_OUTPUT);
    test_patch("vibrato", Box::new(Vibrato::new(400, 1.0)), VIBRATO_OUTPUT);
    test_patch("linear_vibrato", Box::new(LinearVibrato::new(400, 1.0)), LINEAR_VIBRATO_OUTPUT);
    //test_patch("speed_test", Box::new(SpeedTest::new()), SPEED_TEST_OUTPUT);
    test_patch("chorus", Box::new(Chorus::new()), CHORUS_OUTPUT);
    test_patch("sine", Box::new(SineGenerator::new()), SINE_OUTPUT);
    test_patch("reso", reso, &RESO_OUTPUT);
}
