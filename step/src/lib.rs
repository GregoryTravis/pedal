#![no_std]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused)]

extern crate alloc;

pub mod filter;

use core::mem;
use core::slice;

use alloc::boxed::Box;
//use alloc::format;
use alloc_cortex_m::CortexMHeap;

use crate::filter::high_pass::*;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

const HEAP_START: usize = 0x24020000;
const HEAP_SIZE: usize = (512 - 128) * 1024; // 384KB

pub fn init_allocator(heap_start: usize, heap_size: usize) {
    unsafe { ALLOCATOR.init(heap_start, heap_size) }
}

#[no_mangle]
pub fn rust_setup() {
  init_allocator(HEAP_START, HEAP_SIZE);
}

// TODO pub needed?
// TODO it's mono so don't do both channels
pub struct Patch {
  hpf_left: HighPassFilter,
  hpf_right: HighPassFilter,
}

#[no_mangle]
pub fn get_size() -> usize {
    return core::mem::size_of::<Box<Patch>>();
}

extern "C" {
  pub fn cpp_main() -> i32;
  pub fn ping();
  //#[link_name = "\u{1}__Z9PrintLinePKcz"]
  //pub fn PrintLine(format: *const ::std::os::raw::c_char, ...);
  pub fn UnsafeDelay(delay_ms: u32);
}

pub fn delay(delay_ms: u32) {
  unsafe { UnsafeDelay(delay_ms); }
}

//libc::c_char
//let c_str = CString::new(format).unwrap();
//let c_world: *const c_char = c_str.as_ptr() as *const c_char;

#[no_mangle]
pub fn main() -> i32 {
  unsafe { cpp_main() }
}

#[no_mangle]
pub fn get_patch() -> Box<Patch> {
  Box::new(Patch { hpf_left: HighPassFilter::new(), hpf_right: HighPassFilter::new() })
}

#[no_mangle]
pub fn use_patch(patch: Box<Patch>) -> f32 {
  patch.foo(100.1)
}

//#[no_mangle]
//pub extern "C" fn rust_patch_main(mut patch: Box<Patch>) {
  //patch.main();
//}

#[no_mangle]
pub extern "C" fn rust_process_audio_stub(patch: &mut Patch, in_ptr: *const *const f32, out_ptr: *const *mut f32, len: usize) {
  patch.rust_process_audio(in_ptr, out_ptr, len);
}

impl Patch {
  pub fn foo(&self, x: f32) -> f32 {
      return x + 1.2;
  }

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

  #[no_mangle]
  pub fn patch_main(&mut self) {
      //let _foo = format!("hey {} yeah {}", 12, 2.3);
    loop {
      //PrintLine("helleau");
      //PrintLine("dl %f %f %f %f %d", inl, inr, outl, outr, frames);
      unsafe { ping(); }
      delay(500);
    }
	//loop {}
  }
}
