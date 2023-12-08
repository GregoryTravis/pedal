extern crate std;

use std::println;

#[cfg(not(feature = "stdd"))]
use hound;

#[cfg(feature = "stdd")]
pub fn sim_main() {
  let mut reader = hound::WavReader::open("hms0.wav").unwrap();
  let sqr_sum = reader.samples::<i16>()
                      .fold(0.0, |sqr_sum, s| {
      let sample = s.unwrap() as f64;
      sqr_sum + sample * sample
  });
  println!("RMS is {}", (sqr_sum / reader.len() as f64).sqrt());
}
