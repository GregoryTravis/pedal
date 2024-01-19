extern crate std;

use std::env;
use std::println;

use hound;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut reader0 = hound::WavReader::open(&args[1]).unwrap();
    let mut reader1 = hound::WavReader::open(&args[2]).unwrap();
    let input_spec0 = reader0.spec();
    let input_spec1 = reader1.spec();
    assert!(input_spec0 == input_spec1);
    assert!(input_spec0.channels == 1);
    let mut samples0 = reader0.samples::<i16>();
    let mut samples1 = reader1.samples::<i16>();

    match input_spec0.channels {
        1 => {
            let mut total_diff_squared: f32 = 0.0;
            let num_samples = samples0.len();

            while samples0.len() > 0 {
                assert!(samples0.len() == samples1.len());
                let sample0: f32 = (samples0.next().unwrap().unwrap() as f32) / 32768.0;
                let sample1: f32 = (samples1.next().unwrap().unwrap() as f32) / 32768.0;
                let diff = sample0 - sample1;
                total_diff_squared += diff * diff;
            }
            let rms = (total_diff_squared / num_samples as f32).sqrt();
            println!("RMS {}", rms);
        }
        _ => assert!(false),
    }
}
