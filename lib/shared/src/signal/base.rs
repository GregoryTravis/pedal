extern crate libm;

use crate::signal::Signal;

pub struct Sin {}

impl Signal<f32> for Sin {
    fn f(&self, t: f32) -> f32 {
        libm::sinf(t)
    }
}

pub struct Const<T> {
    pub x: T,
}

impl<T> Signal<T> for Const<T>
  where T: Copy + Clone + Send + Sync
{
    fn f(&self, _t: f32) -> T {
        self.x
    }
}
