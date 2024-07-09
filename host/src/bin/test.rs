extern crate alloc;

use shared::test::*;

pub fn main() {
    let chk = test_reso();
    println!("reso: ok {} {}", chk, chk.to_bits());
}
