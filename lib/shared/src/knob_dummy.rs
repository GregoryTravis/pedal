use crate::knob::Knobs;

pub struct DummyKnobs { }

impl Knobs for DummyKnobs {
    fn process(&self) {
    }

    fn read(&self, _knob_id: usize) -> f32 {
        1.0
    }
}
