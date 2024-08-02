extern crate alloc;

use alloc::boxed::Box;
use alloc::sync::Arc;

use shared::filter::reso::*;
use shared::knob_board::*;
use shared::rig::*;
use shared::rig_board::*;
use shared::signal::base::*;
use shared::signal::combinators::*;
#[cfg(not(feature = "for_host"))]
use shared::daisy_seed_board::*;
use shared::constants::*;
use shared::load::*;
use shared::r#override::*;
use shared::spew::*;
use shared::test::test_direct;

#[allow(dead_code)]
fn live_main() {
    hw_init(true, BLOCK_SIZE);
    spew!("hi");
    load_init();

    let siner = PostCompose { signal: Arc::new(Sin {}), ff: scale_range(0.3, 0.9) };
    let q = Const { x: 0.95 };
    rig_install_patch(Box::new(ResoFilter::new(Arc::new(siner), Arc::new(q))));

    rig_install_callback();

    loop {
        rig_log();
        load_spew();
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
    loop {
        knob_process();
        spew!("knob", knob_get_value(0), knob_get_value(1), knob_get_value(2), knob_get_value(3), knob_get_value(4), knob_get_value(5));
        hw_delay(200);
    }
}

#[no_mangle]
pub fn main() {
    spew!("start of main");

    //live_main();
    all_tests();
    //try_knobs();
    //oom_test();

    spew!("end of main");
}
