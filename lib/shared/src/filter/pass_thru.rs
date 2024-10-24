use alloc::boxed::Box;

use crate::knob::Knobs;
use crate::patch::Patch;
use crate::playhead::Playhead;
use crate::switch::Switches;

pub struct PassThruFilter {}

impl PassThruFilter {
    pub fn new() -> PassThruFilter {
        PassThruFilter {}
    }
}

impl Patch for PassThruFilter {
    fn rust_process_audio(
        &mut self,
        input_slice: &[f32],
        output_slice: &mut [f32],
        _knobs: &Box<dyn Knobs>,
        _switches: &Box<dyn Switches>,
        mut _playhead: Playhead,
    ) {
        for i in 0..input_slice.len() {
            output_slice[i] = input_slice[i];
        }
    }
}
