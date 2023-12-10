#![no_std]

pub mod filter;
pub mod load;
pub mod rig;
pub mod spew;

#[cfg(feature = "stdd")]
pub mod sim;

//#[cfg(not(feature = "std"))]
//#[panic_handler]
//#[cfg_attr(feature = "no_std", panic_handler)]
#[cfg_attr(not(feature = "stdd"), panic_handler)]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
