use core::marker::Send;

use crate::playhead::*;

pub trait Patch: Send {
    fn rust_process_audio(
        &mut self,
        input_slice: &[f32],
        output_slice: &mut [f32],
        playhead: Playhead,
    );
}
