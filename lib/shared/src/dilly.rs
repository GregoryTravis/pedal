extern crate alloc;

use alloc::boxed::Box;

use crate::glep;
use crate::patch::*;
use crate::playhead::*;
use crate::spew::*;

const BUFSIZE: usize = 16;

const SIZE: usize = 4;

pub struct Dilly {
    input: Box<[f32]>,
    output: [f32; BUFSIZE],
    done: bool,
    has_dumped: bool,
    _counter: usize,
}

impl Dilly {
    pub fn new(input: Box<[f32]>) -> Dilly {
        assert!(input.len() == BUFSIZE);

        Dilly {
            input: input,
            output: [0.0; BUFSIZE],
            done: false,
            has_dumped: false,
            _counter: 0,
        }
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    // While not done, do nothing. Once done, dump the output, but only once.
    pub fn dump_maybe(&mut self) {
        if self.done && !self.has_dumped {
            for i in 0..BUFSIZE {
                glep!("DILLY", self.input[i], self.output[i]);
            }
            self.has_dumped = true;
        }
    }

    pub fn rust_process_audio(
        &mut self,
        patch: &mut Box<dyn Patch>,
        input_slice: &[f32],
        output_slice: &mut [f32],
        playhead: Playhead,
    ) {
        assert!(SIZE == input_slice.len());
        patch.rust_process_audio(input_slice, output_slice, playhead);
        self.done = true;
        /*
        for i in 0..input_slice.len() {
            output_slice[i] = input_slice[i];
        }
        */
    }
}
