extern crate alloc;

use alloc::boxed::Box;

use host::sim::*;
use shared::filter::reso::*;
use shared::signal::base::Sin;

pub fn main() {
    let siner = Sin {};
    sim_main(Box::new(ResoFilter::new(Box::new(siner))));
}
