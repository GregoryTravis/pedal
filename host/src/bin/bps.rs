extern crate std;
extern crate alloc;
extern crate libm;

use alloc::vec::Vec;
use std::env;

use shared::file::*;
use shared::hop_fft::*;
#[allow(unused)]
use shared::spew::*;
use shared::unit::band_pass_bank::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_filename = &args[1];
    let bw = args[2].parse::<f32>().unwrap();

    let input = file_read(input_filename);
    let mut output = vec![0.0; input.len()];

    let mut buf: [f32; 2048] = [0.0; 2048];

    let mut bank = BandPassBank::new();

    let hop = 48;

    for current_start in (0..input.len()).step_by(hop) {
        if current_start+hop > input.len() {
            break;
        }

        let input_batch = &input[current_start..current_start+hop];

        assert!(input_batch.len() == hop);

        // Shift new samples in
        for i in 0..2048-hop {
            buf[i] = buf[i+hop];
        }
        for i in 0..hop {
            buf[2048-hop+i] = input_batch[i];
        }

        let fas: Vec<f32> = hop_peaks(current_start, &buf, 2048, 2048);
        bank.update(&fas);

        let mut output_batch: Vec<f32> = vec![0.0; hop];

        for i in 0..hop {
            output_batch[i] = bank.process(input_batch[i]);
        }

        for i in 0..hop {
            output[current_start+i] = output_batch[i];
        }

        bank.dump("FINAL FINAL");
    }

    let output_filename = format!("{}-bp-{}.wav", input_filename, bw);
    file_write(&output_filename, &output);
}

