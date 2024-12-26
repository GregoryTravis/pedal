#![allow(dead_code)]

extern crate std;

use alloc::rc::Rc;
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;
use std::format;
use std::println;

use crate::edsl::runtime::range::Range;

#[derive(Debug)]
pub enum Node {
    Input,
    PassThru(Rc<Node>),
    Add(Rc<Node>, Rc<Node>),
    SumFilter(Rc<Node>, isize, isize),
}

impl Node {
    pub fn name(&self) -> &str {
        match self {
            Node::Input => "Input",
            Node::PassThru(_) => "PassThru",
            Node::Add(_, _) => "Add",
            Node::SumFilter(_, _, _) => "SumFilter",
        }
    }

    pub fn shew(&self) -> String {
        match self {
            Node::Input => "Input".to_string(),
            Node::PassThru(inn) => format!("PassThru({})", inn.name()),
            Node::Add(a, b) => format!("Add({}, {})", a.name(), b.name()),
            Node::SumFilter(inn, low, high) => format!("SumFilter({}, {}, {})", inn.name(), low, high),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Port {
    range: Range,
    main_sample: isize,
}

impl Port {
    pub fn translate(&self, x: isize) -> Port {
        Port {
            range: self.range.translate(x),
            main_sample: self.main_sample + x,
        }
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
    pub fn make_causal(&self) -> GNode {
        let futurest: isize = self.ports.iter().map(|p| p.range.1).fold(std::isize::MIN, |a, b| a.max(b));
        println!("Translate node {:?}", self.node);
        for port in &self.ports {
            println!("port {:?}", port);
        }
        println!("Translate node {:?} {}", self.node.shew(), futurest);
        GNode {
            node: self.node.clone(),
            inputs: self.inputs.iter().map(|input| Rc::new(input.make_causal())).collect(),
            ports: self.ports.iter().map(|port| {
                let translated = port.translate(-futurest);
                println!("Translate {} {:?} {:?} {:?}", futurest, self.node.shew(), port, translated);
                translated
            }).collect(),
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
        Node::SumFilter(inn, low, high) => GNode {
            node: (*node).clone(),
            inputs: vec![
                Rc::new(genericize(&inn)),
            ],
            ports: vec![
                Port {
                    range: Range(*low, *high),
                    main_sample: 0,
                },
            ]
        },
    }
}

/*

+ add main sample field
+ make claims causal; add causality delay
- make main samples line up (try rtl, then ltr if that fails); add main sample delay
- combine claims to get stream window sizes
- generate signals, windows, prim calls

*/
