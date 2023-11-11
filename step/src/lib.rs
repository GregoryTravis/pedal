#![no_std]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused)]

extern crate alloc;

use core::mem;
use core::slice;
use alloc::boxed::Box;
use alloc_cortex_m::CortexMHeap;

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
pub struct Patch {
  statel: f32,
  stater: f32,
}

#[no_mangle]
pub fn get_size() -> usize {
    return core::mem::size_of::<Box<Patch>>();
}

extern "C" {
  pub fn cpp_main() -> i32;
}

#[no_mangle]
pub fn main() -> i32 {
  unsafe { cpp_main() }
}

#[no_mangle]
pub fn get_patch() -> Box<Patch> {
  Box::new(Patch { statel: 0.0, stater: 0.0 })
}

#[no_mangle]
pub fn use_patch(patch: Box<Patch>) -> f32 {
  patch.foo(100.1)
}

pub fn rust_process_audio(mut patch: Box<Patch>, in_ptr: *const *const f32, out_ptr: *const *mut f32, len: usize) {
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
          left_output_slice[i] = (left_input_slice[i] - self.statel) / 2.0;
          self.statel = left_input_slice[i];
          right_output_slice[i] = (right_input_slice[i] - self.stater) / 2.0;
          self.stater = right_input_slice[i];
      }
  }
}
