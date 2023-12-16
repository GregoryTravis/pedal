#![no_std]

extern crate alloc;
extern crate board;
extern crate pedalhost;
extern crate shared;

use alloc::boxed::Box;

use board::rig::*;
use pedalhost::*;
use shared::filter::high_pass::*;
use shared::filter::low_pass::*;
use shared::filter::reso::*;

#[no_mangle]
pub fn low_pass_main() -> i32{
  let box_patch = Box::new(LowPassPatch {
      lpf_left: LowPassFilter::new(),
      lpf_right: LowPassFilter::new(),
  });
  gogogo(box_patch)
}

#[no_mangle]
pub fn high_pass_main() -> i32{
  let box_patch = Box::new(HighPassPatch {
      hpf_left: HighPassFilter::new(),
      hpf_right: HighPassFilter::new(),
  });
  gogogo(box_patch)
}

#[no_mangle]
pub fn reso_main() -> i32{
  let box_patch = Box::new(ResoPatch {
      left: ResoFilter::new(),
      right: ResoFilter::new(),
  });
  gogogo(box_patch)
}
