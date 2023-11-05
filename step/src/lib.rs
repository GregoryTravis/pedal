#![no_std]

use core::slice;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn rust_function() {
}

#[no_mangle]
pub extern "C" fn rust_function2(in_ptr: *const f32, out_ptr: *mut f32, len: usize) {
  //println!("usize {}", sizeof(usize));
  let input_slice = unsafe { slice::from_raw_parts(in_ptr, len) };
  let output_slice = unsafe { slice::from_raw_parts_mut(out_ptr, len) };
  lala(input_slice, output_slice, len);
}

fn lala(input_slice: &[f32], output_slice: &mut [f32], size: usize) {
    assert!((size%2) == 0);
    let half = size/2;
    for i in 0..half {
        output_slice[i] = input_slice[i+half];
        output_slice[i+half] = input_slice[i];
    }
}

//fn send_audio(input_slice: &[f32], output_slice: &mut [f32], size: usize) {

    //typedef const float* const* InputBuffer;
    //typedef float** OutputBuffer;
