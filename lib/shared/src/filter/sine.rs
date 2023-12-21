extern crate libm;

// Ignores its input, just generates a sine.

pub struct SineGenerator {
}

impl SineGenerator {
  pub fn new() -> SineGenerator {
    SineGenerator {}
  }

  pub fn filter(&mut self, _input_slice: &[f32], output_slice: &mut [f32], size: usize,
                time_in_seconds: f64) {
    for i in 0..size {
      // TODO should get sampling rate from env
      // TODO this is ragged
      output_slice[i] = libm::sinf(((time_in_seconds / 6.283) * 9000.0) as f32) / 32.0;
    }
  }
}
