#[cfg(feature = "for_host")]
extern crate std;

use alloc::boxed::Box;
#[cfg(feature = "for_host")]
#[allow(unused_imports)]
use std::println;

use crate::filter::linear_vibrato::*;
use crate::knob::Knobs;
use crate::patch::Patch;
use crate::playhead::Playhead;

const BATCH_SIZE: usize = 48;

pub struct Chorus {
    vibrato_a: LinearVibrato,
    vibrato_b: LinearVibrato,
    vibrato_c: LinearVibrato,
    buffer: [f32; BATCH_SIZE],
}

impl Chorus {
    pub fn new() -> Chorus {
        let n: f32 = 3.0;
        let d: f32 = 0.3;
        Chorus {
            vibrato_a: LinearVibrato::new(20, n-d, 1),
            vibrato_b: LinearVibrato::new(22, n, 1),
            vibrato_c: LinearVibrato::new(18, n+d, 1),
            buffer: [0.0; BATCH_SIZE],
        }
    }
}

impl Patch for Chorus {
    fn rust_process_audio(
        &mut self,
        input_slice: &[f32],
        output_slice: &mut [f32],
        knobs: &Box<dyn Knobs>,
        playhead: Playhead,
    ) {
        self.vibrato_a.rust_process_audio(input_slice, output_slice, knobs, playhead);
        self.vibrato_b.rust_process_audio(input_slice, &mut self.buffer, knobs, playhead);
        for i in 0..input_slice.len() {
            output_slice[i] += self.buffer[i];
        }
        self.vibrato_c.rust_process_audio(input_slice, &mut self.buffer, knobs, playhead);
        for i in 0..input_slice.len() {
            output_slice[i] += self.buffer[i];
        }
        for i in 0..input_slice.len() {
            output_slice[i] /= 3.0;
        }

        /* This was for figuring out why it was clipping, but it doesn't seem to be
         * clipping now.
        let mut temp0: [f32; BATCH_SIZE] = [0.0; BATCH_SIZE];
        let mut temp1: [f32; BATCH_SIZE] = [0.0; BATCH_SIZE];
        let mut temp2: [f32; BATCH_SIZE] = [0.0; BATCH_SIZE];
        self.vibrato_a.rust_process_audio(input_slice, &mut temp0, knobs, playhead);
        self.vibrato_b.rust_process_audio(input_slice, &mut temp1, knobs, playhead);
        self.vibrato_c.rust_process_audio(input_slice, &mut temp2, knobs, playhead);
        for i in 0..input_slice.len() {
            output_slice[i] /= 3.0;
            if output_slice[i] < -1.0 || output_slice[i] > 1.0 {
#[cfg(feature = "for_host")]
                println!("Overflow {} {} {} {} {}", playhead.time_in_samples()+(i as u32),
                    output_slice[i], temp0[i], temp1[i], temp2[i]);
            }
        }
        */
    }
}

