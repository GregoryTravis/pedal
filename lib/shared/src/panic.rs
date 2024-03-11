#[cfg(not(feature = "for_host"))]
#[cfg_attr(not(feature = "for_host"), panic_handler)]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
