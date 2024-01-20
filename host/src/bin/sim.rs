extern crate alloc;

use alloc::boxed::Box;

use host::sim::*;
use shared::filter::reso::*;
use shared::signal::base::*;
use shared::signal::combinators::*;

pub fn main() {
    let siner = PostCompose { signal: Box::new(Sin {}), ff: scale_range(0.3, 0.9) };
    let q = Const { x: 0.95 };
    sim_main(Box::new(ResoFilter::new(Box::new(siner), Box::new(q))));
}
