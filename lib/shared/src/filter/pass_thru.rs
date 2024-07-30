use crate::patch::Patch;
use crate::playhead::Playhead;

use core::any::Any;

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
        mut _playhead: Playhead,
    ) {
        for i in 0..input_slice.len() {
            output_slice[i] = input_slice[i];
        }
    }

    //fn as_any<'a>(&self) -> &(dyn Any + 'a) { self }
    fn as_any(&self) -> &dyn Any { self }
}
