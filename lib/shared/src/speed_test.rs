// use core::cell::OnceCell;

use crate::ds::circbuf::CircBuf;

#[allow(unused_imports)]
use crate::spew::*;

const DOT_SIZE: usize = 10;
#[allow(unused)]
const CIRC_NEW_ADD: usize = 3;
#[allow(unused)]
const CIRC_SUM_SIZE: usize = 7;

#[allow(non_upper_case_globals)]
static mut a: [f32; DOT_SIZE] = [0.0; DOT_SIZE];
#[allow(non_upper_case_globals)]
static mut b: [f32; DOT_SIZE] = [0.0; DOT_SIZE];
#[allow(non_upper_case_globals)]
static mut accum: f32 = 0.0;
#[allow(non_upper_case_globals)]
static mut cbuf: CircBuf::<f32> = CircBuf::<f32>::new_empty();

#[no_mangle]
pub fn rust_speed_test_init() {
    unsafe {
        for i in 0..DOT_SIZE {
            a[i] = i as f32;
            b[i] = i as f32;
        }
        accum = 0.0;
        cbuf = CircBuf::<f32>::new(DOT_SIZE, 0.0);
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

/*
#[no_mangle]
#[inline(never)]
pub fn rust_f32_circsum() -> f32 {
    unsafe {
        for i in 0..CIRC_NEW_ADD {
            cbuf.push(a[i]);
        }
        let sum_offset: usize = (DOT_SIZE - CIRC_SUM_SIZE) / 2;
        let sum_end: usize = sum_offset + CIRC_SUM_SIZE;
        //assert!(sum_offset <= DOT_SIZE);
        //assert!(sum_end <= DOT_SIZE);
        let mut totes: f32 = 0.0;
        for i in sum_offset..sum_end {
          totes += cbuf.get(i);
        }
        accum = totes;
        return accum;
    }
}
*/
