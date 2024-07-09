extern crate std;

use std::println;

// TEST_INPUT generated with:

    /*
    let mut input = vec![0.0; TEST_LEN];

    for i in 0..TEST_LEN {
        let freq: f32 = 440.0;
        let t = (i as f32) / (SAMPLE_RATE as f32);
        let ph = t * 2.0 * PI * freq;
        input[i] = libm::sinf(ph);
    }

    test_dump_as_source("TEST_INPUT", &input);
    */

pub fn test_dump_as_source(var: &str, a: &[f32]) {
    println!("pub const {}: &'static [f32] = &[", var);

    for i in 0..a.len() {
        println!("{:?},", a[i]);
    }

    println!("];");
}

/*
pub fn dump_test_data(name: &str, patch: Box<dyn Patch>) {
    for testinfo in testinfos {
        let mut output: Vec<f32> = vec![0.0; TEST_INPUT.len()];
        rig_run_patch_on_buffer(testinfo.patch, &TEST_INPUT, &mut output);

        test_dump_as_source(testinfo.name, &output);
    }
}
*/
