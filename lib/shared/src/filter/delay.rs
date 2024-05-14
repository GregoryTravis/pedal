extern crate alloc;
extern crate libm;

use circular_buffer::CircularBuffer;

use crate::patch::Patch;
use crate::playhead::Playhead;

pub struct Delay {
    cbuf: CircularBuffer::<48, f32>,
}

impl Delay {
    pub fn new() -> Delay {
        Delay { cbuf: CircularBuffer::<48, f32>::new() }
    }
}

impl Patch for Delay {
    fn rust_process_audio(
        &mut self,
        input_slice: &[f32],
        output_slice: &mut [f32],
        mut _playhead: Playhead,
    ) {
        for i in 0..input_slice.len() {
            let inp = input_slice[i];
            let out: f32;
            if self.cbuf.len() < 48 {
                out = 0.0;
            } else {
                out = *self.cbuf.front().unwrap();
            }
            self.cbuf.push_back(inp);
            output_slice[i] = out;
        }
    }
}
