#![allow(non_snake_case)]

use core::mem::size_of;

use shared::constants::*;

extern "C" {
    pub fn cpp_hw_get_size_t_size() -> usize;
    pub fn cpp_hw_init(b: bool, block_size: usize);
    pub fn cpp_hw_kshep_init();
}

pub fn hw_get_size_t_size() -> usize {
    unsafe {
        cpp_hw_get_size_t_size()
    }
}

fn hw_sanity_check() {
    // The irony here is not lost on me.
    let size_t_size = hw_get_size_t_size();
    assert!(size_t_size == size_of::<usize>());
}

pub fn hw_init(b: bool, block_size: usize) {
    hw_sanity_check();

    unsafe {
        cpp_hw_init(b, block_size);
    }

    if KSHEP {
        unsafe {
            cpp_hw_kshep_init();
        }
    }
}
