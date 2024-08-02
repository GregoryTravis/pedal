extern "C" {
    pub fn cpp_knob_init();
    pub fn cpp_knob_process();
    pub fn cpp_knob_get_value(knobID: usize) -> f32;
}

pub fn knob_init() {
    unsafe {
        cpp_knob_init();
    }
}

pub fn knob_process() {
    unsafe {
        cpp_knob_process();
    }
}

pub fn knob_get_value(knob_id: usize) -> f32 {
    unsafe {
        cpp_knob_get_value(knob_id)
    }
}
