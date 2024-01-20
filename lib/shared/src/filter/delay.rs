extern crate alloc;
extern crate libm;

use circular_buffer::CircularBuffer;

use crate::patch::Patch;
use crate::playhead::Playhead;

pub struct Delay {
    cbuf: CircularBuffer::<4, f32>,
}

impl Delay {
    pub fn new() -> Delay {
        Delay { cbuf: CircularBuffer::<4, f32>::new() }
    }
}

impl Patch for Delay {
    fn rust_process_audio(
        &mut self,
        _input_slice: &[f32],
        _output_slice: &mut [f32],
        mut _playhead: Playhead,
    ) {
        self.cbuf.push_back(1.2);
        // assert_eq!(self.cbuf, [1.2]);
    }
}
