// https://www.musicdsp.org/en/latest/Filters/29-resonant-filter.html

extern crate libm;

pub struct ResoFilter {
  pub buf0: f32,
  pub buf1: f32,
  pub q: f32,
}

impl ResoFilter {
  pub fn new() -> ResoFilter {
    let q = 0.95;
    ResoFilter { buf0: 0.0, buf1: 0.0, q: q }
  }

  pub fn filter(&mut self, input_slice: &[f32], output_slice: &mut [f32], size: usize,
                time_in_seconds: f64) {
    for i in 0..size {
      // Rolls over every 68 years
      let osc = libm::sinf(time_in_seconds as f32);
      let max_f = 0.9;
      let min_f = 0.3;
      let oscf = min_f + ((max_f-min_f) * ((osc+1.0)/2.0));
      let fb = self.q + self.q/(1.0 - oscf);
      let inp = input_slice[i];
      self.buf0 = self.buf0 + oscf * (inp - self.buf0 + fb * (self.buf0 - self.buf1));
      self.buf1 = self.buf1 + oscf * (self.buf0 - self.buf1);
      let out = self.buf1;
      output_slice[i] = out;
    }
  }
}
