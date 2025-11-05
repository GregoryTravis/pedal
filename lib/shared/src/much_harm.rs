use alloc::boxed::Box;
use alloc::vec::Vec;

use crate::constants::*;
use crate::filter::harmoneer::*;
use crate::filter::knob_gain::*;
use crate::filter::mixer::*;
use crate::filter::seq::*;
use crate::sdram::*;

const TONES: [f32; 6] = [0.25, 0.5, 1.0, 1.5, 2.0, 3.0];

pub fn much_harm(sdram: &mut SDRAM) -> Box<Mixer> {
    let mut harms: Vec<MixerChannel> = vec![];
    for (index, ratio) in TONES.iter().enumerate() {
        let h = Box::new(Harmoneer::new(*ratio, sdram));
        let k = Box::new(KnobGain::new(index));
        let s = Box::new(Seq::new(BLOCK_SIZE, h, k));
        harms.push(MixerChannel(1.0, s));
    }
    Box::new(Mixer::new(harms))
}
