#![no_std]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused)]

extern crate alloc;

use crate::dsp::*;
#[path = "../../../dsp/src/lib.rs"] mod dsp;

use crate::dsp::load::*;
use crate::dsp::spew::*;

use core::mem;
use core::slice;

use alloc::boxed::Box;
//use alloc::format;
use alloc_cortex_m::CortexMHeap;

use dsp::filter::high_pass::*;

use core::cell::RefCell;
use cortex_m::interrupt::{self, Mutex};

use core::ops::DerefMut;

static THE_PATCH: Mutex<RefCell<Option<Rig>>> =
    Mutex::new(RefCell::new(None));

#[global_allocator]
static ALLOCATOR: emballoc::Allocator<32768> = emballoc::Allocator::new();

pub struct Rig {
  patch: Box<dyn Patch>,
  inl: f32,
  inr: f32,
  outl: f32,
  outr: f32,
  framesize: usize,
}

pub trait Patch: Send {
  fn rust_process_audio(&mut self, left_input_slice: &[f32], right_input_slice: &[f32],
                        left_output_slice: &mut [f32], right_output_slice: &mut [f32],
                        size: usize);
}

// TODO pub needed?
// TODO it's mono so don't do both channels
pub struct MyPatch {
  hpf_left: HighPassFilter,
  hpf_right: HighPassFilter,
}

extern "C" {
  pub fn cpp_main() -> i32;
  pub fn UnsafeDelay(delay_ms: u32);
}

pub fn delay(delay_ms: u32) {
  unsafe { UnsafeDelay(delay_ms); }
}

#[no_mangle]
pub fn main() -> i32 {
  // The audio handler must be installed AFTER this line.
  // TODO is this use of get_patch() an unnecessary copy?
  let rig = Rig {
    patch: Box::new(get_patch()),
    inl: 0.0,
    inr: 0.0,
    outl: 0.0,
    outr: 0.0,
    framesize: 0,
  };
  interrupt::free(|cs| THE_PATCH.borrow(cs).replace(Some(rig)));
  unsafe { cpp_main() }
}

fn get_patch() -> MyPatch {
  MyPatch {
      hpf_left: HighPassFilter::new(),
      hpf_right: HighPassFilter::new(),
  }
}

#[no_mangle]
pub extern "C" fn rust_process_audio_stub(in_ptr: *const *const f32, out_ptr: *const *mut f32, len: usize) {
  interrupt::free(|cs| {
    if let Some(ref mut rig) = THE_PATCH.borrow(cs).borrow_mut().deref_mut().as_mut() {
      let ilen = len as isize;

      let left_input_slice = unsafe { slice::from_raw_parts(*(in_ptr.wrapping_add(0)), len) };
      let right_input_slice = unsafe { slice::from_raw_parts(*(in_ptr.wrapping_add(1)), len) };
      let left_output_slice = unsafe { slice::from_raw_parts_mut(*(out_ptr.wrapping_add(0)), len) };
      let right_output_slice = unsafe { slice::from_raw_parts_mut(*(out_ptr.wrapping_add(1)), len) };

      rig.patch.rust_process_audio(left_input_slice, right_input_slice, left_output_slice, right_output_slice, len);

      rig.inl = left_input_slice[0];
      rig.inr = right_input_slice[0];
      rig.outl = left_output_slice[0];
      rig.outr = right_output_slice[0];
      rig.framesize = len;
    }
  });
}

#[no_mangle]
pub fn patch_main() {
  loop {
    let mut inl: f32 = 0.0;
    let mut inr: f32 = 0.0;
    let mut outl: f32 = 0.0;
    let mut outr: f32 = 0.0;
    let mut framesize : usize = 0;

    interrupt::free(|cs| {
      if let Some(ref mut rig) = THE_PATCH.borrow(cs).borrow_mut().deref_mut().as_mut() {
          inl = rig.inl;
          inr = rig.inr;
          outl = rig.outl;
          outr = rig.outr;
          framesize = rig.framesize;
      }
    });

    glep!("dl adf afdjadfjasdadfaaf asfd", inl, inr, outl, outr, framesize);

    show_load();
    delay(500);
  }
  loop {} // Just to be safe -- TODO: necessary?
}

impl Patch for MyPatch {
  fn rust_process_audio(&mut self, left_input_slice: &[f32], right_input_slice: &[f32],
            left_output_slice: &mut [f32], right_output_slice: &mut [f32],
            size: usize) {
      self.hpf_left.filter(left_input_slice, left_output_slice, size);
      self.hpf_right.filter(right_input_slice, right_output_slice, size);
  }
}
