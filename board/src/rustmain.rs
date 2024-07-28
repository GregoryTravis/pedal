extern crate alloc;

use alloc::boxed::Box;
use alloc::sync::Arc;

use shared::filter::reso::*;
use shared::rig::*;
use shared::signal::base::*;
use shared::signal::combinators::*;
use crate::daisy_seed::*;
use shared::constants::*;
use shared::load::*;
use shared::override::*;
use shared::spew::*;
use shared::test::test_reso;

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
    hw_init(true, BLOCK_SIZE);
    // TODO move interrupt install here, once it's possible to do that with no patch installed.
    run_override_test();
}

#[allow(dead_code)]
fn test_main() {
    hw_init(true, BLOCK_SIZE);
    spew!("reso test running");
    test_reso();
    spew!("reso test running");
    test_reso();
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

#[no_mangle]
pub fn main() {
    spew!("start of main");

    //live_main();
    //test_main();
    override_test_main();
    //oom_test();

    spew!("end of main");
}
