
#![allow(non_snake_case)]

extern crate alloc;
extern crate libm;

use alloc::boxed::Box;
use core::any::Any;

#[allow(unused_imports)]
use shared::edsl::runtime::{signal::Signal, window::Window, range::Range, prim::{AddPrim, Const, PassThru, SumFilter, HighPass, LowPass, LinearVibrato}};
use shared::knob::Knobs;
use shared::patch::Patch;
use shared::playhead::Playhead;
const MAX: usize = 10;
pub struct EdslLowPass6 {
    unitLowPass_5: LowPass,
    unitLowPass_4: LowPass,
    unitLowPass_3: LowPass,
    unitLowPass_2: LowPass,
    unitLowPass_1: LowPass,
    unitLowPass_0: LowPass,
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
                            unitLowPass_5: LowPass::new(),
    unitLowPass_4: LowPass::new(),
    unitLowPass_3: LowPass::new(),
    unitLowPass_2: LowPass::new(),
    unitLowPass_1: LowPass::new(),
    unitLowPass_0: LowPass::new(),
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
self.unitLowPass_5.go(&port5_0, &mut self.signal5);

let port4_0: Window<f32> = Window::new(&self.signal5, Range(-1, 0));
self.unitLowPass_4.go(&port4_0, &mut self.signal4);

let port3_0: Window<f32> = Window::new(&self.signal4, Range(-1, 0));
self.unitLowPass_3.go(&port3_0, &mut self.signal3);

let port2_0: Window<f32> = Window::new(&self.signal3, Range(-1, 0));
self.unitLowPass_2.go(&port2_0, &mut self.signal2);

let port1_0: Window<f32> = Window::new(&self.signal2, Range(-1, 0));
self.unitLowPass_1.go(&port1_0, &mut self.signal1);

let port0_0: Window<f32> = Window::new(&self.signal1, Range(-1, 0));
self.unitLowPass_0.go(&port0_0, &mut self.signal0);


            output_slice[i] = self.signal0.read(0);

            playhead.inc();
            
        }
    }

    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}

