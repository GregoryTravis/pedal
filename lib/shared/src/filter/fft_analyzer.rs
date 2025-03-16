#[cfg(feature = "for_host")]
extern crate std;
extern crate libm;

use alloc::boxed::Box;
use alloc::vec::Vec;
use core::f32::consts::PI;

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

// Return peak of the curve described by the values.
// Returns (x_peak, y_peak).
// x_peak is relative to xp which is treated as 0.
// https://www.physics.drexel.edu/~steve/Courses/Comp_Phys/Physics-105/quad_int.pdf
fn quadratic_interpolate(xpp: f32, xp: f32, x: f32) -> (f32, f32) {
    // Remove mult by 1 and other stupid things.
    let dt = 1.0;
    let tp = 0.0;
    let a = xp;
    let b = (x - xpp) / (2.0 * dt);
    let c = (x - (2.0 * xp) + xpp) / (2.0 * dt * dt);
    let tau = tp - (b / (2.0 * c));
    let x_max = a - ((b * b) / (4.0 * c));
    (tau, x_max)
}
//
// Hann window
// w(n) = 0.5 * [1 - cos(2*pi*n / N)]
fn hann(n: usize, num_samples: usize) -> f32 {
    0.5 * (1.0 - libm::cosf((2.0 * PI * n as f32) / num_samples as f32))
}

impl Patch for FFTAnalyzer {
    fn rust_process_audio(
        &mut self,
        input_slice: &[f32],
        output_slice: &mut [f32],
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

        // Hann window
        for i in 0..FFT_SIZE {
            // TODO Don't pass FFT_SIZE?
            self.input[i] *= hann(i, FFT_SIZE);
        }

        fft(&mut self.input, &mut self.output);

        let mut peaks: Vec<(f32, f32)> = Vec::new();

        let amp_threshold = 0.01;

        spew!("====");
        for i in 0..FFT_SIZE {
            //spew!("fft", i, self.output[i]);

            let not_edge = i > 0 && i < FFT_SIZE-1;
            if not_edge {
                let a = self.output[i-1];
                let b = self.output[i];
                let c = self.output[i+1];
                // TODO what if they are equal
                let is_peak = not_edge && a < b && b > c;
                if is_peak {
                    let (relative_x_peak, y_peak) = quadratic_interpolate(self.output[i-1], self.output[i], self.output[i+1]);
                    let x_peak = (i as f32) + relative_x_peak;
                    let amp_peak = y_peak / (FFT_SIZE / 2) as f32;
                    let freq_peak = x_peak * (SAMPLE_RATE as f32 / FFT_SIZE as f32);
                    if amp_peak > amp_threshold {
                        peaks.push((freq_peak, amp_peak));
                    }
                    spew!("*** peak", x_peak, y_peak, freq_peak, amp_peak);
                }
            }
        }

        for i in 0..input_slice.len() {
            output_slice[i] = 0.0;
            for (frequency, amp) in &peaks {
                let ph = playhead.sinf(*frequency);
                output_slice[i] += amp * libm::sinf(ph);
            }
            playhead.inc();
        }
    }
}

