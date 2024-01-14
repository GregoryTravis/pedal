extern crate libm;

use crate::signal::Signal;

pub struct Sin {}

impl Signal<f32> for Sin {
    fn f(&self, t: f32) -> f32 {
        libm::sinf(t)
    }
}
