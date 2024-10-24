use alloc::boxed::Box;

use crate::knob::Knobs;
use crate::patch::Patch;
use crate::playhead::Playhead;
use crate::switch::Switches;

pub struct HighPassFilter {
    pub state: f32,
}

impl HighPassFilter {
    pub fn new() -> HighPassFilter {
        HighPassFilter { state: 0.0 }
    }
}

impl Patch for HighPassFilter {
    fn rust_process_audio(
        &mut self,
        input_slice: &[f32],
        output_slice: &mut [f32],
        _knobs: &Box<dyn Knobs>,
        _switches: &Box<dyn Switches>,
        _playhead: Playhead,
    ) {
        for i in 0..input_slice.len() {
            output_slice[i] = 5.0 * ((input_slice[i] - self.state) / 2.0);
            self.state = input_slice[i];
        }
    }
}
