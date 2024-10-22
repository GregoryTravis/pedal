extern crate alloc;

use std::env;
use alloc::boxed::Box;
use alloc::sync::Arc;

use shared::constants::*;
use shared::filter::chorus::*;
use shared::filter::delay::*;
use shared::filter::envelope_follower::*;
use shared::filter::fuzz::*;
use shared::filter::harmoneer::*;
use shared::filter::linear_vibrato::*;
use shared::filter::low_pass::*;
use shared::filter::mixer::*;
use shared::filter::pass_thru::*;
use shared::filter::reso::*;
use shared::filter::seq::*;
use shared::filter::sweep::*;
use shared::filter::vibrato::*;
use shared::filter::waveshaper::*;
use shared::sdram::*;
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
fn fuzz(input_file: &str, output_file: &str) {
    let patch = Seq::new(BLOCK_SIZE,
                         Box::new(Fuzz::new()),
                         Box::new(ResoFilter::new(0, 0)),
                         );
    sim_main(input_file, output_file, Box::new(patch));
}

#[allow(dead_code)]
fn envelope_follower(input_file: &str, output_file: &str) {
    let filter = Seq::new(BLOCK_SIZE, Box::new(LowPassFilter::new()), Box::new(Seq::new(BLOCK_SIZE, Box::new(LowPassFilter::new()), Box::new(LowPassFilter::new()))));
    let patch = Seq::new(BLOCK_SIZE, Box::new(EnvelopeFollower::new()), Box::new(filter));
    sim_main(input_file, output_file, Box::new(patch));
}

// min 3rd: 1.189
// maj 3rd 1.26
// 4th 1.335
// 5th 1.498
// 6th 1.682
#[allow(dead_code)]
fn harmoneer(sdram: &mut SDRAM, input_file: &str, output_file: &str) {
    #[allow(unused)]
    let orig = PassThruFilter {};
    #[allow(unused)]
    let h0 = Harmoneer::new(2.0, sdram);
    //let fh0 = Seq::new(BLOCK_SIZE, Box::new(h0), Box::new(LowPassFilter::new()) );
    #[allow(unused)]
    let h1 = Harmoneer::new(0.5, sdram);
    let fh1 = Seq::new(BLOCK_SIZE, Box::new(h1), Box::new(LowPassFilter::new()) );
    let channels = vec![
        MixerChannel(1.0, Box::new(orig)),
        MixerChannel(0.7, Box::new(h0)),
        MixerChannel(1.0, Box::new(fh1)),
    ];
    let mixer = Mixer::new(channels);
    sim_main(input_file, output_file, Box::new(mixer));
}

pub fn main() {
    let mut sdram = SDRAM::new();

    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 3);
    let input_file = &args[1];
    let output_file = &args[2];
    //harmoneer(&mut sdram, input_file, output_file);
    let rubin = shared::rubin::rubin(&mut sdram);
    sim_main(input_file, output_file, rubin);
}
