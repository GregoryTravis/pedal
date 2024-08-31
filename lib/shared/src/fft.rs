use crate::constants::*;
use crate::spew::*;

#[cfg(not(feature = "for_host"))]
use crate::fft_board::*;

const VERBOSE: bool = true;

pub fn fft_test() {
    let mut input: [f32; FFT_SIZE] = [0.0; FFT_SIZE];
    for i in 0..FFT_SIZE {
        input[i] = (i as f32) / (FFT_SIZE as f32);
    }

    let mut input_copy: [f32; FFT_SIZE] = [0.0; FFT_SIZE];
    input_copy.copy_from_slice(&input);

    let mut fft_buffer: [f32; FFT_SIZE] = [0.0; FFT_SIZE];
    let mut output: [f32; FFT_SIZE] = [0.0; FFT_SIZE];
    fft(&mut input_copy, &mut fft_buffer);
    ifft(&mut fft_buffer, &mut output);

    if VERBOSE {
        // Difference
        let diff: f32 = input.iter().zip(output.iter()).map(|pr| {
            let (ix, ox) = pr;
            ix + ox
        }).sum();
        // FFT sum
        let fft_sum: f32 = fft_buffer.iter().sum();
        spew!("fft diff", diff, "sum", fft_sum);
    }
}

// NOTE: stomps its input
pub fn fft(input: &mut [f32; FFT_SIZE], output: &mut [f32; FFT_SIZE]) {
    fft_boh(input, output);
}

// NOTE: stomps its input, I think
pub fn ifft(input: &mut [f32; FFT_SIZE], output: &mut [f32; FFT_SIZE]) {
    ifft_boh(input, output);
}
