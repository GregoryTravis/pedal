extern crate alloc;

use alloc::sync::Arc;
//use core::ops::Add;

use crate::signal::Signal;

pub struct ScaleTime {
    pub signal: Arc<dyn Signal<f32>>,
    pub s: f32,
}

impl Signal<f32> for ScaleTime {
    fn f(&self, t: f32) -> f32 {
        self.signal.f(t * self.s)
    }
}

/*
pub struct Adder {
    pub a: Box<dyn Signal<f32>>,
    pub b: Box<dyn Signal<f32>>,
}

impl Signal<f32> for Adder {
    fn f(&self, t: f32) -> f32 {
        self.a.f(t) + self.b.f(t)
    }
}

impl Add<Box<dyn Signal<f32>>> for Box<dyn Signal<f32>> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Box::new(Adder { a: self, b: other })
    }
}
*/

pub struct PostCompose<T>
  where T: Send
{
    pub signal: Arc<dyn Signal<T>>,
    pub ff: Arc<dyn Fn(T) -> T + Send + Sync + 'static>,
}

impl<T> Signal<T> for PostCompose<T>
  where T: Send + Sync
{
    fn f(&self, t: f32) -> T {
        (self.ff)(self.signal.f(t))
    }
}

// Scale -1..1 to a..b
pub fn scale_range(a: f32, b: f32) -> Arc<dyn Fn(f32) -> f32 + Send + Sync + 'static> {
    Arc::new(move |x| a + ((b - a) * ((x + 1.0) / 2.0)))
}
