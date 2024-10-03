// You must only ever create one SDRAM!!!
// You must only ever create one SDRAM!!!
// You must only ever create one SDRAM!!!
// You must only ever call get_base_pointer once!!!

use core::slice;

use crate::constants::SDRAM_SIZE_BYTES;

pub struct SDRAM {
    base: *mut f32,
    so_far: usize,
}

impl SDRAM {
    pub fn new -> SDRAM {
        SDRAM {
            base: SDRAM::get_base_pointer(),
            so_far: 0,
        }
    }

    pub fn alloc(&mut self, num_floats: usize) -> &[f32] {
        let float_size = core::mem::size_of<f32>();
        let total_bytes = num_floats * float_size;
        let total_after_allocation = self.so_far + total_bytes;
        if total_after_allocation > SDRAM_SIZE_BYTES {
            panic!("Out of SDRAM!!");
        }
        let ptr: *mut f32 = self.base + self.so_far;
        self.so_far += total_bytes;
        unsafe {
            slice::from_raw_parts(ptr, float_size)
        }
    }
}
