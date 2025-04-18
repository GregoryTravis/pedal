#[cfg(feature = "for_host")]
extern crate std;
extern crate libm;

use alloc::boxed::Box;
//use alloc::vec::Vec;
use core::f32::consts::PI;
#[allow(unused)]
use std::println;

//use crate::constants::*;
use crate::knob::Knobs;
use crate::patch::Patch;
use crate::playhead::Playhead;
#[allow(unused)]
use crate::spew::*;

pub struct GuitarSynth {
}

impl GuitarSynth {
    pub fn new() -> GuitarSynth {
        GuitarSynth {
        }
    }
}

// Hann window
// w(n) = 0.5 * [1 - cos(2*pi*n / N)]
// Usage;
//   self.input[i] *= hann(i, FFT_SIZE);
#[allow(unused)]
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

        for i in 0..input_slice.len() {
            output_slice[i] = input_slice[i];
            playhead.inc();
        }
    }
}
