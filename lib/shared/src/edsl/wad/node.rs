#![allow(dead_code)]

extern crate std;

use alloc::rc::Rc;
use alloc::vec::Vec;

use crate::edsl::runtime::range::Range;

#[derive(Debug)]
pub enum Node {
    Input,
    PassThru(Rc<Node>),
    Add(Rc<Node>, Rc<Node>),
}

#[derive(Copy, Clone, Debug)]
pub struct Port {
    range: Range,
    main_sample: isize,
}

impl Port {
    pub fn translate(&mut self, x: isize) {
        self.range.translate(x);
        self.main_sample += x;
    }
}

// Genericized node.
#[derive(Debug)]
pub struct GNode {
    node: Rc<Node>,
    inputs: Vec<Rc<GNode>>,
    ports: Vec<Port>,
}

impl GNode {
    pub fn make_causal(&mut self) {
        let futurest: isize = self.ports.iter().map(|p| p.range.1).fold(std::isize::MIN, |a, b| a.max(b));
        for port in &mut self.ports {
            port.translate(-futurest);
        }
    }
}

pub fn genericize(node: &Rc<Node>) -> GNode {
    match &**node {
        Node::Input => GNode {
            node: (*node).clone(),
            inputs: vec![],
            ports: vec![],
        },
        Node::PassThru(inn) => GNode {
            node: (*node).clone(),
            inputs: vec![
                Rc::new(genericize(&inn)),
            ],
            ports: vec![
                Port {
                    range: Range::empty(),
                    main_sample: 0,
                },
            ]
        },
        Node::Add(a, b) => GNode {
            node: (*node).clone(),
            inputs: vec![
                Rc::new(genericize(&a)),
                Rc::new(genericize(&b)),
            ],
            ports: vec![
                Port {
                    range: Range::empty(),
                    main_sample: 0,
                },
                Port {
                    range: Range::empty(),
                    main_sample: 0,
                },
            ]
        },
    }
}

/*

- add main sample field
- make claims causal; add causality delay
- make main samples line up (try rtl, then ltr if that fails); add main sample delay
- combine claims to get stream window sizes
- generate signals, windows, prim calls

*/
