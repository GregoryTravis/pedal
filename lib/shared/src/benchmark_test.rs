extern crate alloc;

use crate::bench::benchmark;
use crate::spew::*;
use crate::test_cases::*;
use crate::test::*;

/*
 * Unit tests for patches.
 */

pub fn benchmark_direct() {
    for test_case in get_test_cases() {
        let mut patch = test_case.patch;
        let canned_input = test_case.canned_input;
        let results = benchmark(1.0, &mut || {
            run_patch_direct(&mut patch, canned_input);
        });
        spew!("bench", test_case.name,
            results.requested_duration, results.duration,
            results.execution_count, results.avg_time, results.per_second);
    }
}
