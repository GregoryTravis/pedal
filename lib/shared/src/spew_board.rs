extern crate alloc;

use crate::constants::PROD;

use crate::spew::Hex;

extern "C" {
    pub fn spew_int_c(x: i32);
    pub fn spew_ulonglong_c(x: u64);
    pub fn spew_ulong_c(x: u32);
    pub fn spew_size_t_c(x: usize);
    pub fn spew_float_c(x: f32);
    pub fn spew_double_c(x: f64);
    pub fn spew_string_c(s: *const core::ffi::c_char);
    pub fn spew_char_c(c: core::ffi::c_char);
    pub fn spew_newline_c();
    pub fn spew_space_c();
    pub fn spew_ulonglong_hex(x: u64);
}

pub trait Spewable {
    fn do_spew(&self);
}

impl Spewable for i32 {
    fn do_spew(&self) {
        if !PROD {
            unsafe {
                spew_int_c(*self);
            }
        }
    }
}

impl Spewable for u32 {
    fn do_spew(&self) {
        if !PROD {
            unsafe {
                spew_ulong_c(*self);
            }
        }
    }
}

impl Spewable for u64 {
    fn do_spew(&self) {
        if !PROD {
            unsafe {
                spew_ulonglong_c(*self);
            }
        }
    }
}

impl Spewable for usize {
    fn do_spew(&self) {
        if !PROD {
            unsafe {
                spew_size_t_c(*self);
            }
        }
    }
}

impl Spewable for isize {
    fn do_spew(&self) {
        if !PROD {
            (*self as i32).do_spew();
        }
    }
}

impl Spewable for f32 {
    fn do_spew(&self) {
        if !PROD {
            unsafe {
                spew_float_c(*self);
            }
        }
    }
}

impl Spewable for f64 {
    fn do_spew(&self) {
        if !PROD {
            unsafe {
                spew_double_c(*self);
            }
        }
    }
}

impl Spewable for char {
    fn do_spew(&self) {
        if !PROD {
            unsafe {
                spew_char_c(*self as core::ffi::c_char);
            }
        }
    }
}

// TODO: don't allocate
// TODO: or if not, then merge this back into Spewable for &str
fn spew_str(s: &str) {
    // TODO make this a default for the trait
    for c in s.chars() {
        c.do_spew();
    }
    /*
    let c_str = CString::new(s).unwrap();
    let c_world: *const core::ffi::c_char = c_str.as_ptr() as *const core::ffi::c_char;
    unsafe {
        spew_string_c(c_world);
    }
    */
}

impl Spewable for &str {
    fn do_spew(&self) {
        if !PROD {
            spew_str(*self);
        }
    }
}

impl Spewable for bool {
    fn do_spew(&self) {
        if !PROD {
            spew_str(if *self { "true" } else { "false" });
        }
    }
}

pub fn spew_space() {
    if !PROD {
        unsafe {
            spew_space_c();
        }
    }
}

pub fn spew_newline() {
    if !PROD {
        unsafe {
            spew_newline_c();
        }
    }
}

impl Spewable for Hex {
    fn do_spew(&self) {
        if !PROD {
            unsafe {
                spew_ulonglong_hex(self.0);
            }
        }
    }
}
