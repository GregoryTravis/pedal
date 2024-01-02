use std::f32::consts::PI;

use host::graphing::*;

pub fn main() {
    let fun: fn(f32) -> f32 = |x| (x * 10.0).sin();
    graph_2d_fun("out2d.png", 1024, 768, -3.4f32..3.4, -1.2f32..1.2f32, fun).unwrap();
    let canonical_complex_sinusoid = |t: f32| {
        let tt = t * 2.0 * PI;
        (tt.cos(), tt.sin(), tt)
    };
    graph_3d_line_fun(
        "out3d.svg",
        1024,
        768,
        -7.0..7.0,
        -7.0..7.0,
        -7.0..7.0,
        canonical_complex_sinusoid,
        500,
    )
    .unwrap();
}
