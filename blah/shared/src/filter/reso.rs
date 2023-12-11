// https://www.musicdsp.org/en/latest/Filters/29-resonant-filter.html

pub struct ResoFilter {
  pub buf0: f32,
  pub buf1: f32,
  pub f: f32,
  pub fb: f32,
}

impl ResoFilter {
  pub fn new() -> ResoFilter {
    let f = 0.6;
    let q = 0.99;
    let fb = q + q/(1.0 - f);
    ResoFilter { buf0: 0.0, buf1: 0.0, f: f, fb: fb }
  }

  pub fn filter(&mut self, input_slice: &[f32], output_slice: &mut [f32], size: usize) {
    for i in 0..size {
      let inp = input_slice[i];
      self.buf0 = self.buf0 + self.f * (inp - self.buf0 + self.fb * (self.buf0 - self.buf1));
      self.buf1 = self.buf1 + self.f * (self.buf0 - self.buf1);
      let out = self.buf1;
      output_slice[i] = out;
    }
  }
}
