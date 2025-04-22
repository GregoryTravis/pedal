#[cfg(feature = "for_host")]
extern crate std;

use core::any::Any;
use alloc::boxed::Box;

use crate::knob::Knobs;
use crate::patch::Patch;
use crate::playhead::Playhead;

pub struct Gain {
    knob_id: usize,
    gain: f32,
}

impl Gain {
    pub fn new(knob_id: usize) -> Gain {
        Gain {
            knob_id: knob_id,
            gain: 0.5,
        }
    }
}

impl Patch for Gain {
    fn rust_process_audio(
        &mut self,
        input_slice: &[f32],
        output_slice: &mut [f32],
        knobs: &Box<dyn Knobs>,
        _playhead: Playhead,
    ) {
        for i in 0..input_slice.len() {
            self.gain = knobs.read(self.knob_id);
            output_slice[i] = self.gain * input_slice[i];
        }
    }

    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}
