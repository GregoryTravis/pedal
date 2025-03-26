extern crate std;
extern crate alloc;
extern crate libm;

use alloc::vec::Vec;
use std::env;
use std::format;

use shared::constants::*;
use shared::file::*;
use shared::fft::*;
use shared::unit::reso::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    // prog, filename, duration, then (freq, amp) pairs
    let input_filename = &args[1];
    let start = args[2].parse::<usize>().unwrap();
    let q = args[3].parse::<f32>().unwrap();
    let freq = args[4].parse::<f32>().unwrap();

    let input_entire = file_read(input_filename);
    let input_urr = &input_entire.as_slice()[start..start + FFT_SIZE];
    let input: Vec<f32> = Vec::from(input_urr);
    let input_unmodified_filename = format!("{}-input-{}-{}-{}.wav", input_filename, start, q, freq);
    file_write(&input_unmodified_filename, &input);

    let mut input_filtered = vec![0.0; input.len()];
    let mut reso: Reso = Reso::new(q);
    reso.set_pitch(freq);
    reso.set_amp(1.0);
    for i in 0..input.len() {
        input_filtered[i] = reso.process(input[i]);
    }

    let filtered_filename = format!("{}-filtered-{}-{}-{}.wav", input_filename, start, q, freq);
    file_write(&filtered_filename, &input_filtered);

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
    let output_fft_filename = format!("{}-fft-{}-{}-{}.wav", input_filename, start, q, freq);
    file_write(&output_fft_filename , &output_vec);

    let output_filtered_vec = Vec::from(output_filtered_array);
    let output_filtered_fft_filename = format!("{}-filtered-fft-{}-{}-{}.wav", input_filename, start, q, freq);
    file_write(&output_filtered_fft_filename, &output_filtered_vec);
}
