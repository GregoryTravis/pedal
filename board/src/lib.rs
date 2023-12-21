#![no_std]

extern crate alloc;
extern crate board;
extern crate pedalhost;
extern crate shared;

use alloc::boxed::Box;

use board::rig::*;
use shared::filter::high_pass::*;
use shared::filter::low_pass::*;
use shared::filter::reso::*;
use shared::filter::sine::*;

#[no_mangle]
pub fn low_pass_main() -> i32{
  gogogo(Box::new(LowPassFilter::new()))
}

#[no_mangle]
pub fn high_pass_main() -> i32{
  gogogo(Box::new(HighPassFilter::new()))
}

#[no_mangle]
pub fn reso_main() -> i32{
  gogogo(Box::new(ResoFilter::new()))
}

#[no_mangle]
pub fn sine_main() -> i32{
  gogogo(Box::new(SineGenerator::new()))
}
