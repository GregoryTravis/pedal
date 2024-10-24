use crate::switch::Switches;

extern "C" {
    pub fn cpp_switch_init();
    pub fn cpp_switch_get_value(switchID: usize) -> bool;
}

pub struct BoardSwitches { }

pub fn switch_init() {
    unsafe {
        cpp_switch_init();
    }
}

impl Switches for BoardSwitches {
    fn read(&self, switch_id: usize) -> bool {
        unsafe {
            cpp_switch_get_value(switch_id)
        }
    }
}
