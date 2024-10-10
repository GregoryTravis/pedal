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
        //let mut sdram: Option<SDRAM> = None;
        SDRAM_BUFFER.map(|mut buffer| {
            let ptr = (&mut buffer).as_mut_ptr();
            let num_floats = buffer.len();
            let a_sdram = SDRAM {
                ptr: ptr,
                num_floats: num_floats,
            };
            //sdram = Some(a_sdram);
            a_sdram
        }).unwrap()
        //sdram.unwrap()
    }

    pub fn alloc(&mut self, num_floats: usize) -> &'static mut [f32] {
        let slice = unsafe { slice::from_raw_parts_mut(self.ptr, num_floats) };
        self.ptr = self.ptr.wrapping_add(num_floats);
        slice
    }
}
