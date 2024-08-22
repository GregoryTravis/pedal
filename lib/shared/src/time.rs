#[cfg(feature = "for_host")]
pub use crate::time_host::*;

#[cfg(not(feature = "for_host"))]
pub use crate::time_board::*;

pub fn time_call<F>(f: F) -> f32 where
    F: FnOnce() {
    let start = hw_relative_time();
    f();
    let end = hw_relative_time();
    ((end - start) as f32) / 1000.0
}
