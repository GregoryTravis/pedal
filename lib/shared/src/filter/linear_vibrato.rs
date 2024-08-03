extern crate alloc;
extern crate libm;
#[cfg(feature = "for_host")]
extern crate std;

use alloc::boxed::Box;
use core::f32::consts::PI;

use crate::ds::circbuf::CircBuf;
use crate::knob::Knobs;
use crate::patch::Patch;
use crate::playhead::Playhead;

//#[cfg(feature = "for_host")]
//use std::println;

// The sinc convolution window is twice this.
const NUM_SINC_TAPS_ONE_SIDE: usize = 3;

// Add this many samples on either side to prevent under/overruns in production. Should
// pass rigorous testing with this set to 0, though.
const GUARD_SAMPLES: usize = 1;

pub struct LinearVibrato {
    // The fractional playhead can only deviate from the regular one by this much on either side.
    // Range is *exclusive*. -- TODO ??
    max_sample_deviation: usize,
    // Hz
    vibrato_frequency: f32,

    now_index: usize,
    cbuf: CircBuf::<f32>,
}

impl LinearVibrato {
    pub fn new(max_sample_deviation: usize, vibrato_frequency: f32) -> LinearVibrato {
        let buffer_length: usize = 2 * (max_sample_deviation + NUM_SINC_TAPS_ONE_SIDE + GUARD_SAMPLES) + 1;
        let now_index: usize = max_sample_deviation + NUM_SINC_TAPS_ONE_SIDE + GUARD_SAMPLES;

        let vibrato = LinearVibrato {
            max_sample_deviation: max_sample_deviation,
            vibrato_frequency: vibrato_frequency,
            now_index: now_index,
            cbuf: CircBuf::<f32>::new(buffer_length, 0.0)
        };
        vibrato
    }
}

impl Patch for LinearVibrato {
    fn rust_process_audio(
        &mut self,
        input_slice: &[f32],
        output_slice: &mut [f32],
        _knobs: &Box<dyn Knobs>,
        mut playhead: Playhead,
    ) {
        for i in 0..input_slice.len() {
            self.cbuf.push(input_slice[i]);
            let tis = playhead.time_in_seconds();
            let vibrato_deviation = libm::sinf(
                tis * self.vibrato_frequency as f32 * 2.0 * PI as f32) * (self.max_sample_deviation as f32);
            // Fractional playhead
            let fph = (self.now_index as f32) + vibrato_deviation as f32;
            let fph_floor = libm::floorf(fph) as usize;
            let fph_ceiling = fph_floor + 1;
            let alpha = fph - (fph_floor as f32);
            let low_sample = self.cbuf.get(fph_floor);
            let high_sample = self.cbuf.get(fph_ceiling);
            let interped = (low_sample * (1.0 - alpha)) + (high_sample * alpha);
            output_slice[i] = interped;
            playhead.inc();
        }
    }
}
