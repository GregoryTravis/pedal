pub struct HighPassFilter {
  pub state: f32,
}

impl HighPassFilter {
  pub fn new() -> HighPassFilter {
    HighPassFilter { state: 0.0 }
  }
}
