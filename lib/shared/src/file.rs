extern crate std;
extern crate alloc;

use alloc::vec::Vec;

use hound;
use crate::constants::*;
use crate::convert::*;

pub fn file_read(filename: &str) -> Vec<f32> {
    let mut reader = hound::WavReader::open(filename).unwrap();
    let input_spec = reader.spec();
    assert!(input_spec.channels == 1);
    let mut samples = reader.samples::<i16>();
    let mut vec: Vec<f32> = Vec::with_capacity(samples.len());

    match input_spec.channels {
        1 => {
            while samples.len() > 0 {
                vec.push(sample_i16_to_f32(samples.next().unwrap().unwrap()));
            }
        }
        _ => assert!(false),
    }

    vec
}

pub fn file_write(filename: &str, samples: Vec<f32>) {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create(filename, spec).unwrap();

    for sample in samples.iter() {
        writer
            .write_sample(sample_f32_to_i16(*sample))
            .unwrap();
    }

    writer.finalize().unwrap();
}
