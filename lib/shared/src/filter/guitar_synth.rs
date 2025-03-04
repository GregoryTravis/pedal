extern crate alloc;
extern crate libm;
#[cfg(feature = "for_host")]
extern crate std;

use alloc::boxed::Box;
//#[cfg(feature = "for_host")]
//use std::println;

use crate::convert::*;
use crate::knob::Knobs;
use crate::patch::Patch;
use crate::playhead::Playhead;
use crate::spew::*;
use crate::yin::*;

const BUFFER_SIZE: usize = 1024;

pub struct GuitarSynth {
    // TODO put in sdram
    buffer: [f32; BUFFER_SIZE],
    int_buffer: [i16; BUFFER_SIZE],
}

impl GuitarSynth {
    pub fn new() -> GuitarSynth {
        yin_init(BUFFER_SIZE as i16, 0.05);
        GuitarSynth {
            buffer: [0.0; BUFFER_SIZE],
            int_buffer: [0; BUFFER_SIZE],
        }
    }
}

impl Patch for GuitarSynth {
    fn rust_process_audio(
        &mut self,
        input_slice: &[f32],
        _output_slice: &mut [f32],
        _knobs: &Box<dyn Knobs>,
        mut playhead: Playhead,
    ) {
        // Copy + and convert history, making room for new samples.
        // TODO don't do this.
        let slice_len = input_slice.len();
        for i in 0..BUFFER_SIZE-slice_len {
            self.int_buffer[i] = self.int_buffer[i+slice_len];
        }
        for i in 0..slice_len {
            //spew!("um", input_slice[i]);
            self.int_buffer[i+BUFFER_SIZE-slice_len] = sample_f32_to_i16(input_slice[i]);
        }

        for i in 0..4 {
            spew!(self.buffer[i], self.int_buffer[1]);
        }

        let pitch: f32 = yin_process(&self.int_buffer);

        spew!(pitch);

        // TODO: need this?
        playhead.inc();
    }
}
