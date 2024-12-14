extern crate std;

use std::cmp::{min,max};
use alloc::rc::Rc;

#[derive(Copy, Clone, Debug)]
pub struct TimeRange {
    past: usize,
    future: usize,
}

#[derive(Copy, Clone, Debug)]
pub struct NodeInfo {
    index: usize,
    time_range: TimeRange,
}

pub fn tr_union(a: &TimeRange, b: &TimeRange) -> TimeRange {
    TimeRange {
        past: min(a.past, b.past),
        future: max(a.future, b.future),
    }
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
    //PassThru { n: Rc<Node> },
    PassThru(NodeInfo, Rc<Node>),
    Add(NodeInfo, Rc<Node>, Rc<Node>),
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
