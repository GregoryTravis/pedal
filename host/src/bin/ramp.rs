extern crate alloc;

use alloc::boxed::Box;

use shared::filter::tremolo::*;
use shared::sim::*;

#[allow(dead_code)]
fn tremolo() {
    sim_ramp_patch(Box::new(Tremolo::new()), 40);
}

pub fn main() {
    tremolo();
}
