#![no_std]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused)]

use core::slice;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn rust_function() {
}

#[no_mangle]
//pub extern "C" fn rust_function2(in_ptr: *const f32, out_ptr: *mut f32, len: usize) {
// TODO out_ptr type seems wrong, mut+const swapped?
pub extern "C" fn rust_function2(in_ptr: *const *const f32, out_ptr: *const *mut f32, len: usize) {
  //println!("usize {}", sizeof(usize));
  let ilen = len as isize;

  let left_input_slice = unsafe { slice::from_raw_parts(*(in_ptr.wrapping_add(0)), len) };
  let right_input_slice = unsafe { slice::from_raw_parts(*(in_ptr.wrapping_add(1)), len) };
  let left_output_slice = unsafe { slice::from_raw_parts_mut(*(out_ptr.wrapping_add(0)), len) };
  let right_output_slice = unsafe { slice::from_raw_parts_mut(*(out_ptr.wrapping_add(1)), len) };

  // let left_input_slice = unsafe { slice::from_raw_parts(in_ptr, len) };
  // let right_input_slice = unsafe { slice::from_raw_parts(in_ptr.offset(ilen), len) };
  // let left_output_slice = unsafe { slice::from_raw_parts_mut(out_ptr, len) };
  // let right_output_slice = unsafe { slice::from_raw_parts_mut(out_ptr.offset(ilen), len) };

  lala(left_input_slice, right_input_slice, left_output_slice, right_output_slice, len);
}

static mut STATEL: f32 = 0.0;
static mut STATER: f32 = 0.0;

fn lala(left_input_slice: &[f32], right_input_slice: &[f32],
        left_output_slice: &mut [f32], right_output_slice: &mut [f32],
        size: usize) {
    /*
    for i in 0..size-1 {
        left_output_slice[i] = left_input_slice[i];
        right_output_slice[i] = right_input_slice[i];
    }
    */

    let mut sttl : f32;
    let mut sttr : f32;
    unsafe { sttl = STATEL; }
    unsafe { sttr = STATER; }
    for i in 0..size {
        left_output_slice[i] = (left_input_slice[i] - sttl) / 2.0;
        sttl = left_input_slice[i];
        right_output_slice[i] = (right_input_slice[i] - sttr) / 2.0;
        sttr = right_input_slice[i];
        //right_output_slice[i] = 0.5;
        //left_output_slice[i] = right_input_slice[i];
        //right_output_slice[i] = left_input_slice[i];
    }
    unsafe { STATEL = sttl; }
    unsafe { STATER = sttr; }
}

//fn send_audio(input_slice: &[f32], output_slice: &mut [f32], size: usize) {

    //typedef const float* const* InputBuffer;
    //typedef float** OutputBuffer;
