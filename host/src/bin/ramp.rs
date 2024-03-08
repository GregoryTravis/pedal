extern crate alloc;

use alloc::boxed::Box;

use shared::filter::vibrato::*;
use shared::sim::*;

#[allow(dead_code)]
fn vibrato() {
    sim_ramp_patch(Box::new(Vibrato::new()), 40);
}

pub fn main() {
    vibrato();
}
