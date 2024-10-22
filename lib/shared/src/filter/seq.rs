extern crate alloc;
extern crate libm;
#[cfg(feature = "for_host")]
extern crate std;

use alloc::boxed::Box;
use alloc::vec::Vec;

use crate::knob::Knobs;
use crate::patch::Patch;
use crate::playhead::Playhead;

pub struct Seq {
    patch0: Box<dyn Patch>,
    patch1: Box<dyn Patch>,
    buf: Vec<f32>,
}

impl Seq {
    pub fn new(block_size: usize, patch0: Box<dyn Patch>, patch1: Box<dyn Patch>) -> Seq {
        Seq {
            patch0: patch0,
            patch1: patch1,
            buf: vec![0.0; block_size],
        }
    }
}

impl Patch for Seq {
    fn rust_process_audio(
        &mut self,
        input_slice: &[f32],
        output_slice: &mut [f32],
        knobs: &Box<dyn Knobs>,
        playhead: Playhead,
    ) {
        assert!(input_slice.len() <= self.buf.len());
        let slice: &mut [f32] = &mut self.buf;
        let sub_buf: &mut [f32] = &mut slice[0..input_slice.len()];

        let first_playhead = playhead.clone();
        self.patch0.rust_process_audio(input_slice, sub_buf, knobs, first_playhead);
        self.patch1.rust_process_audio(sub_buf, output_slice, knobs, playhead);
    }
}
