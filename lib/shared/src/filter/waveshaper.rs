extern crate alloc;
extern crate libm;
#[cfg(feature = "for_host")]
extern crate std;

use alloc::boxed::Box;
#[cfg(feature = "for_host")]
use std::println;

use crate::knob::Knobs;
use crate::patch::Patch;
use crate::playhead::Playhead;
use crate::switch::Switches;

pub struct WaveShaper {
#[cfg(feature = "for_host")]
    min: f32,
#[cfg(feature = "for_host")]
    max: f32,
}

impl WaveShaper {
    pub fn new() -> WaveShaper {
        WaveShaper {
#[cfg(feature = "for_host")]
            min: 0.0,
#[cfg(feature = "for_host")]
            max: 0.0,
        }
    }
}

impl Patch for WaveShaper {
    fn rust_process_audio(
        &mut self,
        input_slice: &[f32],
        output_slice: &mut [f32],
        _knobs: &Box<dyn Knobs>,
        _switches: &Box<dyn Switches>,
        mut _playhead: Playhead,
    ) {
        for i in 0..input_slice.len() {
            let s = input_slice[i];
            let input_gain = 3.0; // Adjust this so that the min/max below is close to 1
            let gain = 7.5;
            let x = s * input_gain * 10.0 + gain;
            let y = x / (1.0 + libm::fabsf(x));
            output_slice[i] = y;

#[cfg(feature = "for_host")]
            {
                let mut wider = false;
                if s < self.min {
                    self.min = s;
                    wider = true;
                } else if s > self.max {
                    self.max = s;
                    wider = true;
                }
                if wider {
                    println!("min max {} {}", self.min, self.max);
                }
            }

            //playhead.inc();
        }
    }
}
