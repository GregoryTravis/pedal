pub struct LowPassFilter {
  pub state: f32,
}

impl LowPassFilter {
  pub fn new() -> LowPassFilter {
    LowPassFilter { state: 0.0 }
  }

  pub fn filter(&mut self, input_slice: &[f32], output_slice: &mut [f32], size: usize) {
    for i in 0..size {
      output_slice[i] = 5.0 * ((input_slice[i] + self.state) / 2.0);
      self.state = input_slice[i];
    }
  }
}
