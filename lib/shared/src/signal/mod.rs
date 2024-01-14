pub mod base;
pub mod combinators;

pub trait Signal<T> {
    fn f(&self, t: f32) -> T;
}
