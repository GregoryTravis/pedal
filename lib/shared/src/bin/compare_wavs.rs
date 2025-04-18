extern crate std;

use std::env;
use std::println;

use shared::file::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 3);
    let samples0 = file_read(&args[1]);
    let samples1 = file_read(&args[2]);

    assert!(samples0.len() == samples1.len());

    let num_samples = samples0.len();

    let mut total_diff_squared: f32 = 0.0;

    for i in 0..samples0.len() {
        let sample0: f32 = samples0[i];
        let sample1: f32 = samples1[i];
        let diff = sample0 - sample1;
        /*
        if diff != 0.0 {
            println!("{} {} {} {}", i, diff, sample0, sample1);
        }
        */
        total_diff_squared += diff * diff;
    }
    let rms = (total_diff_squared / num_samples as f32).sqrt();
    println!("RMS {}", rms);
}
