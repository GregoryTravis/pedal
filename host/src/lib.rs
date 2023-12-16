#![cfg_attr(not(for_host), no_std)]

//extern crate board;
extern crate shared;

mod tests;

use shared::*;
use shared::filter::high_pass::*;
use shared::filter::low_pass::*;
use shared::filter::reso::*;

// TODO pub needed?
// TODO it's mono so don't do both channels
pub struct LowPassPatch {
  pub lpf_left: LowPassFilter,
  pub lpf_right: LowPassFilter,
}

impl Patch for LowPassPatch {
  fn rust_process_audio(&mut self, left_input_slice: &[f32], right_input_slice: &[f32],
            left_output_slice: &mut [f32], right_output_slice: &mut [f32],
            size: usize, _time_in_seconds: f64) {
      self.lpf_left.filter(left_input_slice, left_output_slice, size);
      self.lpf_right.filter(right_input_slice, right_output_slice, size);
  }
}

// TODO pub needed?
// TODO it's mono so don't do both channels
pub struct HighPassPatch {
  pub hpf_left: HighPassFilter,
  pub hpf_right: HighPassFilter,
}

impl Patch for HighPassPatch  {
  fn rust_process_audio(&mut self, left_input_slice: &[f32], right_input_slice: &[f32],
            left_output_slice: &mut [f32], right_output_slice: &mut [f32],
            size: usize, _time_in_seconds: f64) {
      self.hpf_left.filter(left_input_slice, left_output_slice, size);
      self.hpf_right.filter(right_input_slice, right_output_slice, size);
  }
}

// TODO pub needed?
// TODO it's mono so don't do both channels
pub struct ResoPatch {
  pub left: ResoFilter,
  pub right: ResoFilter,
}

impl Patch for ResoPatch  {
  fn rust_process_audio(&mut self, left_input_slice: &[f32], right_input_slice: &[f32],
            left_output_slice: &mut [f32], right_output_slice: &mut [f32],
            size: usize, time_in_seconds: f64) {
      self.left.filter(left_input_slice, left_output_slice, size, time_in_seconds);
      self.right.filter(right_input_slice, right_output_slice, size, time_in_seconds);
  }
}
