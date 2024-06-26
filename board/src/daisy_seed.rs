#![allow(non_snake_case)]

use core::mem::size_of;

extern "C" {
    pub fn cpp_hw_get_size_t_size() -> usize;
    pub fn cpp_hw_Init();
    pub fn cpp_hw_StartLog(b: bool);
}

pub fn hw_get_size_t_size() -> usize {
    unsafe {
        cpp_hw_get_size_t_size()
    }
}
pub fn hw_sanity_check() {
    // The irony here is not lost on me.
    let size_t_size = hw_get_size_t_size();
    assert!(size_t_size == size_of::<usize>());
}

pub fn hw_Init() {
    unsafe {
        cpp_hw_Init();
    }
}

pub fn hw_StartLog(b: bool) {
    unsafe {
        cpp_hw_StartLog(b);
    }
}
