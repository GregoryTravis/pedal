use crate::constants::*;
use crate::fft::*;
use crate::microfft_fft::*;
use crate::sdram::*;

pub struct MicroFFTSDRAM {
    input: &'static mut [f32; FFT_SIZE],
    output: &'static mut [f32; FFT_SIZE],
}

impl MicroFFTSDRAM {
    pub fn new(sdram: &mut SDRAM) -> MicroFFTSDRAM {
        MicroFFTSDRAM {
            input: sdram.alloc(),
            output: sdram.alloc(),
        }
    }
}

impl FFT for MicroFFTSDRAM {
    fn get_input(&mut self) -> &mut [f32; FFT_SIZE] {
        self.input
    }

    fn run(&mut self) -> &[f32; FFT_SIZE] {
        run_on_arrays(self.input, self.output);
        self.output
    }
}

