extern crate alloc;

use std::env;
use alloc::boxed::Box;
use alloc::sync::Arc;

use host::sim::*;
use shared::filter::delay::*;
use shared::filter::pass_thru::*;
use shared::filter::reso::*;
use shared::filter::tremolo::*;
use shared::signal::base::*;
use shared::signal::combinators::*;

#[allow(dead_code)]
fn reso(input_file: &str, output_file: &str) {
    let siner = PostCompose { signal: Arc::new(Sin {}), ff: scale_range(0.3, 0.9) };
    let q = Const { x: 0.95 };
    sim_main(input_file, output_file, Box::new(ResoFilter::new(Arc::new(siner), Arc::new(q))));
}

#[allow(dead_code)]
fn delay(input_file: &str, output_file: &str) {
    sim_main(input_file, output_file, Box::new(Delay::new()));
}

#[allow(dead_code)]
fn pass_thru(input_file: &str, output_file: &str) {
    sim_main(input_file, output_file, Box::new(PassThruFilter {}));
}

#[allow(dead_code)]
fn tremolo(input_file: &str, output_file: &str) {
    sim_main(input_file, output_file, Box::new(Tremolo::new()));
}

pub fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 3);
    let input_file = &args[1];
    let output_file = &args[2];
    tremolo(input_file, output_file);
}
