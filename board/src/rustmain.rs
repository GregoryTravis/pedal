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
fn rubin_main() {
    hw_init(!PROD, BLOCK_SIZE);
    spew!("hi");
    load_init();

    /*
    let mut sdram: SDRAM = SDRAM::new();
    let fooie: &[f32] = sdram.alloc(16);
    let ptr_int = fooie.as_ptr() as u32;
    spew!("got", ptr_int, fooie.len(), fooie[0]);
    */
    let mut sdram = SDRAM::new();
    let a0 = sdram.alloc_slice(10);
    spew!("a0", hex(a0.as_ptr() as u64));
    let a1 = sdram.alloc_slice(10);
    spew!("a1", hex(a1.as_ptr() as u64));

    //let patch = harmoneer(&mut sdram);
    let patch = shared::rubin::rubin(&mut sdram);

    let knobs = Box::new(BoardKnobs { });
    let switches = Box::new(BoardSwitches { });
    let toggle = Toggle::new(switches, 0);
    rig_install_patch(patch, knobs, toggle);

    rig_install_callback();

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

    let mut sdram = SDRAM::new();

    let patch = Box::new(GuitarSynth::new(&mut sdram));

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

#[no_mangle]
pub fn main() {
    spew!("start of main");

    //rubin_main();
    gs_main();
    //all_tests();
    //oom_test();
    //benchmark_fft();
    //do_fft_output_comparison();

    spew!("end of main");
}
