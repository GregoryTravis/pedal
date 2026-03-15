
#![allow(non_snake_case)]

extern crate alloc;
extern crate libm;

use alloc::boxed::Box;
use core::any::Any;

#[allow(unused_imports)]
use shared::edsl::runtime::{signal::Signal, window::Window, range::Range, prim::{AddPrim, DivPrim, Const, PassThru, SumFilter, HighPass, LowPass, LinearVibrato, SinePrim, VariSpeed, PatchPrim}};
use shared::knob::Knobs;
use shared::patch::Patch;
use shared::playhead::Playhead;
const MAX: usize = 100;
pub struct EdslEasyTweak {
    unitConst_2: Const,
    unitConst_3: Const,
    unitConst_4: Const,
    unitSinePrim_1: SinePrim,
    unitLinearVibrato_0: LinearVibrato,
    signal0: Signal<f32>,
    signal1: Signal<f32>,
    signal2: Signal<f32>,
    signal3: Signal<f32>,
    signal4: Signal<f32>,
    signal5: Signal<f32>,
}

            impl EdslEasyTweak {
                pub fn new() -> EdslEasyTweak {
                    EdslEasyTweak {
                            unitConst_2: Const::new(1f32),
    unitConst_3: Const::new(22f32),
    unitConst_4: Const::new(0f32),
    unitSinePrim_1: SinePrim::new(),
    unitLinearVibrato_0: LinearVibrato::new(18usize, -22isize),
    signal0: Signal::new(MAX),
    signal1: Signal::new(MAX),
    signal2: Signal::new(MAX),
    signal3: Signal::new(MAX),
    signal4: Signal::new(MAX),
    signal5: Signal::new(MAX),

                    }
                }
                
            }
            
impl Patch for EdslEasyTweak {
    fn rust_process_audio(
        &mut self,
        input_slice: &[f32],
        output_slice: &mut [f32],
        _knobs: &Box<dyn Knobs>,
        mut playhead: Playhead,
    ) {
        for i in 0..input_slice.len() {
            self.signal5.write(input_slice[i]);


            self.unitConst_2.go(playhead, &mut self.signal2);

self.unitConst_3.go(playhead, &mut self.signal3);

self.unitConst_4.go(playhead, &mut self.signal4);

let port1_0: Window<f32> = Window::new(&self.signal2, Range(0, 0));
let port1_1: Window<f32> = Window::new(&self.signal3, Range(0, 0));
let port1_2: Window<f32> = Window::new(&self.signal4, Range(0, 0));
self.unitSinePrim_1.go(playhead, &port1_0, &port1_1, &port1_2, &mut self.signal1);

let port0_0: Window<f32> = Window::new(&self.signal1, Range(0, 0));
let port0_1: Window<f32> = Window::new(&self.signal5, Range(-45, 0));
self.unitLinearVibrato_0.go(playhead, &port0_0, &port0_1, &mut self.signal0);


            output_slice[i] = self.signal0.read(0);

            playhead.inc();
            
        }
    }

    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}

