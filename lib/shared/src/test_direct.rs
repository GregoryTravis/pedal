extern crate alloc;

use crate::testdata::*;
use crate::test_cases::*;
use crate::test::*;
//#[cfg(feature = "for_host")]

/*
 * Unit tests for patches.
 */

pub fn test_direct() {
    if DO_DUMP {
        local_test_dump_as_source("TEST_INPUT", &TEST_INPUT);
        local_test_dump_as_source("LONG_TEST_INPUT", &LONG_TEST_INPUT);
    }

    for test_case in get_test_cases() {
        let patch = test_case.patch;
        let canned_input = test_case.canned_input;
        let expected_output = test_case.expected_output;
        //spew!(test_case.name);
        test_patch(test_case.name, patch, canned_input, expected_output);
    }
}
