extern crate alloc;
extern crate libm;

use alloc::boxed::Box;

#[allow(unused_imports)]
use crate::edsl::runtime::{
    prim::{add, high_pass, low_pass, pass_thru, sum_filter},
    range::Range,
    signal::Signal,
    window::Window,
};
use crate::knob::Knobs;
use crate::patch::Patch;
use crate::playhead::Playhead;
use crate::test::*;
const MAX: usize = 10;
pub struct EdslLowPass {
    signal0: Signal<f32>,
    signal1: Signal<f32>,
}

impl EdslLowPass {
    pub fn new() -> EdslLowPass {
        EdslLowPass {
            signal0: Signal::new(MAX),
            signal1: Signal::new(MAX),
        }
    }
}

impl Patch for EdslLowPass {
    fn rust_process_audio(
        &mut self,
        input_slice: &[f32],
        output_slice: &mut [f32],
        _knobs: &Box<dyn Knobs>,
        mut playhead: Playhead,
    ) {
        for i in 0..input_slice.len() {
            self.signal1.write(input_slice[i]);

            let port0_0: Window<f32> = Window::new(&self.signal1, Range(-1, 0));
            low_pass(&port0_0, &mut self.signal0);

            output_slice[i] = self.signal0.read(0);

            playhead.inc();
        }
    }
}

pub const INPUT: &'static [f32] = &[0.0, 0.1, 0.2, 0.3];

pub const OUTPUT: &'static [f32] = &[0.0, 0.4, 1.2, 2.4];

pub fn main() {
    let patch = Box::new(EdslLowPass::new());
    let test_case = Box::new(TestCase {
        name: "EdslLowPass",
        patch: patch,
        canned_input: INPUT,
        expected_output: OUTPUT,
    });
    test_patch(
        test_case.name,
        test_case.patch,
        test_case.canned_input,
        test_case.expected_output,
    );
}
