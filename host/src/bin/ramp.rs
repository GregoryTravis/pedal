extern crate alloc;

use alloc::boxed::Box;

use host::sim::*;
use shared::filter::tremolo::*;

#[allow(dead_code)]
fn tremolo() {
    sim_ramp_patch(Box::new(Tremolo::new()), 40);
}

pub fn main() {
    tremolo();
}
