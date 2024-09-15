extern crate std;

use std::env;

use shared::file::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 4);
    let samples0 = file_read(&args[1]);
    let samples1 = file_read(&args[2]);
    let output_file = &args[3];

    assert!(samples0.len() == samples1.len());

    let num_samples = samples0.len();
    let mut output = Vec::with_capacity(num_samples);

    for i in 0..samples0.len() {
        let sample0: f32 = samples0[i];
        let sample1: f32 = samples1[i];
        let diff = sample0 - sample1;
        output.push(diff);
    }

    file_write(output_file, output);
}
