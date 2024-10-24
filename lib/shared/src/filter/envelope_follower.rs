extern crate alloc;
extern crate libm;
#[cfg(feature = "for_host")]
extern crate std;

use alloc::boxed::Box;

use crate::knob::Knobs;
use crate::patch::Patch;
use crate::playhead::Playhead;
use crate::switch::Switches;

pub struct EnvelopeFollower {
    last_x: f32,
    last_y: f32,
    attack_eagerness: f32,
    decay_eagerness: f32,
}

impl EnvelopeFollower  {
    pub fn new() -> EnvelopeFollower {
        EnvelopeFollower {
            last_x: 0.0,
            last_y: 0.0,
            attack_eagerness: 0.90,
            decay_eagerness: 0.001,
        }
    }
}

impl Patch for EnvelopeFollower {
    fn rust_process_audio(
        &mut self,
        input_slice: &[f32],
        output_slice: &mut [f32],
        _knobs: &Box<dyn Knobs>,
        _switches: &Box<dyn Switches>,
        mut _playhead: Playhead,
    ) {
        for i in 0..input_slice.len() {
            let x = input_slice[i];
            let y = if x > self.last_y {
                // attack
                // TODO preocmpute 1-blah
                (self.attack_eagerness * x) + ((1.0 - self.attack_eagerness) * self.last_y)
            } else {
                // decay
                (self.decay_eagerness * x) + ((1.0 - self.decay_eagerness) * self.last_y)
            };
            output_slice[i] = y;

            self.last_x = x;
            self.last_y = y;

            //playhead.inc();
        }
    }
}
