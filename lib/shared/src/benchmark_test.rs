extern crate alloc;

use crate::bench::benchmark;
use crate::spew::*;
use crate::test_cases::*;
use crate::test::*;

/*
 * Unit tests for patches.
 */

const BENCH_SIZE: usize = 100;

pub fn benchmark_direct() {
    let mut first: Option<f32> = None;

    for test_case in get_test_cases() {
        let mut patch = test_case.patch;
        let canned_input = test_case.canned_input;
        assert!(canned_input.len() >= BENCH_SIZE);
        let some_input = &canned_input[0..BENCH_SIZE];
        let results = benchmark(0.2, &mut || {
            run_patch_direct(&mut patch, some_input);
        });
        if first == None {
            first = Some(results.per_second);
        }
        let relative = first.unwrap() / results.per_second;
        spew!("bench", test_case.name,
            results.requested_duration, results.duration,
            results.execution_count, results.avg_time, results.per_second, relative);
    }
}
