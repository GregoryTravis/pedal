use core::f32::consts::PI;

use shared::graphing::*;

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

    let ws: fn(f32) -> f32 = |s| {
        let lo = -3.825;
        let hi = 1.85;
        // -3.75..2
        let x = (s * (hi - lo)) + lo;
        ((x*x*x + 3.0*x*x - 3.0*x + 1.0) / 6.0) - 1.0
        /*
        let s_0_1 = (s + 1.0) / 2.0;
        let x = (s_0_1 * 5.5) - 3.75;
        let mut y = x*x*x + 3.0*x*x - 3.0*x + 1.0;
        y = (y / 6.0) - 1.0;
        y
        */
    };
    //graph_2d_fun("waveshaper.png", 1024, 768, -10.0..10.0, -20.0..20.0, ws).unwrap();
    graph_2d_fun("waveshaper.png", 1024, 768, -1.0..2.0, -1.0..3.0, ws).unwrap();
}
