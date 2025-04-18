use crate::constants::*;
use crate::fft::*;

extern "C" {
    pub fn do_arm_fft(input: *mut f32, output: *mut f32);
    pub fn do_arm_ifft(input: *mut f32, output: *mut f32);
}

pub struct ArmFFT {
    input: [f32; FFT_SIZE],
    output: [f32; FFT_SIZE],
}

impl ArmFFT {
    pub fn new() -> ArmFFT {
        ArmFFT {
            input: [0.0; FFT_SIZE],
            output: [0.0; FFT_SIZE],
        }
    }
}

impl FFT for ArmFFT {
    fn get_input(&mut self) -> &mut [f32; FFT_SIZE] {
        &mut self.input
    }

    fn run(&mut self) -> &[f32; FFT_SIZE] {
        let input_ptr: *mut f32 = self.input.as_mut_ptr();
        let output_ptr: *mut f32 = self.output.as_mut_ptr();
        unsafe {
            do_arm_fft(input_ptr, output_ptr);
        }
        &self.output
    }
}

