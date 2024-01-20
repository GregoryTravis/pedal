extern crate alloc;

use alloc::boxed::Box;
use alloc::sync::Arc;

use host::sim::*;
use shared::filter::reso::*;
use shared::signal::base::*;
use shared::signal::combinators::*;

pub fn main() {
    let siner = PostCompose { signal: Arc::new(Sin {}), ff: scale_range(0.3, 0.9) };
    let q = Const { x: 0.95 };
    sim_main(Box::new(ResoFilter::new(Arc::new(siner), Arc::new(q))));
}
