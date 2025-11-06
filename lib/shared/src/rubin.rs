use alloc::boxed::Box;

use crate::constants::*;
use crate::filter::chorus::*;
use crate::filter::reso::*;
use crate::filter::seq::*;
use crate::patch::Patch;
use crate::sdram::*;

pub fn rubin(sdram: &mut SDRAM) -> Box<dyn Patch> {
    let harmo = crate::much_harm::much_harm(sdram);

    let reso = Box::new(ResoFilter::new(3, 4));
    let chorus = Box::new(Chorus::new(5));
    let seq0 = Box::new(Seq::new(BLOCK_SIZE, harmo, chorus));
    let seq1 = Box::new(Seq::new(BLOCK_SIZE, seq0, reso));

    seq1
}

