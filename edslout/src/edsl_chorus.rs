
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
const MAX: usize = 100;
pub struct EdslChorus {
    unitConst_2: Const,
    unitLinearVibrato_1: LinearVibrato,
    unitConst_6: Const,
    unitLinearVibrato_5: LinearVibrato,
    unitConst_8: Const,
    unitLinearVibrato_7: LinearVibrato,
    unitAddPrim_4: AddPrim,
    unitAddPrim_0: AddPrim,
    signal0: Signal<f32>,
    signal1: Signal<f32>,
    signal2: Signal<f32>,
    signal3: Signal<f32>,
    signal4: Signal<f32>,
    signal5: Signal<f32>,
    signal6: Signal<f32>,
    signal7: Signal<f32>,
    signal8: Signal<f32>,
}

            impl EdslChorus {
                pub fn new() -> EdslChorus {
                    EdslChorus {
                            unitConst_2: Const::new(2.7f32),
    unitLinearVibrato_1: LinearVibrato::new(20usize, -24isize),
    unitConst_6: Const::new(3f32),
    unitLinearVibrato_5: LinearVibrato::new(22usize, -26isize),
    unitConst_8: Const::new(3.3f32),
    unitLinearVibrato_7: LinearVibrato::new(18usize, -22isize),
    unitAddPrim_4: AddPrim::new(),
    unitAddPrim_0: AddPrim::new(),
    signal0: Signal::new(MAX),
    signal1: Signal::new(MAX),
    signal2: Signal::new(MAX),
    signal3: Signal::new(MAX),
    signal4: Signal::new(MAX),
    signal5: Signal::new(MAX),
    signal6: Signal::new(MAX),
    signal7: Signal::new(MAX),
    signal8: Signal::new(MAX),

                    }
                }
                
            }
            
impl Patch for EdslChorus {
    fn rust_process_audio(
        &mut self,
        input_slice: &[f32],
        output_slice: &mut [f32],
        _knobs: &Box<dyn Knobs>,
        mut playhead: Playhead,
    ) {
        for i in 0..input_slice.len() {
            self.signal3.write(input_slice[i]);


            self.unitConst_2.go(playhead, &mut self.signal2);

let port1_0: Window<f32> = Window::new(&self.signal2, Range(0, 0));
let port1_1: Window<f32> = Window::new(&self.signal3, Range(-49, 0));
self.unitLinearVibrato_1.go(playhead, &port1_0, &port1_1, &mut self.signal1);

self.unitConst_6.go(playhead, &mut self.signal6);

let port5_0: Window<f32> = Window::new(&self.signal6, Range(0, 0));
let port5_1: Window<f32> = Window::new(&self.signal3, Range(-53, 0));
self.unitLinearVibrato_5.go(playhead, &port5_0, &port5_1, &mut self.signal5);

self.unitConst_8.go(playhead, &mut self.signal8);

let port7_0: Window<f32> = Window::new(&self.signal8, Range(0, 0));
let port7_1: Window<f32> = Window::new(&self.signal3, Range(-45, 0));
self.unitLinearVibrato_7.go(playhead, &port7_0, &port7_1, &mut self.signal7);

let port4_0: Window<f32> = Window::new(&self.signal5, Range(0, 0));
let port4_1: Window<f32> = Window::new(&self.signal7, Range(0, 0));
self.unitAddPrim_4.go(playhead, &port4_0, &port4_1, &mut self.signal4);

let port0_0: Window<f32> = Window::new(&self.signal1, Range(0, 0));
let port0_1: Window<f32> = Window::new(&self.signal4, Range(0, 0));
self.unitAddPrim_0.go(playhead, &port0_0, &port0_1, &mut self.signal0);


            output_slice[i] = self.signal0.read(0);

            playhead.inc();
            
        }
    }

    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}

