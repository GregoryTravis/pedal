extern crate alloc;
extern crate libm;

use alloc::boxed::Box;

use crate::knob::Knobs;
use crate::patch::Patch;
use crate::playhead::Playhead;
use crate::spew::*;

// Must be even.
const SIZE: usize = 4096;
const RAMPLEN: usize = 48;
const JUMP_MARGIN: f32 = 2.0;

pub struct Harmoneer {
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
            ratio: ratio,
            read_head: (SIZE / 2) as f32,
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

            // Write, but don't advance head until the end.
            self.write_buf(self.write_head, inp);

            let mut r = self.read_head;
            let mut w = self.write_head;
            let p = self.ratio;

            // ====

            let n_f: f32 = ((w as f32) - r) / (p - 1.0);
            let t_f: f32 = (w as f32) + n_f;
            // TODO try size/p-1
            let n_r: f32 = ((w as f32) - r - (SIZE as f32)) / (p - 1.0);
            let t_r: f32 = (w - SIZE) as f32 + n_r;

            let forward_ramp_start: f32 = t_f - RAMPLEN as f32;
            let forward_ramp_end: f32 = t_f;
            let backward_ramp_start: f32 = t_r;
            let backward_ramp_end: f32 = t_r + RAMPLEN as f32;

            let inside_forward_ramp = forward_ramp_start <= w as f32 && w as f32 <= forward_ramp_end;
            let inside_backward_ramp = backward_ramp_start <= w as f32 && w as f32 <= backward_ramp_end;
            assert!(!inside_forward_ramp || !inside_backward_ramp);

            let mut should_flip = false;

            let dbg_lo = 12235;
            let dbg_hi = 12245;

            let out: f32 = if inside_forward_ramp {
                let alpha_unclipped = (w as f32 - forward_ramp_start) / (backward_ramp_end - backward_ramp_start);
                assert!(alpha_unclipped <= 1.0);
                let alpha = if alpha_unclipped < 0.0 { 0.0 } else { alpha_unclipped  };
                let beta = 1.0 - alpha;
                let alt_r = r - ((SIZE / 2) as f32);
                if forward_ramp_end - (w as f32) < JUMP_MARGIN {
                    should_flip = true;
                }
                if w >= dbg_lo && w <= dbg_hi {
                    spew!("ab", alpha, beta);
                }
                beta * self.buf_f(r) + alpha * self.buf_f(alt_r)
            } else if inside_backward_ramp {
                let alpha_unclipped = (w as f32 - backward_ramp_start) / (backward_ramp_end - backward_ramp_start);
                assert!(alpha_unclipped >= 0.0);
                let alpha = if alpha_unclipped > 1.0 { 1.0 } else { alpha_unclipped };
                let beta = 1.0 - alpha;
                let alt_r = r + ((SIZE / 2) as f32);
                if (w as f32) - backward_ramp_start < JUMP_MARGIN {
                    should_flip = true;
                }
                if w >= dbg_lo && w <= dbg_hi {
                    spew!("ab", alpha, beta);
                }
                alpha * self.buf_f(r) + beta * self.buf_f(alt_r)
            } else {
                self.buf_f(r)
            };

            if w >= dbg_lo && w <= dbg_hi {
                spew!("r", r, "w", w, "t", t_f, t_r, "fr", forward_ramp_start, forward_ramp_end, "br", backward_ramp_start, backward_ramp_end, "in f", inside_forward_ramp, "in r", inside_backward_ramp);
            }

            if should_flip {
                if p >= 1.0 {
                    r -= (SIZE/2) as f32;
                } else {
                    r += (SIZE/2) as f32;
                }
            }

            r += p;
            w += 1;

            // ====

            self.read_head = r;
            self.write_head = w;

            output_slice[i] = out;
            playhead.inc();
        }
    }
}
