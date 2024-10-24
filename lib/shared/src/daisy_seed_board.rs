#![allow(non_snake_case)]

use core::mem::size_of;

use crate::constants::*;
use crate::knob_board::*;
use crate::switch_board::*;

extern "C" {
    pub fn cpp_hw_get_size_t_size() -> usize;
    pub fn cpp_hw_init(b: bool, block_size: usize);
    pub fn cpp_hw_kshep_init();
    pub fn cpp_hw_set_led(index: u32, brightness: f32);
    pub fn cpp_hw_delay(delay_ms: u32);
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

pub fn hw_set_led(index: u32, brightness: f32) {
    if KSHEP {
        unsafe {
            cpp_hw_set_led(index, brightness);
        }
    }
}

pub fn hw_init(b: bool, block_size: usize) {
    hw_sanity_check();

    unsafe {
        cpp_hw_init(b, block_size);
    }

    if KSHEP {
        knob_init();
        switch_init();

        unsafe {
            cpp_hw_kshep_init();
        }

        hw_set_led(0, 1.0);
        hw_set_led(1, 1.0);
    }
}

pub fn hw_delay(delay_ms: u32) {
    unsafe {
        cpp_hw_delay(delay_ms);
    }
}

