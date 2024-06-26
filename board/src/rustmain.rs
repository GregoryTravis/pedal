use crate::daisy_seed::*;
use shared::constants::*;
use shared::glep;
use shared::spew::*;

#[no_mangle]
pub fn main() {
    hw_sanity_check();
    hw_Init();
    hw_StartLog(true);
    glep!("hi");
    hw_SetAudioBlockSize(BLOCK_SIZE);
    loop {
        let a = 1;
        let b = a;
        let _c = b;
    }
}
