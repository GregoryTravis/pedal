use alloc::string::ToString;
use crate::spew::*;

#[cfg(not(feature = "for_host"))]
#[cfg_attr(not(feature = "for_host"), panic_handler)]
fn panic(info: &core::panic::PanicInfo) -> ! {
    spew!(info.to_string().as_str());
    loop {}
}
