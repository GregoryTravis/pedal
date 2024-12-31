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
pub struct EdslLowPass6 {
    signal0: Signal<f32>,
    signal1: Signal<f32>,
    signal2: Signal<f32>,
    signal3: Signal<f32>,
    signal4: Signal<f32>,
    signal5: Signal<f32>,
    signal6: Signal<f32>,
}

impl EdslLowPass6 {
    pub fn new() -> EdslLowPass6 {
        EdslLowPass6 {
            signal0: Signal::new(MAX),
            signal1: Signal::new(MAX),
            signal2: Signal::new(MAX),
            signal3: Signal::new(MAX),
            signal4: Signal::new(MAX),
            signal5: Signal::new(MAX),
            signal6: Signal::new(MAX),
        }
    }
}

impl Patch for EdslLowPass6 {
    fn rust_process_audio(
        &mut self,
        input_slice: &[f32],
        output_slice: &mut [f32],
        _knobs: &Box<dyn Knobs>,
        mut playhead: Playhead,
    ) {
        for i in 0..input_slice.len() {
            self.signal6.write(input_slice[i]);

            let port5_0: Window<f32> = Window::new(&self.signal6, Range(-1, 0));
            low_pass(&port5_0, &mut self.signal5);

            let port4_0: Window<f32> = Window::new(&self.signal5, Range(-1, 0));
            low_pass(&port4_0, &mut self.signal4);

            let port3_0: Window<f32> = Window::new(&self.signal4, Range(-1, 0));
            low_pass(&port3_0, &mut self.signal3);

            let port2_0: Window<f32> = Window::new(&self.signal3, Range(-1, 0));
            low_pass(&port2_0, &mut self.signal2);

            let port1_0: Window<f32> = Window::new(&self.signal2, Range(-1, 0));
            low_pass(&port1_0, &mut self.signal1);

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
    let patch = Box::new(EdslLowPass6::new());
    let test_case = Box::new(TestCase {
        name: "EdslLowPass6",
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
