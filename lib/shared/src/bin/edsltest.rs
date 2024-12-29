#![allow(unused_imports)]

extern crate alloc;

use alloc::rc::Rc;

use shared::edsl::wad::node::{Node, compile};

fn build_edsl_nodey() {
    let input = Rc::new(Node::Input);
    let pt = Rc::new(Node::PassThru(input.clone()));
    let add = Rc::new(Node::Add(input.clone(), pt.clone()));
    let sf = Rc::new(Node::SumFilter(add.clone(), -1, 1));
    let sf2 = Rc::new(Node::SumFilter(add.clone(), -3, 3));
    let sfadd = Rc::new(Node::Add(sf.clone(), sf2.clone()));
    let out = sfadd;
    compile(&out, "src/filter/edsl_nodey.rs", "EdslNodey");
}

fn build_edsl_high_pass() {
    let input = Rc::new(Node::Input);
    let out = Rc::new(Node::HighPass(input.clone()));
    compile(&out, "src/filter/edsl_high_pass.rs", "EdslHighPass");
}

fn build_edsl_low_pass() {
    let input = Rc::new(Node::Input);
    let out = Rc::new(Node::LowPass(input.clone()));
    compile(&out, "src/filter/edsl_low_pass.rs", "EdslLowPass");
}

fn build_edsl_pass_thru() {
    let input = Rc::new(Node::Input);
    let out = Rc::new(Node::PassThru(input.clone()));
    compile(&out, "src/filter/edsl_pass_thru.rs", "EdslPassThru");
}

fn main() {
    build_edsl_nodey();
    build_edsl_high_pass();
    build_edsl_low_pass();
    build_edsl_pass_thru();
    println!("hi edsl");
}
