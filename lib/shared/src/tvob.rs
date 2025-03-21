#[cfg(feature = "for_host")]
extern crate std;
extern crate libm;

use std::print;
use std::println;
//use alloc::boxed::Box;
use alloc::vec::Vec;
use core::f32::consts::PI;
use core::fmt;

use crate::constants::*;

/*

Time-varying oscillator bank.

Each frame, you give it some (frequency, amplitude) pairs, and it generates sine waves.

It creates continuity by matching each sine to an existing one; when it cannot, it fades new ones
in and old ones out.

*/

pub struct TVO {
    // Per sample TODO make this per second
    max_frequency_delta: f32,
    max_amplitude_delta: f32,
    // In Hz
    phase: f32,
    frequency: f32,
    amplitude: f32,
    target_frequency: f32,
    target_amplitude: f32,
    ramping_down: bool,
}

// a is going to b, but no faster than max
fn clip_delta(a: f32, b:f32, max: f32) -> f32 {
    if b > a {
        let delta = (b-a).min(max);
        delta
    } else {
        let delta = (a-b).min(max);
        -delta
    }
}

impl TVO {
    pub fn new(max_frequency_delta: f32, max_amplitude_delta: f32,
               frequency: f32, amplitude: f32, target_amplitude: f32) -> TVO {
        TVO {
            max_frequency_delta: max_frequency_delta,
            max_amplitude_delta: max_amplitude_delta,
            phase: 0.0,
            frequency: frequency,
            amplitude: amplitude,
            target_frequency: frequency,
            target_amplitude: target_amplitude,
            ramping_down: false,
        }
    }

    pub fn update_targets(&mut self, target_frequency: f32, target_amplitude: f32) {
        self.target_frequency = target_frequency;
        self.target_amplitude = target_amplitude;
    }

    pub fn next_sample(&mut self) -> f32 {
        self.update_freq_amp();
        let phase_delta = self.frequency / SAMPLE_RATE as f32;
        let oph = self.phase;
        self.phase += phase_delta;
        /*
        while self.phase > 2.0 * PI {
            self.phase -= 2.0 * PI;
        }
        */
        let s = libm::sinf(self.phase * 2.0 * PI) * self.amplitude;
        println!("ren {:?} {} {} {} {}", (self.frequency, self.amplitude),
            oph, phase_delta, self.phase, s);
        s
    }

    fn update_freq_amp(&mut self) {
        println!("update_fa b {:?}", (self.frequency, self.amplitude));
        let frequency_delta = clip_delta(self.frequency, self.target_frequency, self.max_frequency_delta);
        self.frequency += frequency_delta;
        let amplitude_delta = clip_delta(self.amplitude, self.target_amplitude, self.max_amplitude_delta);
        self.amplitude += amplitude_delta;
        println!("update_fa a {:?}", (self.frequency, self.amplitude));
    }

    pub fn to_zero(&mut self) {
        self.target_amplitude = 0.0;
        self.ramping_down = true;
    }

    fn is_done(&self) -> bool {
        self.amplitude <= 0.0 && self.ramping_down
    }
}

impl fmt::Debug for TVO {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.frequency, self.amplitude)
    }
}

/*
fn match_loudest(before: Vec<TVO>, after: Vec<(f32, f32)>) -> MatchResult {
    let (bi, loudest_before): (usize, TVO) = before.iter().enumerate().max_by(|tvo| tvo.amplitude);
    let (ai, loudest_after): (usize, fa) = after.iter().enumerate().max_by(|fa| fa.1);
    if (bi < ai) {
}
*/

#[derive(Debug)]
struct MatchResult {
    matches: Vec<(usize, usize)>,
    removed: Vec<usize>,
    added: Vec<usize>,
}

fn line_up(before: &Vec<TVO>, biu: usize, after: &Vec<(f32, f32)>, aiu: usize) -> MatchResult {
    let mut matches: Vec<(usize, usize)> = Vec::new();
    let mut removed: Vec<usize> = Vec::new();
    let mut added: Vec<usize> = Vec::new();

    let bi = biu as isize;
    let ai = aiu as isize;

    matches.push((bi as usize, ai as usize));

    let mut d: isize = -1;
    while bi+d >= 0 || ai+d >= 0 {
        //println!("line_up 0");
        if bi+d >= 0 && ai+d >= 0 {
            matches.push(((bi+d) as usize, (ai+d) as usize));
        } else if bi+d >= 0 {
            removed.push((bi+d) as usize);
        } else if ai+d >= 0 {
            added.push((ai+d) as usize);
        } else {
            assert!(false);
        }
        d -= 1;
    }

    let mut d = 1;
    while bi+d < before.len() as isize || ai+d < after.len() as isize {
        //println!("line_up 1");
        if bi+d < before.len() as isize && ai+d < after.len() as isize {
            matches.push(((bi+d) as usize, (ai+d) as usize));
        } else if bi+d < before.len() as isize {
            removed.push((bi+d) as usize);
        } else if ai+d < after.len() as isize {
            added.push((ai+d) as usize);
        } else {
            assert!(false);
        }
        d += 1;
    }

    MatchResult {
        matches: matches,
        removed: removed,
        added: added,
    }
}

fn closest_freq(before: &Vec<TVO>, after: &Vec<(f32, f32)>) -> Option<MatchResult> {
    let mut best: (usize, usize, f32) = (0, 0, 0.0);
    let mut found: bool = false;

    for (bi, &ref tvo) in before.iter().enumerate() {
        for (ai, &fa) in after.iter().enumerate() {
            let bf = tvo.frequency;
            let af = fa.0;
            let diff = (af - bf).abs();
            if !found || diff < best.2 {
                best = (bi, ai, diff);
                println!("best {:?}", best);
                found = true;
            }
        }
    }

    if !found {
        None
    } else {
        Some(line_up(before, best.0, after, best.1))
    }
}

#[derive(Debug)]
pub enum Matcher {
    //MatchLoudest,
    ClosestFreq,
}

impl Matcher {
    fn mtch(&self, before: &Vec<TVO>, after: &Vec<(f32, f32)>) -> Option<MatchResult> {
        match &self {
            //MatchLoudest => match_loudest(before, after),
            Matcher::ClosestFreq => closest_freq(before, after),
        }
    }
}

#[derive(Debug)]
pub struct TVOB {
    // Per sample TODO make this per second
    max_frequency_delta: f32,
    max_amplitude_delta: f32,
    matcher: Matcher,
    oscs: Vec<TVO>,
}

impl TVOB {
    pub fn new(max_frequency_delta: f32, max_amplitude_delta: f32, matcher: Matcher) -> TVOB {
        TVOB {
            max_frequency_delta: max_frequency_delta,
            max_amplitude_delta: max_amplitude_delta,
            matcher: matcher,
            oscs: Vec::new(),
        }
    }

    pub fn next_sample(&mut self) -> f32 {
        let mut sample: f32 = 0.0;
        // do with iter
        for i in 0..self.oscs.len() {
            let osc = &mut self.oscs[i];
            //println!("osc {}: {:?}", i, osc);
            sample += osc.next_sample();
        }
        sample
    }

    // remove t
    pub fn update(&mut self, t: u32, after: Vec<(f32, f32)>) {
        println!("update");
        println!("{:?}", self.oscs);
        println!("{:?}", after);

        if self.oscs.is_empty() {
            // First time, just adopt them
            self.oscs = after.iter().map(
                |fa| TVO::new(self.max_frequency_delta, self.max_amplitude_delta, fa.0, 0.0, fa.1)).collect();
        } else {
            let mr = self.matcher.mtch(&self.oscs, &after);
            match mr {
                Some(mr) => {
                    println!("{} {:?}", t, mr);

                    for fa in mr.matches {
                        let new_fa = after[fa.1];
                        self.oscs[fa.0].update_targets(new_fa.0, new_fa.1);
                    }

                    for i in mr.removed {
                        self.oscs[i].to_zero();
                    }

                    for i in mr.added {
                        let fa = after[i];
                        let new_osc = TVO::new(self.max_frequency_delta, self.max_amplitude_delta, fa.0, 0.0, fa.1);
                        self.oscs.push(new_osc);
                    }

                    println!("updated");
                    println!("{:?}", self.oscs);

                    self.sort_oscs();
                    self.cleanup();

                    println!("done");
                    println!("{:?}", self.oscs);
                },
                None => {
                    println!("No match result");
                }
            }
        }
    }

    fn sort_oscs(&mut self) {
        println!("sort");
        println!("{:?}", self.oscs);
        //self.oscs.sort_by_key(|o| o.frequency);
        // TODO could crash
        self.oscs.sort_by(|a, b| a.frequency.partial_cmp(&b.frequency).unwrap());
        println!("{:?}", self.oscs);
    }

    fn cleanup(&mut self) {
        println!("cleanup");
        println!("{:?}", self.oscs);
        //self.oscs = self.oscs.iter().filter(|o| !o.is_done()).collect::<Vec<TVO>>();
        // TODO no clone?
        //let new_oscs = Vec::new();
        // TODO no
        //self.oscs = self.oscs.clone().into_iter().filter(|o| !o.is_done()).collect::<Vec<TVO>>();
        //self.oscs = self.oscs.iter().filter(|o| !o.is_done()).collect::<Vec<TVO>>();
        self.oscs.retain(|o| !o.is_done());
        println!("{:?}", self.oscs);
    }

    pub fn ratio_report(&self) {
        print!("rats ");
        if self.oscs.len() > 1 {
            for i in 0..self.oscs.len()-1 {
                print!("{} ", self.oscs[i+1].frequency / self.oscs[i].frequency);
            }
        }
        println!("");
    }
}
