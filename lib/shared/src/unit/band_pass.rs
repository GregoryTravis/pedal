#[cfg(feature = "for_host")]
extern crate std;
extern crate libm;

use core::f32::consts::PI;

use crate::constants::*;
#[allow(unused)]
use crate::spew::*;

const GAIN: f32 = 1.0; // I don't know why this is needed

// Thanks of course to https://webaudio.github.io/Audio-EQ-Cookbook/Audio-EQ-Cookbook.txt

pub struct BandPass {
    // Center frequency
    freq: f32,
    bw: f32,

    // Filter params
    b0: f32,
    b1: f32,
    b2: f32,
    a0: f32,
    a1: f32,
    a2: f32,

    // History
    x_n_1: f32,
    x_n_2: f32,
    y_n_1: f32,
    y_n_2: f32,
}

impl BandPass {
    // Center frequency; bandwidth in octaves between -3 frequencies
    pub fn new(freq: f32, bw: f32) -> BandPass {
        let mut bp = BandPass {
            freq: 0.0,
            bw: bw,

            b0: 0.0,
            b1: 0.0,
            b2: 0.0,
            a0: 0.0,
            a1: 0.0,
            a2: 0.0,

            x_n_1: 0.0,
            x_n_2: 0.0,
            y_n_1: 0.0,
            y_n_2: 0.0,
        };

        bp.set_freq(freq);
        bp
    }

    pub fn get_freq(&self) -> f32 { self.freq }

    pub fn set_freq(&mut self, freq: f32) {
        let w0 = 2.0 * PI * (freq / SAMPLE_RATE as f32);
        let alpha = libm::sinf(w0) * libm::sinhf(
                (libm::logf(2.0)/2.0) * self.bw * (w0 / libm::sinf(w0)) );
        self.freq = freq;

        // Constant skirt gain
        // b0:  libm::sinf(w0)/2.0,
        // b1:  0.0,
        // b2: -(libm::sinf(w0)/2.0),
        // a0:  1.0 + alpha,
        // a1: -2.0 * libm::cosf(w0),
        // a2:  1.0 - alpha,

        // Constant peak 0 db gain
        self.b0 =  alpha;
        self.b1 =  0.0;
        self.b2 = -alpha;
        self.a0 =  1.0 + alpha;
        self.a1 = -2.0*libm::cosf(w0);
        self.a2 =  1.0 - alpha;
    }

    pub fn process(&mut self, x_n: f32) -> f32 {
        let y_n = ((self.b0/self.a0)*x_n) + ((self.b1/self.a0)*self.x_n_1) + ((self.b2/self.a0)*self.x_n_2)
                  - ((self.a1/self.a0)*self.y_n_1) - ((self.a2/self.a0)*self.y_n_2);
        // Shift history
        self.x_n_2 = self.x_n_1;
        self.x_n_1 = x_n;
        self.y_n_2 = self.y_n_1;
        self.y_n_1 = y_n;
        y_n * GAIN
    }
}
