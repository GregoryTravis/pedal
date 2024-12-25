extern crate std;

use alloc::rc::Rc;
use alloc::vec::Vec;

#[derive(Debug)]
pub enum Node {
    Input,
    PassThru(Rc<Node>),
    Add(Rc<Node>, Rc<Node>),
}

// Genericized node.
#[derive(Debug)]
pub struct GNode {
    _node: Rc<Node>,
    _inputs: Vec<Rc<GNode>>,
}

impl GNode {
    pub fn new(node: Rc<Node>, inputs: Vec<Rc<GNode>>) -> GNode {
        GNode {
            _node: node,
            _inputs: inputs,
        }
    }
}

pub fn genericize(node: &Rc<Node>) -> GNode {
    match &**node {
        Node::Input => GNode::new((*node).clone(), vec![]),
        Node::PassThru(inn) => GNode::new((*node).clone(), vec![Rc::new(genericize(&inn))]),
        Node::Add(a, b) => GNode::new((*node).clone(), vec![Rc::new(genericize(&a)), Rc::new(genericize(&b))]),
    }
}

/*

- add main sample field
- make claims causal; add causality delay
- make main samples line up (try rtl, then ltr if that fails); add main sample delay
- combine claims to get stream window sizes
- generate signals, windows, prim calls

*/
