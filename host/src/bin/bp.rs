extern crate std;
extern crate alloc;
extern crate libm;

use alloc::vec::Vec;
use std::env;

use shared::file::*;
#[allow(unused)]
use shared::spew::*;
use shared::unit::band_pass::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_filename = &args[1];
    let freq = args[2].parse::<f32>().unwrap();
    let bw = args[3].parse::<f32>().unwrap();

    let input = file_read(input_filename);
    let mut output = vec![0.0; input.len()];

    let mut bp: BandPass = BandPass::new(freq, bw);
    for i in 0..input.len() {
        output[i] = bp.process(input[i]);
    }

    let output_filename = format!("{}-bp-{}-{}.wav", input_filename, freq, bw);
    file_write(&output_filename, &output);
}
