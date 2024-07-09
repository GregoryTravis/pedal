use alloc::ffi::CString;

extern crate alloc;

extern "C" {
    pub fn spew_int_c(x: i32);
    pub fn spew_ulonglong_c(x: u64);
    pub fn spew_ulong_c(x: u32);
    pub fn spew_size_t_c(x: usize);
    pub fn spew_float_c(x: f32);
    pub fn spew_double_c(x: f64);
    pub fn spew_string_c(s: *const core::ffi::c_char);
    pub fn spew_newline_c();
    pub fn spew_space_c();
}

pub trait Spewable {
    fn do_spew(&self);
}

impl Spewable for i32 {
    fn do_spew(&self) {
        unsafe {
            spew_int_c(*self);
        }
    }
}

impl Spewable for u32 {
    fn do_spew(&self) {
        unsafe {
            spew_ulong_c(*self);
        }
    }
}

impl Spewable for u64 {
    fn do_spew(&self) {
        unsafe {
            spew_ulonglong_c(*self);
        }
    }
}

impl Spewable for usize {
    fn do_spew(&self) {
        unsafe {
            spew_size_t_c(*self);
        }
    }
}

impl Spewable for f32 {
    fn do_spew(&self) {
        unsafe {
            spew_float_c(*self);
        }
    }
}

impl Spewable for f64 {
    fn do_spew(&self) {
        unsafe {
            spew_double_c(*self);
        }
    }
}

impl Spewable for &str {
    fn do_spew(&self) {
        let c_str = CString::new(*self).unwrap();
        let c_world: *const core::ffi::c_char = c_str.as_ptr() as *const core::ffi::c_char;
        unsafe {
            spew_string_c(c_world);
        }
    }
}

pub fn spew_space() {
    unsafe {
        spew_space_c();
    }
}

pub fn spew_newline() {
    unsafe {
        spew_newline_c();
    }
}
