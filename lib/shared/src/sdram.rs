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
    sofar: usize,
    total_floats: usize,
}

impl SDRAM {
    pub fn new() -> SDRAM {
        SDRAM_BUFFER.map(|buffer| {
            let ptr: *mut f32 = (*buffer).as_ptr() as *mut f32;
            let total_floats = buffer.len();
            let a_sdram = SDRAM {
                ptr: ptr,
                sofar: 0,
                total_floats: total_floats,
            };
            a_sdram
        }).unwrap()
    }

    pub fn alloc(&mut self, num_floats: usize) -> &'static mut [f32] {
        let new_sofar = self.sofar + num_floats;
        if new_sofar > self.total_floats {
            panic!("Out of SDRAM! (already allocated {}, new allocation {}", self.sofar, num_floats);
        }
        let slice = unsafe { slice::from_raw_parts_mut(self.ptr, num_floats) };
        self.sofar = new_sofar;
        self.ptr = self.ptr.wrapping_add(num_floats);
        slice
    }
}

#[cfg(test)]
use core::mem::size_of;
#[cfg(test)]
use crate::constants::SDRAM_SIZE_F32;

#[test]
fn alloc_sequential() {
    let mut sdram = SDRAM::new();
    let size: usize = 40;
    let a0 = sdram.alloc(size);
    let a1 = sdram.alloc(size);
    let diff_bytes = (a1.as_ptr() as usize) - (a0.as_ptr() as usize);
    let expected_diff_bytes = size * size_of::<f32>();
    assert_eq!(diff_bytes, expected_diff_bytes);
}

#[test]
fn layout() {
    let mut sdram0 = SDRAM::new();
    let a16 = sdram0.alloc(16);
    let mut sdram1 = SDRAM::new();
    let a8 = sdram1.alloc(8);
    let a8_2 = sdram1.alloc(8);
    a16[7] = 12.0;
    a16[8] = 23.0;
    assert_eq!(a8[7], 12.0);
    assert_eq!(a8_2[0], 23.0);
}

#[test]
#[should_panic]
fn oom() {
    let mut sdram = SDRAM::new();
    sdram.alloc(SDRAM_SIZE_F32);
    sdram.alloc(1);
}

