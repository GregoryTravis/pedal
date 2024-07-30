use crate::patch::Patch;
use crate::playhead::Playhead;

use core::any::Any;

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
        _playhead: Playhead,
    ) {
        for i in 0..input_slice.len() {
            output_slice[i] = 5.0 * ((input_slice[i] - self.state) / 2.0);
            self.state = input_slice[i];
        }
    }

    //fn as_any<'a>(&self) -> &(dyn Any + 'a) { self }
    fn as_any(&self) -> &dyn Any { self }
}
