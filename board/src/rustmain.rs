use crate::daisy_seed::*;
use shared::glep;
use shared::spew::*;

#[no_mangle]
pub fn main() {
    hw_Init();
    hw_StartLog(true);
    glep!(2);
    loop {
        let a = 1;
        let b = a;
        let _c = b;
    }
}
