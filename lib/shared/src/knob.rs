use crate::spew::*;

pub trait Knobs: Send {
    fn process(&self);
    fn read(&self, knob_id: usize) -> f32;
    fn spew(&self) {
        spew!("knobs", self.read(0), self.read(1), self.read(2), self.read(3), self.read(4), self.read(5));
    }
}
