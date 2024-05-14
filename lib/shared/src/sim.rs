extern crate std;
extern crate alloc;

use std::cmp::min;
use alloc::boxed::Box;
use std::path::Path;
use std::println;
use std::vec;
use std::vec::Vec;

use hound;
use crate::convert::*;
use crate::patch::Patch;
use crate::playhead::Playhead;

const BATCH_SIZE: usize = 48;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

pub fn sim_main(input_file: &str, output_file: &str, mut patch: Box<dyn Patch>) {
    let mut reader = hound::WavReader::open(input_file).unwrap();
    let input_spec = reader.spec();
    assert!(input_spec.channels == 1 || input_spec.channels == 2);
    let mut samples = reader.samples::<i16>();
    print_type_of(&samples);

    let path: &Path = output_file.as_ref();
    assert!(!path.is_file());

    let mut output_spec = input_spec;
    output_spec.channels = 1;
    let mut writer = hound::WavWriter::create(path, output_spec).unwrap();
    assert_eq!(output_spec, writer.spec());

    let mut input_buf: [f32; BATCH_SIZE] = [0.0; BATCH_SIZE];
    let mut output_buf: [f32; BATCH_SIZE] = [0.0; BATCH_SIZE];

    let mut playhead: Playhead = Playhead::new();

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

        patch.rust_process_audio(&input_buf, &mut output_buf, playhead);

        for i in 0..num_frames {
            writer
                .write_sample(sample_f32_to_i16(output_buf[i]))
                .unwrap();
        }

        playhead.increment_samples(num_frames as u64);
    }
    writer.finalize().unwrap();
}

pub fn sim_run_patch_on_buffer(mut patch: Box<dyn Patch>, input: &[f32]) -> Box<[f32]> {
    let len = input.len();
    let mut output: Vec<f32> = vec![0.0; len];

    let mut playhead: Playhead = Playhead::new();

    let mut sofar = 0;
    while sofar < len {
        let start = sofar;
        let end = min(sofar + BATCH_SIZE, len);
        println!("rpob {} {} {} {}", sofar, len, start, end);
        let sub_input = &input[start..end];
        let mut sub_output: &mut [f32] = &mut output[start..end];
//let body_slice: &mut [u8] = &mut myvec[10..1034];
        patch.rust_process_audio(&sub_input, &mut sub_output, playhead);
        playhead.increment_samples(BATCH_SIZE as u64);
        sofar += BATCH_SIZE;
    }

    output.into_boxed_slice()
}

pub fn sim_ramp_patch(patch: Box<dyn Patch>, num_samples: usize) {
    let mut input: Vec<f32> = vec![0.0; num_samples];
    for i in 0..num_samples {
        input[i] = i as f32;
    }
    let output = sim_run_patch_on_buffer(patch, input.as_slice());
    for i in 0..num_samples {
        println!("ramp {} {}", i, output[i]);
    }
}
