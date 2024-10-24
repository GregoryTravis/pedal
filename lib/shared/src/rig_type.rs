use crate::patch::Patch;

use alloc::boxed::Box;

use crate::knob::Knobs;
use crate::playhead::*;
use crate::switch::Switches;

pub struct Rig {
    pub patch: Box<dyn Patch>,
    pub knobs: Box<dyn Knobs>,
    pub switches: Box<dyn Switches>,
    pub inl: f32,
    pub inr: f32,
    pub outl: f32,
    pub outr: f32,
    pub framesize: usize,
    pub playhead: Playhead,
}

