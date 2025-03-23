extern crate std;
extern crate alloc;
extern crate libm;

use alloc::vec::Vec;
use std::env;

use shared::constants::*;
use shared::file::*;
use shared::fft::*;
use shared::unit::reso::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    // prog, filename, duration, then (freq, amp) pairs
    let input_filename = &args[1];
    let start = args[2].parse::<usize>().unwrap();

    let input_entire = file_read(input_filename);
    let input_urr = &input_entire.as_slice()[start..start + FFT_SIZE];
    let input: Vec<f32> = Vec::from(input_urr);
    file_write(&(input_filename.to_owned() + "-input.wav"), &input);

    let mut input_filtered = vec![0.0; input.len()];
    let mut reso: Reso = Reso::new();
    for i in 0..input.len() {
        input_filtered[i] = reso.process(input[i]);
    }

    file_write(&(input_filename.to_owned() + "-filtered.wav"), &input_filtered);

    // 15 minutes of googling could not tell me how to do this any other way.
    let mut input_array: [f32; FFT_SIZE] = [0.0; FFT_SIZE];
    for i in 0..FFT_SIZE {
        input_array[i] = input[i];
    }
    let mut input_filtered_array: [f32; FFT_SIZE] = [0.0; FFT_SIZE];
    for i in 0..FFT_SIZE {
        input_filtered_array[i] = input_filtered[i];
    }

    let mut output_array: [f32; FFT_SIZE] = [0.0; FFT_SIZE];
    fft(&mut input_array, &mut output_array);

    for i in 0..FFT_SIZE {
        output_array[i] /= FFT_SIZE as f32; // *= 0.00001;
    }

    let mut output_filtered_array: [f32; FFT_SIZE] = [0.0; FFT_SIZE];
    fft(&mut input_filtered_array, &mut output_filtered_array);

    for i in 0..FFT_SIZE {
        output_filtered_array[i] /= FFT_SIZE as f32; // *= 0.00001;
    }

    let output_vec = Vec::from(output_array);
    file_write(&(input_filename.to_owned() + "-input-fft.wav"), &output_vec);
    let output_filtered_vec = Vec::from(output_filtered_array);
    file_write(&(input_filename.to_owned() + "-filtered-fft.wav"), &output_filtered_vec);
}
