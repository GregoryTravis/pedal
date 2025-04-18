#[cfg(feature = "for_host")]
extern crate std;
extern crate libm;

use alloc::vec::Vec;
#[allow(unused)]
use std::println;

use crate::constants::*;
use crate::inertial::*;
use crate::frequency_matcher::*;
#[allow(unused)]
use crate::spew::*;
use crate::unit::band_pass::*;

const BW: f32 = 0.01;
//const AMP_DM: f32 = 10000000.0;
const AMP_DM: f32 = 16.0 / SAMPLE_RATE as f32;

const VERBOSE: bool = false;

pub struct BandPassBank {
    // (bp, amp)
    bps: Vec<(BandPass, Inertial, bool)>,

    old_freqs: Vec<f32>,
}

// This disgustingly-named function returns 1.0, except that it ramps it up to a higher value over
// the course of a frequency range. So freqs (0..HIGH) go from 1.0 to 1+GAIN.
const RAMP_ONE_HIGH: f32 = 1200.0;
const RAMP_ONE_GAIN: f32 = 0.0;
fn ramp_one(freq: f32) -> f32 {
    1.0 + (RAMP_ONE_GAIN * (freq / RAMP_ONE_HIGH))
}

impl BandPassBank {
    pub fn new() -> BandPassBank {
        BandPassBank {
            bps: Vec::new(),

            old_freqs: Vec::new(),
        }
    }

    pub fn process(&mut self, input: f32) -> f32 {
        let mut output: f32 = 0.0;
        //let final_fas: Vec<(f32, f32)> = self.bps.iter().map(|(bp, a)| (bp.get_freq(), *a)).collect();
        //println!("PROCESS {:?}", final_fas);
        for (ref mut bp, ref mut amp, _) in &mut self.bps {
            amp.update();
            output += amp.get() * bp.process(input);
        }
        output
    }

    // (freq, amp)
    pub fn update(&mut self, new_freqs: &Vec<f32>) {
        //self.dump("INITIAL");
        self.old_freqs.clear();
        self.old_freqs.extend(self.bps.iter().map(|(bp, _, _)| bp.get_freq()));

        //println!("NEWF {:?}", new_freqs);

        if VERBOSE { println!("OLD {:?}", self.old_freqs); }
        if VERBOSE { println!("NEW {:?}", new_freqs); }

        let results = match_values(&self.old_freqs, &new_freqs);

        if VERBOSE {
            println!("MR {:?}", results);
            for mr in &results {
                match mr {
                    MatchResult::AddNew(i) => {
                        println!("Add {}", new_freqs[*i]);
                    },
                    MatchResult::DropOld(i) => {
                        println!("Drop {}", self.old_freqs[*i]);
                    },
                    MatchResult::Match(i, j) => {
                        println!("Match {} {}", self.old_freqs[*i], new_freqs[*j]);
                    },
                }
            }
        }

        for mr in &results {
            match mr {
                MatchResult::AddNew(i) => {
                    let amp = ramp_one(new_freqs[*i]);
                    self.bps.push((BandPass::new(new_freqs[*i], BW), Inertial::new_from(0.0, amp, AMP_DM), false));
                },
                MatchResult::DropOld(i) => {
                    // Doing this in case we make it inertial and it doesn't drop out
                    // right away.
                    self.bps[*i].1.set(0.0);
                    self.bps[*i].2 = true;
                },
                MatchResult::Match(i, j) => {
                    let amp = ramp_one(new_freqs[*j]);
                    //println!("GGG set f {} {} {} {}", *i, self.bps[*i].0.get_freq(), *j, fas[*j].0);
                    self.bps[*i].0.set_freq(new_freqs[*j]);
                    self.bps[*i].1.set(amp);
                    self.bps[*i].2 = false;
                },
            }
        }

        // Remove the ones that have been dropped and then gone to 0.
        self.bps.retain(|(_, amp, dropping)| !*dropping || (*amp).get() > 0.0);

        // Sort the added ones in.
        // TODO remove this by generating them in order.
        self.bps.sort_by(|(bp0, _, _), (bp1, _, _)| (bp0.get_freq()).partial_cmp(&bp1.get_freq()).unwrap());

        //let final_fas: Vec<(f32, f32)> = self.bps.iter().map(|(bp, a, _)| (bp.get_freq(), (*a).get())).collect();
        //println!("FINAL {:?}", final_fas);
        //self.dump("FINAL");
    }

    pub fn dump(&self, tag: &str) {
        let final_fas: Vec<(f32, f32)> = self.bps.iter().map(|(bp, a, _)| (bp.get_freq(), (*a).get())).collect();
        println!("{} {:?}", tag, final_fas);
    }
}
