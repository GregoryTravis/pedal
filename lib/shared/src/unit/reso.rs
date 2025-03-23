#[cfg(feature = "for_host")]
extern crate std;
extern crate libm;

use core::f32::consts::PI;

use crate::constants::*;
#[allow(unused)]
use crate::spew::*;

const MAX_PITCH_DELTA_PER_SEC: f32 = 10000000.0;
const MAX_PITCH_DELTA_PER_SAMPLE: f32 = MAX_PITCH_DELTA_PER_SEC / SAMPLE_RATE as f32;

// a is going to b, but no faster than max
fn clip_delta(a: f32, b:f32, max: f32) -> f32 {
    if b > a {
        let delta = (b-a).min(max);
        delta
    } else {
        let delta = (a-b).min(max);
        -delta
    }
}

pub struct Reso {
    target_pitch: f32,
    pitch: f32,
    amp: f32,
    buf0: f32,
    buf1: f32,
}

impl Reso {
    pub fn new() -> Reso {
        Reso {
            target_pitch: 450.0,
            pitch: 450.0,
            amp: 1.0,
            buf0: 0.0,
            buf1: 0.0,
        }
    }

    pub fn set_pitch(&mut self, target_pitch: f32) {
        self.target_pitch = target_pitch;
    }

    pub fn set_amp(&mut self, amp: f32) {
        self.amp = amp;
    }

    fn update(&mut self) {
        let before = self.pitch;
        self.pitch += clip_delta(self.pitch, self.target_pitch, MAX_PITCH_DELTA_PER_SAMPLE);
        spew!("update", before, self.pitch, self.target_pitch, MAX_PITCH_DELTA_PER_SAMPLE);
    }

    pub fn process(&mut self, inp: f32) -> f32 {
        self.update();

        let oscf: f32 = 2.0 * libm::sinf(PI * (self.pitch / SAMPLE_RATE as f32));
        let q = 0.8; // 0.97f32;
        let fb = q + q / (1.0 - oscf);
        self.buf0 = self.buf0 + oscf * (inp - self.buf0 + fb * (self.buf0 - self.buf1));
        self.buf1 = self.buf1 + oscf * (self.buf0 - self.buf1);
        let filtered = self.buf1;
        let out = (self.amp * filtered) + ((1.0 - self.amp) * inp);
        out
    }
}
