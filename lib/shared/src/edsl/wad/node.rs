#![allow(dead_code)]

extern crate std;

use alloc::borrow::ToOwned;
use alloc::rc::Rc;
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;
use core::cmp::{Eq, PartialEq};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::format;
use std::hash::Hash;
use std::println;

use crate::edsl::runtime::range::Range;

#[derive(PartialEq, Eq, Hash, Debug)]
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

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
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
    pub index: u32,
    pub node: Rc<Node>,
    inputs: Vec<Rc<RefCell<GNode>>>,
    ports: Vec<Port>,
}

impl GNode {
    pub fn trav<F>(&self, f: &F)
    where F: Fn(&GNode) {
        let mut hs = HashSet::new();
        self.trav1(f, &mut hs);
    }

    pub fn trav1<F>(&self, f: &F, hs: &mut HashSet<Rc<Node>>)
    where F: Fn(&GNode) {
        if hs.contains(&self.node) {
            return;
        }

        hs.insert(self.node.clone());

        f(self);
        for input in &self.inputs {
            input.borrow().trav1(f, hs);
        }
    }

    pub fn travm<F>(&mut self, f: &F)
    where F: Fn(&mut GNode) {
        let mut hs = HashSet::new();
        self.travm1(f, &mut hs);
    }

    pub fn travm1<F>(&mut self, f: &F, hs: &mut HashSet<Rc<Node>>)
    where F: Fn(&mut GNode) {
        if hs.contains(&self.node) {
            return;
        }

        hs.insert(self.node.clone());

        f(self);
        for input in &self.inputs {
            input.borrow_mut().travm1(f, hs);
        }
    }

    pub fn trav_mut<F>(&self, f: &mut F)
    where F: FnMut(&GNode) {
        let mut hs = HashSet::new();
        self.trav_mut1(f, &mut hs);
    }

    pub fn trav_mut1<F>(&self, f: &mut F, hs: &mut HashSet<Rc<Node>>)
    where F: FnMut(&GNode) {
        if hs.contains(&self.node) {
            return;
        }

        hs.insert(self.node.clone());

        f(self);
        for input in &self.inputs {
            input.borrow_mut().trav_mut1(f, hs);
            //f(&mut *input.borrow_mut());
        }
    }

    pub fn travm_mut<F>(&mut self, f: &mut F)
    where F: FnMut(&mut GNode) {
        let mut hs = HashSet::new();
        self.travm_mut1(f, &mut hs);
    }

    pub fn travm_mut1<F>(&mut self, f: &mut F, hs: &mut HashSet<Rc<Node>>)
    where F: FnMut(&mut GNode) {
        if hs.contains(&self.node) {
            return;
        }

        hs.insert(self.node.clone());

        f(self);
        for input in &self.inputs {
            input.borrow_mut().travm_mut1(f, hs);
            //f(&mut *input.borrow_mut());
        }
    }

    fn make_causal_me(&mut self) {
        //println!("mc {}", self.node.shew());
        let futurest: isize = self.ports.iter().map(|p| p.range.1).fold(std::isize::MIN, |a, b| a.max(b));
        for port in &mut self.ports {
            //let orig = port.clone();
            *port = port.translate(-futurest);
            //println!("mc {} {:?} {:?}", self.node.shew(), orig, port);
        }
    }

    pub fn make_causal(&mut self) {
        self.travm(&|gn: &mut GNode| gn.make_causal_me());
    }

    pub fn number_nodes(&mut self) {
        let mut serial = 0;
        let mut numberer = |gnode: &mut GNode| {
            let next = serial;
            serial += 1;
            //println!("nn {} {}", gnode.index, next);
            gnode.index = next;
        };
        self.travm_mut(&mut numberer)
    }

    pub fn shew(&self) -> String {
        format!("{} {}", self.index, self.node.shew())
    }

    pub fn dump(&self) {
        self.trav(&|gn: &GNode| {
            println!("{}", gn.shew());
            for input in &gn.inputs {
                println!("  {}", input.borrow().shew());
            }
        });
    }

    fn generate_struct(&self, name: &str) -> String {
        let mut acc: String = "".to_owned();
        acc.push_str(&format!("pub struct {} {{\n", name).to_owned());
        self.trav_mut(&mut |gn: &GNode| {
            acc.push_str(&format!("    signal{}: Signal<{}>,\n", gn.index, gn.node.type_name()).to_owned());
        });
        acc.push_str("}\n");
        acc
    }

    fn generate_impl(&self, name: &str) -> String {
        let mut acc: String = "".to_owned();
        let mut acc_lines: String = "".to_owned();

        self.trav_mut(&mut |gn: &GNode| {
            acc_lines.push_str(&format!("    signal{}: Signal::new(MAX),\n", gn.index));
        });

        acc.push_str(&format!(
            r#"
impl {} {{
    pub fn new() -> {} {{
        {} {{
            {}
        }}
    }}
}}
"#,
                name, name, name, acc_lines));

        acc
    }

    pub fn generate(&self, name: &str) -> String {
        let mut acc: String = "".to_owned();
        acc.push_str("const MAX: usize = 10;\n\n");
        acc.push_str(&self.generate_struct(name));
        acc.push_str(&self.generate_impl(name));
        acc
    }
}

pub fn genericize(node: &Rc<Node>) -> Rc<RefCell<GNode>> {
    let mut hm = HashMap::new();
    genericize1(node, &mut hm)
}

pub fn genericize1(node: &Rc<Node>, hm: &mut HashMap<Rc<Node>, Rc<RefCell<GNode>>>) -> Rc<RefCell<GNode>> {
    if hm.contains_key(node) {
        hm.get(node).unwrap().clone()
    } else {
        let gn = match &**node {
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
                    genericize1(&inn, hm),
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
                    genericize1(&a, hm),
                    genericize1(&b, hm),
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
                    genericize1(&inn, hm),
                ],
                ports: vec![
                    Port {
                        range: Range(*low, *high),
                        main_sample: 0,
                    },
                ]
            },
        };
        let gnrc = Rc::new(RefCell::new(gn));
        hm.insert(node.clone(), gnrc.clone());
        gnrc
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
