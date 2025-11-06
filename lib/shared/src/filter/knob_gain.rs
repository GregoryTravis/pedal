#[cfg(feature = "for_host")]
extern crate std;

use core::any::Any;
use alloc::boxed::Box;

use crate::knob::Knobs;
use crate::patch::Patch;
use crate::playhead::Playhead;

pub struct KnobGain {
    knob_id: usize,
    low: f32,
    high: f32,
    gain: f32,
}

impl KnobGain {
    pub fn new(knob_id: usize, low: f32, high: f32) -> KnobGain {
        KnobGain {
            knob_id: knob_id,
            low: low,
            high: high,
            gain: 0.5,
        }
    }
}

impl Patch for KnobGain {
    fn rust_process_audio(
        &mut self,
        input_slice: &[f32],
        output_slice: &mut [f32],
        knobs: &Box<dyn Knobs>,
        _playhead: Playhead,
    ) {
        for i in 0..input_slice.len() {
            let alpha = knobs.read(self.knob_id);
            self.gain = ((1.0 - alpha) * self.low) + (alpha * self.high);
            output_slice[i] = self.gain * input_slice[i];
        }
    }

    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}
