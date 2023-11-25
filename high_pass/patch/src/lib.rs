#![no_std]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused)]

extern crate alloc;

use crate::dsp::*;
#[path = "../../../dsp/src/lib.rs"] mod dsp;

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

static THE_PATCH: Mutex<RefCell<Option<Patch>>> =
    Mutex::new(RefCell::new(None));

#[global_allocator]
static ALLOCATOR: emballoc::Allocator<32768> = emballoc::Allocator::new();

// TODO pub needed?
// TODO it's mono so don't do both channels
pub struct Patch {
  hpf_left: HighPassFilter,
  hpf_right: HighPassFilter,
  inl: f32,
  inr: f32,
  outl: f32,
  outr: f32,
  framesize: usize,
}

extern "C" {
  pub fn cpp_main() -> i32;
  pub fn UnsafeDelay(delay_ms: u32);
  pub fn load_spew();
}

pub fn show_load() {
  unsafe { load_spew(); }
}

pub fn delay(delay_ms: u32) {
  unsafe { UnsafeDelay(delay_ms); }
}

#[no_mangle]
pub fn main() -> i32 {
  // The audio handler must be installed AFTER this line.
  // TODO is this use of get_patch() an unnecessary copy?
  interrupt::free(|cs| THE_PATCH.borrow(cs).replace(Some(get_patch())));
  unsafe { cpp_main() }
}

fn get_patch() -> Patch {
  Patch {
      hpf_left: HighPassFilter::new(),
      hpf_right: HighPassFilter::new(),
      inl: 0.0,
      inr: 0.0,
      outl: 0.0,
      outr: 0.0,
      framesize: 0,
  }
}

#[no_mangle]
pub extern "C" fn rust_process_audio_stub(in_ptr: *const *const f32, out_ptr: *const *mut f32, len: usize) {
  interrupt::free(|cs| {
    if let Some(ref mut patch) = THE_PATCH.borrow(cs).borrow_mut().deref_mut().as_mut() {
      patch.rust_process_audio(in_ptr, out_ptr, len);
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
      if let Some(ref mut patch) = THE_PATCH.borrow(cs).borrow_mut().deref_mut().as_mut() {
          inl = patch.inl;
          inr = patch.inr;
          outl = patch.outl;
          outr = patch.outr;
          framesize = patch.framesize;
      }
    });

    glep!("dl adf afdjadfjasdadfaaf asfd", inl, inr, outl, outr, framesize);
    show_load();
    delay(500);
  }
  loop {} // Just to be safe -- TODO: necessary?
}

impl Patch {
  #[no_mangle]
  // TODO out_ptr type seems wrong, mut+const swapped?
  pub fn rust_process_audio(&mut self, in_ptr: *const *const f32, out_ptr: *const *mut f32, len: usize) {
    let ilen = len as isize;

    let left_input_slice = unsafe { slice::from_raw_parts(*(in_ptr.wrapping_add(0)), len) };
    let right_input_slice = unsafe { slice::from_raw_parts(*(in_ptr.wrapping_add(1)), len) };
    let left_output_slice = unsafe { slice::from_raw_parts_mut(*(out_ptr.wrapping_add(0)), len) };
    let right_output_slice = unsafe { slice::from_raw_parts_mut(*(out_ptr.wrapping_add(1)), len) };

    //self.copy_in_to_out(left_input_slice, right_input_slice, left_output_slice, right_output_slice, len);
    self.filter(left_input_slice, right_input_slice, left_output_slice, right_output_slice, len);

    self.inl = left_input_slice[0];
    self.inr = right_input_slice[0];
    self.outl = left_output_slice[0];
    self.outr = right_output_slice[0];
    self.framesize = len;
  }

  fn copy_in_to_out(&self, left_input_slice: &[f32], right_input_slice: &[f32],
          left_output_slice: &mut [f32], right_output_slice: &mut [f32],
          size: usize) {
      for i in 0..size-1 {
          left_output_slice[i] = left_input_slice[i];
          right_output_slice[i] = right_input_slice[i];
      }
  }

  fn filter(&mut self, left_input_slice: &[f32], right_input_slice: &[f32],
          left_output_slice: &mut [f32], right_output_slice: &mut [f32],
          size: usize) {
      for i in 0..size {
          left_output_slice[i] = (left_input_slice[i] - self.hpf_left.state) / 2.0;
          self.hpf_left.state = left_input_slice[i];
          right_output_slice[i] = (right_input_slice[i] - self.hpf_right.state) / 2.0;
          self.hpf_right.state = right_input_slice[i];
      }
  }
}
