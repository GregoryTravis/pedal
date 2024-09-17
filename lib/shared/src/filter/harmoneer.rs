extern crate alloc;
extern crate libm;

use alloc::boxed::Box;

use crate::knob::Knobs;
use crate::patch::Patch;
use crate::playhead::Playhead;
use crate::spew::*;

const SIZE: usize = 4096;
const RAMPLEN: usize = 48;
const ALT_BUMP: usize = 8;

pub struct Harmoneer {
    p: f32,
    r_now: f32,
    alt_r_now: f32,
    alpha: f32,
    delta_alpha: f32,
    ramping: bool,
    w_now: usize,
    w_s: usize,
    w_e: usize,
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
            p: ratio,
            r_now: 0.0, // Doesn't matter, will be set before use
            alt_r_now: ALT_BUMP as f32, // will be the first read head beacuse of the first jump on
                                        // the first sample
            alpha: 0.0, // WBS
            delta_alpha: 0.0, // WBS
            ramping: true, // So that we (re-) initialize the first time through
            w_now: SIZE,
            w_e: SIZE, // Setting w_now == w_e means we trigger a jump on the first sample
            w_s: 0, // Doesn't matter, will be set before use
            buf: [0.0; SIZE],
        }
    }

    fn buf_f(&self, r: f32) -> f32 {
        let r_0: usize = libm::floorf(r) as usize;
        let r_1: usize = r_0 + 1;
        let alpha: f32 = (r - r_0 as f32) / ((r_1 - r_0) as f32);
        let beta: f32 = 1.0 - alpha;
        if !(0.0 <= alpha && alpha <= 1.0) {
            spew!("buf_f", "r", r, "r_0", r_0, "r_1", r_1, "alpha", alpha, "beta", beta);
        }
        assert!(0.0 <= alpha && alpha <= 1.0);
        assert!(0.0 <= beta && beta <= 1.0);
        (beta * self.buf(r_0)) + (alpha * self.buf(r_1))
    }

    fn buf(&self, r: usize) -> f32 {
        self.buf[r % SIZE]
    }

    fn write_buf(&mut self, w: usize, x: f32) {
        self.buf[w % SIZE] = x;
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

            // This all only works for ratio >= 1.

            // Write, but don't advance head until the end.
            //self.buf[self.write_head] = inp;
            self.write_buf(self.w_now, inp);

            //////

            if self.w_now == self.w_e {
                assert!(self.ramping == true);

                spew!("JUMP");

                // Jump
                self.r_now = self.alt_r_now;
                self.ramping = false;

                let delta_t_m: f32 = (self.w_now as f32 - self.r_now) / (self.p - 1.0);

                let w_m: f32 = self.w_now as f32 + delta_t_m;  // == r_m
                self.w_e = libm::floorf(w_m) as usize;
                let delta_t_rampdur: usize = RAMPLEN;
                let delta_t_ramplen: usize = libm::ceilf(delta_t_rampdur as f32 * self.p) as usize;
                self.w_s = self.w_e - (delta_t_ramplen - 1);

                self.alpha = (self.r_now - (self.w_s as f32)) / ((self.w_e - self.w_s) as f32);
                let num_steps_til_next_jump: usize = self.w_e - self.w_now;
                self.delta_alpha = (1.0 - self.alpha) / (num_steps_til_next_jump as f32);
            }

            // Start ramping if we cross the ramp start
            if !self.ramping && self.alpha >= 0.0 {
                spew!("START RAMP");
                self.ramping = true;
                self.alt_r_now = (self.w_now - SIZE + ALT_BUMP) as f32;
                assert!(self.alt_r_now > 0.0);
            }

            // Calculate output sample, ramped or no
            let out = if !self.ramping {
                self.buf_f(self.r_now)
            } else {
                let beta = 1.0 - self.alpha;
                (beta * self.buf_f(self.r_now)) + (self.alpha * self.buf_f(self.alt_r_now))
            };
            spew!("JUMP", "out", out, "ramping", self.ramping, "w_now", self.w_now, "r_now", self.r_now, "alt_r_now", self.alt_r_now, "w_s", self.w_s, "w_e", self.w_e, "alpha", self.alpha, "delta_alpha", self.delta_alpha);

            // Increment counters
            self.r_now += self.p;
            self.alt_r_now += self.p;
            self.w_now += 1;
            self.alpha += self.delta_alpha;

            output_slice[i] = out;
            playhead.inc();
        }
    }
}
