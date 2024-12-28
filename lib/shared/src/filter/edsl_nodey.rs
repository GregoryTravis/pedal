extern crate alloc;
extern crate libm;

use alloc::boxed::Box;

use crate::edsl::runtime::{
    prim::{add, pass_thru, sum_filter},
    range::Range,
    signal::Signal,
    window::Window,
};
use crate::knob::Knobs;
use crate::patch::Patch;
use crate::playhead::Playhead;
use crate::test::*;
const MAX: usize = 10;
pub struct EdslNodey {
    signal0: Signal<f32>,
    signal1: Signal<f32>,
    signal2: Signal<f32>,
    signal3: Signal<f32>,
    signal4: Signal<f32>,
    signal5: Signal<f32>,
}

impl EdslNodey {
    pub fn new() -> EdslNodey {
        EdslNodey {
            signal0: Signal::new(MAX),
            signal1: Signal::new(MAX),
            signal2: Signal::new(MAX),
            signal3: Signal::new(MAX),
            signal4: Signal::new(MAX),
            signal5: Signal::new(MAX),
        }
    }
}

impl Patch for EdslNodey {
    fn rust_process_audio(
        &mut self,
        input_slice: &[f32],
        output_slice: &mut [f32],
        _knobs: &Box<dyn Knobs>,
        mut playhead: Playhead,
    ) {
        for i in 0..input_slice.len() {
            self.signal3.write(input_slice[i]);

            let port4_0: Window<f32> = Window::new(&self.signal3, Range(0, 0));
            pass_thru(&port4_0, &mut self.signal4);

            let port2_0: Window<f32> = Window::new(&self.signal3, Range(0, 0));
            let port2_1: Window<f32> = Window::new(&self.signal4, Range(0, 0));
            add(&port2_0, &port2_1, &mut self.signal2);

            let port1_0: Window<f32> = Window::new(&self.signal2, Range(-2, 0));
            sum_filter(&port1_0, &mut self.signal1);

            let port5_0: Window<f32> = Window::new(&self.signal2, Range(-6, 0));
            sum_filter(&port5_0, &mut self.signal5);

            let port0_0: Window<f32> = Window::new(&self.signal1, Range(0, 0));
            let port0_1: Window<f32> = Window::new(&self.signal5, Range(0, 0));
            add(&port0_0, &port0_1, &mut self.signal0);

            output_slice[i] = self.signal0.read(0);

            playhead.inc();
        }
    }
}

pub const INPUT: &'static [f32] = &[0.0, 0.1, 0.2, 0.3];

pub const OUTPUT: &'static [f32] = &[0.0, 0.4, 1.2, 2.4];

pub fn main() {
    let patch = Box::new(EdslNodey::new());
    let test_case = Box::new(TestCase {
        name: "EdslNodey",
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
