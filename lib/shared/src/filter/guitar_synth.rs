extern crate alloc;
extern crate libm;
#[cfg(feature = "for_host")]
extern crate std;

use alloc::boxed::Box;
use core::f32::consts::PI;
//#[cfg(feature = "for_host")]
//use std::println;

use crate::convert::*;
use crate::constants::*;
use crate::knob::Knobs;
use crate::patch::Patch;
use crate::playhead::Playhead;
#[allow(unused)]
use crate::spew::*;
use crate::yin::*;

const BUFFER_SIZE: usize = 1024;

// Use 0 instead of last pitch, and write the pitch to the output
const PITCH_MODE: bool = false;

pub struct GuitarSynth {
    // TODO put in sdram
    int_buffer: [i16; BUFFER_SIZE],
    last_pitch: f32,

    // Resonant filter history
    pub buf0: f32,
    pub buf1: f32,
}

impl GuitarSynth {
    pub fn new() -> GuitarSynth {
        yin_init(BUFFER_SIZE as i16, 0.05);
        GuitarSynth {
            int_buffer: [0; BUFFER_SIZE],
            last_pitch: 440.0,
            buf0: 0.0,
            buf1: 0.0,
        }
    }
}

impl Patch for GuitarSynth {
    fn rust_process_audio(
        &mut self,
        input_slice: &[f32],
        output_slice: &mut [f32],
        _knobs: &Box<dyn Knobs>,
        mut playhead: Playhead,
    ) {
        // Copy + and convert history, making room for new samples.
        // TODO don't do this.
        let slice_len = input_slice.len();
        for i in 0..BUFFER_SIZE-slice_len {
            self.int_buffer[i] = self.int_buffer[i+slice_len];
        }
        for i in 0..slice_len {
            self.int_buffer[i+BUFFER_SIZE-slice_len] = sample_f32_to_i16(input_slice[i]) >> 6;
        }

        let pitch_maybe: f32 = yin_process(&self.int_buffer);
        let pitch = if PITCH_MODE {
            if pitch_maybe == -1.0 { 0.0 } else { pitch_maybe }
        } else {
            if pitch_maybe == -1.0 { self.last_pitch } else { pitch_maybe }
        };
        self.last_pitch = pitch;

        //spew!("pitch", pitch);

        let oscf: f32 = 2.0 * libm::sinf(PI * (pitch / SAMPLE_RATE as f32));

        //let q_lo = 0.4;
        //let q_hi = 0.99;
        let q = 0.97f32;

        for i in 0..input_slice.len() {
            //let _qq = add(&self.q_sig, &self.q_sig);
            let fb = q + q / (1.0 - oscf);
            let inp = input_slice[i];
            self.buf0 = self.buf0 + oscf * (inp - self.buf0 + fb * (self.buf0 - self.buf1));
            self.buf1 = self.buf1 + oscf * (self.buf0 - self.buf1);
            let out = self.buf1;
            output_slice[i] = out;

            if PITCH_MODE {
                // Render pitch as wav output
                output_slice[i] = pitch / 500.0;
            }

            playhead.inc();
        }
    }
}
