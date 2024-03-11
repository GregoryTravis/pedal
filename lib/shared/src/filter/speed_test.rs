extern crate alloc;
extern crate libm;
#[cfg(feature = "for_host")]
extern crate std;

use crate::patch::Patch;
use crate::playhead::Playhead;

//#[cfg(feature = "for_host")]
//use std::println;

const DOT_SIZE: usize = 48;
const NROUNDS: u32 = 400;

pub struct SpeedTest {
    fys: [f32; DOT_SIZE],
}

impl SpeedTest {
    pub fn new() -> SpeedTest {
        SpeedTest {
            fys: [0.0; DOT_SIZE],
        }
    }
}

impl Patch for SpeedTest {
    fn rust_process_audio(
        &mut self,
        input_slice: &[f32],
        output_slice: &mut [f32],
        mut _playhead: Playhead,
    ) {
        for _r in 0..NROUNDS {
            for i in 0..DOT_SIZE {
                output_slice[i] = input_slice[i] * self.fys[i];
            }
        }
    }
}
