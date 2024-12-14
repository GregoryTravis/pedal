use std::fs::File;
use std::io::{BufWriter, Write};

fn main() {
    let f = File::create("src/bin/edslpatch.rs").expect("Unable to create file");
    let mut f = BufWriter::new(f);
    f.write_all(S0.as_bytes()).expect("Unable to write data");
    f.write_all(S1.as_bytes()).expect("Unable to write data");
    println!("hi edsl");
}

const S0: &str = r#"
extern crate alloc;
extern crate libm;

use alloc::boxed::Box;

use shared::edsl::Buffer;
use shared::knob::Knobs;
use shared::patch::Patch;
use shared::playhead::Playhead;
use shared::test::*;

const BATCH_SIZE: usize = 4;

pub struct EdslPatch<const B: usize> {
    // TODO calculcate 19
    input_buffer: Buffer<10, 5, B, 19>,
    output_buffer: Buffer<0, 0, B, B>,
}

impl <const B: usize> EdslPatch<B> {
    pub fn new() -> EdslPatch<B> {
        EdslPatch {
            input_buffer: Buffer::<10, 5, B, 19>::new(),
            output_buffer: Buffer::<0, 0, B, B>::new(),
        }
    }
}

impl <const B: usize> Patch for EdslPatch<B> {
    fn rust_process_audio(
        &mut self,
        input_slice: &[f32],
        output_slice: &mut [f32],
        _knobs: &Box<dyn Knobs>,
        mut playhead: Playhead,
    ) {
        for i in 0..input_slice.len() {
            let input_sample = input_slice[i];

            self.input_buffer.write(i, input_sample);

            self.output_buffer.write(i, self.input_buffer.read(i.try_into().unwrap()));

            let output_sample = self.output_buffer.read(i.try_into().unwrap());

            output_slice[i] = output_sample;
            playhead.inc();
        }
"#;

const S1: &str = r#"
    }
}

pub const INPUT: &'static [f32] = &[
0.0,
0.057564028,
0.11493716,
0.1719291,
];

pub const OUTPUT: &'static [f32] = &[
0.0,
0.057564028,
0.11493716,
0.1719291,
];

pub fn main() {
    let patch = Box::new(EdslPatch::<BATCH_SIZE>::new());
    let test_case = Box::new(TestCase {
            name: "edsl_patch",
            patch: patch,
            canned_input: INPUT,
            expected_output: OUTPUT,
        });
    test_patch(test_case.name, test_case.patch, test_case.canned_input, test_case.expected_output);
}
"#;
