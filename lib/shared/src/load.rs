extern "C" {
    pub fn cpp_load_init();
    pub fn cpp_load_before();
    pub fn cpp_load_after();
    pub fn cpp_load_spew();
}

pub fn load_spew() {
    unsafe {
        cpp_load_spew();
    }
}

pub fn load_init() {
    unsafe {
        cpp_load_init();
    }
}

pub fn load_before() {
    unsafe {
        cpp_load_before();
    }
}

pub fn load_after() {
    unsafe {
        cpp_load_after();
    }
}
