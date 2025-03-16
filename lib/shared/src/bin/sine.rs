extern crate std;
extern crate alloc;
extern crate libm;

use alloc::vec::Vec;
use core::f32::consts::PI;
use std::env;

use hound;

use shared::convert::*;
use shared::constants::SAMPLE_RATE;

fn main() {
    let args: Vec<String> = env::args().collect();
    // prog, filename, duration, then (freq, amp) pairs
    assert_eq!(args.len() % 2, 1);
    let filename = &args[1];
    let duration = args[2].parse::<f32>().unwrap();
    let num_freq_amp_pairs = (args.len() - 3) / 2;

    let mut freq_amp_pairs: Vec<(f32, f32)> = vec![(0.0, 0.0); num_freq_amp_pairs];
    for i in 0..num_freq_amp_pairs {
      let frequency: f32 = args[3+(i*2)].parse::<f32>().unwrap();
      let amp: f32 = args[3+(i*2)+1].parse::<f32>().unwrap();
      freq_amp_pairs[i] = (frequency, amp);
    }

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
        output[i] = 0.0;
        for (frequency, amp) in &freq_amp_pairs {
            let ph = t * 2.0 * PI * frequency;
            output[i] += amp * libm::sinf(ph);
        }
    }

    for i in 0..length {
        writer
            .write_sample(sample_f32_to_i16(output[i]))
            .unwrap();
    }

    writer.finalize().unwrap();
}
