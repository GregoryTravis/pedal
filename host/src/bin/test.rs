extern crate alloc;

use shared::spew::*;
use shared::test::*;

pub fn main() {
    let chk = test_reso();
    spew!("reso: ok", chk, chk.to_bits());
}
