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
fn rubin() -> Box<dyn Patch> {
    let hp = Box::new(HighPassFilter::new());
    let lp = Box::new(LowPassFilter::new());
    let high_low = Box::new(Interp::new(BLOCK_SIZE, lp, hp, 2));

    let reso = Box::new(ResoFilter::new(0, 3));
    //let lvib = Box::new(LinearVibrato::new(400, 10.0, 1));
    let chorus = Box::new(Chorus::new());
    let both0 = Box::new(Seq::new(BLOCK_SIZE, chorus, reso));
    let triple = Box::new(Seq::new(BLOCK_SIZE, both0, high_low));
    //let sweep = Box::new(SweepFilter::example());

    triple
}

#[allow(dead_code)]
fn live_main() {
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
    let a0 = sdram.alloc(10);
    spew!("a0", hex(a0.as_ptr() as u64));
    let a1 = sdram.alloc(10);
    spew!("a1", hex(a1.as_ptr() as u64));

    let patch = harmoneer(&mut sdram);
    //let patch = rubin();

    let knobs = Box::new(BoardKnobs { });
    rig_install_patch(patch, knobs);

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

fn sdram_test() {
    let mut sdram = SDRAM::new();
    let a0 = sdram.alloc(10);
    let a1 = sdram.alloc(10);
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
pub fn benchmark_fft() {
    hw_init(true, BLOCK_SIZE);
    do_benchmark_fft();
}

#[no_mangle]
pub fn main() {
    spew!("start of main");

    //live_main();
    all_tests();
    //try_knobs();
    //oom_test();
    //benchmark_fft();

    spew!("end of main");
}
