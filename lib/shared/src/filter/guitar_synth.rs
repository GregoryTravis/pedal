#[cfg(feature = "for_host")]
extern crate std;
extern crate libm;

use alloc::boxed::Box;
use alloc::vec::Vec;
use core::any::Any;
use core::f32::consts::PI;

use crate::constants::*;
use crate::hop_fft::*;
use crate::knob::Knobs;
use crate::maxes::*;
use crate::microfft_fft::*;
use crate::patch::Patch;
use crate::playhead::Playhead;
#[allow(unused)]
use crate::spew::*;
use crate::unit::band_pass_bank::*;

const DO_MAXES: bool = true;

pub struct GuitarSynth {
    buf: [f32; FFT_SIZE],
    bank: BandPassBank,

    maxes: Option<Maxes>,

    fft: MicroFFT,

    // TODO remove
    current_start: usize,

    peaks: Vec<(f32, usize)>,
    mags: [f32; FFT_SIZE/2],
}

// Convention for labeling mem params:
// /*out*/ -- output is returned through this
// /*mem*/ -- just passing the mem in for internal use

impl GuitarSynth {
    pub fn new() -> GuitarSynth {
        GuitarSynth {
            buf: [0.0; FFT_SIZE],
            bank: BandPassBank::new(),

            fft: MicroFFT::new(),

            maxes: if DO_MAXES { Some(Maxes::new()) } else { None },

            current_start: 0,

            peaks: Vec::new(), // Vec::with_capacity(200),
            mags: [0.0; FFT_SIZE/2],
        }
    }
}

// Hann window
// w(n) = 0.5 * [1 - cos(2*pi*n / N)]
// Usage;
//   self.input[i] *= hann(i, FFT_SIZE);
#[allow(unused)]
fn hann(n: usize, num_samples: usize) -> f32 {
    0.5 * (1.0 - libm::cosf((2.0 * PI * n as f32) / num_samples as f32))
}

impl Patch for GuitarSynth {
    fn rust_process_audio(
        &mut self,
        input_slice: &[f32],
        output_slice: &mut [f32],
        knobs: &Box<dyn Knobs>,
        mut _playhead: Playhead,
    ) {
        let hop = input_slice.len();

        // Shift new samples in
        // TODO use a method
        for i in 0..FFT_SIZE-hop {
            self.buf[i] = self.buf[i+hop];
        }
        for i in 0..hop {
            self.buf[FFT_SIZE-hop+i] = input_slice[i];
        }

        let wid: f32 = knobs.read(0);
        let ness: f32 = knobs.read(1);

        hop_peaks(wid, ness, &mut self.fft, self.current_start, &self.buf, &mut self.mags, &mut self.peaks);
        self.bank.update(&self.peaks);

        for i in 0..hop {
            output_slice[i] = self.bank.process(input_slice[i]);
            //output_slice[i] = 0.0;
        }

        self.current_start += hop;

        if DO_MAXES {
            use Item::*;
            self.maxes.as_mut().unwrap().update(Peaks, self.peaks.len());
        }
    }

    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}
