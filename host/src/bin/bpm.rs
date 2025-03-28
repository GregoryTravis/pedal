extern crate std;
extern crate alloc;
extern crate libm;

use alloc::vec::Vec;
use std::env;

use shared::file::*;
use shared::unit::band_pass::*;
#[allow(unused)]
use shared::spew::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_filename = &args[1];
    let bw = args[2].parse::<f32>().unwrap();

    let freqs: Vec<f32> = vec![172.0, 220.0, 328.0];
    //let freqs: Vec<f32> = vec![770.0];

    let input = file_read(input_filename);
    let mut output = vec![0.0; input.len()];

    for freq in freqs {
        let mut bp: BandPass = BandPass::new(freq, bw);
        for i in 0..input.len() {
            output[i] += bp.process(input[i]);
        }
    }

    let output_filename = format!("{}-bp-{}.wav", input_filename, bw);
    file_write(&output_filename, &output);
}
