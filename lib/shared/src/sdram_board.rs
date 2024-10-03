use core::intrinsics:;transmute;

impl SDRAM {
  pub fn get_base_pointer() -> *mut f32 {
      transmute(0xC0000000)
  }
}
