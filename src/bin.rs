#[cfg(feature = "std")]
extern crate std;

use crate::dsp::*;
#[path = "../dsp/src/lib.rs"] mod dsp;

#[cfg(feature = "stdd")]
use crate::dsp::sim::*;

pub fn main() {
  sim_main();
}
