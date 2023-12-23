use host::graphing::*;

pub fn main() {
  let fun: fn(f32)-> f32 = |x| (x*10.0).sin();
  graph2dfun(1024, 768, -3.4f32..3.4, -1.2f32..1.2f32, fun).unwrap();
}

