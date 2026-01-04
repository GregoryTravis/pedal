// TODO remove for board
//extern crate std;
use core::default::Default;
use core::ops::{Add, AddAssign};
//use std::println;

use crate::edsl::runtime::{signal::Signal, window::Window};

pub struct PassThru {}
impl PassThru {
    pub fn new() -> Self { Self {} }

    pub fn go<T: Default + Copy>(&mut self, inn: &Window<T>, out: &mut Signal<T>) {
        out.write(inn.read(0));
    }
}

pub struct AddPrim {}
impl AddPrim {
    pub fn new() -> Self { Self {} }

    pub fn go<T: Add<Output = T> + Default + Copy + core::fmt::Display>(&mut self, a: &Window<T>, b: &Window<T>, sum: &mut Signal<T>) {
        //println!("add: {} {} {}", a.read(0), b.read(0), a.read(0) + b.read(0));
        sum.write(a.read(0) + b.read(0));
    }
}

pub struct HighPass {}
impl HighPass {
    pub fn new() -> Self { Self {} }

    pub fn go(&mut self, inn: &Window<f32>, out: &mut Signal<f32>) {
        out.write(5.0 * ((inn.read(0) - inn.read(-1)) / 2.0));
    }
}

pub struct LowPass {}
impl LowPass {
    pub fn new() -> Self { Self {} }

    #[inline(always)]
    pub fn go(&mut self, inn: &Window<f32>, out: &mut Signal<f32>) {
        out.write(5.0 * ((inn.read(0) + inn.read(-1)) / 2.0));
    }
}

// Dum filter that sums the entire input range.
pub struct SumFilter {}
impl SumFilter {
    pub fn new() -> Self { Self {} }

    pub fn go<T: Add<Output = T> + AddAssign + Default + Copy>(&mut self, inn: &Window<T>, out: &mut Signal<T>) {
        let mut sum: T = Default::default();
        for i in inn.range().0..=inn.range().1 {
            sum += inn.read(i);
        }
        out.write(sum);
    }
}
