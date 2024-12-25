
extern crate alloc;
extern crate libm;

use alloc::boxed::Box;

use shared::edsl::runtime::{signal::Signal, window::Window, range::Range, prim::add, prim::pass_thru};
use shared::knob::Knobs;
use shared::patch::Patch;
use shared::playhead::Playhead;
use shared::test::*;

pub struct EdslPatch {
    input_signal: Signal<f32>,
    signal0: Signal<f32>,
    output_signal: Signal<f32>,
}

impl EdslPatch {
    pub fn new() -> EdslPatch {
        EdslPatch {
            input_signal: Signal::new(),
            signal0: Signal::new(),
            output_signal: Signal::new(),
        }
    }
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
            self.input_signal.write(input_slice[i]);

            let pass_thru_0: Window<f32> = Window::new(&self.input_signal, Range(-3, 0));

            pass_thru(&pass_thru_0, &mut self.signal0);

            let add_0: Window<f32> = Window::new(&self.input_signal, Range(-2, 0));
            let add_1: Window<f32> = Window::new(&self.signal0, Range(-1, 0));
            add(&add_0, &add_1, &mut self.output_signal);

            output_slice[i] = self.output_signal.read(0);

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

pub const EDSL_PATCH_OUTPUT: &'static [f32] = &[
0.0,
0.115128055,
0.22987431,
0.3438582,
];

pub fn main() {
    let patch = Box::new(EdslPatch::new());
    let test_case = Box::new(TestCase {
            name: "edsl_patch",
            patch: patch,
            canned_input: INPUT,
            expected_output: EDSL_PATCH_OUTPUT,
        });
    test_patch(test_case.name, test_case.patch, test_case.canned_input, test_case.expected_output);
}
