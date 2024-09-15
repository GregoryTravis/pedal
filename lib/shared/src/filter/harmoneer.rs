extern crate alloc;
extern crate libm;

use alloc::boxed::Box;

use crate::knob::Knobs;
use crate::patch::Patch;
use crate::playhead::Playhead;
use crate::spew::*;

const SIZE: usize = 4096;
const RAMPLEN: usize = 48;
const ALT_BUMP: usize = 2;

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
            write_head: SIZE,
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
            self.write_buf(self.write_head, inp);

            let r_now: f32 = self.read_head;
            let w_now: usize = self.write_head;
            let p: f32 = self.ratio;

            //////

            let delta_t_m: f32 = (w_now as f32 - r_now) / (p - 1.0);

            let w_m: f32 = w_now as f32 + delta_t_m;  // == r_m
            let w_e: usize = libm::floorf(w_m) as usize;
            let r_e: f32 = r_now + (p * (w_e - w_now) as f32);
            let delta_t_rampdur: usize = RAMPLEN;
            let delta_t_ramplen: usize = libm::ceilf(delta_t_rampdur as f32 * self.ratio) as usize;
            let w_s: usize = w_e - (delta_t_ramplen - 1);
            //spew!("main", "r_now", r_now, "w_now", w_now, "delta_t_m", delta_t_m, "w_m", w_m, "w_s", w_s, "r_e", r_e, "w_e", w_e);
            let r_s: f32 = r_now + (p * ((w_s as isize - w_now as isize) as f32));
            //spew!("main", "r_now", r_now, "w_now", w_now, "delta_t_m", delta_t_m, "w_m", w_m, "r_s", r_s, "w_s", w_s, "r_e", r_e, "w_e", w_e);
            //spew!("r_s", r_s);
            //spew!("w_s", w_s);
            let alpha_prime: f32 = (r_now - r_s) / (r_e - r_s);
            let alpha: f32 = if alpha_prime < 0.0 { 0.0 } else { alpha_prime };
            let beta: f32 = 1.0 - alpha;
            //let w_s_hat: isize = (w_s as isize) - (SIZE as isize);
            let w_s_hat: usize = w_s - SIZE;
            let fudge: usize = ALT_BUMP;
            let r_s_prime: f32 = (w_s_hat + fudge) as f32;
            let delta_r_alt: f32 = r_s - r_s_prime;
            let r_now_prime: f32 = r_now - delta_r_alt;
            let out: f32 = if alpha == 0.0 {
                // Not ramping
                let out: f32 = self.buf_f(r_now);
                out
            } else {
                let out: f32 = (beta * self.buf_f(r_now)) + (alpha * self.buf_f(r_now_prime));
                out
            };

            let mut r_now = r_now;
            let mut w_now = w_now;

            ////// Move the read head to the alt read head if we're at the end of the ramp.

            if w_now == w_e {
                r_now = r_now_prime;
            }

            //////

            r_now += self.ratio;
            w_now += 1;

            /*
            if r_now >= ((SIZE * 2) as f32) && w_now >= (SIZE * 2) {
                r_now -= SIZE as f32;
                w_now -= SIZE;
            }
            */

            //////

            self.read_head = r_now;
            self.write_head = w_now;

            output_slice[i] = out;
            playhead.inc();

            self.ts += 1;
        }
    }
}
