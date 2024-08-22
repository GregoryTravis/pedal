extern crate alloc;

use alloc::boxed::Box;

use shared::bench::benchmark;
#[allow(unused_imports)]
use shared::filter::chorus::*;
#[allow(unused_imports)]
use shared::filter::high_pass::*;
#[allow(unused_imports)]
use shared::filter::interp::*;
#[allow(unused_imports)]
use shared::filter::linear_vibrato::*;
#[allow(unused_imports)]
use shared::filter::low_pass::*;
#[allow(unused_imports)]
use shared::filter::reso::*;
#[allow(unused_imports)]
use shared::filter::seq::*;
use shared::knob::Knobs;
use shared::knob_board::*;
use shared::rig::*;
use shared::rig_board::*;
#[cfg(not(feature = "for_host"))]
use shared::daisy_seed_board::*;
use shared::constants::*;
use shared::load_board::*;
use shared::r#override::*;
use shared::spew::*;
use shared::test::test_direct;

extern "C" {
    pub fn do_arm_fft();
    pub fn do_shy_fft();
}

#[allow(dead_code)]
fn live_main() {
    hw_init(!PROD, BLOCK_SIZE);
    spew!("hi");
    load_init();

    let hp = Box::new(HighPassFilter::new());
    let lp = Box::new(LowPassFilter::new());
    let high_low = Box::new(Interp::new(BLOCK_SIZE, lp, hp, 2));

    let reso = Box::new(ResoFilter::new(0, 3));
    //let lvib = Box::new(LinearVibrato::new(400, 10.0, 1));
    let chorus = Box::new(Chorus::new());
    let both0 = Box::new(Seq::new(BLOCK_SIZE, chorus, reso));
    let triple = Box::new(Seq::new(BLOCK_SIZE, both0, high_low));
    let knobs = Box::new(BoardKnobs { });
    rig_install_patch(triple, knobs);

    rig_install_callback();

    // TODO don't duplicate this.
    let knobs2 = Box::new(BoardKnobs { });
    loop {
        rig_log();
        load_spew();
        knobs2.spew();
        hw_delay(500);
    }
}

#[allow(dead_code)]
fn override_test_main() {
    // TODO move interrupt install here, once it's possible to do that with no patch installed.
    run_override_test();
}

#[allow(dead_code)]
fn test_main() {
    test_direct();
}

// Turn on logging in mem to see this in action.
#[allow(dead_code)]
pub fn oom_test() {
    hw_init(true, BLOCK_SIZE);
    loop {
        let _foo = Box::new([0.0; 4*32]);
        Box::leak(_foo);
    }
}

#[allow(dead_code)]
fn all_tests() {
    hw_init(true, BLOCK_SIZE);
    test_main();
    override_test_main();
}

#[allow(dead_code)]
fn try_knobs() {
    hw_init(true, BLOCK_SIZE);
    let knobs = BoardKnobs { };
    loop {
        knobs.process();
        knobs.spew();
        spew!("knobs", knobs.read(0), knobs.read(1), knobs.read(2), knobs.read(3), knobs.read(4), knobs.read(5));
        hw_delay(200);
    }
}

#[allow(dead_code)]
fn benchmark_fft() {
    hw_init(true, BLOCK_SIZE);

    let dur = 1.0;
    let arm_bench = benchmark(dur, || {
        unsafe {
            do_arm_fft();
        }
    });
    let shy_bench = benchmark(dur, || {
        unsafe {
            do_shy_fft();
        }
    });
    spew!("arm", arm_bench.execution_count, arm_bench.avg_time, "shy", shy_bench.execution_count, shy_bench.avg_time);
}

#[no_mangle]
pub fn main() {
    spew!("start of main");

    //live_main();
    //all_tests();
    //try_knobs();
    //oom_test();
    benchmark_fft();

    spew!("end of main");
}
