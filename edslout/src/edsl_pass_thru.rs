
#![allow(non_snake_case)]

extern crate alloc;
extern crate libm;

use alloc::boxed::Box;
use core::any::Any;

#[allow(unused_imports)]
use shared::edsl::runtime::{signal::Signal, window::Window, range::Range, prim::{AddPrim, DivPrim, Const, PassThru, SumFilter, HighPass, LowPass, LinearVibrato, SinePrim, VariSpeed}};
use shared::knob::Knobs;
use shared::patch::Patch;
use shared::playhead::Playhead;
const MAX: usize = 100;
pub struct EdslPassThru {
    unitPassThru_0: PassThru,
    signal0: Signal<f32>,
    signal1: Signal<f32>,
}

            impl EdslPassThru {
                pub fn new() -> EdslPassThru {
                    EdslPassThru {
                            unitPassThru_0: PassThru::new(),
    signal0: Signal::new(MAX),
    signal1: Signal::new(MAX),

                    }
                }
                
            }
            
impl Patch for EdslPassThru {
    fn rust_process_audio(
        &mut self,
        input_slice: &[f32],
        output_slice: &mut [f32],
        _knobs: &Box<dyn Knobs>,
        mut playhead: Playhead,
    ) {
        for i in 0..input_slice.len() {
            self.signal1.write(input_slice[i]);


            let port0_0: Window<f32> = Window::new(&self.signal1, Range(0, 0));
self.unitPassThru_0.go(playhead, &port0_0, &mut self.signal0);


            output_slice[i] = self.signal0.read(0);

            playhead.inc();
            
        }
    }

    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}

