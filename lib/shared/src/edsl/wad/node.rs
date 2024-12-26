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

    pub fn type_name(&self) -> &'static str {
        match self {
            Node::Input => "f32",
            Node::PassThru(inn) => inn.type_name(),
            Node::Add(a, b) => same_type(a.type_name(), b.type_name()),
            Node::SumFilter(inn, _, _) => inn.type_name(),
        }
    }
}

pub fn same_type<'a, 'b>(a: &'a str, b: &'b str) -> &'a str {
    assert!(a == b);
    a
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
#[derive(Clone, Debug)]
pub struct GNode {
    index: u32,
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
            index: self.index,
            node: self.node.clone(),
            inputs: self.inputs.iter().map(|input| Rc::new(input.make_causal())).collect(),
            ports: self.ports.iter().map(|port| {
                let translated = port.translate(-futurest);
                println!("Translate {} {:?} {:?} {:?}", futurest, self.node.shew(), port, translated);
                translated
            }).collect(),
        }
    }

    pub fn traverse_each<F>(&self, f: &F)
    where F: Fn(&GNode) {
        f(self);
        for input in &self.inputs {
            input.traverse_each(f);
        }
    }

    // Mutable in the sense of mutating the closure.
    pub fn traverse_mut<F>(&self, f: &mut F) -> GNode
    where F: FnMut(&GNode) -> GNode {
        let new_self = f(self);
        GNode {
            inputs: self.inputs.iter().map(|gnode| Rc::new(gnode.traverse_mut(f))).collect(),
            ..new_self
        }
    }

    pub fn number_nodes(&self) -> GNode {
        let mut serial = 0;
        let mut numberer = |gnode: &GNode| {
            let next = serial;
            serial += 1;
            GNode {
                index: next,
                ..(*gnode).clone()
            }
        };
        self.traverse_mut(&mut numberer)
    }

    pub fn shew(&self) -> String {
        format!("{} {:?} {:?}", self.node.shew(), self.inputs, self.ports)
    }

    pub fn dump(&self) {
        self.traverse_each(&|gnode| {
            println!("TR {}", gnode.shew());
        });
    }

    /*
    pub fn traverse<F, R>(&mut self, f: F, r: R) -> R
    where F: Fn(&mut GNode, R) -> R {
        let mut r_new = f(self, r);
        for input in &self.inputs {
            r_new = input.traverse(f, r_new);
        }
        r_new
    }
    */
}

pub fn genericize(node: &Rc<Node>) -> GNode {
    match &**node {
        Node::Input => GNode {
            index: 0,
            node: (*node).clone(),
            inputs: vec![],
            ports: vec![],
        },
        Node::PassThru(inn) => GNode {
            index: 0,
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
            index: 0,
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
            index: 0,
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
  - signal decl type
  - window type (from signal)
  - window size

*/
