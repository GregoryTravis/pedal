extern crate alloc;

use alloc::boxed::Box;

use crate::glep;
use crate::patch::*;
use crate::playhead::*;
use crate::spew::*;

const BUFSIZE: usize = 16;

const SIZE: usize = 4;

pub struct Dilly {
    patch: Box<dyn Patch>,
    input: Box<[f32]>,
    output: [f32; BUFSIZE],
    done: bool,
    has_dumped: bool,
    _counter: usize,
}

impl Dilly {
    pub fn new(patch: Box<dyn Patch>, input: Box<[f32]>) -> Dilly {
        assert!(input.len() == BUFSIZE);

        Dilly {
            patch: patch,
            input: input,
            output: [0.0; BUFSIZE],
            done: false,
            has_dumped: false,
            _counter: 0,
        }
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
}

impl Patch for Dilly {
    fn rust_process_audio(
        &mut self,
        input_slice: &[f32],
        output_slice: &mut [f32],
        playhead: Playhead,
    ) {
        assert!(SIZE == input_slice.len());
        self.patch.rust_process_audio(input_slice, output_slice, playhead);
        self.done = true;
        /*
        for i in 0..input_slice.len() {
            output_slice[i] = input_slice[i];
        }
        */
    }
}
