use host::graphing::*;

pub fn main() {
  let fun: fn(f32)-> f32 = |x| (x*10.0).sin();
  graph_2d_fun("out2d.png", 1024, 768, -3.4f32..3.4, -1.2f32..1.2f32, fun).unwrap();
  graph_3d_line_fun("out3d.svg", 1024, 768, -3.0..3.0, -3.0..3.0, -3.0..3.0,
                    |t| (t*2.0, (t*100.0).sin(), (t*100.0).cos()), 500).unwrap();
}

