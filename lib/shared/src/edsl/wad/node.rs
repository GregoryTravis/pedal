#![allow(dead_code)]

extern crate std;

use alloc::borrow::ToOwned;
use alloc::rc::Rc;
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;
use ordered_float::OrderedFloat;
use core::cmp::{Eq, PartialEq};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::format;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::hash::Hash;
use std::println;

use crate::edsl::runtime::range::Range;
//use crate::spew::*;

const PATCH_LOGGING: bool = false;

pub fn input() -> Rc<Node> {
    Rc::new(Node::Input)
}

pub fn pass_thru(x: &Rc<Node>) -> Rc<Node> {
    Rc::new(Node::PassThru(x.clone()))
}

pub fn add(a: &Rc<Node>, b: &Rc<Node>) -> Rc<Node> {
    Rc::new(Node::Add(a.clone(), b.clone()))
}

pub fn sum_filter(x: &Rc<Node>, low: isize, high: isize) -> Rc<Node> {
    Rc::new(Node::SumFilter(x.clone(), low, high))
}

pub fn high_pass(x: &Rc<Node>) -> Rc<Node> {
    Rc::new(Node::HighPass(x.clone()))
}

pub fn low_pass(x: &Rc<Node>) -> Rc<Node> {
    Rc::new(Node::LowPass(x.clone()))
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum Node {
    Input,
    Const(OrderedFloat<f32>),
    PassThru(Rc<Node>),
    Add(Rc<Node>, Rc<Node>),
    SumFilter(Rc<Node>, isize, isize),
    HighPass(Rc<Node>),
    LowPass(Rc<Node>),
    // max_sample_deviation, vibrato_frequency, input
    LinearVibrato(usize, Rc<Node>, Rc<Node>),
}

impl Node {
    pub fn name(&self) -> &str {
        match self {
            Node::Input => "Input",
            Node::Const(_) => "Const",
            Node::PassThru(_) => "PassThru",
            Node::Add(_, _) => "AddPrim",
            Node::SumFilter(_, _, _) => "SumFilter",
            Node::HighPass(_) => "HighPass",
            Node::LowPass(_) => "LowPass",
            Node::LinearVibrato(_, _, _) => "LinearVibrato",
        }
    }

    pub fn shew(&self) -> String {
        match self {
            Node::Input => "Input".to_string(),
            Node::Const(k) => format!("Const({})", k),
            Node::PassThru(inn) => format!("PassThru({})", inn.name()),
            Node::Add(a, b) => format!("Add({}, {})", a.name(), b.name()),
            Node::SumFilter(inn, low, high) => format!("SumFilter({}, {}, {})", inn.name(), low, high),
            Node::HighPass(inn) => format!("HighPass({})", inn.name()),
            Node::LowPass(inn) => format!("LowPass({})", inn.name()),
            Node::LinearVibrato(max_sample_deviation, vibrato_frequency, inn) => format!("LinearVibrato({}, {}, {})", max_sample_deviation, vibrato_frequency.name(), inn.name()),
        }
    }

    pub fn prim_struct_name(&self) -> &str {
        match self {
            Node::Input => "vwarlar",
            Node::Const(_) => "Const",
            Node::PassThru(_) => "PassThru",
            Node::Add(_, _) => "AddPrim",
            Node::SumFilter(_, _, _) => "SumFilter",
            Node::HighPass(_) => "HighPass",
            Node::LowPass(_) => "LowPass",
            Node::LinearVibrato(_, _, _) => "LinearVibrato",
        }
    }

    pub fn type_name(&self) -> &'static str {
        match self {
            Node::Input => "f32",
            Node::Const(_) => "f32",
            Node::PassThru(inn) => inn.type_name(),
            Node::Add(a, b) => same_type(a.type_name(), b.type_name()),
            Node::SumFilter(inn, _, _) => inn.type_name(),
            Node::HighPass(inn) => inn.type_name(),
            Node::LowPass(inn) => inn.type_name(),
            Node::LinearVibrato(_, _, inn) => inn.type_name(),
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
    ctor_args: Vec<String>,
    inputs: Vec<Rc<RefCell<GNode>>>,
    ports: Vec<Port>,
}

// Step: ports, ctor args, struct name, index
// Port: index, range, type
#[derive(Debug)]
pub struct Step(Vec<(u32,Range,String)>,Vec<String>,String,u32);

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

    pub fn rtrav_mut<F>(&self, f: &mut F)
    where F: FnMut(&GNode) {
        let mut hs = HashSet::new();
        self.rtrav_mut1(f, &mut hs);
    }

    pub fn rtrav_mut1<F>(&self, f: &mut F, hs: &mut HashSet<Rc<Node>>)
    where F: FnMut(&GNode) {
        if hs.contains(&self.node) {
            return;
        }

        hs.insert(self.node.clone());

        for input in &self.inputs {
            input.borrow_mut().rtrav_mut1(f, hs);
            //f(&mut *input.borrow_mut());
        }
        f(self);
    }

    // Breath-first, so it's in dependency order.
    pub fn dep_trav_mut<F>(&self, f: &mut F)
    where F: FnMut(&GNode) {
        let mut hs = HashSet::new();
        self.dep_trav_mut1(f, &mut hs);
        f(self);
    }

    pub fn dep_trav_mut1<F>(&self, f: &mut F, hs: &mut HashSet<Rc<Node>>)
    where F: FnMut(&GNode) {
        for input in &self.inputs {
            input.borrow_mut().dep_trav_mut1(f, hs);

            if !hs.contains(&input.borrow().node) {
                hs.insert(input.borrow().node.clone());
                f(&*input.borrow());
            }
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

    /*
    // TODO creating new refcells means its not really shared
    pub fn find_nodes<P>(&self, pred: P) -> Vec<Rc<RefCell<GNode>>>
    where P: Fn(&GNode) -> bool {
        let mut vec = Vec::new();
        self.trav(&|gn: &GNode| {
            if pred(gn) {
                vec.push(Rc::new(RefCell::new(*gn)));
            }
        });
        vec
    }

    pub fn find_node<P>(&self, pred: P) -> Rc<RefCell<GNode>>
    where P: Fn(&GNode) -> bool {
        let vec = self.find_nodes(pred);
        assert!(vec.len() == 1);
        vec[0]
    }

    pub fn get_input_slice(&self) -> Rc<RefCell<GNode>> {
        self.find_node(|gn: &GNode| {
            // TODO shouldn't need match here
            match gn.node {
                Input => true,
                _ => false,
            }
        })
    }
    */

    // TODO doing this because find is hard
    pub fn get_input_slice_index(&self) -> Option<u32> {
        let mut index: Option<u32> = None;
        self.trav_mut(&mut |gn: &GNode| {
            match *gn.node {
                Node::Input => {
                    assert!(index == None);
                    index = Some(gn.index);
                },
                _ => {}
            }
        });
        index
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
            for (input, port) in gn.inputs.iter().zip(&gn.ports) {
                println!("  {} : {:?}", input.borrow().shew(), port);
            }
        });
    }

    fn generate_struct(&self, name: &str, steps: &Vec<Step>) -> String {
        let mut acc: String = "".to_owned();
        acc.push_str(&format!("pub struct {} {{\n", name).to_owned());

        // units
        for Step(_, _, prim_struct_name, output_signal_index) in steps {
            acc.push_str(&format!("    unit{}_{}: {},\n", prim_struct_name, output_signal_index, prim_struct_name));
        }

        // signals
        self.trav_mut(&mut |gn: &GNode| {
            acc.push_str(&format!("    signal{}: Signal<{}>,\n", gn.index, gn.node.type_name()).to_owned());
        });

        acc.push_str("}\n");
        acc
    }

    fn generate_per_loop_log(&self) -> String {
        let mut acc: String = "".to_owned();
        acc.push_str("\nfn per_loop_log(&self) {\n");
        self.trav_mut(&mut |gn: &GNode| {
            acc.push_str(&format!("    println!(\"Signal {}: {{:?}}\", &self.signal{});\n", gn.index, gn.index).to_string());
        });
        acc.push_str("}\n");
        acc
    }

    fn generate_impl(&self, name: &str, steps: &Vec<Step>) -> String {
        let mut acc: String = "".to_owned();
        let mut acc_lines: String = "".to_owned();

        // units
        for Step(_, ctor_args, prim_struct_name, output_signal_index) in steps {
            let ctor_arglist: String = ctor_args.join(", ");
            acc_lines.push_str(&format!("    unit{}_{}: {}::new({}),\n", prim_struct_name, output_signal_index, prim_struct_name, ctor_arglist));
        }

        // signals
        self.trav_mut(&mut |gn: &GNode| {
            acc_lines.push_str(&format!("    signal{}: Signal::new(MAX),\n", gn.index));
        });

        let per_loop_log = if PATCH_LOGGING { self.generate_per_loop_log() } else { "".to_string() };

        acc.push_str(&format!(
            r#"
            impl {} {{
                pub fn new() -> {} {{
                    {} {{
                        {}
                    }}
                }}
                {}
            }}
            "#,
            name, name, name, acc_lines, per_loop_log));

        acc
    }

    fn gather_steps(&self) -> Vec<Step> {
        let mut steps = Vec::new();
        self.dep_trav_mut(&mut |gn: &GNode| {
            if *gn.node != Node::Input {
                let ports:Vec<(u32,Range,String)> = gn.inputs.iter().zip(&gn.ports).map(|(input, port)| {
                    (input.borrow().index, port.range, input.borrow().node.type_name().to_string())
                }).collect();
                steps.push(Step(ports, gn.ctor_args.clone(), gn.node.prim_struct_name().to_string(), gn.index));
            }
        });
        //steps.reverse();
        steps
    }

    fn generate_patch_routing(&self, steps: &Vec<Step>) -> String {
        let mut acc: String = "".to_owned();
        /*
        println!("Steps:");
        for step in &steps {
            println!("  {:?}", step);
        }
        */

        for Step(ports, _, prim_struct_name, output_signal_index) in steps {
            if PATCH_LOGGING {
                acc.push_str(&format!("println!(\"{{}} {{}}\", {}, \"{}\");\n", output_signal_index, prim_struct_name));
            }
            let mut port_serial: usize = 0;
            let mut port_numbers: Vec<usize> = Vec::new();
            for (port_index, range, type_name) in ports {
                let next = port_serial;
                port_serial += 1;
                port_numbers.push(next);
                acc.push_str(&format!("let port{}_{}: Window<{}> = Window::new(&self.signal{}, Range({}, {}));\n",
                    output_signal_index, next, type_name, port_index, range.0, range.1));
            }
            let signals: Vec<String> = port_numbers.iter().map(|port_index| format!("&port{}_{}", output_signal_index, port_index)).collect();
            let signals_joined: String = format!("{}{}", signals.join(", "), if signals.len() == 0 { "" } else { ", " });
            acc.push_str(&format!("self.unit{}_{}.go(playhead, {}&mut self.signal{});\n", prim_struct_name, output_signal_index, signals_joined, output_signal_index));
            acc.push_str("\n");
        }

        acc
    }

    fn generate_patch_impl(&self, name: &str, steps: &Vec<Step>) -> String {
        let mut acc: String = "".to_owned();

        let input_signal_write = match self.get_input_slice_index() {
            Some(index) => format!("self.signal{}.write(input_slice[i]);\n", index),
            None => "".to_string(),
        };

        let output_signal = format!("signal{}", self.index);
        let body = self.generate_patch_routing(steps);

        let per_loop_log = if PATCH_LOGGING { "self.per_loop_log();\n" } else { "" };

        acc.push_str(&format!(r#"
impl Patch for {} {{
    fn rust_process_audio(
        &mut self,
        input_slice: &[f32],
        output_slice: &mut [f32],
        _knobs: &Box<dyn Knobs>,
        mut playhead: Playhead,
    ) {{
        for i in 0..input_slice.len() {{
            {}

            {}
            output_slice[i] = self.{}.read(0);

            playhead.inc();
            {}
        }}
    }}

    fn into_any(self: Box<Self>) -> Box<dyn Any> {{
        self
    }}
}}

"#,
            name, input_signal_write, body, output_signal, per_loop_log));
        acc
    }

    pub fn generate_header(&self) -> String {
        r#"
#![allow(non_snake_case)]

extern crate alloc;
extern crate libm;

use alloc::boxed::Box;
use core::any::Any;

#[allow(unused_imports)]
use shared::edsl::runtime::{signal::Signal, window::Window, range::Range, prim::{AddPrim, Const, PassThru, SumFilter, HighPass, LowPass, LinearVibrato}};
use shared::knob::Knobs;
use shared::patch::Patch;
use shared::playhead::Playhead;
const MAX: usize = 100;
"#.to_string()
    }

    pub fn generate(&self, name: &str) -> String {
        let steps = self.gather_steps();
        let mut acc: String = "".to_owned();
        acc.push_str(&self.generate_header());
        acc.push_str(&self.generate_struct(name, &steps));
        acc.push_str(&self.generate_impl(name, &steps));
        acc.push_str(&self.generate_patch_impl(name, &steps));
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
                ctor_args: vec![],
                inputs: vec![],
                ports: vec![],
            },
            Node::PassThru(inn) => GNode {
                index: 0,
                node: (*node).clone(),
                ctor_args: vec![],
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
                ctor_args: vec![],
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
                ctor_args: vec![],
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
            Node::HighPass(inn) => GNode {
                index: 0,
                node: (*node).clone(),
                ctor_args: vec![],
                inputs: vec![
                    genericize1(&inn, hm),
                ],
                ports: vec![
                    Port {
                        range: Range(-1, 0),
                        main_sample: 0,
                    },
                ]
            },
            Node::LowPass(inn) => GNode {
                index: 0,
                node: (*node).clone(),
                ctor_args: vec![],
                inputs: vec![
                    genericize1(&inn, hm),
                ],
                ports: vec![
                    Port {
                        range: Range(-1, 0),
                        main_sample: 0,
                    },
                ]
            },
            Node::Const(k) => GNode {
                index: 0,
                node: (*node).clone(),
                ctor_args: vec![format!("{}f32", k)],
                inputs: vec![],
                ports: vec![
                    Port {
                        range: Range(0, 0),
                        main_sample: 0,
                    },
                ]
            },
            Node::LinearVibrato(max_sample_deviation, vibrato_frequency, inn) => {
                // All the setup here is copied from the original non-edsl implementation.
                // The sinc taps aren't used (nor are they in the original) and the guard samples
                // shouldn't be necessary. But just replicating it to get the same results.

                // The sinc convolution window is twice this.
                const NUM_SINC_TAPS_ONE_SIDE: usize = 3;

                // Add this many samples on either side to prevent under/overruns in production. Should
                // pass rigorous testing with this set to 0, though.
                const GUARD_SAMPLES: usize = 1;

                let buffer_length: usize = 2 * (max_sample_deviation + NUM_SINC_TAPS_ONE_SIDE + GUARD_SAMPLES) + 1;
                let now_index: usize = max_sample_deviation + NUM_SINC_TAPS_ONE_SIDE + GUARD_SAMPLES;
                //spew!("BUF LEN", buffer_length);

                GNode {
                    index: 0,
                    node: (*node).clone(),
                    ctor_args: vec![format!("{}usize", max_sample_deviation), format!("{}usize", now_index)],
                    inputs: vec![
                        genericize1(&vibrato_frequency, hm),
                        genericize1(&inn, hm),
                    ],
                    ports: vec![
                        Port {
                            range: Range(0, 0),
                            main_sample: 0,
                        },
                        Port {
                            range: Range(-(buffer_length as isize), 0),
                            main_sample: 0,
                        },
                    ]
                }
            },
        };
        let gnrc = Rc::new(RefCell::new(gn));
        hm.insert(node.clone(), gnrc.clone());
        gnrc
    }
}

pub fn compile(node: &Rc<Node>, filename: &str, patch_name: &str) {
    let groot = genericize(&node);
    groot.borrow_mut().make_causal();
    groot.borrow_mut().number_nodes();
    groot.borrow().dump();

    let f = File::create(filename).expect("Unable to create file");
    let mut f = BufWriter::new(f);
    f.write_all(GNode::generate(&mut groot.borrow(), patch_name).as_bytes()).unwrap();
}

/*

+ add main sample field
+ make claims causal; add causality delay
- make main samples line up (try rtl, then ltr if that fails); add main sample delay
+ generate signals, windows, prim calls
  + signal decl type
  + window type (from signal)
  + window size
- look at optimized asm
- better approach to graph
- combine claims to get stream window sizes

*/
