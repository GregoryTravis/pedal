extern "C" {
    fn yin_init_c(buffer_size: i16, threshold: f32);
    fn yin_process_c(buffer: *const i16) -> f32;
}

pub fn yin_init(buffer_size: i16, threshold: f32) {
    unsafe {
        yin_init_c(buffer_size, threshold);
    }
}

pub fn yin_process(int_buffer_slice: &[i16]) -> f32 {
    let int_buffer_ptr: *const i16 = int_buffer_slice.as_ptr();
    unsafe {
        yin_process_c(int_buffer_ptr)
    }
}
