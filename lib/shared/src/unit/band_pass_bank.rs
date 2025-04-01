#[cfg(feature = "for_host")]
extern crate std;
extern crate libm;

use alloc::vec::Vec;
#[allow(unused)]
use std::println;

#[allow(unused)]
use crate::spew::*;
use crate::unit::band_pass::*;

const BW: f32 = 0.01;

#[derive(Debug)]
pub enum MatchResult {
    DropOld(usize),
    AddNew(usize),
    Match(usize, usize),
}

pub fn closest(x: f32, xs: &Vec<f32>) -> usize {
    assert!(!xs.is_empty());
    let mut dists: Vec<(usize, f32)> = xs.iter().enumerate()
        .map(|(i, xx)| (i, libm::fabsf(x-xx)))
        .collect();
    dists.sort_by(|(_, x), (_, xx)| xx.partial_cmp(x).unwrap());
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
    bps: Vec<(BandPass, f32)>,
}

impl BandPassBank {
    pub fn new() -> BandPassBank {
        BandPassBank {
            bps: Vec::new(),
        }
    }

    pub fn process(&mut self, input: f32) -> f32 {
        let mut output: f32 = 0.0;
        for (ref mut bp, ref amp) in &mut self.bps {
            output += amp * bp.process(input);
        }
        output
    }

    // (freq, amp)
    pub fn update(&mut self, fas: &Vec<(f32, f32)>) {
        let old_freqs: Vec<f32> = self.bps.iter().map(|(bp, _)| bp.get_freq()).collect();
        let new_freqs: Vec<f32> = fas.iter().map(|&fa| fa.0).collect();
        let results = match_values(&old_freqs, &new_freqs);

        println!("MR {:?}", results);

        for mr in &results {
            match mr {
                MatchResult::AddNew(i) => {
                    self.bps.push((BandPass::new(fas[*i].0, BW), fas[*i].1));
                },
                MatchResult::DropOld(i) => {
                    // Doing this in case we make it inertial and it doesn't drop out
                    // right away.
                    self.bps[*i].1 = 0.0;
                },
                MatchResult::Match(i, j) => {
                    self.bps[*i].0.set_freq(fas[*j].0);
                    self.bps[*i].1 = fas[*j].1;
                },
            }
        }

        // Remove the ones that have gone to 0.
        self.bps.retain(|(_, amp)| *amp > 0.0);

        // Sort the added ones in.
        self.bps.sort_by(|(_, a0), (_, a1)| a0.partial_cmp(a1).unwrap());

        /*
        // Matches: update freq and amp
        for &mr in results {
            match mr {
                Match(i, j) => {
                    bps[i].0.set_freq(fas[i].0);
                    bps[i].1 = fas[i].1;
                },
                _ => {},
            }
        }

        // TODO slow
        let remaining = self.bps.iter().enumerate()
            .filter(|i, _| results.contains(DropOld(i)))
            .map(|_, v| v)
            .collect();
        self.bps = remaining;

        // Adds: add new bp.
        for &mr in results {
            match mr {
                AddNew(i) => {
                    bps.push((BandPass::new(fas[i].0), fas[i].1));
                },
            }
        }
        */
    }
}
