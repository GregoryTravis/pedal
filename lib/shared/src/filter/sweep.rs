extern crate alloc;
extern crate libm;

use alloc::boxed::Box;
use alloc::sync::Arc;

//use crate::knob_board::*;
use crate::knob::Knobs;
use crate::patch::Patch;
use crate::playhead::Playhead;
use crate::signal::Signal;
//use crate::signal::combinators::*;

pub struct SweepFilter {
    pub buf0: f32,
    pub buf1: f32,
    pub q_sig: Arc<dyn Signal<f32>>,
    pub siner: Arc<dyn Signal<f32>>,
}

// From https://www.musicdsp.org/en/latest/Filters/29-resonant-filter.html
impl SweepFilter {
    pub fn new(siner: Arc<dyn Signal<f32>>, q_sig: Arc<dyn Signal<f32>>) -> SweepFilter {
        SweepFilter {
            buf0: 0.0,
            buf1: 0.0,
            q_sig: q_sig,
            siner: siner,
        }
    }
}

impl Patch for SweepFilter {
    fn rust_process_audio(
        &mut self,
        input_slice: &[f32],
        output_slice: &mut [f32],
        _knobs: &Box<dyn Knobs>,
        mut playhead: Playhead,
    ) {
        /*
        let freq_knob_value = knobs.read(0);
        let q_knob_value = knobs.read(3);

        let freq_lo = 0.3;
        let freq_hi = 0.9;
        let oscf = freq_lo + (freq_knob_value * (freq_hi - freq_lo));

        let q_lo = 0.4;
        let q_hi = 0.99;
        let q = q_lo + (q_knob_value * (q_hi - q_lo));
        */

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
            playhead.inc();
        }
    }
}
