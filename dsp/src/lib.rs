#![no_std]

pub mod filter;
pub mod load;
pub mod spew;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
