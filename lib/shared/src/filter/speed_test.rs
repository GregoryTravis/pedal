extern crate alloc;
extern crate libm;
#[cfg(feature = "for_host")]
extern crate std;

use crate::patch::Patch;
use crate::playhead::Playhead;

//#[cfg(feature = "for_host")]
//use std::println;

const DOT_SIZE: usize = 48;
const LESS: usize = 0;
const NROUNDS: u32 = 400;

#[allow(dead_code)]
pub struct SpeedTest {
    fys: [f32; DOT_SIZE],
    dys: [f64; DOT_SIZE],
}

#[allow(dead_code)]
fn ddot(speed_test: &SpeedTest, input_slice: &[f32], output_slice: &mut [f32]) {
    for i in 0..(DOT_SIZE-LESS) {
        output_slice[i] = ((input_slice[i] as f64) * speed_test.dys[i]) as f32;
    }
}

#[allow(dead_code)]
fn dfdot(speed_test: &SpeedTest, input_slice: &[f32], output_slice: &mut [f32]) {
    for i in 0..(DOT_SIZE-LESS) {
        output_slice[i] = ((input_slice[i] as f64) * (speed_test.fys[i] as f64)) as f32;
    }
}

// fdot but it uses dys anyway
#[allow(dead_code)]
fn fddot(speed_test: &SpeedTest, input_slice: &[f32], output_slice: &mut [f32]) {
    for i in 0..(DOT_SIZE-LESS) {
        output_slice[i] = input_slice[i] * (speed_test.dys[i] as f32);
    }
}

#[allow(dead_code)]
fn fdot(speed_test: &SpeedTest, input_slice: &[f32], output_slice: &mut [f32]) {
    for i in 0..(DOT_SIZE-LESS) {
        output_slice[i] = input_slice[i] * speed_test.fys[i];
    }
}

impl SpeedTest {
    pub fn new() -> SpeedTest {
        SpeedTest {
            fys: [0.0; DOT_SIZE],
            dys: [0.0; DOT_SIZE],
        }
    }
}

impl Patch for SpeedTest {
    fn rust_process_audio(
        &mut self,
        input_slice: &[f32],
        output_slice: &mut [f32],
        mut _playhead: Playhead,
    ) {
        for _r in 0..NROUNDS {
            ddot(&self, input_slice, output_slice);
        }
    }
}
