
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
pub struct EdslLinearVibrato {
    unitConst_1: Const,
    unitLinearVibrato_0: LinearVibrato,
    signal0: Signal<f32>,
    signal1: Signal<f32>,
    signal2: Signal<f32>,
}

            impl EdslLinearVibrato {
                pub fn new() -> EdslLinearVibrato {
                    EdslLinearVibrato {
                            unitConst_1: Const::new(1f32),
    unitLinearVibrato_0: LinearVibrato::new(10usize, 14usize),
    signal0: Signal::new(MAX),
    signal1: Signal::new(MAX),
    signal2: Signal::new(MAX),

                    }
                }
                
            }
            
impl Patch for EdslLinearVibrato {
    fn rust_process_audio(
        &mut self,
        input_slice: &[f32],
        output_slice: &mut [f32],
        _knobs: &Box<dyn Knobs>,
        mut playhead: Playhead,
    ) {
        for i in 0..input_slice.len() {
            self.signal2.write(input_slice[i]);


            self.unitConst_1.go(playhead, &mut self.signal1);

let port0_0: Window<f32> = Window::new(&self.signal1, Range(0, 0));
let port0_1: Window<f32> = Window::new(&self.signal2, Range(-29, 0));
self.unitLinearVibrato_0.go(playhead, &port0_0, &port0_1, &mut self.signal0);


            output_slice[i] = self.signal0.read(0);

            playhead.inc();
            
        }
    }

    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}

