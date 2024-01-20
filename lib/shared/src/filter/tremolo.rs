extern crate alloc;
extern crate libm;

use circular_buffer::CircularBuffer;

use crate::patch::Patch;
use crate::playhead::Playhead;

// The fractional playhead can only deviate from the regular one by this much on either side.
// Range is *exclusive*. -- TODO ??
const _MAX_SAMPLE_DEVIATION: i32 = 10;

// The sinc convolution window is twice this.
const _NUM_SINC_TAPS_ONE_SIDE: i32 = 5;

// Add this many samples on either side to prevent under/overruns in production. Should
// pass rigorous testing with this set to 0, though.
const _GUARD_SAMPLES: i32 = 0;

pub struct Tremolo {
    #[allow(dead_code)] // TODO
    cbuf: CircularBuffer::<4, f32>,
}

impl Tremolo {
    pub fn new() -> Tremolo {
        Tremolo { cbuf: CircularBuffer::<4, f32>::new() }
    }
}

impl Patch for Tremolo {
    fn rust_process_audio(
        &mut self,
        input_slice: &[f32],
        output_slice: &mut [f32],
        mut _playhead: Playhead,
    ) {
        for i in 0..input_slice.len() {
            output_slice[i] = input_slice[i];
        }
    }
}
