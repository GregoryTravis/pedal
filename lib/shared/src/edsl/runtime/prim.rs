// TODO remove for board
extern crate libm;

use core::default::Default;
use core::f32::consts::PI;
use core::ops::{Add, AddAssign};
use ordered_float::OrderedFloat;
//use std::println;

use crate::filter::sine_table::*;
use crate::playhead::Playhead;
use crate::spew::*;

use crate::edsl::runtime::{signal::Signal, window::Window};

pub struct Const {
    k: OrderedFloat<f32>
}
impl Const {
    pub fn new(k: f32) -> Self { Self { k: OrderedFloat(k) } }

    pub fn go(&mut self, _playhead: Playhead, out: &mut Signal<f32>) {
        out.write(self.k.0);
    }
}

pub struct PassThru {}
impl PassThru {
    pub fn new() -> Self { Self {} }

    pub fn go<T: Default + Copy>(&mut self, _playhead: Playhead, inn: &Window<T>, out: &mut Signal<T>) {
        out.write(inn.read(0));
    }
}

pub struct AddPrim {}
impl AddPrim {
    pub fn new() -> Self { Self {} }

    pub fn go<T: Add<Output = T> + Default + Copy + core::fmt::Display>(&mut self, _playhead: Playhead, a: &Window<T>, b: &Window<T>, sum: &mut Signal<T>) {
        //println!("add: {} {} {}", a.read(0), b.read(0), a.read(0) + b.read(0));
        sum.write(a.read(0) + b.read(0));
    }
}

pub struct HighPass {}
impl HighPass {
    pub fn new() -> Self { Self {} }

    pub fn go(&mut self, _playhead: Playhead, inn: &Window<f32>, out: &mut Signal<f32>) {
        out.write(5.0 * ((inn.read(0) - inn.read(-1)) / 2.0));
    }
}

pub struct LowPass {}
impl LowPass {
    pub fn new() -> Self { Self {} }

    #[inline(always)]
    pub fn go(&mut self, _playhead: Playhead, inn: &Window<f32>, out: &mut Signal<f32>) {
        out.write(5.0 * ((inn.read(0) + inn.read(-1)) / 2.0));
    }
}

// Dum filter that sums the entire input range.
pub struct SumFilter {}
impl SumFilter {
    pub fn new() -> Self { Self {} }

    pub fn go<T: Add<Output = T> + AddAssign + Default + Copy>(&mut self, _playhead: Playhead, inn: &Window<T>, out: &mut Signal<T>) {
        let mut sum: T = Default::default();
        for i in inn.range().0..=inn.range().1 {
            //spew!("sum", i, inn.range().0, inn.range().1);
            sum += inn.read(i);
        }
        out.write(sum);
    }
}

pub struct LinearVibrato { max_sample_deviation: usize, now_index: usize }
impl LinearVibrato {
    pub fn new(max_sample_deviation: usize, now_index: usize) -> Self {
        Self { max_sample_deviation, now_index }
    }

    pub fn go(&mut self, playhead: Playhead, vibrato_frequency: &Window<f32>, inn: &Window<f32>, out: &mut Signal<f32>) {
        let tis = playhead.time_in_seconds();
        let knob_value = 1.0; // knobs.read(self.deviation_knob_id)
        let deviation = knob_value * (self.max_sample_deviation as f32);
        //let vibrato_deviation = libm::sinf(
        let vibrato_deviation = table_sin(
            tis * vibrato_frequency.read(0) as f32 * 2.0 * PI as f32) * deviation;
        spew!("LV", playhead.time_in_samples(), vibrato_deviation);
        // Fractional playhead
        let fph = (self.now_index as f32) + vibrato_deviation as f32;
        let fph_floor = libm::floorf(fph) as usize;
        let fph_ceiling = fph_floor + 1;
        let alpha = fph - (fph_floor as f32);
        let low_sample = inn.read(-(fph_floor as isize));
        let high_sample = inn.read(-(fph_ceiling as isize));
        let interped = (low_sample * (1.0 - alpha)) + (high_sample * alpha);
        out.write(interped);
    }
}
