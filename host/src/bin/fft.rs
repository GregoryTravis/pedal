extern crate std;
extern crate alloc;
extern crate libm;

use alloc::vec::Vec;
use std::env;

use shared::constants::*;
use shared::file::*;
use shared::fft::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    // prog, filename, duration, then (freq, amp) pairs
    let input_filename = &args[1];
    let output_filename = &args[2];

    let input = file_read(input_filename);
    //let mut input_array: [f32; FFT_SIZE] = input[..].try_into().unwrap();

    // 15 minutes of googling could not tell me how to do this any other way.
    let mut input_array: [f32; FFT_SIZE] = [0.0; FFT_SIZE];
    for i in 0..FFT_SIZE {
        input_array[i] = input[i];
    }
    let mut output: [f32; FFT_SIZE] = [0.0; FFT_SIZE];
    fft(&mut input_array, &mut output);

    for i in 0..FFT_SIZE {
        output[i] /= FFT_SIZE as f32; // *= 0.00001;
    }

    let output_vec = Vec::from(output);
    file_write(output_filename, output_vec);
}
