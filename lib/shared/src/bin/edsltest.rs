#![allow(unused_imports)]

extern crate alloc;

use std::fs::File;
use std::io::{BufWriter, Write};
use alloc::rc::Rc;

use shared::edsl::wad::node::{Node, genericize, GNode};

fn build() {
    let input = Rc::new(Node::Input);
    let pt = Rc::new(Node::PassThru(input.clone()));
    let add = Rc::new(Node::Add(input.clone(), pt.clone()));
    let sf = Rc::new(Node::SumFilter(add.clone(), -1, 1));
    let sf2 = Rc::new(Node::SumFilter(add.clone(), -3, 3));
    let sfadd = Rc::new(Node::Add(sf.clone(), sf2.clone()));
    let out = sfadd;
    let groot = genericize(&out);
    groot.borrow_mut().make_causal();
    groot.borrow_mut().number_nodes();
    groot.borrow().dump();

    let f = File::create("src/filter/edsl_nodey.rs").expect("Unable to create file");
    let mut f = BufWriter::new(f);
    f.write_all(GNode::generate(&mut groot.borrow(), "EdslNodey").as_bytes()).unwrap();
}

fn main() {
    build();
    println!("hi edsl");
}
