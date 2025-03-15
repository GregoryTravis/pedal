#[cfg(feature = "for_host")]
extern crate std;

use alloc::boxed::Box;

use crate::constants::*;
use crate::fft::*;
use crate::knob::Knobs;
use crate::patch::Patch;
use crate::playhead::Playhead;
use crate::spew::*;

pub struct FFTAnalyzer {
    buf: [f32; FFT_SIZE],
    input: [f32; FFT_SIZE],
    output: [f32; FFT_SIZE],
}

impl FFTAnalyzer {
    pub fn new() -> FFTAnalyzer {
        FFTAnalyzer {
            buf: [0.0; FFT_SIZE],
            input: [0.0; FFT_SIZE],
            output: [0.0; FFT_SIZE],
        }
    }
}

impl Patch for FFTAnalyzer {
    fn rust_process_audio(
        &mut self,
        input_slice: &[f32],
        _output_slice: &mut [f32],
        _knobs: &Box<dyn Knobs>,
        mut playhead: Playhead,
    ) {
        let inlen = input_slice.len();
        let overlap = FFT_SIZE - inlen;

        // TODO use circiular buf for buf, or for fft itself
        for i in 0..overlap {
            self.buf[i] = self.buf[i + inlen];
        }

        for i in 0..input_slice.len() {
            self.buf[overlap + i] = input_slice[i];
        }

        for i in 0..FFT_SIZE {
            self.input[i] = self.buf[i]
        }

        fft(&mut self.input, &mut self.output);

        for i in 0..FFT_SIZE {
            spew!("fft", i, self.output[i]);
        }

        playhead.increment_samples(inlen as u32);
    }
}

