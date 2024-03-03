#[cfg(feature = "for_host")]
extern crate std;

use crate::filter::tremolo::*;
use crate::patch::Patch;
use crate::playhead::Playhead;

#[cfg(feature = "for_host")]
use std::println;

const BATCH_SIZE: usize = 4;

pub struct Chorus {
    tremolo_a: Tremolo,
    tremolo_b: Tremolo,
    tremolo_c: Tremolo,
}

impl Chorus {
    pub fn new() -> Chorus {
        let n: f32 = 3.0;
        let d: f32 = 0.3;
        Chorus {
            tremolo_a: Tremolo::new(20, n-d),
            tremolo_b: Tremolo::new(22, n),
            tremolo_c: Tremolo::new(18, n+d),
        }
    }
}

impl Patch for Chorus {
    fn rust_process_audio(
        &mut self,
        input_slice: &[f32],
        output_slice: &mut [f32],
        playhead: Playhead,
    ) {
        let mut temp: [f32; BATCH_SIZE] = [0.0; BATCH_SIZE];
        self.tremolo_a.rust_process_audio(input_slice, output_slice, playhead);
        self.tremolo_b.rust_process_audio(input_slice, &mut temp, playhead);
        for i in 0..input_slice.len() {
            output_slice[i] += temp[i];
        }
        self.tremolo_c.rust_process_audio(input_slice, &mut temp, playhead);
        for i in 0..input_slice.len() {
            output_slice[i] += temp[i];
        }
        for i in 0..input_slice.len() {
            output_slice[i] /= 3.0;
        }

        let mut temp0: [f32; BATCH_SIZE] = [0.0; BATCH_SIZE];
        let mut temp1: [f32; BATCH_SIZE] = [0.0; BATCH_SIZE];
        let mut temp2: [f32; BATCH_SIZE] = [0.0; BATCH_SIZE];
        self.tremolo_a.rust_process_audio(input_slice, &mut temp0, playhead);
        self.tremolo_b.rust_process_audio(input_slice, &mut temp1, playhead);
        self.tremolo_c.rust_process_audio(input_slice, &mut temp2, playhead);
        for i in 0..input_slice.len() {
            output_slice[i] /= 3.0;
            if output_slice[i] < -1.0 || output_slice[i] > 1.0 {
#[cfg(feature = "for_host")]
                println!("Overflow {} {} {} {} {}", playhead.time_in_samples()+(i as u64),
                    output_slice[i], temp0[i], temp1[i], temp2[i]);
            }
        }

    }
}

