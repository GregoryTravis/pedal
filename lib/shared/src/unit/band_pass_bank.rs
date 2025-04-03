#[cfg(feature = "for_host")]
extern crate std;
extern crate libm;

use alloc::vec::Vec;
#[allow(unused)]
use std::println;

use crate::constants::*;
use crate::inertial::*;
#[allow(unused)]
use crate::spew::*;
use crate::unit::band_pass::*;

const BW: f32 = 0.01;
//const AMP_DM: f32 = 10000000.0;
const AMP_DM: f32 = 16.0 / SAMPLE_RATE as f32;

const GAIN: f32 = 15.0;
const ALL_ONES: bool = true;

#[derive(Debug)]
pub enum MatchResult {
    DropOld(usize),
    AddNew(usize),
    Match(usize, usize),
}

// Two frequencies aren't considered by closest() unless they're closer than this.
//const MAX_CLOSE: f32 = 50.0;
pub fn closest(x: f32, xs: &Vec<f32>) -> usize {
    assert!(!xs.is_empty());
    let mut dists: Vec<(usize, f32)> = xs.iter().enumerate()
        .map(|(i, xx)| (i, libm::fabsf(x-xx)))
        .collect();
    dists.sort_by(|(_, x), (_, xx)| x.partial_cmp(xx).unwrap());
    //dists.retain(|(_,x)| *x < MAX_CLOSE);
    dists[0].0
}

pub fn match_values(old: &Vec<f32>, nu: &Vec<f32>) -> Vec<MatchResult> {
    let mut old_faves: Vec<usize> = vec![0; old.len()];
    let mut nu_faves: Vec<usize> = vec![0; nu.len()];

    let mut results: Vec<MatchResult> = Vec::new();

    // If old is empty, return all AddNew
    if old.len() == 0 {
        for i in 0..nu.len() {
            results.push(MatchResult::AddNew(i));
        }
        return results;
    }

    // If nu is empty, return all DropOld
    if nu.len() == 0 {
        for i in 0..old.len() {
            results.push(MatchResult::DropOld(i));
        }
        return results;
    }

    // For each value, find the one in the other array that is closest to it.
    for i in 0..old.len() {
        old_faves[i] = closest(old[i], nu);
    }
    for i in 0..nu.len() {
        nu_faves[i] = closest(nu[i], old);
    }
    println!("old_faves {:?}", old_faves);
    println!(" nu_faves {:?}", nu_faves);

    // For each pair of values (old and new) that agree, add a match. For any value
    // that doesn't agree with its fave nu, add a drop.
    for i in 0..old.len() {
        if nu_faves[old_faves[i]] == i {
            results.push(MatchResult::Match(i, old_faves[i]));
        } else {
            results.push(MatchResult::DropOld(i));
        }
    }

    // Same for nu -> old, but no need to add the matches again
    for i in 0..nu.len() {
        if old_faves[nu_faves[i]] == i {
            // Do nothing
        } else {
            results.push(MatchResult::AddNew(i));
        }
    }

    // Check everything is accounted for exactly once.
    // TODO comment out / test only
    let check = true;
    if check {
        let mut old_used: Vec<bool> = vec![false; old.len()];
        let mut nu_used: Vec<bool> = vec![false; nu.len()];
        for mr in &results {
            match mr {
                MatchResult::DropOld(i) => {
                    assert!(!old_used[*i]);
                    old_used[*i] = true;
                }
                MatchResult::AddNew(i) => {
                    assert!(!nu_used[*i]);
                    nu_used[*i] = true;
                }
                MatchResult::Match(i, j) => {
                    assert!(!old_used[*i]);
                    assert!(!nu_used[*j]);
                    old_used[*i] = true;
                    nu_used[*j] = true;
                }
            }
        }
        assert!(old_used.iter().all(|&b| b));
        assert!(nu_used.iter().all(|&b| b));
    }

    results
}

pub struct BandPassBank {
    // (bp, amp)
    bps: Vec<(BandPass, Inertial, bool)>,
}

/*
#[allow(unused)]
fn filter_some(freqs: Vec<f32>) -> Vec<f32> {
    freqs.into_iter().filter(|f| {
        *f < 1700.0
    }).collect()
}
*/

fn filter_some_fas(freqs: Vec<(f32, f32)>) -> Vec<(f32, f32)> {
    freqs.into_iter().filter(|_f| {
        true
        /*
        let low = 0.0;
        let high = 800.0;
        (*f).0 >= low && (*f).0 <= high
        */
    }).collect()
}

// This disgustingly-named function returns 1.0, except that it ramps it up to a higher value over
// the course of a frequency range. So freqs (0..HIGH) go from 1.0 to 1+GAIN.
const RAMP_ONE_HIGH: f32 = 1200.0;
const RAMP_ONE_GAIN: f32 = 0;
fn ramp_one(freq: f32) -> f32 {
    1.0 + (RAMP_ONE_GAIN * (freq / RAMP_ONE_HIGH))
}

impl BandPassBank {
    pub fn new() -> BandPassBank {
        BandPassBank {
            bps: Vec::new(),
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
    pub fn update(&mut self, orig_fas: &Vec<(f32, f32)>) {
        //self.dump("INITIAL");
        let old_freqs: Vec<f32> = self.bps.iter().map(|(bp, _, _)| bp.get_freq()).collect();

        // good
        let fas: Vec<(f32, f32)> = filter_some_fas(orig_fas.to_vec());
        let new_freqs: Vec<f32> = fas.iter().map(|&fa| fa.0).collect();

        // bad
        //let new_freqs: Vec<f32> = filter_some(fas.iter().map(|&fa| fa.0).collect());

        //println!("NEWF {:?}", new_freqs);

        println!("OLD {:?}", old_freqs);
        println!("NEW {:?}", new_freqs);

        let results = match_values(&old_freqs, &new_freqs);

        println!("MR {:?}", results);
        for mr in &results {
            match mr {
                MatchResult::AddNew(i) => {
                    println!("Add {}", new_freqs[*i]);
                },
                MatchResult::DropOld(i) => {
                    println!("Drop {}", old_freqs[*i]);
                },
                MatchResult::Match(i, j) => {
                    println!("Match {} {}", old_freqs[*i], new_freqs[*j]);
                },
            }
        }

        for mr in &results {
            match mr {
                MatchResult::AddNew(i) => {
                    let amp = if ALL_ONES { ramp_one(fas[*i].0) } else { fas[*i].1 * GAIN };
                    self.bps.push((BandPass::new(fas[*i].0, BW), Inertial::new_from(0.0, amp, AMP_DM), false));
                },
                MatchResult::DropOld(i) => {
                    // Doing this in case we make it inertial and it doesn't drop out
                    // right away.
                    self.bps[*i].1.set(0.0);
                    self.bps[*i].2 = true;
                },
                MatchResult::Match(i, j) => {
                    let amp = if ALL_ONES { ramp_one(fas[*j].0) } else { fas[*j].1 * GAIN };
                    //println!("GGG set f {} {} {} {}", *i, self.bps[*i].0.get_freq(), *j, fas[*j].0);
                    self.bps[*i].0.set_freq(fas[*j].0);
                    self.bps[*i].1.set(amp);
                    self.bps[*i].2 = false;
                },
            }
        }

        // Remove the ones that have been dropped and then gone to 0.
        self.bps.retain(|(_, amp, dropping)| !*dropping || (*amp).get() > 0.0);

        // Sort the added ones in.
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
