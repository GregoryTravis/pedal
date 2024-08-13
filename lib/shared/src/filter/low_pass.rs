use alloc::boxed::Box;

use crate::knob::Knobs;
use crate::patch::Patch;
use crate::playhead::Playhead;

pub struct LowPassFilter {
    pub state: f32,
}

impl LowPassFilter {
    pub fn new() -> LowPassFilter {
        LowPassFilter { state: 0.0 }
    }
}

impl Patch for LowPassFilter {
    fn rust_process_audio(
        &mut self,
        input_slice: &[f32],
        output_slice: &mut [f32],
        _knobs: &Box<dyn Knobs>,
        _playhead: Playhead,
    ) {
        for i in 0..input_slice.len() {
            output_slice[i] = 1.0 * ((input_slice[i] + self.state) / 2.0);
            self.state = input_slice[i];
        }
    }
}
