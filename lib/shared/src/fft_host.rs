use microfft::real::rfft_512;

use crate::constants::*;

pub fn fft_boh(input: &mut [f32; FFT_SIZE], output: &mut [f32; FFT_SIZE]) {
    // Don't do this copy
    let mut input_copy: [f32; FFT_SIZE] = [0.0; FFT_SIZE];
    input_copy.copy_from_slice(input);
    // Figure out in what sense microfft works in-place
    let returned_output = rfft_512(input);
    // Why is this half size
    for i in 0..FFT_SIZE/2 {
        output[i] = returned_output[i].norm_sqr().sqrt();
    }
    // Don't do this, maybe it's not used anyway
    for i in FFT_SIZE/2..FFT_SIZE {
        output[i] = 0.0;
    }
}

pub fn ifft_boh(_input: &mut [f32; FFT_SIZE], _output: &mut [f32; FFT_SIZE]) {
    unimplemented!();
}
