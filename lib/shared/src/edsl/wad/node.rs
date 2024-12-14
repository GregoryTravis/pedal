use alloc::rc::Rc;

#[derive(Debug)]
pub struct TimeRange {
    past: usize,
    future: usize,
}

#[derive(Debug)]
pub struct NodeInfo {
    index: usize,
    time_range: TimeRange,
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
