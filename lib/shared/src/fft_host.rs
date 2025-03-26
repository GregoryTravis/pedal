use microfft::real::*;

use crate::constants::*;

pub fn fft_boh(input: &mut [f32; FFT_SIZE], output: &mut [f32; FFT_SIZE]) {
    // Don't do this copy
    let mut input_copy: [f32; FFT_SIZE] = [0.0; FFT_SIZE];
    input_copy.copy_from_slice(input);
    // Figure out in what sense microfft works in-place
    let returned_output = rfft_4096(input);
    // Why is this half size
    for i in 0..FFT_SIZE/2 {
        output[i] = returned_output[i].norm_sqr().sqrt();
    }
    // Don't do this, maybe it's not used anyway
    for i in FFT_SIZE/2..FFT_SIZE {
        output[i] = 0.0;
    }
}

/*
rfft_2
rfft_4
rfft_8
rfft_16
rfft_32
rfft_64
rfft_128
rfft_256
rfft_512
rfft_1024
rfft_2048
rfft_4096
rfft_8192
*/

/*
fn array<T: Copy, const N: usize>(slice: &[T]) -> [T; N] {
    slice.try_into().expect("Slice has the wrong length")
}
*/

// Returns magnitude
pub fn fft_slice(input: &mut [f32], output: &mut [f32]) {
    assert!(input.len() == output.len());
    let len = input.len();

    // TODO don't do copy
    let input_copy: &mut [f32] = &mut vec![0.0; input.len()];
    input_copy.clone_from_slice(input);

    let complex_result = match input.len() {
        //4096 => rfft_4096(input.try_into().unwrap()),
        4096 => rfft_4096(<&mut [f32; 4096]>::try_from(input).unwrap()),
        _ => unimplemented!(),
    };

    for i in 0..len/2 {
        output[i] = complex_result[i].norm_sqr().sqrt();
    }
    // Don't do this, maybe it's not used anyway
    for i in len/2..len {
        output[i] = 0.0;
    }
}

pub fn ifft_boh(_input: &mut [f32; FFT_SIZE], _output: &mut [f32; FFT_SIZE]) {
    unimplemented!();
}
