extern crate alloc;
extern crate libm;
#[cfg(feature = "for_host")]
extern crate std;

use alloc::boxed::Box;

use crate::knob::Knobs;
use crate::patch::Patch;
use crate::playhead::Playhead;

pub struct Fuzz {
    last_x: f32,
    last_y: f32,
    attack_eagerness: f32,
    decay_eagerness: f32,
}

impl Fuzz {
    pub fn new() -> Fuzz {
        Fuzz {
            last_x: 0.0,
            last_y: 0.0,
            attack_eagerness: 0.90,
            decay_eagerness: 0.001,
        }
    }
}

impl Patch for Fuzz {
    fn rust_process_audio(
        &mut self,
        input_slice: &[f32],
        output_slice: &mut [f32],
        _knobs: &Box<dyn Knobs>,
        mut _playhead: Playhead,
    ) {
        for i in 0..input_slice.len() {
            let x = input_slice[i];


            let env = if x > self.last_y {
                // attack
                // TODO preocmpute 1-blah
                (self.attack_eagerness * x) + ((1.0 - self.attack_eagerness) * self.last_y)
            } else {
                // decay
                (self.decay_eagerness * x) + ((1.0 - self.decay_eagerness) * self.last_y)
            };
            self.last_x = x;
            self.last_y = env;


            let min_env = 0.05;
            //let input_gain = 3.0; // Adjust this so that the min/max below is close to 1
            let abs_env = libm::fabsf(env); // TODO maybe not necessary?
            let clamped_env = if abs_env < min_env { min_env } else { abs_env };
            let input_gain = 0.8 / clamped_env;
            let gain = 7.5;
            let xg = x * input_gain * 10.0 + gain;
            let y = xg / (1.0 + libm::fabsf(xg));


            let wet = 0.1;
            let mix_y = (wet * y) + ((1.0 - wet) * x);
            output_slice[i] = mix_y;

            //playhead.inc();
        }
    }
}
