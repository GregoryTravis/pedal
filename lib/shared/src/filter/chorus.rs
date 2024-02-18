use crate::filter::tremolo::*;
use crate::patch::Patch;
use crate::playhead::Playhead;

const BATCH_SIZE: usize = 4;

pub struct Chorus {
    tremolo_a: Tremolo,
    tremolo_b: Tremolo,
    tremolo_c: Tremolo,
}

impl Chorus {
    pub fn new() -> Chorus {
        Chorus {
            tremolo_a: Tremolo::new(400, 1.0),
            tremolo_b: Tremolo::new(300, 0.9),
            tremolo_c: Tremolo::new(500, 1.1),
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
    }
}

