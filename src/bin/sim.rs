//extern crate host;
//extern crate shared;

use host::sim::*;
use shared::Patch;
use shared::filter::reso::*;

pub struct ResoPatch {
  left: ResoFilter,
  right: ResoFilter,
}

impl Patch for ResoPatch  {
  fn rust_process_audio(&mut self, left_input_slice: &[f32], right_input_slice: &[f32],
            left_output_slice: &mut [f32], right_output_slice: &mut [f32],
            size: usize, time_in_seconds: f64) {
      self.left.filter(left_input_slice, left_output_slice, size, time_in_seconds);
      self.right.filter(right_input_slice, right_output_slice, size, time_in_seconds);
  }
}

pub fn main() {
  // TODO remove box?
  let box_patch = Box::new(ResoPatch {
      left: ResoFilter::new(),
      right: ResoFilter::new(),
  });
  sim_main(box_patch);
}
