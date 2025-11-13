#[cfg(feature = "for_host")]
extern crate std;
extern crate libm;

use circular_buffer::CircularBuffer;

use crate::constants::*;
#[allow(unused)]
use crate::spew::*;

const MEM_LENGTH: usize = 2000;
const INIT_FREQ: f32 = 440.0; // Just to have something non-degenerate
const ALPHA: f32 = 0.9;

pub struct Comb {
    delay: usize,
    mem: CircularBuffer::<MEM_LENGTH, f32>,
}

impl Comb {
    pub fn new() -> Comb {
        let mut comb = Comb {
            delay: 0,
            mem: CircularBuffer::<MEM_LENGTH, f32>::new(),
        };

        comb.set_freq(INIT_FREQ);
        comb
    }

    pub fn set_freq(&mut self, freq: f32) {
        self.delay = (SAMPLE_RATE as f32 / freq) as usize;
    }

    pub fn process(&mut self, x: f32) -> f32 {
        let old: f32 = match self.mem.get(self.delay) {
            Some(x) => *x,
            None => 0.0,
        };
        let out: f32 = x + (ALPHA * old);
        self.mem.push_front(out);
        out
    }
}
