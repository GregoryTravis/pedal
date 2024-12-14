use alloc::rc::Rc;

pub enum Node {
    Input,
    //PassThru { n: Rc<Node> },
    PassThru(Rc<Node>),
    Add(Rc<Node>, Rc<Node>),
}
