use alloc::boxed::Box;
use alloc::vec::Vec;

use crate::constants::*;
use crate::filter::harmoneer::*;
use crate::filter::knob_gain::*;
use crate::filter::mixer::*;
use crate::filter::pass_thru::*;
use crate::filter::seq::*;
use crate::patch::Patch;
use crate::sdram::*;

// For guitar
// knob_num, ratio, (low, high)
const TONES: [(usize, f32, (f32, f32)); 3] =
  [(0, 0.5, (0.0, 1.0)),
   (1, 2.0, (0.0, 1.0)),
   (2, 1.5, (0.0, 1.0))];

// For yo-yo ma and Rubin
// const TONES: [(usize, f32, (f32, f32)); 7] =
//   [(0, 0.25, (0.0, 2.0)), (0, 0.5, (0.0, 1.0)),
//    (1, 2.0, (0.0, 1.0)), (1, 4.0, (0.0, 1.0)),
//    (2, 0.75, (0.0, 1.0)), (2, 1.5, (0.0, 1.0)), (2, 3.0, (0.0, 1.0))];

pub fn much_harm(sdram: &mut SDRAM) -> Box<dyn Patch> {
    let mut harms: Vec<MixerChannel> = vec![];
    for (knob_num, ratio, (low, high)) in TONES.iter() {
        let h = if *ratio == 1.0 {
            Box::new(PassThruFilter::new()) as Box<dyn Patch>
        } else {
            Box::new(Harmoneer::new(*ratio, sdram)) as Box<dyn Patch>
        };
        let k = Box::new(KnobGain::new(*knob_num, *low, *high));
        let s = Box::new(Seq::new(BLOCK_SIZE, h, k));
        harms.push(MixerChannel(1.0, s));
    }
    harms.push(MixerChannel(1.0, Box::new(PassThruFilter::new())));
    Box::new(Mixer::new(harms))
}
