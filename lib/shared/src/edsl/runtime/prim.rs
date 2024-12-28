// TODO remove for board
extern crate std;
use core::default::Default;
use core::ops::{Add, AddAssign};
use std::println;

use crate::edsl::runtime::{signal::Signal, window::Window};

pub fn pass_thru<T: Default + Copy>(inn: &Window<T>, out: &mut Signal<T>) {
    out.write(inn.read(0));
}

pub fn add<T: Add<Output = T> + Default + Copy + core::fmt::Display>(a: &Window<T>, b: &Window<T>, sum: &mut Signal<T>) {
    println!("add: {} {} {}", a.read(0), b.read(0), a.read(0) + b.read(0));
    sum.write(a.read(0) + b.read(0));
}

// Dum filter that sums the entire input range.
pub fn sum_filter<T: Add<Output = T> + AddAssign + Default + Copy>(inn: &Window<T>, out: &mut Signal<T>) {
    let mut sum: T = Default::default();
    for i in inn.range().0..=inn.range().1 {
        sum += inn.read(i);
    }
    out.write(sum);
}
