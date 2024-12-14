
extern crate alloc;
extern crate libm;

use alloc::boxed::Box;

use shared::knob::Knobs;
use shared::patch::Patch;
use shared::playhead::Playhead;
use shared::test::*;

pub struct EdslPatch {
}

impl Patch for EdslPatch {
    fn rust_process_audio(
        &mut self,
        input_slice: &[f32],
        output_slice: &mut [f32],
        _knobs: &Box<dyn Knobs>,
        mut playhead: Playhead,
    ) {
        for i in 0..input_slice.len() {
            output_slice[i] = input_slice[i];
            playhead.inc();
        }

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
    let patch = Box::new(EdslPatch {});
    let test_case = Box::new(TestCase {
            name: "edsl_patch",
            patch: Box::new(EdslPatch {}),
            canned_input: INPUT,
            expected_output: OUTPUT,
        });
    test_patch(test_case.name, patch, test_case.canned_input, test_case.expected_output);
}
