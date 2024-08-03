use crate::knob::Knobs;

extern "C" {
    pub fn cpp_knob_init();
    pub fn cpp_knob_process();
    pub fn cpp_knob_get_value(knobID: usize) -> f32;
}

pub struct BoardKnobs { }

pub fn knob_init() {
    unsafe {
        cpp_knob_init();
    }
}

impl Knobs for BoardKnobs {
    fn process(&self) {
        unsafe {
            cpp_knob_process();
        }
    }

    fn read(&self, knob_id: usize) -> f32 {
        unsafe {
            cpp_knob_get_value(knob_id)
        }
    }
}
