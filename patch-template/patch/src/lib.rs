#![no_std]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused)]

extern crate alloc;

use crate::dsp::*;
#[path = "../../../dsp/src/lib.rs"] mod dsp;

use crate::dsp::rig::*;

use core::mem;

use alloc::boxed::Box;
//use alloc::format;
use alloc_cortex_m::CortexMHeap;

use dsp::filter::high_pass::*;

// TODO pub needed?
// TODO it's mono so don't do both channels
pub struct MyPatch {
  hpf_left: HighPassFilter,
  hpf_right: HighPassFilter,
}

impl Patch for MyPatch {
  fn rust_process_audio(&mut self, left_input_slice: &[f32], right_input_slice: &[f32],
            left_output_slice: &mut [f32], right_output_slice: &mut [f32],
            size: usize) {
      self.hpf_left.filter(left_input_slice, left_output_slice, size);
      self.hpf_right.filter(right_input_slice, right_output_slice, size);
  }
}

#[no_mangle]
pub fn main() -> i32{
  let box_patch = Box::new(MyPatch {
      hpf_left: HighPassFilter::new(),
      hpf_right: HighPassFilter::new(),
  });
  gogogo(box_patch)
}
