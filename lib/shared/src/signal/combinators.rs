extern crate alloc;

use alloc::boxed::Box;

use crate::signal::Signal;

pub struct Scale {
    pub signal: Box<dyn Signal<f32>>,
    pub s: f32,
}

impl Signal<f32> for Scale {
    fn f(&self, t: f32) -> f32 {
        self.signal.f(t * self.s)
    }
}
