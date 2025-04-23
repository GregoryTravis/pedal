use microfft::real::*;

use crate::constants::*;
use crate::fft::*;
use crate::sdram::*;

pub struct MicroFFTSDRAM<'a> {
    input: &'a mut [f32; FFT_SIZE],
    output: &'a mut [f32; FFT_SIZE],
}

impl <'a> MicroFFTSDRAM<'a> {
    pub fn new(sdram: &mut SDRAM) -> MicroFFTSDRAM<'a> {
        MicroFFTSDRAM {
            input: sdram.alloc(),
            output: sdram.alloc(),
        }
    }
}

impl <'a> FFT for MicroFFTSDRAM<'a> {
    fn get_input(&mut self) -> &mut [f32; FFT_SIZE] {
        self.input
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

