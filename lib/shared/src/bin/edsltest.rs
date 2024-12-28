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

fn main() {
    build_edsl_nodey();
    println!("hi edsl");
}
