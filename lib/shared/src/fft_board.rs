use crate::constants::*;

// ARM FFT only supports 2048 or higher (or maybe 1024 or higher)

extern "C" {
    //pub fn do_arm_fft(input: *mut f32, output: *mut f32);
    //pub fn do_arm_ifft(input: *mut f32, output: *mut f32);
    pub fn do_shy_fft(input: *mut f32, output: *mut f32);
    pub fn do_shy_ifft(input: *mut f32, output: *mut f32);
}

pub fn fft_boh(input: &mut [f32; FFT_SIZE], output: &mut [f32; FFT_SIZE]) {
    let input_ptr: *mut f32 = input.as_mut_ptr();
    let output_ptr: *mut f32 = output.as_mut_ptr();
    unsafe {
        //do_arm_fft(input_ptr, output_ptr);
        do_shy_fft(input_ptr, output_ptr);
    }
}

pub fn ifft_boh(input: &mut [f32; FFT_SIZE], output: &mut [f32; FFT_SIZE]) {
    let input_ptr: *mut f32 = input.as_mut_ptr();
    let output_ptr: *mut f32 = output.as_mut_ptr();
    unsafe {
        //do_arm_ifft(input_ptr, output_ptr);
        do_shy_ifft(input_ptr, output_ptr);
    }
}
