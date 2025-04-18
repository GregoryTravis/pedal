extern crate std;
extern crate alloc;
extern crate libm;

use alloc::vec::Vec;
#[allow(unused)]
use std::println;

use crate::constants::*;
use crate::fft::*;
use crate::microfft_fft::*;
use crate::quadratic_interpolate::*;
use crate::spew::*;

const VERBOSE: bool = false;

// Divide input into frames of size hop, fft each one, padded out to fft_size. Get the loud peaks
// for each one, and return a vec of vecs of peaks, one for each hop.
// output: (freq, mix)
//let fases: Vec<Vec<(f32, f32)>> = hop_peaks(&input, 4096, 2048, 48);
pub fn hop_peaks(current:usize, input: &[f32; 2048], _fft_size: usize, batch_size: usize, /*out*/ peaks: &mut Vec<f32>) {
    /*
    let mut fft_in: &mut [f32] = &mut vec![0.0; fft_size];
    let mut fft_out: &mut [f32] = &mut vec![0.0; fft_size];

    for i in 0..input.len() {
        fft_in[i] = input[i];
    }

    if VERBOSE { spew!("====", current); }
    // TODO don't have to clear the beginning
    //fft_in[0..fft_size].fill(0.0);
    // Necessary?
    //fft_out[0..fft_size].fill(0.0);
    */

    assert!(batch_size % 2 == 0);

    let mut mf = MicroFFT::new();
    mf.get_input().copy_from_slice(input);
    let mut mags: [f32; FFT_SIZE/2] = [0.0; FFT_SIZE/2];
    fft_to_magnitudes(mf.run(), &mut mags);

    find_peaks(&mags, peaks);
    if VERBOSE { println!("==== peaks {} {:?}", current, peaks); }
}

fn ramp_threshold(freq: f32) -> f32 {
    0.005 * linmap(60.0, 1500.0, 1.0, 2.0, freq)
}

// TODO do we need bin?
// output: (bin, freq, amp)
fn find_peaks(fft: &[f32], /*out*/ peaks: &mut Vec<f32>) {
    peaks.clear();

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
                let amp_peak = y_peak / fft_len as f32;
                assert!(amp_peak >= 0.0);
                let freq_peak = x_peak * (SAMPLE_RATE as f32 / (fft_len * 2) as f32);
                //spew!("VVV", i, amp_peak, freq_peak, ramp_threshold(freq_peak));
                if amp_peak > ramp_threshold(freq_peak) { // amp_threshold {
                    // Throwing away i and amp_peak here
                    peaks.push(freq_peak);
                    peaks.push(freq_peak * 2.0);
                    peaks.push(freq_peak * 3.0);
                    peaks.push(freq_peak * 5.0);
                    if VERBOSE { spew!("*** peak", i, x_peak, y_peak, freq_peak, amp_peak, a, b, c, peakiness); }
                } else {
                    //spew!("... peak", i, x_peak, y_peak, freq_peak, amp_peak, a, b, c, peakiness);
                }
            } else {
                //let freq = i as f32 * (SAMPLE_RATE as f32 / fft_len as f32);
                //spew!("--- peak", i, i as f32, fft[i], freq, fft[i] / (fft_len / 2) as f32);
            }
        }
    }

    /* Not in order because of added overtones
    // TODO disable this
    for i in 1..peaks.len() {
        assert!(peaks[i-1] < peaks[i]);
    }
    */
    peaks.sort_by(|f0, f1| f0.partial_cmp(f1).unwrap());
}

// Map [x0,y0] to [x1,y1], apply that to x.
fn linmap(x0: f32, y0: f32, x1: f32, y1: f32, x: f32) -> f32 {
    let alpha = (x - x0) / (y0 - x0);
    //spew!("alpha", alpha);
    x1 + (alpha * (y1 - x1))
}
