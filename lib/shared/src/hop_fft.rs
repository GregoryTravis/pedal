extern crate std;
extern crate alloc;
extern crate libm;

use alloc::vec::Vec;
#[allow(unused)]
use std::println;

use crate::constants::*;
use crate::fft_host::*;
use crate::spew::*;

// Divide input into frames of size hop, fft each one, padded out to fft_size, returning
// the peaks. One peak is returned for each input sample; within a hop size, the peak is simply
// repeated.
pub fn hop_fft(input: &[f32], fft_size: usize, batch_size: usize, hop: usize) -> Vec<f32> {
    let mut peaks: Vec<f32> = vec![0.0; input.len()];
    let mut fft_in: &mut [f32] = &mut vec![0.0; fft_size];
    let mut fft_out: &mut [f32] = &mut vec![0.0; fft_size];

    for current in (0..input.len()).step_by(hop) {
        spew!("====", current);
        // TODO don't have to clear the beginning
        fft_in[0..fft_size].fill(0.0);
        // Necessary?
        fft_out[0..fft_size].fill(0.0);

        assert!(batch_size % 2 == 0);

        let batch_start: isize = current as isize - (batch_size/2) as isize;

        for i in 0..batch_size {
            let si = i as isize + batch_start;
            let s = if si < 0 || si >= input.len() as isize { 0.0 } else { input[si as usize] };
            fft_in[i] = s;
        }

        fft_slice(&mut fft_in, &mut fft_out);

        let freq = find_peak(fft_out);
        spew!("==== peak", current, freq);
        peaks[current] = freq;

        // Duplicate to the rest of the batch.
        for i in 1..hop {
            if current+i < input.len() {
                peaks[current+i] = freq;
            }
        }
    }

    peaks
}

// Divide input into frames of size hop, fft each one, padded out to fft_size. Get the loud peaks
// for each one, and return a vec of vecs of peaks, one for each hop.
// output: (freq, mix)
pub fn hop_peaks(input: &[f32], fft_size: usize, batch_size: usize, hop: usize) -> Vec<Vec<(f32, f32)>> {
    let mut peakses: Vec<Vec<(f32, f32)>> = Vec::new();
    let mut fft_in: &mut [f32] = &mut vec![0.0; fft_size];
    let mut fft_out: &mut [f32] = &mut vec![0.0; fft_size];

    for current in (0..input.len()).step_by(hop) {
        spew!("====", current);
        // TODO don't have to clear the beginning
        fft_in[0..fft_size].fill(0.0);
        // Necessary?
        fft_out[0..fft_size].fill(0.0);

        assert!(batch_size % 2 == 0);

        let batch_start: isize = current as isize - (batch_size/2) as isize;

        for i in 0..batch_size {
            let si = i as isize + batch_start;
            let s = if si < 0 || si >= input.len() as isize { 0.0 } else { input[si as usize] };
            fft_in[i] = s;
        }

        fft_slice(&mut fft_in, &mut fft_out);

        let bps = peaks_to_bps(find_peaks(fft_out));
        println!("==== peaks {} {:?}", current, bps);
        peakses.push(bps);
    }

    peakses
}

fn ramp_threshold(freq: f32) -> f32 {
    0.005 * linmap(60.0, 1500.0, 1.0, 2.0, freq)
}

// TODO do we need bin?
// output: (bin, freq, amp)
fn find_peaks(fft: &[f32]) -> Vec<(usize, f32, f32)> {
    let mut peaks: Vec<(usize, f32, f32)> = Vec::new();
    let fft_len = fft.len();

    let _amp_threshold = 0.005;

    for i in 0..fft_len {
        let not_edge = i > 0 && i < fft_len-1;
        if not_edge {
            // let bin_freq = i as f32 * (SAMPLE_RATE as f32 / fft_len as f32); 

            let a = fft[i-1];
            let b = fft[i];
            let c = fft[i+1];
            // TODO what if they are equal
            let is_peak = not_edge && a < b && b > c;
            if is_peak {
                let peakiness = (b - ((a+c)/2.0)) / b;

                let (relative_x_peak, y_peak) = quadratic_interpolate(a, b, c);
                let x_peak = (i as f32) + relative_x_peak;
                let amp_peak = y_peak / (fft_len / 2) as f32;
                let freq_peak = x_peak * (SAMPLE_RATE as f32 / fft_len as f32);
                //spew!("VVV", i, amp_peak, freq_peak, ramp_threshold(freq_peak));
                if amp_peak > ramp_threshold(freq_peak) { // amp_threshold {
                    peaks.push((i, freq_peak, amp_peak));
                    peaks.push((i, freq_peak * 2.0, amp_peak));
                    peaks.push((i, freq_peak * 3.0, amp_peak));
                    //peaks.push((i, freq_peak * 4.0, amp_peak));
                    peaks.push((i, freq_peak * 5.0, amp_peak));
                    spew!("*** peak", i, x_peak, y_peak, freq_peak, amp_peak, a, b, c, peakiness);
                } else {
                    //spew!("... peak", i, x_peak, y_peak, freq_peak, amp_peak, a, b, c, peakiness);
                }
            } else {
                //let freq = i as f32 * (SAMPLE_RATE as f32 / fft_len as f32);
                //spew!("--- peak", i, i as f32, fft[i], freq, fft[i] / (fft_len / 2) as f32);
            }
        }
    }
    peaks
}

// Map [x0,y0] to [x1,y1], apply that to x.
fn linmap(x0: f32, y0: f32, x1: f32, y1: f32, x: f32) -> f32 {
    let alpha = (x - x0) / (y0 - x0);
    //spew!("alpha", alpha);
    x1 + (alpha * (y1 - x1))
}

const LOW_AMP_THRESHOLD: f32 = 0.0;

// input: (bin, freq, amp)
// output: (freq, mix)
fn peaks_to_bps(peaks: Vec<(usize, f32, f32)>) -> Vec<(f32, f32)> {
    // Find highest peak, set low to that times low_amp_threshold, scale all amps to that, remove
    // negative (there are none above 1), return that.
    let mut peaks = peaks.clone();
    if !peaks.is_empty() {
        peaks.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());
        println!("amps {:?}", peaks.iter().map(|x| x.2).collect::<Vec<_>>());
        let high_amp: f32 = peaks[0].2;
        let low_amp: f32 = LOW_AMP_THRESHOLD * high_amp;
        peaks.retain(|x| x.2 >= low_amp);
        println!("amps in range {:?}", peaks.iter().map(|x| x.2));
        peaks.iter().map(|x| (x.1, linmap(low_amp, high_amp, 0.0, 1.0, x.2))).collect()
    } else {
        Vec::new()
    }
}

fn find_peak(fft: &[f32]) -> f32 {
    let peaks = find_peaks(fft);

    let ramp_down_by_freq = true;
    // 1.0 at 200 and 0.5 at 1000
    let low_ramp_freq = 200.0;
    let high_ramp_freq = 500.0;
    let low_ramp_amp_mult = 1.0;
    let high_ramp_amp_mult = 0.5;

    let do_min_freq = true;
    let min_freq = 100.0;

    let highest_pitch: Option<usize> = {
        let mut best: usize = 0;
        let mut best_amp: f32 = 0.0;
        let mut found: bool = false;

        for i in 0..peaks.len() {
            let freq = peaks[i].1;

            if do_min_freq && freq < min_freq {
                continue;
            }

            let orig_amp = peaks[i].2;

            let amp = if ramp_down_by_freq {
                let alpha = (freq - low_ramp_freq) / (high_ramp_freq - low_ramp_freq);
                let multiplier = low_ramp_amp_mult + (alpha * (high_ramp_amp_mult - low_ramp_amp_mult));
                orig_amp * multiplier
            } else {
                orig_amp
            };

            if !found || amp > best_amp {
                best = i;
                best_amp = amp;
                found = true;
            }
        }

        if found {
            spew!("max peak", peaks[best].0);
            Some(best)
        } else {
            //spew!("no peak");
            None
        }
    };

    let freq = match highest_pitch {
        Some(i) => peaks[i].1,
        None => 0.0,
    };

    freq
}

// Return peak of the curve described by the values.
// Returns (x_peak, y_peak).
// x_peak is relative to xp which is treated as 0.
// https://www.physics.drexel.edu/~steve/Courses/Comp_Phys/Physics-105/quad_int.pdf
fn quadratic_interpolate(xpp: f32, xp: f32, x: f32) -> (f32, f32) {
    // Remove mult by 1 and other stupid things.
    let dt = 1.0;
    let tp = 0.0;
    let a = xp;
    let b = (x - xpp) / (2.0 * dt);
    let c = (x - (2.0 * xp) + xpp) / (2.0 * dt * dt);
    let tau = tp - (b / (2.0 * c));
    let x_max = a - ((b * b) / (4.0 * c));
    (tau, x_max)
}
