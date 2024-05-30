#[allow(unused_imports)]
use crate::glep;

#[allow(unused_imports)]
use crate::spew::*;

const DOT_SIZE: usize = 10;

#[allow(non_upper_case_globals)]
static mut a: [f32; DOT_SIZE] = [0.0; DOT_SIZE];
#[allow(non_upper_case_globals)]
static mut b: [f32; DOT_SIZE] = [0.0; DOT_SIZE];
#[allow(non_upper_case_globals)]
static mut accum: f32 = 0.0;

#[no_mangle]
pub fn rust_speed_test_init() {
    unsafe {
        for i in 0..DOT_SIZE {
            a[i] = i as f32;
            b[i] = i as f32;
        }
        accum = 0.0;
    }
}

#[no_mangle]
#[inline(never)]
pub fn rust_f32_dot() -> f32 {
    unsafe {
        let mut totes: f32 = 0.0;
        for i in 0..DOT_SIZE {
           totes += a[i] * b[i];
        }
        accum = totes;
        accum
    }
}
