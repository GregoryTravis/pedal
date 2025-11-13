#[cfg(feature = "for_host")]
extern crate std;
extern crate libm;

use alloc::vec::Vec;

#[allow(unused)]
use crate::spew::*;
use crate::unit::band_pass::*;

// Thanks of course to https://webaudio.github.io/Audio-EQ-Cookbook/Audio-EQ-Cookbook.txt

const NUM_OVERTONES: usize = 6;

const OVERTONES: [f32; NUM_OVERTONES] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];

pub struct BandPassStackSlow {
    // Center frequency
    bps: Vec<BandPass>,
}

impl BandPassStackSlow {
    // Center frequency; bandwidth in octaves between -3 frequencies
    pub fn new(freq: f32, bw: f32) -> BandPassStackSlow {
        let mut bp = BandPassStackSlow {
            bps: Vec::new(),
        };

        for i in 0..NUM_OVERTONES {
            bp.bps.push(BandPass::new(freq * OVERTONES[i], bw));
        }

        bp
    }

    pub fn set_freq(&mut self, freq: f32) {
        for i in 0..NUM_OVERTONES {
            self.bps[i].set_freq(freq * OVERTONES[i]);
        }
    }

    pub fn process(&mut self, x: f32) -> f32 {
        let mut out: f32 = 0.0;
        for i in 0..NUM_OVERTONES {
            out += self.bps[i].process(x);
        }
        out
    }
}
