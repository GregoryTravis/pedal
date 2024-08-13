extern crate alloc;

use std::env;
use alloc::boxed::Box;
use alloc::sync::Arc;

use shared::constants::*;
use shared::filter::chorus::*;
use shared::filter::delay::*;
use shared::filter::envelope_follower::*;
use shared::filter::linear_vibrato::*;
use shared::filter::low_pass::*;
use shared::filter::pass_thru::*;
use shared::filter::seq::*;
use shared::filter::sweep::*;
use shared::filter::vibrato::*;
use shared::filter::waveshaper::*;
use shared::signal::base::*;
use shared::signal::combinators::*;
use shared::sim::*;

#[allow(dead_code)]
fn sweep(input_file: &str, output_file: &str) {
    let siner = PostCompose { signal: Arc::new(Sin {}), ff: scale_range(0.3, 0.9) };
    let q = Const { x: 0.95 };
    sim_main(input_file, output_file, Box::new(SweepFilter::new(Arc::new(siner), Arc::new(q))));
}

#[allow(dead_code)]
fn delay(input_file: &str, output_file: &str) {
    sim_main(input_file, output_file, Box::new(Delay::new()));
}

#[allow(dead_code)]
fn pass_thru(input_file: &str, output_file: &str) {
    sim_main(input_file, output_file, Box::new(PassThruFilter {}));
}

#[allow(dead_code)]
fn vibrato(input_file: &str, output_file: &str) {
    sim_main(input_file, output_file, Box::new(Vibrato::new(400, 1.0)));
}

#[allow(dead_code)]
fn linear_vibrato(input_file: &str, output_file: &str) {
    sim_main(input_file, output_file, Box::new(LinearVibrato::new(400, 1.0, 0)));
}

#[allow(dead_code)]
fn chorus(input_file: &str, output_file: &str) {
    sim_main(input_file, output_file, Box::new(Chorus::new()));
}

#[allow(dead_code)]
fn waveshaper(input_file: &str, output_file: &str) {
    sim_main(input_file, output_file, Box::new(WaveShaper::new()));
}

#[allow(dead_code)]
fn envelope_follower(input_file: &str, output_file: &str) {
    let filter = Seq::new(BLOCK_SIZE, Box::new(LowPassFilter::new()), Box::new(Seq::new(BLOCK_SIZE, Box::new(LowPassFilter::new()), Box::new(LowPassFilter::new()))));
    let patch = Seq::new(BLOCK_SIZE, Box::new(EnvelopeFollower::new()), Box::new(filter));
    sim_main(input_file, output_file, Box::new(patch));
}

pub fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 3);
    let input_file = &args[1];
    let output_file = &args[2];
    envelope_follower(input_file, output_file);
}
