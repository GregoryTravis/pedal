#[cfg(feature = "for_host")]
extern crate std;
extern crate libm;

use alloc::vec::Vec;
use core::iter::*;
#[allow(unused)]
//use std::println;

#[allow(unused)]
use crate::spew::*;

const _VERBOSE: bool = false;

#[derive(Debug, PartialEq)]
pub enum MatchResult {
    DropOld(usize),
    AddNew(usize),
    Match(usize, usize),
}

// Two frequencies aren't considered for matching unless they're closer than this.
const MAX_CLOSE: f32 = 120.0;

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

pub fn match_values(
    old: &Vec<f32>,
    nu: &Vec<f32>,
    /*mem*/ old_faves: &mut Vec<Option<usize>>,
    /*mem*/ nu_faves: &mut Vec<Option<usize>>,
    /*out*/ results: &mut Vec<MatchResult>) {
    // println!("AAA mvf {:?} {:?}", old, nu);
    check_iterator(old, nu);

    old_faves.clear();
    old_faves.extend(repeat(None).take(old.len()));
    nu_faves.clear();
    nu_faves.extend(repeat(None).take(nu.len()));
    results.clear();
    assert!(old_faves.len() == old.len());
    assert!(nu_faves.len() == nu.len());

    for mi in MatchIterator::new(old, nu) {
        // println!("AAA iter {:?}", mi);

        match mi {
            // For the first two, we only assign a fave to the first one.
            First(mi0, mi1) => {
                match (mi0, mi1) {
                    ((Old, oi), (Nu, ni)) => {
                        let dist = nu[ni] - old[oi];
                        if dist < MAX_CLOSE {
                            old_faves[oi] = Some(ni);
                        }
                    }
                    ((Nu, ni), (Old, oi)) => {
                        let dist = old[oi] - nu[ni];
                        if dist < MAX_CLOSE {
                            nu_faves[ni] = Some(oi);
                        }
                    }
                    _ => (),
                }
            },
            // For the middle groups, we only assign a fave to the middle one.
            Middle(mi0, mi1, mi2) => {
                match (mi0, mi1, mi2) {
                    ((Old, oi), (Nu, ni), (Old, oi2)) => {
                        let dist = nu[ni] - old[oi];
                        let dist2 = old[oi2] - nu[ni];
                        if dist < dist2 {
                            if dist < MAX_CLOSE {
                                nu_faves[ni] = Some(oi);
                            }
                        } else {
                            if dist2 < MAX_CLOSE {
                                nu_faves[ni] = Some(oi2);
                            }
                        }
                    },
                    ((Nu, ni), (Old, oi), (Nu, ni2)) => {
                        let dist  = old[oi] - nu[ni];
                        let dist2 = nu[ni2] - old[oi];
                        if dist < dist2 {
                            if dist < MAX_CLOSE {
                                old_faves[oi] = Some(ni);
                            }
                        } else {
                            if dist2 < MAX_CLOSE {
                                old_faves[oi] = Some(ni2);
                            }
                        }
                    },
                    ((Old, oi), (Nu, ni), (Nu, _)) => {
                        let dist = nu[ni] - old[oi];
                        if dist < MAX_CLOSE {
                            nu_faves[ni] = Some(oi);
                        }
                    }
                    ((Nu, ni), (Old, oi), (Old, _)) => {
                        let dist = old[oi] - nu[ni];
                        if dist < MAX_CLOSE {
                            old_faves[oi] = Some(ni);
                        }
                    }
                    ((Nu, _), (Nu, ni), (Old, oi)) => {
                        let dist = old[oi] - nu[ni];
                        if dist < MAX_CLOSE {
                            nu_faves[ni] = Some(oi);
                        }
                    }
                    ((Old, _), (Old, oi), (Nu, ni)) => {
                        let dist = nu[ni] - old[oi];
                        if dist < MAX_CLOSE {
                            old_faves[oi] = Some(ni);
                        }
                    }
                    ((Old, _), (Old, _), (Old, _)) => (),
                    ((Nu, _), (Nu, _), (Nu, _)) => (),
                }
            },
            // For the last two, we only assign a fave to the last one.
            Last(mi0, mi1) => {
                match (mi0, mi1) {
                    ((Old, oi), (Nu, ni)) => {
                        let dist = nu[ni] - old[oi];
                        if dist < MAX_CLOSE {
                            nu_faves[ni] = Some(oi);
                        }
                    }
                    ((Nu, ni), (Old, oi)) => {
                        let dist = old[oi] - nu[ni];
                        if dist < MAX_CLOSE {
                            old_faves[oi] = Some(ni);
                        }
                    }
                    _ => (),
                }
            },
            _ => assert!(false),
        }
    }

    //if VERBOSE { println!("old_faves {:?}", old_faves); }
    //if VERBOSE { println!(" nu_faves {:?}", nu_faves); }

    faves_to_results(old_faves, nu_faves, results);
}

// TODO disable
fn check_iterator(old: &Vec<f32>, nu: &Vec<f32>) {
    for mi in MatchIterator::new(old, nu) {
        // println!("AAA iter {:?}", mi);

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
}

fn faves_to_results(
    old_faves: &Vec<Option<usize>>,
    nu_faves: &Vec<Option<usize>>,
    results: &mut Vec<MatchResult>) {

    // For each pair of values (old and new) that agree, add a match. For any value
    // that doesn't agree with its fave nu, add a drop.
    for i in 0..old_faves.len() {
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
    for i in 0..nu_faves.len() {
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
    let check = false;
    if check {
        let mut old_used: Vec<bool> = vec![false; old_faves.len()];
        let mut nu_used: Vec<bool> = vec![false; nu_faves.len()];
        for mr in results {
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
}
