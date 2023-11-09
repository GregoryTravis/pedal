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
  yep: f32,
}

#[no_mangle]
pub fn get_size() -> usize {
    return core::mem::size_of::<Box<Patch>>();
}

#[no_mangle]
pub fn get_patch() -> Box<Patch> {
  Box::new(Patch { yep: 30.3 })
}

#[no_mangle]
pub fn use_patch(patch: Box<Patch>) -> f32 {
  patch.foo(100.1)
}

impl Patch {
  pub fn foo(&self, x: f32) -> f32 {
      return x + 1.2;
  }
}

#[no_mangle]
// TODO out_ptr type seems wrong, mut+const swapped?
pub extern "C" fn rust_process_audio(in_ptr: *const *const f32, out_ptr: *const *mut f32, len: usize) {
  let ilen = len as isize;

  let left_input_slice = unsafe { slice::from_raw_parts(*(in_ptr.wrapping_add(0)), len) };
  let right_input_slice = unsafe { slice::from_raw_parts(*(in_ptr.wrapping_add(1)), len) };
  let left_output_slice = unsafe { slice::from_raw_parts_mut(*(out_ptr.wrapping_add(0)), len) };
  let right_output_slice = unsafe { slice::from_raw_parts_mut(*(out_ptr.wrapping_add(1)), len) };

  //copy_in_to_out(left_input_slice, right_input_slice, left_output_slice, right_output_slice, len);
  filter(left_input_slice, right_input_slice, left_output_slice, right_output_slice, len);
}

static mut STATEL: f32 = 0.0;
static mut STATER: f32 = 0.0;

fn copy_in_to_out(left_input_slice: &[f32], right_input_slice: &[f32],
        left_output_slice: &mut [f32], right_output_slice: &mut [f32],
        size: usize) {
    for i in 0..size-1 {
        left_output_slice[i] = left_input_slice[i];
        right_output_slice[i] = right_input_slice[i];
    }
}

fn filter(left_input_slice: &[f32], right_input_slice: &[f32],
        left_output_slice: &mut [f32], right_output_slice: &mut [f32],
        size: usize) {
    let mut sttl : f32;
    let mut sttr : f32;
    unsafe { sttl = STATEL; }
    unsafe { sttr = STATER; }
    for i in 0..size {
        left_output_slice[i] = (left_input_slice[i] - sttl) / 2.0;
        sttl = left_input_slice[i];
        right_output_slice[i] = (right_input_slice[i] - sttr) / 2.0;
        sttr = right_input_slice[i];
    }
    unsafe { STATEL = sttl; }
    unsafe { STATER = sttr; }
}
