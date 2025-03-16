extern crate std;
extern crate libm;

use core::f32::consts::PI;
use std::env;

use hound;

use shared::convert::*;
use shared::constants::SAMPLE_RATE;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 4);
    let filename = &args[1];
    let frequency: f32 = args[2].parse::<f32>().unwrap();
    let duration = args[3].parse::<f32>().unwrap();

    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create(filename, spec).unwrap();

    let length = (duration * SAMPLE_RATE as f32) as usize;
    let mut output: Vec<f32> = vec![0.0; length];

    for i in 0..length {
        let t = (i as f32) / (SAMPLE_RATE as f32);
        let ph = t * 2.0 * PI * frequency;
        output[i] = libm::sinf(ph);
    }

    for i in 0..length {
        writer
            .write_sample(sample_f32_to_i16(output[i]))
            .unwrap();
    }

    writer.finalize().unwrap();
}
