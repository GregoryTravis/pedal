extern crate std;

use std::cmp::min;
use std::path::Path;
use std::println;

use hound;
use shared::patch::Patch;

const BATCH_SIZE: usize = 4;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn sample_i16_to_f32(x: i16) -> f32 {
    (x as f32) / 32768.0
}

fn sample_f32_to_i16(x: f32) -> i16 {
    ((x * 32767.0) as i16).try_into().unwrap()
}

pub fn sim_main(mut patch: Box<dyn Patch>) {
    let mut reader = hound::WavReader::open("fd.wav").unwrap();
    let input_spec = reader.spec();
    assert!(input_spec.channels == 1 || input_spec.channels == 2);
    let mut samples = reader.samples::<i16>();
    print_type_of(&samples);

    let path: &Path = "out.wav".as_ref();
    assert!(!path.is_file());

    let mut output_spec = input_spec;
    output_spec.channels = 1;
    let mut writer = hound::WavWriter::create(path, output_spec).unwrap();
    assert_eq!(output_spec, writer.spec());

    let mut input_buf: [f32; BATCH_SIZE] = [0.0; BATCH_SIZE];
    let mut output_buf: [f32; BATCH_SIZE] = [0.0; BATCH_SIZE];

    let mut time_in_samples: u64 = 0;

    let mut num_frames: usize = 0;

    while samples.len() > 0 {
        match input_spec.channels {
            2 => {
                assert!(samples.len() % 2 == 0);
                let input_samples_count = min(samples.len(), BATCH_SIZE * 2);
                num_frames = input_samples_count / 2;
                assert!(num_frames >= 1 && num_frames <= BATCH_SIZE);
                for i in 0..num_frames {
                    input_buf[i] = sample_i16_to_f32(samples.next().unwrap().unwrap());
                    // Skip right channel
                    samples.next().unwrap().unwrap();
                }
            }
            1 => {
                let input_samples_count = min(samples.len(), BATCH_SIZE);
                num_frames = input_samples_count;
                assert!(num_frames >= 1 && num_frames <= BATCH_SIZE);
                for i in 0..num_frames {
                    input_buf[i] = (samples.next().unwrap().unwrap() as f32) / 32768.0;
                }
            }
            _ => assert!(false),
        }

        let time_in_seconds: f64 = (time_in_samples as f64) / (input_spec.sample_rate as f64);
        patch.rust_process_audio(&input_buf, &mut output_buf, time_in_seconds);

        for i in 0..num_frames {
            writer
                .write_sample(sample_f32_to_i16(output_buf[i]))
                .unwrap();
        }

        time_in_samples += num_frames as u64;
    }
    writer.finalize().unwrap();
}
