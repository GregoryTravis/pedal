extern crate libm;

use core::f32::consts::PI;

use crate::arm_fft::*;
use crate::bench::benchmark;
use crate::constants::*;
use crate::fft::*;
use crate::microfft_fft::*;
use crate::spew::*;

#[allow(dead_code)]
pub fn do_benchmark_fft() {
    let mut orig_input: [f32; FFT_SIZE] = [0.0; FFT_SIZE];
    for i in 0..orig_input.len() {
        orig_input[i] = libm::sinf(2.0 * PI * i as f32 * (440.0 / SAMPLE_RATE as f32));
    }

    let dur = 5.0;

    assert!(FFT_SIZE == 2048);

    {
        let mut arm_fft = ArmFFT::new();
        arm_fft.get_input().clone_from_slice(&orig_input);

        let arm_bench = benchmark(dur, || {
            arm_fft.run();
        });
        spew!("arm", arm_bench.execution_count, arm_bench.avg_time, arm_bench.execution_count as f32 / dur);
    }

    // microfft overwrites the input and the others probably do too, so doing it repeatedly is
    // meaningless, but I don't care about the values, except possibly to make sure that 0s aren't
    // special-cased.
    {
        let mut microfft_fft = MicroFFT::new();
        microfft_fft.get_input().clone_from_slice(&orig_input);

        let microfft_bench = benchmark(dur, || {
            microfft_fft.run();
        });
        spew!("microfft", microfft_bench.execution_count, microfft_bench.avg_time, microfft_bench.execution_count as f32 / dur);
    }
}

pub fn fft_output_comparison() {
    let mut input: [f32; FFT_SIZE] = [0.0; FFT_SIZE];
    for i in 0..input.len() {
        //input[i] = libm::sinf(2.0 * PI * i as f32 * (440.0 / SAMPLE_RATE as f32));
        input[i] = ((i as f32 * 2.0) / input.len() as f32) - 1.0;
    }

    let mut arm_fft = ArmFFT::new();
    arm_fft.get_input().clone_from_slice(&input);
    let arm_output = arm_fft.run();

    let mut microfft_fft = MicroFFT::new();
    microfft_fft.get_input().clone_from_slice(&input);
    let microfft_output = microfft_fft.run();

    for i in 0..FFT_SIZE {
        spew!("comp", i, input[i], "arm", arm_output[i], "microfft", microfft_output[i], "diff", arm_output[i] - microfft_output[i]);
    }
}
