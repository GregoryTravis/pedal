extern "C" {
    pub fn cpp_hw_init();
}

pub fn hw_init() {
    unsafe {
        cpp_hw_init();
    }
}
