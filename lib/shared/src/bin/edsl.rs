use std::fs::File;
use std::io::{BufWriter, Write};

fn main() {
    let f = File::create("pout").expect("Unable to create file");
    let mut f = BufWriter::new(f);
    f.write_all(S0.as_bytes()).expect("Unable to write data");
    f.write_all(S1.as_bytes()).expect("Unable to write data");
    println!("hi edsl");
}

const S0: &str = "\
extern crate alloc;
extern crate libm;

use alloc::boxed::Box;

use crate::knob::Knobs;
use crate::patch::Patch;
use crate::playhead::Playhead;

pub struct EdslPatch {
}

impl Patch for EdslPatch {
    fn rust_process_audio(
        &mut self,
        input_slice: &[f32],
        output_slice: &mut [f32],
        knobs: &Box<dyn Knobs>,
        mut playhead: Playhead,
    ) {
";

const S1: &str = "\
    }
}
";
