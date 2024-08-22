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

pub struct Timer {
    start_time: u128,
}

impl Timer {
    pub fn new() -> Timer {
        Timer { start_time: hw_relative_time() }
    }

    pub fn elapsed(&self) -> f32 {
        let elapsed_ms = hw_relative_time() - self.start_time;
        (elapsed_ms as f32) / 1000.0
    }
}
