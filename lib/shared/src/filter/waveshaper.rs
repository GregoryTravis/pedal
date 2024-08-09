extern crate alloc;
extern crate libm;
#[cfg(feature = "for_host")]
extern crate std;

use alloc::boxed::Box;
use std::println;

use crate::knob::Knobs;
use crate::patch::Patch;
use crate::playhead::Playhead;

pub struct WaveShaper {
    min: f32,
    max: f32,
}

impl WaveShaper {
    pub fn new() -> WaveShaper {
        WaveShaper {
            min: 0.0,
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
        mut _playhead: Playhead,
    ) {
        for i in 0..input_slice.len() {
            let mut s = input_slice[i];
            s *= 3.0;

            /*
            if s >= 0.0 {
                output_slice[i] = s*s;
            } else {
                output_slice[i] = -s*s;
            }
            output_slice[i] *= 10.0;
            */

            /*
            // x -3.75 .. 1.75 (totes 5.50)
            // / 12
            let s_0_1 = (s + 1.0) / 2.0;
            let x = (s_0_1 * 5.5) - 3.75;
            let mut y = x*x*x + 3.0*x*x - 3.0*x + 1.0;
            y = (y / 6.0) - 1.0;
            output_slice[i] = y;
            */

            let lo = -3.825;
            let hi = 1.85;
            // -3.75..2
            let x = (s * (hi - lo)) + lo;
            let y = ((x*x*x + 3.0*x*x - 3.0*x + 1.0) / 6.0) - 1.0;

            output_slice[i] = y;

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

            //playhead.inc();
        }
    }
}
