extern crate alloc;

use alloc::boxed::Box;
use alloc::vec;
#[allow(unused)]
use alloc::vec::Vec;

#[allow(unused_imports)]
use shared::bench::benchmark;
#[allow(unused_imports)]
use shared::fft_bench::*;
#[allow(unused_imports)]
use shared::filter::chorus::*;
#[allow(unused_imports)]
use shared::filter::guitar_synth::*;
#[allow(unused_imports)]
use shared::filter::harmoneer::*;
#[allow(unused_imports)]
use shared::filter::high_pass::*;
#[allow(unused_imports)]
use shared::filter::interp::*;
#[allow(unused_imports)]
use shared::filter::linear_vibrato::*;
#[allow(unused_imports)]
use shared::filter::low_pass::*;
#[allow(unused_imports)]
use shared::filter::mixer::*;
#[allow(unused_imports)]
use shared::filter::pass_thru::*;
#[allow(unused_imports)]
use shared::filter::reso::*;
#[allow(unused_imports)]
use shared::filter::seq::*;
#[allow(unused_imports)]
use shared::filter::sweep::SweepFilter;
use shared::knob::Knobs;
use shared::knob_board::*;
use shared::patch::Patch;
use shared::rig::*;
use shared::rig_board::*;
use shared::sdram::*;
#[cfg(not(feature = "for_host"))]
use shared::daisy_seed_board::*;
use shared::constants::*;
use shared::load_board::*;
use shared::r#override::*;
use shared::spew::*;
use shared::switch::Toggle;
use shared::switch_board::*;
use shared::test::test_direct;

#[allow(dead_code)]
fn harmoneer(sdram: &mut SDRAM) -> Box<dyn Patch> {
    #[allow(unused)]
    let orig = PassThruFilter {};
    #[allow(unused)]
    let h0 = Harmoneer::new(2.0, sdram);
    #[allow(unused)]
    let h1 = Harmoneer::new(0.5, sdram);
    let channels = vec![
        MixerChannel(1.0, Box::new(orig)),
        MixerChannel(1.0, Box::new(h0)),
        MixerChannel(1.0, Box::new(h1)),
    ];
    let mixer = Mixer::new(channels);
    Box::new(mixer)
}

#[allow(dead_code)]
fn much_harm_main() {
    hw_init(!PROD, BLOCK_SIZE);
    spew!("hi");
    load_init();

    let knobs = Box::new(BoardKnobs { });
    let switches = Box::new(BoardSwitches { });
    let toggle = Toggle::new(switches, 0);

    rig_install_callback();

    let mut sdram = SDRAM::new();
    let patch = shared::much_harm::much_harm(&mut sdram);

    rig_install_patch(patch, knobs, toggle);

    // TODO don't duplicate this.
    let knobs2 = Box::new(BoardKnobs { });
    let switches2 = Box::new(BoardSwitches { });
    let mut toggle2 = Toggle::new(switches2, 0);
    loop {
        rig_log();
        load_spew();
        knobs2.spew();
        toggle2.process();
        toggle2.spew();
        hw_delay(500);
    }
}

#[allow(dead_code)]
fn rubin_main() {
    hw_init(!PROD, BLOCK_SIZE);
    spew!("hi");
    load_init();

    let knobs = Box::new(BoardKnobs { });
    let switches = Box::new(BoardSwitches { });
    let toggle = Toggle::new(switches, 0);

    rig_install_callback();

    let mut sdram = SDRAM::new();
    let patch = shared::rubin::rubin(&mut sdram);

    rig_install_patch(patch, knobs, toggle);

    // TODO don't duplicate this.
    let knobs2 = Box::new(BoardKnobs { });
    let switches2 = Box::new(BoardSwitches { });
    let mut toggle2 = Toggle::new(switches2, 0);
    loop {
        rig_log();
        load_spew();
        knobs2.spew();
        toggle2.process();
        toggle2.spew();
        hw_delay(500);
    }
}

#[allow(dead_code)]
fn gs_main() {
    hw_init(!PROD, BLOCK_SIZE);
    spew!("hi");
    load_init();

    let patch = Box::new(GuitarSynth::new(1.0));

    let knobs = Box::new(BoardKnobs { });
    let switches = Box::new(BoardSwitches { });
    let toggle = Toggle::new(switches, 0);
    rig_install_patch(patch, knobs, toggle);

    rig_install_callback();

    loop {
        load_spew();
        hw_delay(500);
    }
}

#[allow(dead_code)]
fn rubin2_main_loop(switcher_knob: usize, is_switcher_low: bool) {
    let knobs = Box::new(BoardKnobs { });
    loop {
        let current_is_switcher_low = knobs.read(switcher_knob) < 0.5;
        if current_is_switcher_low != is_switcher_low {
            break;
        }
        load_spew();
        hw_delay(500);
    }
}

#[allow(dead_code)]
// orig rubin (harmoneer+) with gs, switched by knob 5
fn rubin2_main() {
    hw_init(!PROD, BLOCK_SIZE);
    spew!("hi");
    load_init();

    rig_install_callback();

    let switcher_knob = 5;

    loop {
        let knobs = Box::new(BoardKnobs { });
        let knobs2 = Box::new(BoardKnobs { });
        let switches = Box::new(BoardSwitches { });
        let toggle = Toggle::new(switches, 0);

        let is_switcher_low = knobs2.read(switcher_knob) < 0.5;
        spew!("setting patch", if is_switcher_low { "rubin" } else { "gs" });
        if is_switcher_low {
            let mut sdram = SDRAM::new();
            let patch = shared::rubin::rubin(&mut sdram);
            rig_install_patch(patch, knobs, toggle);
            rubin2_main_loop(switcher_knob, is_switcher_low);
            rig_deinstall_patch();
        } else {
            let patch = Box::new(GuitarSynth::new(5.0));
            rig_install_patch(patch, knobs, toggle);
            rubin2_main_loop(switcher_knob, is_switcher_low);
            rig_deinstall_patch();
        }
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

fn sdram_test() {
    let mut sdram = SDRAM::new();
    let a0 = sdram.alloc_slice(10);
    let a1 = sdram.alloc_slice(10);
    assert!(a0.as_ptr() == 0xc0000000 as *const f32);
    assert!(a1.as_ptr() == 0xc0000028 as *const f32);
    spew!("sdram", a0.as_ptr() as u64, a1.as_ptr() as u64);
}

#[allow(dead_code)]
fn all_tests() {
    hw_init(true, BLOCK_SIZE);
    test_main();
    override_test_main();
    sdram_test();
}

#[allow(dead_code)]
pub fn benchmark_fft() {
    hw_init(true, BLOCK_SIZE);
    let mut sdram = SDRAM::new();
    do_benchmark_fft(&mut sdram);
}

#[allow(dead_code)]
pub fn do_fft_output_comparison() {
    hw_init(true, BLOCK_SIZE);
    fft_output_comparison();
}

// TODO remove / not compiled in
pub mod study_mem {
    use alloc::boxed::Box;
    use shared::spew::*;
    pub fn study_mem() {
        crate::rustmain::hw_init(!crate::rustmain::PROD, crate::rustmain::BLOCK_SIZE);
        let mut count = 0;
        loop {
            let b = Box::new(12_u32);
            let p: *mut u32 = Box::<u32>::into_raw(b);
            spew!("alloc", count, hex(p as u64));
            //core::mem::forget(p);
            count += 1;
        }
    }

    /*
    //extern crate libc; // 0.2.65
    use cortex_m::interrupt::free;
    use core::ffi::c_void;
    use core::mem;

    pub fn study_mem() {
        crate::rustmain::hw_init(!crate::rustmain::PROD, crate::rustmain::BLOCK_SIZE);
        unsafe {
            let my_num: *mut i32 = libc::malloc(mem::size_of::<i32>() as libc::size_t) as *mut i32;
            if my_num.is_null() {
                panic!("failed to allocate memory");
            }
            libc::free(my_num as *mut libc::c_void);
        }
    }
    */
}

#[no_mangle]
pub fn main() {
    spew!("start of main");

    //rubin_main();
    //rubin2_main();
    much_harm_main();
    //gs_main();
    //study_mem::study_mem();
    //all_tests();
    //oom_test();
    //benchmark_fft();
    //do_fft_output_comparison();

    spew!("end of main");
}
