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
    // We only remove the too-far ones in case they are all too far.
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

struct MatchIterator<'a> {
    old: &'a Vec<f32>,
    nu: &'a Vec<f32>,
    oi: usize,
    ni: usize,
    // Last returned value
    current: MI,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum NO {
    Old,
    Nu,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum MI {
    Start,
    First((NO, usize), (NO, usize)),
    Middle((NO, usize), (NO, usize), (NO, usize)),
    Last((NO, usize), (NO, usize)),
    Done,
}

use MI::*;
use NO::*;

impl <'a> MatchIterator<'a> {
    pub fn new(old: &'a Vec<f32>, nu: &'a Vec<f32>) -> MatchIterator<'a> {
        MatchIterator {
            old: old,
            nu: nu,
            oi: 0,
            ni: 0,
            current: Start,
        }
    }

    fn next_interleaved(&mut self) -> Option<(NO, usize)> {
        let odone = self.oi >= self.old.len();
        let ndone = self.ni >= self.nu.len();
        match (odone, ndone) {
            (true, true) => None,
            (true, false) => {
                let ret = Some((Nu, self.ni));
                self.ni += 1;
                ret
            },
            (false, true) => {
                let ret = Some((Old, self.oi));
                self.oi += 1;
                ret
            },
            (false, false) => {
                let of = self.old[self.oi];
                let nf = self.nu[self.ni];
                if of < nf {
                    let ret = Some((Old, self.oi));
                    self.oi += 1;
                    ret
                } else {
                    let ret = Some((Nu, self.ni));
                    self.ni += 1;
                    ret
                }
            }
        }
    }
}

impl <'a> Iterator for MatchIterator<'a> {
    type Item = MI;

    fn next(&mut self) -> Option<Self::Item> {
        let next = match self.current {
            Start => {
                let mi0 = self.next_interleaved();
                let mi1 = self.next_interleaved();
                match (mi0, mi1) {
                    (Some(mi0), Some(mi1)) => First(mi0, mi1),
                    _ => Done,
                }
            },
            First(mi0, mi1) => {
                let mi2 = self.next_interleaved();
                match mi2 {
                    Some(mi2) => Middle(mi0, mi1, mi2),
                    None => Last(mi0, mi1),
                }
            },
            Middle(_mi0, mi1, mi2) => {
                let mi3 = self.next_interleaved();
                match mi3 {
                    Some(mi3) => Middle(mi1, mi2, mi3),
                    None => Last(mi1, mi2),
                }
            }
            Last(_mi0, _mi1) => {
                assert!(self.next_interleaved().is_none());
                Done
            },
            Done => Done,
        };
        self.current = next;
        if self.current == Done { None } else { Some(next) }
    }
}

// TODO rename or remove
fn getem(old: &Vec<f32>, nu: &Vec<f32>, mi: (NO, usize)) -> f32 {
    if mi.0 == Old { old[mi.1] } else { nu[mi.1] }
}

fn match_values_fast(old: &Vec<f32>, nu: &Vec<f32>) -> Vec<MatchResult> {
    println!("AAA {:?} {:?}", old, nu);
    for mi in MatchIterator::new(old, nu) {
        println!("AAA iter {:?}", mi);

        match mi {
            First(mi0, mi1) => {
                assert!(getem(old, nu, mi0) <= getem(old, nu, mi1));
            },
            Middle(mi0, mi1, mi2) => {
                assert!(getem(old, nu, mi0) <= getem(old, nu, mi1));
                assert!(getem(old, nu, mi1) <= getem(old, nu, mi2));
            },
            Last(mi0, mi1) => {
                assert!(getem(old, nu, mi0) <= getem(old, nu, mi1));
            },
            _ => assert!(false),
        }
    }

    match_values_slow(old, nu)
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
