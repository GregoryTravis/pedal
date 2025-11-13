#[cfg(feature = "for_host")]
extern crate std;
extern crate libm;

use core::f32::consts::PI;

use crate::constants::*;
use crate::filter::sine_table::*;
#[allow(unused)]
use crate::spew::*;

// Thanks of course to https://webaudio.github.io/Audio-EQ-Cookbook/Audio-EQ-Cookbook.txt

pub struct BandPass {
    // Center frequency
    freq: f32,
    bw: f32,

    // Filter params
    b0_over_a0: f32,
    b1_over_a0: f32,
    b2_over_a0: f32,
    a1_over_a0: f32,
    a2_over_a0: f32,

    // History
    x_n_1: f32,
    x_n_2: f32,
    y_n_1: f32,
    y_n_2: f32,
}

// First time I used AI to solve an equation.
fn fast_sinh(x: f32) -> f32 {
  (0.20335755098 * x * x * x) + x
}

impl BandPass {
    // Center frequency; bandwidth in octaves between -3 frequencies
    pub fn new(freq: f32, bw: f32) -> BandPass {
        let mut bp = BandPass {
            freq: 0.0,
            bw: bw,

            b0_over_a0: 0.0,
            b1_over_a0: 0.0,
            b2_over_a0: 0.0,
            a1_over_a0: 0.0,
            a2_over_a0: 0.0,

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
        let sin_w0 = table_sin(w0);
        let alpha = sin_w0 * fast_sinh(
                (libm::logf(2.0)/2.0) * self.bw * (w0 / sin_w0) );
        self.freq = freq;

        // Constant skirt gain
        // b0:  libm::sinf(w0)/2.0,
        // b1:  0.0,
        // b2: -(libm::sinf(w0)/2.0),
        // a0:  1.0 + alpha,
        // a1: -2.0 * libm::cosf(w0),
        // a2:  1.0 - alpha,

        // Constant peak 0 db gain
        let b0 =  alpha;
        let b1 =  0.0;
        let b2 = -alpha;
        let a0 =  1.0 + alpha;
        let a1 = -2.0*libm::cosf(w0);
        let a2 =  1.0 - alpha;

        self.b0_over_a0 = b0 / a0;
        self.b1_over_a0 = b1 / a0;
        self.b2_over_a0 = b2 / a0;
        self.a1_over_a0 = a1 / a0;
        self.a2_over_a0 = a2 / a0;
    }

    pub fn process(&mut self, x_n: f32) -> f32 {
        let y_n = (self.b0_over_a0*x_n) + (self.b1_over_a0*self.x_n_1) + (self.b2_over_a0*self.x_n_2)
                  - (self.a1_over_a0*self.y_n_1) - (self.a2_over_a0*self.y_n_2);
        // Shift history
        self.x_n_2 = self.x_n_1;
        self.x_n_1 = x_n;
        self.y_n_2 = self.y_n_1;
        self.y_n_1 = y_n;
        y_n
    }
}
