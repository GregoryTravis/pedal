use crate::constants::*;
use crate::fft::*;
use crate::microfft_fft::*;
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
        run_on_arrays(self.input, self.output);
        self.output
    }
}

