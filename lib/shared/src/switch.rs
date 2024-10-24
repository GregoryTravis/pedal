use crate::spew::*;

pub trait Switches: Send {
    fn read(&self, switch_id: usize) -> bool;
    fn spew(&self) {
        spew!("switches", self.read(0), self.read(1));
    }
}

