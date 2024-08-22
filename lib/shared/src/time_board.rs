extern "C" {
    pub fn cpp_relative_time_ms() -> u32;
}

pub fn hw_relative_time() -> u128 {
    unsafe {
        cpp_relative_time_ms() as u128
    }
}
