
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
#[allow(unused)]
use shared::sdram::*;
const MAX: usize = 100;
pub struct EdslModLV {
    unitConst_3: Const,
    unitConst_4: Const,
    unitConst_5: Const,
    unitSinePrim_2: SinePrim,
    unitConst_6: Const,
    unitSinePrim_1: SinePrim,
    unitVariSpeed_0: VariSpeed,
    signal0: Signal<f32>,
    signal1: Signal<f32>,
    signal2: Signal<f32>,
    signal3: Signal<f32>,
    signal4: Signal<f32>,
    signal5: Signal<f32>,
    signal6: Signal<f32>,
    signal7: Signal<f32>,
}

            impl EdslModLV {
                pub fn new() -> EdslModLV {
                    EdslModLV {
                            unitConst_3: Const::new(892.7f32),
    unitConst_4: Const::new(1f32),
    unitConst_5: Const::new(0f32),
    unitSinePrim_2: SinePrim::new(),
    unitConst_6: Const::new(22f32),
    unitSinePrim_1: SinePrim::new(),
    unitVariSpeed_0: VariSpeed::new(-26isize),
    signal0: Signal::new(MAX),
    signal1: Signal::new(MAX),
    signal2: Signal::new(MAX),
    signal3: Signal::new(MAX),
    signal4: Signal::new(MAX),
    signal5: Signal::new(MAX),
    signal6: Signal::new(MAX),
    signal7: Signal::new(MAX),

                    }
                }
                
            }
            
impl Patch for EdslModLV {
    fn rust_process_audio(
        &mut self,
        input_slice: &[f32],
        output_slice: &mut [f32],
        _knobs: &Box<dyn Knobs>,
        mut playhead: Playhead,
    ) {
        for i in 0..input_slice.len() {
            self.signal7.write(input_slice[i]);


            self.unitConst_3.go(playhead, &mut self.signal3);

self.unitConst_4.go(playhead, &mut self.signal4);

self.unitConst_5.go(playhead, &mut self.signal5);

let port2_0: Window<f32> = Window::new(&self.signal3, Range(0, 0));
let port2_1: Window<f32> = Window::new(&self.signal4, Range(0, 0));
let port2_2: Window<f32> = Window::new(&self.signal5, Range(0, 0));
self.unitSinePrim_2.go(playhead, &port2_0, &port2_1, &port2_2, &mut self.signal2);

self.unitConst_6.go(playhead, &mut self.signal6);

let port1_0: Window<f32> = Window::new(&self.signal2, Range(0, 0));
let port1_1: Window<f32> = Window::new(&self.signal6, Range(0, 0));
let port1_2: Window<f32> = Window::new(&self.signal5, Range(0, 0));
self.unitSinePrim_1.go(playhead, &port1_0, &port1_1, &port1_2, &mut self.signal1);

let port0_0: Window<f32> = Window::new(&self.signal1, Range(0, 0));
let port0_1: Window<f32> = Window::new(&self.signal7, Range(-53, 0));
self.unitVariSpeed_0.go(playhead, &port0_0, &port0_1, &mut self.signal0);


            output_slice[i] = self.signal0.read(0);

            playhead.inc();
            
        }
    }

    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}

