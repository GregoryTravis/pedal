//use alloc::boxed::Box;
use core::any::Any;
use core::marker::Send;

use crate::playhead::*;

pub trait Patch: Send {
    fn rust_process_audio(
        &mut self,
        input_slice: &[f32],
        output_slice: &mut [f32],
        playhead: Playhead,
    );

    //fn as_any<'a>(&self) -> &(dyn Any + 'a);
    fn as_any<'a>(&'a self) -> &'a (dyn Any + '_);
    //fn as_any(&self) -> &dyn Any;
}
