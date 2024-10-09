//use alloc::boxed::Box;
use crate::rig_type::Rig;
use crate::globby::*;

extern "C" {
    pub fn cpp_rig_install_callback();
}

pub static THE_PATCH: Globby<Rig> = Globby::empty();

pub fn rig_install_callback() {
    unsafe {
        cpp_rig_install_callback();
    }
}
