extern crate alloc;
extern crate libm;
#[cfg(feature = "for_host")]
extern crate std;

// Uses a knob to fade between two patches.

use alloc::boxed::Box;
use alloc::vec::Vec;

use crate::knob::Knobs;
use crate::patch::Patch;
use crate::playhead::Playhead;
use crate::switch::Switches;

pub struct Interp {
    interp_knob_id: usize,
    patch0: Box<dyn Patch>,
    patch1: Box<dyn Patch>,
    buf: Vec<f32>,
}

impl Interp {
    pub fn new(block_size: usize, patch0: Box<dyn Patch>, patch1: Box<dyn Patch>, interp_knob_id: usize) -> Interp {
        Interp {
            interp_knob_id: interp_knob_id,
            patch0: patch0,
            patch1: patch1,
            buf: vec![0.0; block_size],
        }
    }
}

impl Patch for Interp {
    fn rust_process_audio(
        &mut self,
        input_slice: &[f32],
        output_slice: &mut [f32],
        knobs: &Box<dyn Knobs>,
        switches: &Box<dyn Switches>,
        playhead: Playhead,
    ) {
        assert!(input_slice.len() == output_slice.len());
        assert!(input_slice.len() <= self.buf.len());

        let first_playhead = playhead.clone();

        let interp = knobs.read(self.interp_knob_id);

        let slice: &mut [f32] = &mut self.buf;
        let sub_buf: &mut [f32] = &mut slice[0..input_slice.len()];

        self.patch0.rust_process_audio(input_slice, sub_buf, knobs, switches, first_playhead);
        self.patch1.rust_process_audio(input_slice, output_slice, knobs, switches, playhead);
        for i in 0..input_slice.len() {
            let p0 = sub_buf[i];
            let p1 = output_slice[i];
            output_slice[i] = ((1.0 - interp) * p0) + (interp * p1);
        }
    }
}
