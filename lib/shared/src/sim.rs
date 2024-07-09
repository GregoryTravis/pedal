extern crate std;
extern crate alloc;

use std::cmp::min;
use alloc::boxed::Box;
use std::path::Path;
use std::println;
//use std::vec;
//use std::vec::Vec;

use hound;
use crate::constants::*;
use crate::convert::*;
use crate::patch::Patch;
use crate::rig::*;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

pub fn sim_main(input_file: &str, output_file: &str, patch: Box<dyn Patch>) {
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

    let mut input_buf: [f32; BLOCK_SIZE] = [0.0; BLOCK_SIZE];
    let mut output_buf: [f32; BLOCK_SIZE] = [0.0; BLOCK_SIZE];

    let mut num_frames: usize = 0;

    rig_install_patch(patch);

    while samples.len() > 0 {
        match input_spec.channels {
            2 => {
                assert!(samples.len() % 2 == 0);
                let input_samples_count = min(samples.len(), BLOCK_SIZE * 2);
                num_frames = input_samples_count / 2;
                assert!(num_frames >= 1 && num_frames <= BLOCK_SIZE);
                for i in 0..num_frames {
                    input_buf[i] = sample_i16_to_f32(samples.next().unwrap().unwrap());
                    // Skip right channel
                    samples.next().unwrap().unwrap();
                }
            }
            1 => {
                let input_samples_count = min(samples.len(), BLOCK_SIZE);
                num_frames = input_samples_count;
                assert!(num_frames >= 1 && num_frames <= BLOCK_SIZE);
                for i in 0..num_frames {
                    input_buf[i] = (samples.next().unwrap().unwrap() as f32) / 32768.0;
                }
            }
            _ => assert!(false),
        }

        rust_process_audio_soft(&input_buf, &mut output_buf, BLOCK_SIZE);

        for i in 0..num_frames {
            writer
                .write_sample(sample_f32_to_i16(output_buf[i]))
                .unwrap();
        }
    }

    rig_deinstall_patch();

    writer.finalize().unwrap();
}

pub fn sim_run_patch_on_buffer(patch: Box<dyn Patch>, input: &[f32], output: &mut [f32]) {
    let len = input.len();

    rig_install_patch(patch);

    let mut sofar = 0;
    while sofar < len {
        let start = sofar;
        let end = min(sofar + BLOCK_SIZE, len);
        let block_length = end-start;
        //println!("rpob {} {} {} {}", sofar, len, start, end);
        let sub_input = &input[start..end];
        let mut sub_output: &mut [f32] = &mut output[start..end];
//let body_slice: &mut [u8] = &mut myvec[10..1034];

        //println!("rpob {} {}", sub_input.len(), sub_output.len());
        rust_process_audio_soft(&sub_input, &mut sub_output, block_length);
        //println!("rpob 2 {} {}", sub_input.len(), sub_output.len());

        sofar += BLOCK_SIZE;
        //println!("rpob hey");
    }

    rig_deinstall_patch();
}
