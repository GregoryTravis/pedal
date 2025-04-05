extern crate libm;

use core::f32::consts::PI;

use microfft::real::*;

use crate::bench::benchmark;
use crate::constants::*;
//use crate::fft::*;
use crate::spew::*;

extern "C" {
    pub fn do_arm_fft(input: *mut f32, output: *mut f32);
    pub fn do_arm_ifft(input: *mut f32, output: *mut f32);
    pub fn do_shy_fft(input: *mut f32, output: *mut f32);
    pub fn do_shy_ifft(input: *mut f32, output: *mut f32);
}

pub fn arm_fft(input: &mut [f32; FFT_SIZE], output: &mut [f32; FFT_SIZE]) {
    let input_ptr: *mut f32 = input.as_mut_ptr();
    let output_ptr: *mut f32 = output.as_mut_ptr();
    unsafe {
        do_arm_fft(input_ptr, output_ptr);
    }
}

pub fn shy_fft(input: &mut [f32; FFT_SIZE], output: &mut [f32; FFT_SIZE]) {
    let input_ptr: *mut f32 = input.as_mut_ptr();
    let output_ptr: *mut f32 = output.as_mut_ptr();
    unsafe {
        do_shy_fft(input_ptr, output_ptr);
    }
}

#[allow(dead_code)]
pub fn do_benchmark_fft() {
    let mut orig_input: [f32; FFT_SIZE] = [0.0; FFT_SIZE];
    for i in 0..orig_input.len() {
        orig_input[i] = libm::sinf(2.0 * PI * i as f32 * (440.0 / SAMPLE_RATE as f32));
    }

    let mut output: [f32; FFT_SIZE] = [0.0; FFT_SIZE];

    let dur = 5.0;


    assert!(FFT_SIZE == 2048);

    // microfft overwrites the input and the others probably do too, so doing it repeatedly is
    // meaningless, but I don't care about the values, except possibly to make sure that 0s aren't
    // special-cased.
    {
        let mut input = orig_input.clone();
        let arm_bench = benchmark(dur, || {
            arm_fft(&mut input, &mut output);
        });
        spew!("arm", arm_bench.execution_count, arm_bench.avg_time, arm_bench.execution_count as f32 / dur);
    }

    {
        let mut input = orig_input.clone();
        let shy_bench = benchmark(dur, || {
            shy_fft(&mut input, &mut output);
        });
        spew!("shy", shy_bench.execution_count, shy_bench.avg_time, shy_bench.execution_count as f32 / dur);
    }

    {
        let mut input = orig_input.clone();
        let microfft_bench = benchmark(dur, || {
            let _ = rfft_2048(&mut input);
        });
        spew!("microfft", microfft_bench.execution_count, microfft_bench.avg_time, microfft_bench.execution_count as f32 / dur);
    }
}

pub fn fft_output_comparison(num_to_print: usize) {
    let mut input: [f32; FFT_SIZE] = [0.0; FFT_SIZE];
    for i in 0..input.len() {
        input[i] = libm::sinf(2.0 * PI * i as f32 * (440.0 / SAMPLE_RATE as f32));
    }

    let mut arm_input = input.clone();
    let mut shy_input = input.clone();
    let mut microfft_input = input.clone();

    let arm_input_ptr: *mut f32 = arm_input.as_mut_ptr();
    let shy_input_ptr: *mut f32 = shy_input.as_mut_ptr();

    let mut arm_output: [f32; FFT_SIZE] = [0.0; FFT_SIZE];
    let mut shy_output: [f32; FFT_SIZE] = [0.0; FFT_SIZE];
    let arm_output_ptr: *mut f32 = arm_output.as_mut_ptr();
    let shy_output_ptr: *mut f32 = shy_output.as_mut_ptr();

    unsafe {
        do_arm_fft(arm_input_ptr, arm_output_ptr);
        do_shy_fft(shy_input_ptr, shy_output_ptr);
    }
    let microfft_output = rfft_2048(&mut microfft_input);

    for i in 0..num_to_print {
        spew!("comp", i, "arm", arm_output[i], "shy", shy_output[i], "microfft", microfft_output[i].re, microfft_output[i].im);
    }
}
