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

    let mut bank = BandPassBank::new();

    let hop = 48;
    let fases: Vec<Vec<(f32, f32)>> = hop_peaks(&input, 4096, 2048, hop);

    for (batch, fas) in fases.iter().enumerate() {
        let current_start = batch * hop;
        println!("==== {}", current_start);
        bank.update(&fas);
        for i in 0..hop {
            let current = current_start + i;
            if current >= input.len() {
                break;
            }
            output[current] = bank.process(input[current]);
        }
        bank.dump("FINAL FINAL");
    }

    let output_filename = format!("{}-bp-{}.wav", input_filename, bw);
    file_write(&output_filename, &output);
}

