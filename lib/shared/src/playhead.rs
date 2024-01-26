use core::f32::consts::PI;

use crate::constants::SAMPLE_RATE;

#[derive(Debug, Copy, Clone)]
pub struct Playhead {
    time_in_samples: u64,
}

impl Playhead {
    pub fn new() -> Playhead {
        Playhead { time_in_samples: 0 }
    }

    pub fn time_in_samples(&self) -> u64 {
        self.time_in_samples
    }

    pub fn time_in_seconds(&self) -> f64 {
        // TODO: un-hardcode sampling rate
        (self.time_in_samples as f64) / (SAMPLE_RATE as f64)
    }

    pub fn sinf(&self, hz: f32) -> f32 {
        // TODO this rolls over if you let the pedal run for 68 years, not good
        let radians: f32 = (self.time_in_seconds() as f32) * hz * 2.0 * PI;
        libm::sinf(radians)
    }

    pub fn increment_samples(&mut self, delta_samples: u64) {
        self.time_in_samples += delta_samples;
    }

    pub fn inc(&mut self) {
        self.increment_samples(1);
    }
}
