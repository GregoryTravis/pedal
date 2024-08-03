pub trait Knobs: Send {
    fn process(&self);
    fn read(&self, knob_id: usize) -> f32;
}
