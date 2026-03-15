
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
pub struct EdslChorus2 {
    unitConst_5: Const,
    unitConst_6: Const,
    unitConst_7: Const,
    unitSinePrim_4: SinePrim,
    unitVariSpeed_3: VariSpeed,
    unitConst_11: Const,
    unitConst_12: Const,
    unitSinePrim_10: SinePrim,
    unitVariSpeed_9: VariSpeed,
    unitAddPrim_2: AddPrim,
    unitConst_15: Const,
    unitConst_16: Const,
    unitSinePrim_14: SinePrim,
    unitVariSpeed_13: VariSpeed,
    unitAddPrim_1: AddPrim,
    unitDivPrim_0: DivPrim,
    signal0: Signal<f32>,
    signal1: Signal<f32>,
    signal2: Signal<f32>,
    signal3: Signal<f32>,
    signal4: Signal<f32>,
    signal5: Signal<f32>,
    signal6: Signal<f32>,
    signal7: Signal<f32>,
    signal8: Signal<f32>,
    signal9: Signal<f32>,
    signal10: Signal<f32>,
    signal11: Signal<f32>,
    signal12: Signal<f32>,
    signal13: Signal<f32>,
    signal14: Signal<f32>,
    signal15: Signal<f32>,
    signal16: Signal<f32>,
}

            impl EdslChorus2 {
                pub fn new() -> EdslChorus2 {
                    EdslChorus2 {
                            unitConst_5: Const::new(2.7f32),
    unitConst_6: Const::new(20f32),
    unitConst_7: Const::new(0f32),
    unitSinePrim_4: SinePrim::new(),
    unitVariSpeed_3: VariSpeed::new(-24isize),
    unitConst_11: Const::new(3f32),
    unitConst_12: Const::new(22f32),
    unitSinePrim_10: SinePrim::new(),
    unitVariSpeed_9: VariSpeed::new(-26isize),
    unitAddPrim_2: AddPrim::new(),
    unitConst_15: Const::new(3.3f32),
    unitConst_16: Const::new(18f32),
    unitSinePrim_14: SinePrim::new(),
    unitVariSpeed_13: VariSpeed::new(-22isize),
    unitAddPrim_1: AddPrim::new(),
    unitDivPrim_0: DivPrim::new(),
    signal0: Signal::new(MAX),
    signal1: Signal::new(MAX),
    signal2: Signal::new(MAX),
    signal3: Signal::new(MAX),
    signal4: Signal::new(MAX),
    signal5: Signal::new(MAX),
    signal6: Signal::new(MAX),
    signal7: Signal::new(MAX),
    signal8: Signal::new(MAX),
    signal9: Signal::new(MAX),
    signal10: Signal::new(MAX),
    signal11: Signal::new(MAX),
    signal12: Signal::new(MAX),
    signal13: Signal::new(MAX),
    signal14: Signal::new(MAX),
    signal15: Signal::new(MAX),
    signal16: Signal::new(MAX),

                    }
                }
                
            }
            
impl Patch for EdslChorus2 {
    fn rust_process_audio(
        &mut self,
        input_slice: &[f32],
        output_slice: &mut [f32],
        _knobs: &Box<dyn Knobs>,
        mut playhead: Playhead,
    ) {
        for i in 0..input_slice.len() {
            self.signal8.write(input_slice[i]);


            self.unitConst_5.go(playhead, &mut self.signal5);

self.unitConst_6.go(playhead, &mut self.signal6);

self.unitConst_7.go(playhead, &mut self.signal7);

let port4_0: Window<f32> = Window::new(&self.signal5, Range(0, 0));
let port4_1: Window<f32> = Window::new(&self.signal6, Range(0, 0));
let port4_2: Window<f32> = Window::new(&self.signal7, Range(0, 0));
self.unitSinePrim_4.go(playhead, &port4_0, &port4_1, &port4_2, &mut self.signal4);

let port3_0: Window<f32> = Window::new(&self.signal4, Range(0, 0));
let port3_1: Window<f32> = Window::new(&self.signal8, Range(-49, 0));
self.unitVariSpeed_3.go(playhead, &port3_0, &port3_1, &mut self.signal3);

self.unitConst_11.go(playhead, &mut self.signal11);

self.unitConst_12.go(playhead, &mut self.signal12);

let port10_0: Window<f32> = Window::new(&self.signal11, Range(0, 0));
let port10_1: Window<f32> = Window::new(&self.signal12, Range(0, 0));
let port10_2: Window<f32> = Window::new(&self.signal7, Range(0, 0));
self.unitSinePrim_10.go(playhead, &port10_0, &port10_1, &port10_2, &mut self.signal10);

let port9_0: Window<f32> = Window::new(&self.signal10, Range(0, 0));
let port9_1: Window<f32> = Window::new(&self.signal8, Range(-53, 0));
self.unitVariSpeed_9.go(playhead, &port9_0, &port9_1, &mut self.signal9);

let port2_0: Window<f32> = Window::new(&self.signal3, Range(0, 0));
let port2_1: Window<f32> = Window::new(&self.signal9, Range(0, 0));
self.unitAddPrim_2.go(playhead, &port2_0, &port2_1, &mut self.signal2);

self.unitConst_15.go(playhead, &mut self.signal15);

self.unitConst_16.go(playhead, &mut self.signal16);

let port14_0: Window<f32> = Window::new(&self.signal15, Range(0, 0));
let port14_1: Window<f32> = Window::new(&self.signal16, Range(0, 0));
let port14_2: Window<f32> = Window::new(&self.signal7, Range(0, 0));
self.unitSinePrim_14.go(playhead, &port14_0, &port14_1, &port14_2, &mut self.signal14);

let port13_0: Window<f32> = Window::new(&self.signal14, Range(0, 0));
let port13_1: Window<f32> = Window::new(&self.signal8, Range(-45, 0));
self.unitVariSpeed_13.go(playhead, &port13_0, &port13_1, &mut self.signal13);

let port1_0: Window<f32> = Window::new(&self.signal2, Range(0, 0));
let port1_1: Window<f32> = Window::new(&self.signal13, Range(0, 0));
self.unitAddPrim_1.go(playhead, &port1_0, &port1_1, &mut self.signal1);

let port0_0: Window<f32> = Window::new(&self.signal1, Range(0, 0));
let port0_1: Window<f32> = Window::new(&self.signal11, Range(0, 0));
self.unitDivPrim_0.go(playhead, &port0_0, &port0_1, &mut self.signal0);


            output_slice[i] = self.signal0.read(0);

            playhead.inc();
            
        }
    }

    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}

