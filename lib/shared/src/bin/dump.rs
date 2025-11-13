extern crate std;

use std::env;
use std::println;

use shared::file::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_filename = &args[1];

    let input = file_read(input_filename);
    for i in 0..input.len() {
        println!("{}: {}", i, input[i]);
    }
}
