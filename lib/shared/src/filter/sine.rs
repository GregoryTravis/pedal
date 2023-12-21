extern crate libm;

use crate::patch::Patch;

// Ignores its input, just generates a sine.

pub struct SineGenerator {
}

impl SineGenerator {
  pub fn new() -> SineGenerator {
    SineGenerator {}
  }
}

impl Patch for SineGenerator {
  fn rust_process_audio(&mut self, _input_slice: &[f32], output_slice: &mut [f32],
                        time_in_seconds: f64) {
    for i in 0..output_slice.len() {
      // TODO should get sampling rate from env
      // TODO this is ragged
      output_slice[i] = libm::sinf(((time_in_seconds / 6.283) * 9000.0) as f32) / 32.0;
    }
  }
}
