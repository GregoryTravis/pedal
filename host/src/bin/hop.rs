extern crate std;
extern crate alloc;
extern crate libm;

use alloc::vec::Vec;
use std::env;

use shared::file::*;
use shared::hop_fft::*;
use shared::unit::reso::*;
use shared::spew::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_filename = &args[1];
    let q = args[2].parse::<f32>().unwrap();

    let input = file_read(input_filename);
    let freqs: Vec<f32> = hop_fft(&input, 4096, 2048, 48);
    spew!(freqs.len());
    assert!(freqs.len() == input.len());

    let mut reso = Reso::new(q);

    let mut output = vec![0.0; input.len()];

    for i in 0..input.len() {
        spew!("peak", freqs[i]);
        if freqs[i] != 0.0 {
            reso.set_pitch(freqs[i]);
        }
        output[i] = reso.process(input[i]);
    }
    let output_filename = format!("{}-hop.wav", input_filename);
    file_write(&output_filename, &output);
}

