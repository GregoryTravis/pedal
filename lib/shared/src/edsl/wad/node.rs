extern crate std;

//use std::collections::HashMap;
use std::cmp::{min,max};
use alloc::rc::Rc;
use alloc::vec::Vec;

#[derive(Copy, Clone, Debug)]
pub struct TimeRange {
    past: usize,
    future: usize,
}

#[derive(Copy, Clone, Debug)]
pub struct NodeInfo {
// TODO remove unuseds
#[allow(unused)]
    index: usize,
    time_range: TimeRange,
}

#[derive(Copy, Clone, Debug)]
pub struct BufferInfo {
#[allow(unused)]
    time_range: TimeRange,
}

pub fn tr_union(a: &TimeRange, b: &TimeRange) -> TimeRange {
    TimeRange {
        past: min(a.past, b.past),
        future: max(a.future, b.future),
    }
}

pub fn tr_unions(trs: Vec<TimeRange>) -> TimeRange {
    let mut tr = trs[0];
    for tr2 in &trs[1..] {
        tr = tr_union(&tr, tr2);
    }
    tr
}

pub fn n_union(a: &NodeInfo, b: &NodeInfo) -> (NodeInfo, NodeInfo) {
    let unioned = tr_union(&a.time_range, &b.time_range);
    let new_a = NodeInfo { time_range: unioned.clone(), ..*a };
    let new_b = NodeInfo { time_range: unioned.clone(), ..*b };
    (new_a, new_b)
}

#[derive(Debug)]
pub enum Node {
    Input(NodeInfo),
    PassThru(NodeInfo, Rc<Node>),
    Add(NodeInfo, Rc<Node>, Rc<Node>),
}

fn get_node_info(n: &Rc<Node>) -> NodeInfo {
    match **n {
        Node::Input(ni) => ni,
        Node::PassThru(ni, _) => ni,
        Node::Add(ni, _, _) => ni,
    }
}

fn self_and_srcs(n: Rc<Node>) -> Vec<Rc<Node>> {
    match &*n {
        Node::Input(_) => vec![n],
        Node::PassThru(_, c) => vec![n.clone(), c.clone()],
        Node::Add(_, a, b) => vec![n.clone(), a.clone(), b.clone()],
    }
}

pub fn trn(past: usize, future: usize) -> NodeInfo {
    NodeInfo {
        index: 0,
        time_range: TimeRange {
            past: past,
            future: future,
        },
    }
}

pub fn combine_claims(n: Rc<Node>) -> TimeRange {
    let trs: Vec<TimeRange> = self_and_srcs(n).iter().map(|n| get_node_info(n).time_range).collect();
    let tr = tr_unions(trs);
    tr
}
