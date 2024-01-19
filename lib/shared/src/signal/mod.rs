pub mod base;
pub mod combinators;

pub trait Signal<T>: Send {
    fn f(&self, t: f32) -> T;
}
