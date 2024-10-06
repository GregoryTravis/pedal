use core::intrinsics::transmute;

use crate::sdram::*;

impl SDRAM {
  pub fn get_base_pointer() -> *mut f32 {
      unsafe { transmute(0xC0000000u32) }
  }
}
