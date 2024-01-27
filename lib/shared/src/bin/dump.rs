extern crate std;

use std::env;
use std::println;

use hound;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2);
    let mut reader = hound::WavReader::open(&args[1]).unwrap();
    let input_spec = reader.spec();
    assert!(input_spec.channels == 1);
    let mut samples = reader.samples::<i16>();

    match input_spec.channels {
        1 => {
            while samples.len() > 0 {
                let sample: f32 = (samples.next().unwrap().unwrap() as f32) / 32768.0;
                println!("{}", sample);
            }
        }
        _ => assert!(false),
    }
}
