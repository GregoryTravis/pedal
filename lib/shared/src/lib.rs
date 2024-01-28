#![cfg_attr(not(for_host), no_std)]

pub mod canned_sound;
pub mod convert;
pub mod constants;
pub mod dilly;
pub mod filter;
#[cfg(feature = "for_host")]
pub mod graphing;
pub mod load;
pub mod patch;
pub mod playhead;
pub mod rig;
pub mod signal;
#[cfg(feature = "for_host")]
pub mod sim;
pub mod spew;

#[cfg(not(feature = "for_host"))]
#[cfg_attr(not(feature = "for_host"), panic_handler)]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
