#[cfg(feature = "for_host")]
extern crate std;
extern crate alloc;
extern crate libm;

use alloc::vec::Vec;
#[allow(unused)]
//use std::println;

use crate::constants::*;
use crate::fft::*;
use crate::microfft_fft::*;
use crate::quadratic_interpolate::*;
#[allow(unused)]
use crate::spew::*;

#[allow(unused)]
const VERBOSE: bool = false;

// Divide input into frames of size hop, fft each one, padded out to fft_size. Get the loud peaks
// for each one, and return a vec of vecs of peaks, one for each hop.
// output: (freq, mix)
// TODO remove _wid, ness
pub fn hop_peaks(_wid: f32, ness: f32, fft: &mut MicroFFT, current:usize, input: &[f32; 2048], /*mem*/ mags: &mut [f32; FFT_SIZE/2], /*out*/ peaks: &mut Vec<f32>) {
    fft.get_input().copy_from_slice(input);
    fft_to_magnitudes(fft.run(), mags);

    let here: usize = 147851; // 96314;
    let hop_base = here - (here % 48);
    /*
    if hop_base == current {
        for (_i, x) in mags.iter().enumerate() {
            //spew!("mag", i, x);
            spew!(x);
        }
    }
    */

    find_peaks(hop_base == current, _wid, ness, &mags, peaks);
    //if VERBOSE { println!("==== peaks {} {:?}", current, peaks); }
}

#[allow(unused)]
fn ramp_threshold(freq: f32) -> f32 {
    0.005 * linmap(60.0, 1500.0, 1.0, 2.0, freq)
}

const PEAK_NUM_NEIGHBORS: usize = 2;

// TODO do we need bin?
// output: (bin, freq, amp)
fn find_peaks(dump: bool, _wid: f32, ness: f32, fft: &[f32; FFT_SIZE/2], /*out*/ peaks: &mut Vec<f32>) {
    peaks.clear();

    // TODO this is mags, not fft.
    let fft_len = fft.len();


    // TODO is this the right way?
    //let wid = wid.max(2.0);
    let ness = ness.min(0.95);
    // TODO use this
    let _ = ness;

    // wid in bins
    // ness is 0..1
    // If lower than 2, grunting.
    let low_bin: usize = 2;
    let clump_size: usize = 3;
    let num_clumps: usize = 17;
    let high_bin: low_bin + (clump_size * num_clups) - 1;
    spew("CLUMP", low_bin, high_bin);

    // Make sure there's a full set of neighbors on the sides.
    assert!(low_bin >= PEAK_NUM_NEIGHBORS);
    assert!(high_bin <= (FFT_SIZE/2) - PEAK_NUM_NEIGHBORS);

    for i in low_bin..high_bin {
        // We don't consider the first or last just cuz then we can't do peak interpolation and
        // also they're never frequencies we want.
        if i == 0 || i == fft_len-1 {
            continue;
        }

        let window_start: usize = i - PEAK_NUM_NEIGHBORS;
        let window_end: usize = i + PEAK_NUM_NEIGHBORS;

        // Consider all samples in the window except the (prospective) peak. They all have to be
        // less than the peak. We take the average of all of them, and then the ratio of that to
        // the peak, that is the sharpness.
        // Max sharpness is 0 (lobe is all 0); min sharpness is 1 (lobe == peak).
        // Also punt on ppeak=0 so we don't /0.
        //let ppeak: f32 = fft[i];


#[allow(unused)]
        let (peak_freq, peak_mag, peak_amp) = {
            let (relative_peak_i, peak_mag) = quadratic_interpolate(fft[i-1], fft[i], fft[i+1]);
            let peak_i = i as f32 + relative_peak_i;
            // The *2 is because the mag array is half the length of the fft size.
            let peak_freq = peak_i * (SAMPLE_RATE as f32 / (fft_len * 2) as f32);
            let peak_amp = peak_mag / fft_len as f32;
            // commented out because if it's a trough then this could be negative (?)
            // assert!(peak_amp >= 0.0);
            (peak_freq, peak_mag, peak_amp)
        };

        // If we use the interpolated peak magnitude it should be a more faithful peak, but then we
        // get double hits, on either side of the actual peak. Using the actual bin value is
        // simpler. TODO do the right thing here.
        // let ppeak: f32 = peak_mag;
        let ppeak: f32 = fft[i];


        let sharpness: f32 = if ppeak == 0.0 { 1.0 } else {
            // Look for a neighbor higher than the target; if any, then sharpness=1 (not sharp at
            // all).
            let mut found_higher = false;
            for j in window_start..window_end+1 {
                if i != j && fft[j] > ppeak {
                    found_higher = true;
                    break;
                }
            }
            if found_higher { 1.0 } else {
                let count = window_end - window_start; // no -1 because it's inclusive
                assert!(count > 0);
                let mut totes = 0.0;
                for j in window_start..window_end+1 {
                    if i != j {
                        totes += fft[j];
                    }
                }
                let avg = totes / count as f32;
                if dump {
                    //spew!("avg", i, totes, count, window_start, window_end, avg, avg/ppeak);
                }
                avg / ppeak
            }
        };

        // E.g. 0.7 means the lobes have to be lower than 70% of the peak.
        // To qualify, a peak must be sharper, that is, lower than min_sharpness. Yeah, I know.
        let min_sharpness = 0.7; // 1.0 - ness;
        let fa = if sharpness < min_sharpness {
            // We have a peak that is peaky enough.

            // Interpolate to find the real max freq, using the values on either side. This is why
            // we don't consider the first or last bins.

            // Sharpness is now (1..0), 1 means flat, 0 means as sharp as possible.
            // Convert this to (0..1) with 0 meaning flat, 1 meaning maximally sharp.
            let sharpness = 1.0 - sharpness;

            // TODO Not using amp yet, first use peakiness to scale it in; maybe just peak_amp *
            // peakiness?

            Some((peak_freq, sharpness))
        } else {
            None
        };
        if dump {
            // let (show_f, show_a): (f32, f32) = fa.unwrap_or((0.0, 0.0));
            // let mark = if fa.is_some() { "*" } else { " " };
            // spew!("peak", mark, i, ppeak, fft[i], peak_mag, peak_freq, peak_amp, sharpness, window_start, window_end, show_f, show_a);
        }

        let max_funds = 10;
        let num_overtones = 1;
        let max_num_peaks: usize = max_funds * num_overtones; // 10 * # of overtones
        match fa {
            Some((f, _)) => {
                if peaks.len() < max_num_peaks {
                    peaks.push(f);
                    // peaks.push(f * 2.0);
                    // peaks.push(f * 3.0);
                    // peaks.push(f * 5.0);
                }
            }
            None => (),
        }


        /*
        if peaks.len() >= 40 { break; }

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
                let amp_peak = y_peak / fft_len as f32;
                assert!(amp_peak >= 0.0);
                let freq_peak = x_peak * (SAMPLE_RATE as f32 / (fft_len * 2) as f32);
                //spew!("VVV", i, amp_peak, freq_peak, ramp_threshold(freq_peak));
                if amp_peak > ramp_threshold(freq_peak) { // amp_threshold {
                    // Throwing away i and amp_peak here
                    peaks.push(freq_peak);
                    //peaks.push(freq_peak * 2.0);
                    //peaks.push(freq_peak * 3.0);
                    //peaks.push(freq_peak * 5.0);
                    if VERBOSE { spew!("*** peak", i, x_peak, y_peak, freq_peak, amp_peak, a, b, c, peakiness); }
                } else {
                    //spew!("... peak", i, x_peak, y_peak, freq_peak, amp_peak, a, b, c, peakiness);
                }
            } else {
                //let freq = i as f32 * (SAMPLE_RATE as f32 / fft_len as f32);
                //spew!("--- peak", i, i as f32, fft[i], freq, fft[i] / (fft_len / 2) as f32);
            }
        }
            */
    }

    /* Not in order because of added overtones
    // TODO disable this
    for i in 1..peaks.len() {
        assert!(peaks[i-1] < peaks[i]);
    }
    */
    peaks.sort_by(|f0, f1| f0.partial_cmp(f1).unwrap());

    //spew!("npeaks", peaks.len());
}

// Map [x0,y0] to [x1,y1], apply that to x.
#[allow(unused)]
fn linmap(x0: f32, y0: f32, x1: f32, y1: f32, x: f32) -> f32 {
    let alpha = (x - x0) / (y0 - x0);
    //spew!("alpha", alpha);
    x1 + (alpha * (y1 - x1))
}
