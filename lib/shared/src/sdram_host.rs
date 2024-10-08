use crate::constants::SDRAM_SIZE_F32;

pub static mut SDRAM_BUFFER: [f32; SDRAM_SIZE_F32] = [0.0; SDRAM_SIZE_F32];
