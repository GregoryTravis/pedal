use alloc::boxed::Box;
use core::marker::Send;
use core::any::Any;

use crate::knob::Knobs;
use crate::playhead::*;

pub trait Patch: Send {
    fn rust_process_audio(
        &mut self,
        input_slice: &[f32],
        output_slice: &mut [f32],
        knobs: &Box<dyn Knobs>,
        playhead: Playhead,
    );

    fn done(&self) -> bool { panic!("Should not reach here"); }
    fn passed(&self) -> bool { panic!("Should not reach here"); }

    fn into_any(self: Box<Self>) -> Box<dyn Any>;
}
