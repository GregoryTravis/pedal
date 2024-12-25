use core::ops::Add;

use crate::edsl::runtime::{signal::Signal, window::Window};

pub fn pass_thru<T: Copy>(inn: &Window<T>, out: &mut Signal<T>) {
    out.write(inn.read(0));
}

pub fn add<T: Add<Output = T> + Copy>(a: &Window<T>, b: &Window<T>, sum: &mut Signal<T>) {
    sum.write(a.read(0) + b.read(0));
}
