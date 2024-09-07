extern crate alloc;
extern crate libm;

use alloc::boxed::Box;

use crate::knob::Knobs;
use crate::patch::Patch;
use crate::playhead::Playhead;

const SIZE: usize = 256;

pub struct Harmoneer {
    ratio: f32,
    read_head: f32,
    write_head: usize,
    buf: [f32; SIZE],
}

impl Harmoneer {
    pub fn new() -> Harmoneer {
        // TODO use a static assertion for this.
        assert!(SIZE > 1);

        Harmoneer {
            ratio: 1.5,
            read_head: 1.0,
            write_head: 0,
            buf: [0.0; SIZE],
        }
    }
}

impl Patch for Harmoneer {
    fn rust_process_audio(
        &mut self,
        input_slice: &[f32],
        output_slice: &mut [f32],
        _knobs: &Box<dyn Knobs>,
        mut playhead: Playhead,
    ) {
        for i in 0..input_slice.len() {
            let inp = input_slice[i];

            self.buf[self.write_head] = inp;
            let out = self.buf[libm::floorf(self.read_head) as usize];

            self.write_head += 1;
            if self.write_head >= SIZE {
                // Or just set to 0?
                self.write_head -= SIZE;
            }

            self.read_head += self.ratio;
            if self.read_head >= SIZE as f32 {
                self.read_head -= SIZE as f32;
            }

            let mixed = 0.5 * inp + 0.5 * out;

            output_slice[i] = mixed;
            playhead.inc();
        }
    }
}
