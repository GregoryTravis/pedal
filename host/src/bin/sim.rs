//extern crate host;
//extern crate shared;
//extern crate pedalboard

use host::sim::*;
use pedalhost::*;
use shared::filter::reso::*;

pub fn main() {
  // TODO remove box?
  let box_patch = Box::new(ResoPatch {
      left: ResoFilter::new(),
      right: ResoFilter::new(),
  });
  sim_main(box_patch);
}
