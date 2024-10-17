// You must only ever create one SDRAM!!!
// You must only ever create one SDRAM!!!
// You must only ever create one SDRAM!!!
// You must only ever call get_base_pointer once!!!

use alloc::slice;

#[cfg(not(feature = "for_host"))]
use crate::sdram_board::*;
#[cfg(feature = "for_host")]
use crate::sdram_host::*;

pub struct SDRAM {
    ptr: *mut f32,
    #[allow(unused)]
    num_floats: usize,
}

impl SDRAM {
    pub fn new() -> SDRAM {
        SDRAM_BUFFER.map(|buffer| {
            let _ho: &'static [f32] = *buffer;
            let _hey: *const f32 = _ho.as_ptr();
            let ptr: *mut f32 = _hey as *mut f32;
            let num_floats = buffer.len();
            let a_sdram = SDRAM {
                ptr: ptr,
                num_floats: num_floats,
            };
            a_sdram
        }).unwrap()
    }

    pub fn alloc(&mut self, num_floats: usize) -> &'static mut [f32] {
        let slice = unsafe { slice::from_raw_parts_mut(self.ptr, num_floats) };
        self.ptr = self.ptr.wrapping_add(num_floats);
        slice
    }
}
