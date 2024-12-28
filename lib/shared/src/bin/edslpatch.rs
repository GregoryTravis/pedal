extern crate alloc;
extern crate libm;

use alloc::boxed::Box;

use shared::edsl::runtime::{
    prim::{add, pass_thru, sum_filter},
    range::Range,
    signal::Signal,
    window::Window,
};
use shared::knob::Knobs;
use shared::patch::Patch;
use shared::playhead::Playhead;
use shared::test::*;
const MAX: usize = 10;
pub struct NodeyPatch {
    signal0: Signal<f32>,
    signal1: Signal<f32>,
    signal2: Signal<f32>,
    signal3: Signal<f32>,
    signal4: Signal<f32>,
    signal5: Signal<f32>,
}

impl NodeyPatch {
    pub fn new() -> NodeyPatch {
        NodeyPatch {
            signal0: Signal::new(MAX),
            signal1: Signal::new(MAX),
            signal2: Signal::new(MAX),
            signal3: Signal::new(MAX),
            signal4: Signal::new(MAX),
            signal5: Signal::new(MAX),
        }
    }
}

impl Patch for NodeyPatch {
    fn rust_process_audio(
        &mut self,
        input_slice: &[f32],
        output_slice: &mut [f32],
        _knobs: &Box<dyn Knobs>,
        mut playhead: Playhead,
    ) {
        for i in 0..input_slice.len() {
            self.signal3.write(input_slice[i]);

            println!("{} {}", 5, "sum_filter");
            let port0: Window<f32> = Window::new(&self.signal2, Range(-6, 0));
            sum_filter(&port0, &mut self.signal5);
            println!("{} {}", 4, "pass_thru");
            let port1: Window<f32> = Window::new(&self.signal3, Range(0, 0));
            pass_thru(&port1, &mut self.signal4);
            println!("{} {}", 2, "add");
            let port2: Window<f32> = Window::new(&self.signal3, Range(0, 0));
            let port3: Window<f32> = Window::new(&self.signal4, Range(0, 0));
            add(&port2, &port3, &mut self.signal2);
            println!("{} {}", 1, "sum_filter");
            let port4: Window<f32> = Window::new(&self.signal2, Range(-2, 0));
            sum_filter(&port4, &mut self.signal1);
            println!("{} {}", 0, "add");
            let port5: Window<f32> = Window::new(&self.signal1, Range(0, 0));
            let port6: Window<f32> = Window::new(&self.signal5, Range(0, 0));
            add(&port5, &port6, &mut self.signal0);

            output_slice[i] = self.signal0.read(0);

            playhead.inc();
        }
    }
}

pub const INPUT: &'static [f32] = &[0.0, 0.1, 0.2, 0.3];

/*
0
2
4
6

0
2
6
10

0
2
6
12
*/

pub const OUTPUT: &'static [f32] = &[0.0, 0.4, 0.12, 0.24];

pub fn main() {
    let patch = Box::new(NodeyPatch::new());
    let test_case = Box::new(TestCase {
        name: "NodeyPatch",
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
