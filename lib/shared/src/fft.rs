extern crate libm;

use crate::constants::*;

// This is a goofy usage pattern, but I have two very different ffts to unify.
pub trait FFT {
    // Assume the implementation will bash the input.
    // (True for microfft, not true for ARM.)
    fn get_input(&mut self) -> &mut [f32; FFT_SIZE];
    fn run(&mut self) -> &[f32; FFT_SIZE];
}

// Input is the output of a real-in/complex-out FFT, alternating real/imaginary values.
// Ouptut is half-length magnitudes.
pub fn fft_to_magnitudes(fft_in: &[f32; FFT_SIZE], mag_out: &mut [f32; FFT_SIZE/2]) {
    for i in 0..FFT_SIZE/2 {
        let re = fft_in[i*2];
        let im = fft_in[i*2+1];
        mag_out[i] = libm::sqrtf(re*re + im*im);
    }
}
