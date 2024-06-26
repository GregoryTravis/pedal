use crate::daisy_seed::*;
use shared::constants::*;
use shared::glep;
use shared::load::*;
use shared::spew::*;

#[no_mangle]
pub fn main() {
    hw_sanity_check();
    hw_init(true, BLOCK_SIZE);
    glep!("hi");
    load_init();
    loop {
        let a = 1;
        let b = a;
        let _c = b;
    }
}
