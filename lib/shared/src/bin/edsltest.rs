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

use shared::edsl::{cursor::Cursor, buffer::Buffer, prim::add, prim::pass_thru};
use shared::knob::Knobs;
use shared::patch::Patch;
use shared::playhead::Playhead;
use shared::test::*;

const BATCH_SIZE: usize = 4;

pub struct EdslPatch<const B: usize> {
    // TODO calculcate 19
    input_buffer: Buffer<10, 5, B, 19>,
    a_buffer: Buffer<0, 0, B, B>,
    output_buffer: Buffer<0, 0, B, B>,
}

impl <const B: usize> EdslPatch<B> {
    pub fn new() -> EdslPatch<B> {
        EdslPatch {
            input_buffer: Buffer::<10, 5, B, 19>::new(),
            a_buffer: Buffer::<0, 0, B, B>::new(),
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
        }

        let cursor_sia_to_ib = Cursor::<10, 5, B, 19>::new(&mut self.input_buffer);

        let mut cursor_a_buffer = Cursor::<0, 0, B, B>::new(&mut self.a_buffer);

        for i in 0..input_slice.len() {
            pass_thru(i, &cursor_sia_to_ib, &mut cursor_a_buffer);
        }

        let mut cursor_ob_to_soa = Cursor::<0, 0, B, B>::new(&mut self.output_buffer);

        for i in 0..input_slice.len() {
            add(i, &cursor_sia_to_ib, &cursor_a_buffer, &mut cursor_ob_to_soa);
        }

        for i in 0..input_slice.len() {
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

pub const EDSL_PATCH_OUTPUT: &'static [f32] = &[
0.0,
0.115128055,
0.22987431,
0.3438582,
];

pub fn main() {
    let patch = Box::new(EdslPatch::<BATCH_SIZE>::new());
    let test_case = Box::new(TestCase {
            name: "edsl_patch",
            patch: patch,
            canned_input: INPUT,
            expected_output: EDSL_PATCH_OUTPUT,
        });
    test_patch(test_case.name, test_case.patch, test_case.canned_input, test_case.expected_output);
}
"#;
