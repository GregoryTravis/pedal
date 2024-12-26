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

// Genericized node.
#[derive(Debug)]
pub struct GNode {
    _node: Rc<Node>,
    _inputs: Vec<Rc<GNode>>,
    _ports: Vec<Port>,
}

impl GNode {
    /*
    fn make_causal(&mut self) {
    }
    */
}

pub fn genericize(node: &Rc<Node>) -> GNode {
    match &**node {
        Node::Input => GNode {
            _node: (*node).clone(),
            _inputs: vec![],
            _ports: vec![],
        },
        Node::PassThru(inn) => GNode {
            _node: (*node).clone(),
            _inputs: vec![
                Rc::new(genericize(&inn)),
            ],
            _ports: vec![
                Port {
                    range: Range::empty(),
                    main_sample: 0,
                },
            ]
        },
        Node::Add(a, b) => GNode {
            _node: (*node).clone(),
            _inputs: vec![
                Rc::new(genericize(&a)),
                Rc::new(genericize(&b)),
            ],
            _ports: vec![
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
