extern crate std;
extern crate alloc;
extern crate libm;

use alloc::vec::Vec;
use std::env;

use shared::file::*;
use shared::hop_fft::*;
use shared::spew::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_filename = &args[1];

    let input = file_read(input_filename);
    let result: Vec<f32> = hop_fft(&input, 4096, 48);
    spew!(result.len());
}

