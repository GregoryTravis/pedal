const DOT_SIZE: usize = 18;

#[no_mangle]
pub fn rust_f32_dot() {
    let a: [f32; DOT_SIZE] = [0.0; DOT_SIZE];
    let b: [f32; DOT_SIZE] = [0.0; DOT_SIZE];
    let mut c: f32 = 0.0;
    for i in 0..DOT_SIZE {
       c += a[i] * b[i];
    }
    let mut _use = c;
}
