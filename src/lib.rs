#![no_std]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused)]

extern crate alloc;
extern crate dsp;

//use dsp::*;
use crate::dsp::*;
//#[path = "../dsp/src/lib.rs"] mod dsp;

use crate::dsp::rig::*;

use core::mem;

use alloc::boxed::Box;
//use alloc::format;
use alloc_cortex_m::CortexMHeap;

use dsp::filter::high_pass::*;
use dsp::filter::low_pass::*;
use dsp::filter::reso::*;

// TODO pub needed?
// TODO it's mono so don't do both channels
pub struct LowPassPatch {
  lpf_left: LowPassFilter,
  lpf_right: LowPassFilter,
}

impl Patch for LowPassPatch {
  fn rust_process_audio(&mut self, left_input_slice: &[f32], right_input_slice: &[f32],
            left_output_slice: &mut [f32], right_output_slice: &mut [f32],
            size: usize) {
      self.lpf_left.filter(left_input_slice, left_output_slice, size);
      self.lpf_right.filter(right_input_slice, right_output_slice, size);
  }
}

#[no_mangle]
pub fn low_pass_main() -> i32{
  let box_patch = Box::new(LowPassPatch {
      lpf_left: LowPassFilter::new(),
      lpf_right: LowPassFilter::new(),
  });
  gogogo(box_patch)
}

// TODO pub needed?
// TODO it's mono so don't do both channels
pub struct HighPassPatch {
  hpf_left: HighPassFilter,
  hpf_right: HighPassFilter,
}

impl Patch for HighPassPatch  {
  fn rust_process_audio(&mut self, left_input_slice: &[f32], right_input_slice: &[f32],
            left_output_slice: &mut [f32], right_output_slice: &mut [f32],
            size: usize) {
      self.hpf_left.filter(left_input_slice, left_output_slice, size);
      self.hpf_right.filter(right_input_slice, right_output_slice, size);
  }
}

#[no_mangle]
pub fn high_pass_main() -> i32{
  let box_patch = Box::new(HighPassPatch {
      hpf_left: HighPassFilter::new(),
      hpf_right: HighPassFilter::new(),
  });
  gogogo(box_patch)
}

// TODO pub needed?
// TODO it's mono so don't do both channels
pub struct ResoPatch {
  left: ResoFilter,
  right: ResoFilter,
}

impl Patch for ResoPatch  {
  fn rust_process_audio(&mut self, left_input_slice: &[f32], right_input_slice: &[f32],
            left_output_slice: &mut [f32], right_output_slice: &mut [f32],
            size: usize) {
      self.left.filter(left_input_slice, left_output_slice, size);
      self.right.filter(right_input_slice, right_output_slice, size);
  }
}

#[no_mangle]
pub fn reso_main() -> i32{
  let box_patch = Box::new(ResoPatch {
      left: ResoFilter::new(),
      right: ResoFilter::new(),
  });
  gogogo(box_patch)
}
