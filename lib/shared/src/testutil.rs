use crate::spew::*;

pub fn same(a: &[f32], b: &[f32]) -> bool {
    assert!(a.len() == b.len());
    for i in 0..a.len() {
        if a[i] != b[i] {
            spew!("not same", i, a[i], b[i]);
            return false;
        }
    }
    return true;
}

pub fn sum(a: &[f32]) -> f32 {
    let mut sum: f32 = 0.0;
    for i in 0..a.len() {
        sum += a[i];
    }
    sum
}
