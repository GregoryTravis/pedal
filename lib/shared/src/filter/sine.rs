extern crate libm;

use alloc::boxed::Box;

use crate::knob::Knobs;
use crate::patch::Patch;
use crate::playhead::Playhead;

// Ignores its input, just generates a sine.

pub struct SineGenerator {
    hz: f32,
}

impl SineGenerator {
    pub fn new(hz: f32) -> SineGenerator {
        SineGenerator { hz: hz }
    }
}

impl Patch for SineGenerator {
    fn rust_process_audio(
        &mut self,
        _input_slice: &[f32],
        output_slice: &mut [f32],
        _knobs: &Box<dyn Knobs>,
        mut playhead: Playhead,
    ) {
        for i in 0..output_slice.len() {
            // TODO should get sampling rate from env
            // TODO this is ragged
            output_slice[i] = playhead.sinf(self.hz) / 32.0;
            playhead.inc();
        }
    }
}
