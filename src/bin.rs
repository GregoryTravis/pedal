#[cfg(feature = "std")]
extern crate std;
extern crate dsp;

//use crate::dsp::*;
//#[path = "../dsp/src/lib.rs"] mod dsp;

use crate::dsp::sim::*;

pub fn main() {
  sim_main();
}
