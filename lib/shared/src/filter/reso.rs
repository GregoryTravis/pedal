extern crate alloc;
extern crate libm;

use alloc::boxed::Box;

use crate::knob::Knobs;
use crate::patch::Patch;
use crate::playhead::Playhead;
use crate::switch::Switches;

pub struct ResoFilter {
    pub freq_knob_id: usize,
    pub q_knob_id: usize,
    pub buf0: f32,
    pub buf1: f32,
}

// From https://www.musicdsp.org/en/latest/Filters/29-resonant-filter.html
impl ResoFilter {
    pub fn new(freq_knob_id: usize, q_knob_id: usize) -> ResoFilter {
        ResoFilter {
            freq_knob_id: freq_knob_id,
            q_knob_id: q_knob_id,
            buf0: 0.0,
            buf1: 0.0,
        }
    }
}

impl Patch for ResoFilter {
    fn rust_process_audio(
        &mut self,
        input_slice: &[f32],
        output_slice: &mut [f32],
        knobs: &Box<dyn Knobs>,
        _switches: &Box<dyn Switches>,
        mut playhead: Playhead,
    ) {
        let freq_knob_value = knobs.read(self.freq_knob_id);
        let q_knob_value = knobs.read(self.q_knob_id);

        let freq_lo = 0.3;
        let freq_hi = 0.9;
        let oscf = freq_lo + (freq_knob_value * (freq_hi - freq_lo));

        let q_lo = 0.4;
        let q_hi = 0.99;
        let q = q_lo + (q_knob_value * (q_hi - q_lo));

        for i in 0..input_slice.len() {
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
