extern crate alloc;

use std::env;
use alloc::boxed::Box;

use shared::sim::*;

use edslout::edsl_chorus2::*;

pub fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 3);
    let input_file = &args[1];
    let output_file = &args[2];

    let patch = EdslChorus2::new();

    sim_main(input_file, output_file, Box::new(patch));
}
