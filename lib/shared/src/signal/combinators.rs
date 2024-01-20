extern crate alloc;

use alloc::boxed::Box;

use crate::signal::Signal;

pub struct ScaleTime {
    pub signal: Box<dyn Signal<f32>>,
    pub s: f32,
}

impl Signal<f32> for ScaleTime {
    fn f(&self, t: f32) -> f32 {
        self.signal.f(t * self.s)
    }
}

pub struct PostCompose<T>
  where T: Send
{
    pub signal: Box<dyn Signal<T>>,
    pub ff: Box<dyn Fn(T) -> T + Send + 'static>,
}

impl<T> Signal<T> for PostCompose<T>
  where T: Send
{
    fn f(&self, t: f32) -> T {
        (self.ff)(self.signal.f(t))
    }
}

// Scale -1..1 to a..b
pub fn scale_range(a: f32, b: f32) -> Box<dyn Fn(f32) -> f32 + Send + 'static> {
    Box::new(move |x| a + ((b - a) * ((x + 1.0) / 2.0)))
}
