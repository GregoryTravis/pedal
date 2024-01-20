extern crate alloc;
extern crate libm;

use alloc::sync::Arc;

use crate::patch::Patch;
use crate::playhead::Playhead;
use crate::signal::Signal;
use crate::signal::combinators::*;

pub struct ResoFilter {
    pub buf0: f32,
    pub buf1: f32,
    pub q_sig: Arc<dyn Signal<f32>>,
    pub siner: Arc<dyn Signal<f32>>,
}

// From https://www.musicdsp.org/en/latest/Filters/29-resonant-filter.html
impl ResoFilter {
    pub fn new(siner: Arc<dyn Signal<f32>>, q_sig: Arc<dyn Signal<f32>>) -> ResoFilter {
        ResoFilter {
            buf0: 0.0,
            buf1: 0.0,
            q_sig: q_sig,
            siner: siner,
        }
    }
}

impl Patch for ResoFilter {
    fn rust_process_audio(
        &mut self,
        input_slice: &[f32],
        output_slice: &mut [f32],
        mut playhead: Playhead,
    ) {
        for i in 0..input_slice.len() {
            // Rolls over every 68 years
            let t = playhead.time_in_seconds() as f32;
            let oscf = self.siner.f(t);
            let q = self.q_sig.f(t);
            //let _qq = add(&self.q_sig, &self.q_sig);
            let fb = q + q / (1.0 - oscf);
            let inp = input_slice[i];
            self.buf0 = self.buf0 + oscf * (inp - self.buf0 + fb * (self.buf0 - self.buf1));
            self.buf1 = self.buf1 + oscf * (self.buf0 - self.buf1);
            let out = self.buf1;
            output_slice[i] = out;
            playhead.increment_samples(1);
        }
    }
}
