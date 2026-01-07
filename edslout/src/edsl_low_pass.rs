
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
pub struct EdslLowPass {
    unitLowPass_0: LowPass,
    signal0: Signal<f32>,
    signal1: Signal<f32>,
}

            impl EdslLowPass {
                pub fn new() -> EdslLowPass {
                    EdslLowPass {
                            unitLowPass_0: LowPass::new(),
    signal0: Signal::new(MAX),
    signal1: Signal::new(MAX),

                    }
                }
                
            }
            
impl Patch for EdslLowPass {
    fn rust_process_audio(
        &mut self,
        input_slice: &[f32],
        output_slice: &mut [f32],
        _knobs: &Box<dyn Knobs>,
        mut playhead: Playhead,
    ) {
        for i in 0..input_slice.len() {
            self.signal1.write(input_slice[i]);


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

