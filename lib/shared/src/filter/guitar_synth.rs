#[cfg(feature = "for_host")]
extern crate std;
extern crate libm;

use alloc::boxed::Box;
use alloc::vec::Vec;
use core::f32::consts::PI;
#[allow(unused)]
use std::println;

use crate::constants::*;
use crate::fft::*;
use crate::knob::Knobs;
use crate::patch::Patch;
use crate::playhead::Playhead;
use crate::spew::*;
use crate::unit::reso::*;

const NUM_RESOS: usize = 3;

pub struct GuitarSynth {
    buf: [f32; FFT_SIZE],
    input: [f32; FFT_SIZE],
    output: [f32; FFT_SIZE],
    resos: [Reso; NUM_RESOS],
}

impl GuitarSynth {
    pub fn new() -> GuitarSynth {
        GuitarSynth {
            buf: [0.0; FFT_SIZE],
            input: [0.0; FFT_SIZE],
            output: [0.0; FFT_SIZE],
            resos: [Reso::new(), Reso::new(), Reso::new()],
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

impl Patch for GuitarSynth {
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

        let amp_threshold = 0.005;

        let t = playhead.time_in_samples();
        let low_show_peak = 85776 ;
        let high_show_peak = low_show_peak + (48 * 10);

        spew!("====");
        for i in 0..FFT_SIZE {
            //spew!("fft", i, self.output[i]);

            let not_edge = i > 0 && i < FFT_SIZE-1;
            if not_edge {
                let bin_freq = i as f32 * (SAMPLE_RATE as f32 / FFT_SIZE as f32); 
                let max_freq = 60000.0;
                let too_high = bin_freq > max_freq;

                let a = self.output[i-1];
                let b = self.output[i];
                let c = self.output[i+1];
                // TODO what if they are equal
                let is_peak = not_edge && a < b && b > c;
                if is_peak && !too_high {
                    //let left_peakiness = b - a;
                    //let right_peakiness = b - a;
                    let peakiness = (b - ((a+c)/2.0)) / b;

                    let (relative_x_peak, y_peak) = quadratic_interpolate(self.output[i-1], self.output[i], self.output[i+1]);
                    let x_peak = (i as f32) + relative_x_peak;
                    let amp_peak = y_peak / (FFT_SIZE / 2) as f32;
                    let freq_peak = x_peak * (SAMPLE_RATE as f32 / FFT_SIZE as f32);
                    if amp_peak > amp_threshold {
                        peaks.push((freq_peak, amp_peak));
                        spew!("*** peak", i, x_peak, y_peak, freq_peak, amp_peak, a, b, c, peakiness);
                    } else {
                        if t >= low_show_peak && t < high_show_peak {
                            spew!("... peak", i, x_peak, y_peak, freq_peak, amp_peak, a, b, c, peakiness);
                        }
                    }
                } else {
                    if t >= low_show_peak && t < high_show_peak {
                        let freq = i as f32 * (SAMPLE_RATE as f32 / FFT_SIZE as f32);
                        spew!("--- peak", i, i as f32, self.output[i], freq, self.output[i] / (FFT_SIZE / 2) as f32);
                    }
                }
            }
        }

        for i in 0..NUM_RESOS {
            if i >= peaks.len() {
                break;
            }
            self.resos[i].set_pitch(peaks[i].0);
        }

        for i in 0..input_slice.len() {
            let mut s = input_slice[i];
            for i in 0..NUM_RESOS {
                spew!("reso", i);
                s = self.resos[i].process(s);
            }
            output_slice[i] = s;
            playhead.inc();
        }

        /*
        self.tvob.update(playhead.time_in_samples(), peaks);

        self.tvob.ratio_report();

        for i in 0..input_slice.len() {
            println!("srender {} ", playhead.time_in_samples());
            output_slice[i] = self.tvob.next_sample();
            println!("srender result {} {} ", playhead.time_in_samples(), output_slice[i]);
            playhead.inc();
        }
        */

        /*
        for i in 0..input_slice.len() {
            output_slice[i] = 0.0;
            for (frequency, amp) in &peaks {
                let ph = playhead.sinf(*frequency);
                output_slice[i] += amp * libm::sinf(ph);
            }
            playhead.inc();
        }
        */
    }
}
