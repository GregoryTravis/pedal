extern crate alloc;
extern crate libm;

use alloc::boxed::Box;

use crate::knob::Knobs;
use crate::patch::Patch;
use crate::playhead::Playhead;
use crate::spew::*;

const SIZE: usize = 4096;
const RAMPLEN: usize = 48 * 4;
const ALT_BUMP: usize = 3;

pub struct Harmoneer {
    ts: usize,
    ratio: f32,
    read_head: f32,
    write_head: usize,
    buf: [f32; SIZE],
}

impl Harmoneer {
    pub fn new(ratio: f32) -> Harmoneer {
        // TODO use a static assertion for these.

        // Because otherwise the read head starts off past the end of the array.
        assert!(SIZE > 1);
        // So that the 'other read head' isn't off the end of the array.
        assert!(RAMPLEN * 2 < SIZE);

        Harmoneer {
            ts: 0,
            ratio: ratio,
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

            /*
            let mut read_to_write_dist = (self.write_head as f32) - self.read_head;
            if read_to_write_dist < 0 {
                read_to_write_dist += SIZE;
            }
            */

            let mut ramp_start: isize = self.write_head as isize - RAMPLEN as isize;
            if ramp_start < 0 {
                ramp_start += SIZE as isize;
            }
            let ramp_start: isize = ramp_start;

            let mut dist_into_ramp: f32 = self.read_head - (ramp_start as f32);
            /*
            if dist_into_ramp < 0.0 {
                dist_into_ramp += SIZE as f32;
            }
            */
            if dist_into_ramp >= RAMPLEN as f32 {
                dist_into_ramp -= SIZE as f32;
            }
            let dist_into_ramp: f32 = dist_into_ramp;

            let mut alpha: f32 = dist_into_ramp / (RAMPLEN as f32);
            assert!(alpha <= 1.0);
            if alpha < 0.0 {
                alpha = 0.0;
            }
            let alpha = alpha;

            let mut alt_read_head: f32 = self.read_head - (((SIZE - RAMPLEN) - ALT_BUMP) as f32);
            if alt_read_head < 0.0 {
                alt_read_head += SIZE as f32;
            }
            let alt_read_head: f32 = alt_read_head;
            let main_out: f32 = self.buf[libm::floorf(self.read_head) as usize];
            let alt_out: f32 = self.buf[libm::floorf(alt_read_head) as usize];
            let out: f32 = ((1.0 - alpha) * main_out) + (alpha * alt_out);
            spew!("AAA", "ts", self.ts, "wh", self.write_head, "rh", self.read_head, "arh", alt_read_head, "mo", main_out, "ao", alt_out, "o", out);

            self.read_head += self.ratio;

            if libm::floorf(self.read_head) >= self.write_head as f32 {
                self.read_head = alt_read_head;
                spew!("AAA jump", "rh", self.read_head);
            }

            self.buf[self.write_head] = inp;

            self.write_head += 1;
            assert!(self.write_head <= SIZE);
            if self.write_head >= SIZE {
                // Or just set to 0?
                self.write_head -= SIZE;
            }

            output_slice[i] = out;
            playhead.inc();

            self.ts += 1;
        }
    }
}
