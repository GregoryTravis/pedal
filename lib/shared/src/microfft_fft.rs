use microfft::real::*;

use crate::constants::*;
use crate::fft::*;

pub struct MicroFFT {
    input: [f32; FFT_SIZE],
    output: [f32; FFT_SIZE],
}

impl MicroFFT {
    pub fn new() -> MicroFFT {
        MicroFFT {
            input: [0.0; FFT_SIZE],
            output: [0.0; FFT_SIZE],
        }
    }
}

impl FFT for MicroFFT {
    fn get_input(&mut self) -> &mut [f32; FFT_SIZE] {
        &mut self.input
    }

    fn run(&mut self) -> &[f32; FFT_SIZE] {
        // TODO cast the pointer, don't do the copy
        let input_len = self.input.len();
        let complex_output = rfft_2048(&mut self.input);
        assert!(complex_output.len() * 2 == input_len);
        for i in 0..complex_output.len() {
            self.output[i*2] = complex_output[i].re;
            self.output[i*2+1] = complex_output[i].im;
        }
        &self.output
    }
}
