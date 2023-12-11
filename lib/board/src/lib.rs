//#![no_std]
#![cfg_attr(not(for_host), no_std)]

pub mod load;
pub mod rig;
pub mod spew;

#[cfg(not(feature = "for_host"))]
#[cfg_attr(not(feature = "for_host"), panic_handler)]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
