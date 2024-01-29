extern crate alloc;

use alloc::boxed::Box;
use core::cmp::min;

use crate::glep;
use crate::patch::*;
use crate::playhead::*;
use crate::spew::*;

const DILLY_SIZE: usize = 4096;

const BUFSIZE: usize = 16;

const SIZE: usize = 4;

pub struct Dilly {
    input: Box<[f32]>,
    input_buffer: [f32; SIZE],
    output: [f32; DILLY_SIZE],
    done: bool,
    has_dumped: bool,
    counter: usize,
    pub ever_dilled: bool,
}

impl Dilly {
    pub fn new(input: Box<[f32; BUFSIZE]>) -> Dilly {
        assert!(input.len() == BUFSIZE);
        assert!(SIZE <= DILLY_SIZE);

        Dilly {
            input: input,
            input_buffer: [0.0; SIZE],
            output: [0.0; DILLY_SIZE],
            done: false,
            has_dumped: false,
            counter: 0,
            ever_dilled: false,
        }
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    // While not done, do nothing. Once done, dump the output, but only once.
    pub fn dump_maybe(&mut self) {
        if self.done && !self.has_dumped {
            for i in 0..DILLY_SIZE {
                glep!("DILLY", self.input[i % BUFSIZE], self.output[i]);
            }
            self.has_dumped = true;
            glep!("DILLY done", self.counter, if self.ever_dilled { 1 } else { 0 });
        }
    }

    pub fn bleee(
        &mut self,
        patch: &mut Box<dyn Patch>,
        input_slice: &[f32],
        output_slice: &mut [f32],
        playhead: Playhead,
    ) {
        assert!(SIZE == input_slice.len());
        assert!(SIZE == output_slice.len());

        if self.counter < DILLY_SIZE {
            self.ever_dilled = true;

            for i in 0..SIZE {
                self.input_buffer[i] = self.input[(self.counter+i) % BUFSIZE]
                //self.input_buffer[i] = input_slice[i];
            }

            patch.rust_process_audio(&self.input_buffer, output_slice, playhead);

            for c in self.counter..min(self.counter+SIZE, DILLY_SIZE) {
                self.output[c] = output_slice[c-self.counter];
                //self.output[c] = 13.4;
            }

            /*
            for i in 0..SIZE {
                //self.input_buffer[i] = self.input[(self.counter+i) % BUFSIZE]
                output_slice[i] = 0.0;
            }
            */

            //self.counter += (min(self.counter+SIZE, DILLY_SIZE)-self.counter);
            self.counter += min(SIZE, DILLY_SIZE-self.counter);
        } else {
            self.ever_dilled = true;
            patch.rust_process_audio(input_slice, output_slice, playhead);
            self.done = true;

            /*
            for i in 0..SIZE {
                //self.input_buffer[i] = self.input[(self.counter+i) % BUFSIZE]
                output_slice[i] = 0.0;
            }
            */
        }
    }
}
