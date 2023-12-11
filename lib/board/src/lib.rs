//#![no_std]
#![cfg_attr(not(stdd), no_std)]

pub mod load;
pub mod rig;
pub mod spew;

#[cfg(not(feature = "stdd"))]
#[cfg_attr(not(feature = "stdd"), panic_handler)]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
