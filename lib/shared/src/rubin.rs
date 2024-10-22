use alloc::boxed::Box;

use crate::constants::*;
use crate::filter::chorus::*;
use crate::filter::harmoneer::*;
use crate::filter::high_pass::*;
use crate::filter::interp::*;
use crate::filter::low_pass::*;
use crate::filter::mixer::*;
use crate::filter::pass_thru::*;
use crate::filter::reso::*;
use crate::filter::seq::*;
use crate::patch::Patch;
use crate::sdram::*;

fn harmoneer(sdram: &mut SDRAM) -> Mixer {
    let orig = PassThruFilter {};
    let h0 = Harmoneer::new(2.0, sdram);
    let h1 = Harmoneer::new(0.5, sdram);
    let fh1 = Seq::new(BLOCK_SIZE, Box::new(h1), Box::new(LowPassFilter::new()) );
    let channels = vec![
        MixerChannel(1.0, Box::new(orig)),
        MixerChannel(0.7, Box::new(h0)),
        MixerChannel(1.0, Box::new(fh1)),
    ];
    Mixer::new(channels)
}

pub fn rubin(sdram: &mut SDRAM) -> Box<dyn Patch> {
    let harmo = Box::new(harmoneer(sdram));
    let hp = Box::new(HighPassFilter::new());
    let lp = Box::new(LowPassFilter::new());
    let high_low = Box::new(Interp::new(BLOCK_SIZE, lp, hp, 2));

    let reso = Box::new(ResoFilter::new(0, 3));
    //let lvib = Box::new(LinearVibrato::new(400, 10.0, 1));
    let chorus = Box::new(Chorus::new());
    let seq0 = Box::new(Seq::new(BLOCK_SIZE, harmo, chorus));
    let seq1 = Box::new(Seq::new(BLOCK_SIZE, seq0, reso));
    let seq2 = Box::new(Seq::new(BLOCK_SIZE, seq1, high_low));
    //let sweep = Box::new(SweepFilter::example());

    seq2
}

