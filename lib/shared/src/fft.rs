extern crate libm;

use crate::constants::*;
#[allow(unused)]
use crate::spew::*;

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
        //mag_out[i] = libm::sqrtf(re*re + im*im);
        mag_out[i] = 1.0 / quake_rsqrt(re*re + im*im);
        //mag_out[i] = re*re + im*im;
    }
}

// https://www.reddit.com/r/rust/comments/vdroh6/i_implemented_the_famous_quake_inverse_sqrt/
#[allow(unused)]
fn quake_rsqrt(number: f32) -> f32 {
    let mut y: f32 = number;
    unsafe {
        let mut i: i32 = core::mem::transmute::<f32, i32>(y);
        i = 0x5F375A86 - (i >> 1);
        y = core::mem::transmute::<i32, f32>(i);
    }
    y * (1.5 - (number * 0.5 * y * y))
}

#[allow(unused)]
fn quake_rsqrt2(number: f32) -> f32 {
    let mut i: i32 = number.to_bits() as i32;
    i = 0x5F375A86_i32.wrapping_sub(i >> 1);
    let y = f32::from_bits(i as u32);
    y * (1.5 - (number * 0.5 * y * y))
}
