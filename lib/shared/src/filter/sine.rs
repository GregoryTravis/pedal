extern crate libm;

use crate::patch::Patch;
use crate::playhead::Playhead;

// Ignores its input, just generates a sine.

pub struct SineGenerator {}

impl SineGenerator {
    pub fn new() -> SineGenerator {
        SineGenerator {}
    }
}

impl Patch for SineGenerator {
    fn rust_process_audio(
        &mut self,
        _input_slice: &[f32],
        output_slice: &mut [f32],
        playhead: Playhead,
    ) {
        for i in 0..output_slice.len() {
            // TODO should get sampling rate from env
            // TODO this is ragged
            output_slice[i] = playhead.sinf(9000.0) / 32.0;
        }
    }
}
