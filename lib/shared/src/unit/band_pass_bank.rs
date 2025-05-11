#[cfg(feature = "for_host")]
extern crate std;
extern crate libm;

use alloc::vec::Vec;
#[allow(unused)]
//use std::println;

use crate::constants::*;
use crate::inertial::*;
#[allow(unused)]
use crate::spew::*;
use crate::unit::band_pass_stack::*;

const BW: f32 = 0.01;
//const AMP_DM: f32 = 10000000.0;
const AMP_DM: f32 = 16.0 / SAMPLE_RATE as f32;

const _VERBOSE: bool = false;

const NUM_BPS: usize = 17;

pub struct BandPassBank {
    // (bp, amp)
    bps: Vec<(BandPassStack, Inertial, bool)>,
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
        let mut bpb = BandPassBank {
            bps: Vec::new(),
        };
        for _ in 0..NUM_BPS {
            bpb.bps.push((BandPassStack::new(440.0, BW), Inertial::new(0.0, AMP_DM), false));
        }
        bpb
    }

    pub fn process(&mut self, input: f32) -> f32 {
        let mut output: f32 = 0.0;
        for (ref mut bp, ref mut amp, _) in &mut self.bps {
            amp.update();
            output += amp.get() * bp.process(input);
        }
        output
    }

    // (freq, amp)
    pub fn update(&mut self, new_freqs: &Vec<(f32, usize)>) {
        assert!(new_freqs.len() <= self.bps.len());

        // TODO disable
        if new_freqs.len() > 1 {
            for i in 0..new_freqs.len()-1 {
                assert!(new_freqs[i].0 < new_freqs[i+1].0);
            }
        }

        {
            // TODO disable

            let mut visited: Vec<bool> = vec![false; NUM_BPS];
            let mut i: usize = 0;
            for (freq, ref clump) in new_freqs {
                // Fade out unmentioned freqs.
                for ii in i..*clump {
                    // TODO disable
                    assert!(!visited[ii]);
                    visited[ii] = true;

                    self.bps[ii].1.set(0.0);
                    self.bps[ii].2 = true;
                }
                // TODO disable
                assert!(!visited[*clump]);
                visited[*clump] = true;

                let amp = ramp_one(*freq);
                self.bps[*clump].0.set_freq(*freq);
                self.bps[*clump].1.set(amp);
                self.bps[*clump].2 = false;

                i = *clump+1;
            }

            for ii in i..NUM_BPS {
                // TODO disable
                assert!(!visited[ii]);
                visited[ii] = true;

                self.bps[ii].1.set(0.0);
                self.bps[ii].2 = true;
            }

            // TODO disable
            for b in visited {
                assert!(b);
            }
        }
    }

    /*
    pub fn dump(&self, tag: &str) {
        let final_fas: Vec<(f32, f32)> = self.bps.iter().map(|(bp, a, _)| (bp.get_freq(), (*a).get())).collect();
        println!("{} {:?}", tag, final_fas);
    }
    */
}
