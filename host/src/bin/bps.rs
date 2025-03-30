extern crate std;
extern crate alloc;
extern crate libm;

use alloc::vec::Vec;
use std::env;

use shared::file::*;
use shared::hop_fft::*;
#[allow(unused)]
use shared::spew::*;
use shared::unit::band_pass::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_filename = &args[1];
    let bw = args[2].parse::<f32>().unwrap();

    let input = file_read(input_filename);
    let mut output = vec![0.0; input.len()];

    let hop = 48;
    let fmses: Vec<Vec<(f32, f32)>> = hop_peaks(&input, 4096, 2048, hop);

    for (batch, fms) in fmses.iter().enumerate() {
        let mut bpas: Vec<(BandPass, f32)> = fms.iter().map(|(freq, amp)| (BandPass::new(*freq, bw), *amp)).collect();
        let current_start = batch * hop;
        for i in 0..hop {
            let current = current_start + i;
            if current >= input.len() {
                break;
            }
            for (ref mut bp, amp) in &mut bpas {
                output[current] += *amp * bp.process(input[current]);
            }
        }
    }

    let output_filename = format!("{}-bp-{}.wav", input_filename, bw);
    file_write(&output_filename, &output);
}

