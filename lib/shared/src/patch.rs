use alloc::boxed::Box;
use core::marker::Send;

use crate::knob::Knobs;
use crate::playhead::*;
use crate::switch::Switches;

pub trait Patch: Send {
    fn rust_process_audio(
        &mut self,
        input_slice: &[f32],
        output_slice: &mut [f32],
        knobs: &Box<dyn Knobs>,
        switches: &Box<dyn Switches>,
        playhead: Playhead,
    );

    fn done(&self) -> bool { panic!("Should not reach here"); }
    fn passed(&self) -> bool { panic!("Should not reach here"); }
}
