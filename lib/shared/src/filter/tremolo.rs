extern crate alloc;
extern crate libm;

use core::f32::consts::PI;

use circular_buffer::CircularBuffer;

use crate::patch::Patch;
use crate::playhead::Playhead;

// The fractional playhead can only deviate from the regular one by this much on either side.
// Range is *exclusive*. -- TODO ??
const MAX_SAMPLE_DEVIATION: usize = 400;

// The sinc convolution window is twice this.
const NUM_SINC_TAPS_ONE_SIDE: usize = 3;

// Add this many samples on either side to prevent under/overruns in production. Should
// pass rigorous testing with this set to 0, though.
const GUARD_SAMPLES: usize = 1;

const BUFFER_LENGTH: usize = 2 * (MAX_SAMPLE_DEVIATION + NUM_SINC_TAPS_ONE_SIDE + GUARD_SAMPLES) + 1;

const NOW_INDEX: usize = MAX_SAMPLE_DEVIATION + NUM_SINC_TAPS_ONE_SIDE + GUARD_SAMPLES;

// Hz
const TREMOLO_FREQUENCY: f32 = 1.0;

fn sinc(x: f32) -> f32 {
    if x == 0.0 {
        1.0
    } else {
        libm::sinf(x) / x
    }
}

pub struct Tremolo {
    #[allow(dead_code)] // TODO
    cbuf: CircularBuffer::<BUFFER_LENGTH, f32>,
}

impl Tremolo {
    pub fn new() -> Tremolo {
        let mut tremolo = Tremolo { cbuf: CircularBuffer::<BUFFER_LENGTH, f32>::new() };
        for _i in 0..BUFFER_LENGTH {
            tremolo.cbuf.push_back(0.0);
        }
        tremolo
    }
}

impl Patch for Tremolo {
    fn rust_process_audio(
        &mut self,
        input_slice: &[f32],
        output_slice: &mut [f32],
        mut playhead: Playhead,
    ) {
        for i in 0..input_slice.len() {
            self.cbuf.push_back(input_slice[i]);
            let tis = playhead.time_in_seconds();
            let tremolo_deviation = libm::sin(
                tis * TREMOLO_FREQUENCY as f64 * 2.0 * PI as f64) * (MAX_SAMPLE_DEVIATION as f64);
            // Fractional playhead
            let fph = (NOW_INDEX as f32) + tremolo_deviation as f32;
            let window_low_f = fph - (NUM_SINC_TAPS_ONE_SIDE as f32);
            let window_high_f = fph + (NUM_SINC_TAPS_ONE_SIDE as f32);
            assert!(window_low_f > 0.0);
            assert!(window_high_f < BUFFER_LENGTH as f32);
            let window_low_i = libm::ceilf(window_low_f) as usize;
            let window_high_i = libm::floorf(window_high_f) as usize;
            assert!(window_low_i < window_high_i);
            let mut convolution_sum: f32 = 0.0;
            for si in window_low_i..(window_high_i+1) {
                let sinc_x = fph - (si as f32);
                let sinc_value = sinc(sinc_x);
                let si_sample = self.cbuf.get(si).unwrap();
                convolution_sum += sinc_value * si_sample;
            }
            output_slice[i] = convolution_sum;
            playhead.inc();
        }
    }
}
