use alloc::boxed::Box;

use crate::constants::SDRAM_SIZE_F32;
use crate::sdram::*;

impl SDRAM {
  pub fn get_base_pointer() -> *mut f32 {
      let mut boxed = Box::new([0.0; SDRAM_SIZE_F32]);
      boxed.as_mut_ptr()
  }
}
