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

fn sinc(x: f32) -> f32 {
    // TODO how on earth does this work?
    let small = 0.000000000000000000000000000000000000000000001;
    if x < small || x > -small {
        1.0
    } else {
        libm::sinf(x) / x
    }
}

pub struct Vibrato {
    // The fractional playhead can only deviate from the regular one by this much on either side.
    // Range is *exclusive*. -- TODO ??
    max_sample_deviation: usize,
    // Hz
    vibrato_frequency: f32,

    buffer_length: usize,
    now_index: usize,
    cbuf: CircBuf::<f32>,
}

impl Vibrato {
    pub fn new(max_sample_deviation: usize, vibrato_frequency: f32) -> Vibrato {
        let buffer_length: usize = 2 * (max_sample_deviation + NUM_SINC_TAPS_ONE_SIDE + GUARD_SAMPLES) + 1;
        let now_index: usize = max_sample_deviation + NUM_SINC_TAPS_ONE_SIDE + GUARD_SAMPLES;

        let vibrato = Vibrato {
            max_sample_deviation: max_sample_deviation,
            vibrato_frequency: vibrato_frequency,
            buffer_length: buffer_length,
            now_index: now_index,
            cbuf: CircBuf::<f32>::new(buffer_length, 0.0)
        };
        vibrato
    }
}

impl Patch for Vibrato {
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
            let window_low_f = fph - (NUM_SINC_TAPS_ONE_SIDE as f32);
            let window_high_f = fph + (NUM_SINC_TAPS_ONE_SIDE as f32);
            assert!(window_low_f > 0.0);
            assert!(window_high_f < self.buffer_length as f32);
            let window_low_i = libm::ceilf(window_low_f) as usize;
            let window_high_i = libm::floorf(window_high_f) as usize;
            assert!(window_low_i < window_high_i);
            let mut convolution_sum: f32 = 0.0;
            for si in window_low_i..(window_high_i+1) {
                let sinc_x = fph - (si as f32);
                let sinc_value = sinc(sinc_x);
                let si_sample = self.cbuf.get(si);
                convolution_sum += sinc_value * si_sample;
            }
            convolution_sum /= 2.0;
            /*
#[cfg(feature = "for_host")]
            if !(convolution_sum <= 1.0 && convolution_sum >= -1.0) {
                println!("Overflow {}", convolution_sum);
            }
            */
            output_slice[i] = convolution_sum;
            playhead.inc();
        }
    }
}
