#![allow(non_snake_case)]

extern "C" {
    pub fn cpp_hw_Init();
    pub fn cpp_hw_StartLog(b: bool);
}

pub fn hw_Init() {
    unsafe {
        cpp_hw_Init();
    }
}

pub fn hw_StartLog(b: bool) {
    unsafe {
        cpp_hw_StartLog(b);
    }
}
