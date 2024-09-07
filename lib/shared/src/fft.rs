extern crate libm;

use crate::constants::*;

#[cfg(not(feature = "for_host"))]
use crate::fft_board::*;
#[cfg(feature = "for_host")]
use crate::fft_host::*;

// NOTE: stomps its input
pub fn fft(input: &mut [f32; FFT_SIZE], output: &mut [f32; FFT_SIZE]) {
    fft_boh(input, output);
}

// NOTE: stomps its input, I think
pub fn ifft(input: &mut [f32; FFT_SIZE], output: &mut [f32; FFT_SIZE]) {
    ifft_boh(input, output);
}
