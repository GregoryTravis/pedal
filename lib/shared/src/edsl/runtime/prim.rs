// TODO remove for board
//extern crate std;
use core::default::Default;
use core::ops::{Add, AddAssign};
//use std::println;

use crate::edsl::runtime::{signal::Signal, window::Window};

pub fn pass_thru<T: Default + Copy>(inn: &Window<T>, out: &mut Signal<T>) {
    out.write(inn.read(0));
}

pub fn add<T: Add<Output = T> + Default + Copy + core::fmt::Display>(a: &Window<T>, b: &Window<T>, sum: &mut Signal<T>) {
    //println!("add: {} {} {}", a.read(0), b.read(0), a.read(0) + b.read(0));
    sum.write(a.read(0) + b.read(0));
}

/*
pub fn multiply<T: Mul<Output = T> + Default + Copy + core::fmt::Display>(a: &Window<T>, b: &Window<T>, prod: &mut Signal<T>) {
    //println!("add: {} {} {}", a.read(0), b.read(0), a.read(0) + b.read(0));
    prod.write(a.read(0) * b.read(0));
}

pub fn divide<T: Div<Output = T> + Default + Copy + core::fmt::Display>(a: &Window<T>, b: &Window<T>, quot: &mut Signal<T>) {
    //println!("add: {} {} {}", a.read(0), b.read(0), a.read(0) + b.read(0));
    quot.write(a.read(0) / b.read(0));
}

pub fn const<T: Default + Copy + core::fmt::Display>(t: T, out: &mut Signal<T>) {
    //println!("add: {} {} {}", a.read(0), b.read(0), a.read(0) + b.read(0));
    out.write(t);
}
*/

pub fn high_pass(inn: &Window<f32>, out: &mut Signal<f32>) {
    out.write(5.0 * ((inn.read(0) - inn.read(-1)) / 2.0));
}

pub fn low_pass(inn: &Window<f32>, out: &mut Signal<f32>) {
    out.write(5.0 * ((inn.read(0) + inn.read(-1)) / 2.0));
}

// Dum filter that sums the entire input range.
pub fn sum_filter<T: Add<Output = T> + AddAssign + Default + Copy>(inn: &Window<T>, out: &mut Signal<T>) {
    let mut sum: T = Default::default();
    for i in inn.range().0..=inn.range().1 {
        sum += inn.read(i);
    }
    out.write(sum);
}
