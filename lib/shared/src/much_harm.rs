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

const TONES: [(usize, f32); 7] =
  [(0, 0.25), (0, 0.5),
   (1, 2.0), (1, 4.0),
   (2, 0.75), (2, 1.5), (2, 3.0)];

pub fn much_harm(sdram: &mut SDRAM) -> Box<dyn Patch> {
    let mut harms: Vec<MixerChannel> = vec![];
    for (knob_num, ratio) in TONES.iter() {
        let h = if *ratio == 1.0 {
            Box::new(PassThruFilter::new()) as Box<dyn Patch>
        } else {
            Box::new(Harmoneer::new(*ratio, sdram)) as Box<dyn Patch>
        };
        let k = Box::new(KnobGain::new(*knob_num));
        let s = Box::new(Seq::new(BLOCK_SIZE, h, k));
        harms.push(MixerChannel(1.0, s));
    }
    harms.push(MixerChannel(1.0, Box::new(PassThruFilter::new())));
    Box::new(Mixer::new(harms))
}
