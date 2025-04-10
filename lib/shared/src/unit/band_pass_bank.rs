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

#[derive(Debug, PartialEq)]
pub enum MatchResult {
    DropOld(usize),
    AddNew(usize),
    Match(usize, usize),
}

// Two frequencies aren't considered by closest() unless they're closer than this.
const MAX_CLOSE: f32 = 120.0;
fn closest(x: f32, xs: &Vec<f32>) -> Option<usize> {
    assert!(!xs.is_empty());
    let mut dists: Vec<(usize, f32)> = xs.iter().enumerate()
        .map(|(i, xx)| (i, libm::fabsf(x-xx)))
        .collect();
    dists.sort_by(|(_, x), (_, xx)| x.partial_cmp(xx).unwrap());
    dists.retain(|(_,x)| *x < MAX_CLOSE);
    if dists.is_empty() { None } else { Some(dists[0].0) }
}

pub fn match_values(old: &Vec<f32>, nu: &Vec<f32>) -> Vec<MatchResult> {
    let slow_results = match_values_slow(old, nu);
    let fast_results = match_values_fast(old, nu);
    println!("slow results {:?}", slow_results);
    println!("fast results {:?}", fast_results);
    //assert!(slow_results == fast_results);
    fast_results
}

fn closests(xs: &Vec<f32>, ys: &Vec<f32>) -> Vec<Option<usize>> {
    let mut faves: Vec<Option<usize>> = vec![None; xs.len()];

    let mut xi: usize = 0;
    let mut yi: usize = 0;

    println!("CLOSESTS");
    println!("xs {} {:?}", xs.len(), xs);
    println!("ys {} {:?}", ys.len(), ys);

    // In the middle of this, either the current x is between the current y and its successor, in
    // which case we pick whichever is closer (and within threshold). Then advance x.
    //
    // At this point, y and its successor are both below x, so we advance until again x is between
    // y and its successor.
    //
    // The edge case at the end: y has no successor, and is less than x, pick it.
    //
    // The edge case at the beginning: xi == yi == 0, so pick yi, and advance x.
    while xi < xs.len() && yi < ys.len() {
        println!("loop {} {}", xi, yi);
        println!("vals {} {}", xs[xi], ys[yi]);
        if yi + 1 < ys.len() && ys[yi] < xs[xi] && xs[xi] <= ys[yi+1] {
            // Current x is between two ys, pick closest.
            //
            //   y
            // x
            //   y
            let prev_dist = xs[xi] - ys[yi];
            let next_dist = ys[yi+1] - xs[xi];

            assert!(prev_dist >= 0.0);
            assert!(next_dist >= 0.0);

            if prev_dist < next_dist {
                // Pick previous
                if prev_dist < MAX_CLOSE {
                    faves[xi] = Some(yi);
                }
            } else {
                // Pick next
                if prev_dist < MAX_CLOSE {
                    faves[xi] = Some(yi+1);
                }
            }

            // Next x
            xi += 1;
        } else if yi + 1 < ys.len() && ys[yi] < xs[xi] && ys[yi+1] < xs[xi] {
            // Both ys are less, next y
            //
            // x
            //   y
            //   y
            yi += 1;
        } else if yi == ys.len() - 1 {
            // Last y, pick it
            //
            // x
            //   y
            assert!(ys[yi] < xs[xi]);
            let dist = xs[xi] - ys[yi];
            if dist < MAX_CLOSE {
                faves[xi] = Some(yi);
            }
            xi += 1;
        } else {
            // Only remaining case, pick y
            // xi == 0
            // y is greater
            assert!(yi == 0);
            assert!(ys[yi] > xs[xi]);
            let dist = ys[yi] - xs[xi];
            if dist < MAX_CLOSE {
                faves[xi] = Some(yi);
            }
            xi += 1;
        }
    }

    faves
}

fn match_values_fast(old: &Vec<f32>, nu: &Vec<f32>) -> Vec<MatchResult> {
    println!("closest to olds");
    let old_faves: Vec<Option<usize>> = closests(old, nu);
    println!("closest to news");
    let nu_faves: Vec<Option<usize>> = closests(nu, old);

    assert!(old_faves.len() == old.len());
    assert!(nu_faves.len() == nu.len());

    println!("old_faves {:?}", old_faves);
    println!(" nu_faves {:?}", nu_faves);

    let mut results: Vec<MatchResult> = Vec::new();

    // For each pair of values (old and new) that agree, add a match. For any value
    // that doesn't agree with its fave nu, add a drop.
    for i in 0..old.len() {
        // TODO don't use unwrap?
        if old_faves[i].is_some()
            && nu_faves[old_faves[i].unwrap()].is_some()
            && nu_faves[old_faves[i].unwrap()].unwrap() == i {
            results.push(MatchResult::Match(i, old_faves[i].unwrap()));
        } else {
            results.push(MatchResult::DropOld(i));
        }
    }

    // Same for nu -> old, but no need to add the matches again
    for i in 0..nu.len() {
        // TODO don't repeat this, don't do this at all maybe.
        if nu_faves[i].is_some()
            && old_faves[nu_faves[i].unwrap()].is_some()
            && old_faves[nu_faves[i].unwrap()].unwrap() == i {
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

fn match_values_slow(old: &Vec<f32>, nu: &Vec<f32>) -> Vec<MatchResult> {
    let mut old_faves: Vec<Option<usize>> = vec![None; old.len()];
    let mut nu_faves: Vec<Option<usize>> = vec![None; nu.len()];

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
        // TODO don't use unwrap?
        if old_faves[i].is_some()
            && nu_faves[old_faves[i].unwrap()].is_some()
            && nu_faves[old_faves[i].unwrap()].unwrap() == i {
            results.push(MatchResult::Match(i, old_faves[i].unwrap()));
        } else {
            results.push(MatchResult::DropOld(i));
        }
    }

    // Same for nu -> old, but no need to add the matches again
    for i in 0..nu.len() {
        // TODO don't repeat this, don't do this at all maybe.
        if nu_faves[i].is_some()
            && old_faves[nu_faves[i].unwrap()].is_some()
            && old_faves[nu_faves[i].unwrap()].unwrap() == i {
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
        let old_freqs: Vec<f32> = self.bps.iter().map(|(bp, _, _)| bp.get_freq()).collect();

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
