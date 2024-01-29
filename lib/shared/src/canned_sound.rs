extern crate alloc;
extern crate libm;

use alloc::boxed::Box;
use core::f32::consts::PI;

const BUFSIZE: usize = 16;

pub fn canned_sound_0() -> Box<[f32; BUFSIZE]> {
    let mut ret: [f32; BUFSIZE] = [0.0; BUFSIZE];
    for i in 0..BUFSIZE {
        let ph = 2.0 * PI * ((i as f32) / 100.0);
        ret[i] = libm::sinf(ph);
    }
    Box::new(ret)
}
