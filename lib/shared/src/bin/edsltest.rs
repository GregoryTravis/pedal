#![allow(unused_imports)]

extern crate alloc;

use std::fs::File;
use std::io::{BufWriter, Write};
use alloc::rc::Rc;

use shared::edsl::wad::node::{Node, genericize, GNode};

fn build() {
    let input = Rc::new(Node::Input);

    /*
    let a = Rc::new(Node::PassThru(input.clone()));
    let b = Rc::new(Node::PassThru(input.clone()));
    let c = b.clone();
    println!("{} {} {}", Rc::ptr_eq(&a, &b), Rc::ptr_eq(&b, &c), Rc::ptr_eq(&a, &c));
    */

    let pt = Rc::new(Node::PassThru(input.clone()));
    let add = Rc::new(Node::Add(input.clone(), pt.clone()));
    let sf = Rc::new(Node::SumFilter(add.clone(), -1, 1));
    let sf2 = Rc::new(Node::SumFilter(add.clone(), -3, 3));
    let sfadd = Rc::new(Node::Add(sf.clone(), sf2.clone()));
    //let out = sfadd;
    let out = sfadd;
    let groot = genericize(&out);
    groot.borrow_mut().make_causal();
    groot.borrow_mut().number_nodes();
    groot.borrow().dump();

    //println!("{}", GNode::generate(&mut groot.borrow(), "NodeyPatch"));
    let f = File::create("src/bin/edslpatch.rs").expect("Unable to create file");
    let mut f = BufWriter::new(f);
    f.write_all(GNode::generate(&mut groot.borrow(), "NodeyPatch").as_bytes()).unwrap();
}

fn main() {
    /*
    let f = File::create("src/bin/edslpatch_old.rs").expect("Unable to create file");
    let mut f = BufWriter::new(f);
    f.write_all(S0.as_bytes()).expect("Unable to write data");
    f.write_all(S1.as_bytes()).expect("Unable to write data");
    */
    build();
    println!("hi edsl");
}

const S0: &str = r#"
extern crate alloc;
extern crate libm;

use alloc::boxed::Box;

use shared::edsl::runtime::{signal::Signal, window::Window, range::Range, prim::{add, pass_thru, sum_filter}};
use shared::knob::Knobs;
use shared::patch::Patch;
use shared::playhead::Playhead;
use shared::test::*;

// Generate this; rename it.
const MAX: usize = 10;

pub struct EdslPatch {
    input_signal: Signal<f32>,
    signal0: Signal<f32>,
    signal1: Signal<f32>,
    output_signal: Signal<f32>,
}

impl EdslPatch {
    pub fn new() -> EdslPatch {
        EdslPatch {
            input_signal: Signal::new(MAX),
            signal0: Signal::new(MAX),
            signal1: Signal::new(MAX),
            output_signal: Signal::new(MAX),
        }
    }
}

impl Patch for EdslPatch {
    fn rust_process_audio(
        &mut self,
        input_slice: &[f32],
        output_slice: &mut [f32],
        _knobs: &Box<dyn Knobs>,
        mut playhead: Playhead,
    ) {
        for i in 0..input_slice.len() {
            self.input_signal.write(input_slice[i]);

            let pass_thru_0: Window<f32> = Window::new(&self.input_signal, Range(-3, 0));

            pass_thru(&pass_thru_0, &mut self.signal0);

            let add_0: Window<f32> = Window::new(&self.input_signal, Range(-2, 0));
            let add_1: Window<f32> = Window::new(&self.signal0, Range(-1, 0));
            add(&add_0, &add_1, &mut self.signal1);

            let sum_filter_0: Window<f32> = Window::new(&self.signal1, Range(-2, 0));
            sum_filter(&sum_filter_0, &mut self.output_signal);

            output_slice[i] = self.output_signal.read(0);

            playhead.inc();
        }
"#;

const S1: &str = r#"
    }
}

pub const INPUT: &'static [f32] = &[
0.0,
0.057564028,
0.11493716,
0.1719291,
];

pub const EDSL_PATCH_OUTPUT: &'static [f32] = &[
    0.0,
    0.115128055,
    0.34500235,
    0.68886054,
];

pub fn main() {
    let patch = Box::new(EdslPatch::new());
    let test_case = Box::new(TestCase {
            name: "edsl_patch",
            patch: patch,
            canned_input: INPUT,
            expected_output: EDSL_PATCH_OUTPUT,
        });
    test_patch(test_case.name, test_case.patch, test_case.canned_input, test_case.expected_output);
}
"#;
