use crate::daisy_seed::hw_init;

#[no_mangle]
pub fn main() {
    hw_init();
    loop {
        let a = 1;
        let b = a;
        let _c = b;
    }
}
