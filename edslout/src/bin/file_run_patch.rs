extern crate alloc;

use std::env;
use alloc::boxed::Box;

use shared::sim::*;

use edslout::edsl_harmoneer_wrapper::*;

pub fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 3);
    let input_file = &args[1];
    let output_file = &args[2];

    let patch = EdslHarmoneerWrapper::new();

    sim_main(input_file, output_file, Box::new(patch));
}
