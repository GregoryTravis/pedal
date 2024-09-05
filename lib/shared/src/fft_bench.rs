use crate::bench::benchmark;
use crate::constants::*;
use crate::fft::*;
use crate::spew::*;

#[allow(dead_code)]
pub fn do_benchmark_fft() {
    let mut input: [f32; FFT_SIZE] = [0.0; FFT_SIZE];
    let mut fft_buffer: [f32; FFT_SIZE] = [0.0; FFT_SIZE];
    let mut output: [f32; FFT_SIZE] = [0.0; FFT_SIZE];

    let dur = 1.0;
    let bench = benchmark(dur, || {
        fft(&mut input, &mut fft_buffer);
        ifft(&mut fft_buffer, &mut output);
    });
    spew!("fft", bench.execution_count, bench.avg_time);
}
