// You must only ever create one SDRAM!!!
// You must only ever create one SDRAM!!!
// You must only ever create one SDRAM!!!
// You must only ever call get_base_pointer once!!!

use core::slice;

use crate::constants::SDRAM_SIZE_F32;

pub struct SDRAM {
    base: *mut f32,
    so_far: usize,
}

impl SDRAM {
    pub fn new() -> SDRAM {
        SDRAM {
            base: SDRAM::get_base_pointer(),
            so_far: 0,
        }
    }

    pub fn alloc(&mut self, num_floats: usize) -> &[f32] {
        let so_far_after_allocation = self.so_far + num_floats;
        if so_far_after_allocation  > SDRAM_SIZE_F32 {
            panic!("Out of SDRAM!!");
        }
        let ptr: *mut f32 = self.base.wrapping_add(self.so_far);
        self.so_far += num_floats;
        unsafe {
            slice::from_raw_parts(ptr, num_floats)
        }
    }
}
