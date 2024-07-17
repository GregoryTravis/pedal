extern crate std;

use std::print;
use std::println;

pub trait Spewable {
    fn do_spew(&self);
}

// TODO dedup or share these.
impl Spewable for i32 {
    fn do_spew(&self) {
        print!("{}", self);
    }
}

impl Spewable for u32 {
    fn do_spew(&self) {
        print!("{}", self);
    }
}

impl Spewable for u64 {
    fn do_spew(&self) {
        print!("{}", self);
    }
}

impl Spewable for usize {
    fn do_spew(&self) {
        print!("{}", self);
    }
}

impl Spewable for f32 {
    fn do_spew(&self) {
        print!("{}", self);
    }
}

impl Spewable for f64 {
    fn do_spew(&self) {
        print!("{}", self);
    }
}

impl Spewable for &str {
    fn do_spew(&self) {
        print!("{}", self);
    }
}

impl Spewable for bool {
    fn do_spew(&self) {
        print!("{}", self);
    }
}

pub fn spew_space() {
    print!(" ");
}

pub fn spew_newline() {
    println!("");
}
