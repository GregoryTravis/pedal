use crate::patch::Patch;

use alloc::boxed::Box;
use crate::playhead::*;

pub struct Rig {
    pub patch: Box<dyn Patch>,
    pub inl: f32,
    pub inr: f32,
    pub outl: f32,
    pub outr: f32,
    pub framesize: usize,
    pub playhead: Playhead,
}

