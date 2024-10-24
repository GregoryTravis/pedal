use crate::switch::Switches;

pub struct DummySwitches { }

pub fn switch_init() {
}

impl Switches for DummySwitches {
    fn read(&self, _switch_id: usize) -> bool {
        false
    }
}
