
#![allow(non_snake_case)]

extern crate alloc;
extern crate libm;

use alloc::boxed::Box;
use core::any::Any;

#[allow(unused_imports)]
use shared::edsl::runtime::{signal::Signal, window::Window, range::Range, prim::{AddPrim, Const, PassThru, SumFilter, HighPass, LowPass}};
use shared::knob::Knobs;
use shared::patch::Patch;
use shared::playhead::Playhead;
const MAX: usize = 10;
pub struct EdslNodey {
    unitPassThru_4: PassThru,
    unitAddPrim_2: AddPrim,
    unitSumFilter_1: SumFilter,
    unitSumFilter_5: SumFilter,
    unitAddPrim_0: AddPrim,
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
                            unitPassThru_4: PassThru::new(),
    unitAddPrim_2: AddPrim::new(),
    unitSumFilter_1: SumFilter::new(),
    unitSumFilter_5: SumFilter::new(),
    unitAddPrim_0: AddPrim::new(),
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
self.unitPassThru_4.go(&port4_0, &mut self.signal4);

let port2_0: Window<f32> = Window::new(&self.signal3, Range(0, 0));
let port2_1: Window<f32> = Window::new(&self.signal4, Range(0, 0));
self.unitAddPrim_2.go(&port2_0, &port2_1, &mut self.signal2);

let port1_0: Window<f32> = Window::new(&self.signal2, Range(-2, 0));
self.unitSumFilter_1.go(&port1_0, &mut self.signal1);

let port5_0: Window<f32> = Window::new(&self.signal2, Range(-6, 0));
self.unitSumFilter_5.go(&port5_0, &mut self.signal5);

let port0_0: Window<f32> = Window::new(&self.signal1, Range(0, 0));
let port0_1: Window<f32> = Window::new(&self.signal5, Range(0, 0));
self.unitAddPrim_0.go(&port0_0, &port0_1, &mut self.signal0);


            output_slice[i] = self.signal0.read(0);

            playhead.inc();
            
        }
    }

    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}

