#[cfg(feature = "for_host")]
extern crate std;

use core::any::Any;
use alloc::boxed::Box;

use crate::knob::Knobs;
use crate::patch::Patch;
use crate::playhead::Playhead;

pub struct Gain {
    gain: f32,
}

impl Gain {
    pub fn new(gain: f32) -> Gain {
        Gain {
            gain: gain,
        }
    }
}

impl Patch for Gain {
    fn rust_process_audio(
        &mut self,
        input_slice: &[f32],
        output_slice: &mut [f32],
        _knobs: &Box<dyn Knobs>,
        _playhead: Playhead,
    ) {
        for i in 0..input_slice.len() {
            output_slice[i] = self.gain * input_slice[i];
        }
    }

    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}
