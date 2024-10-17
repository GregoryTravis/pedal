use crate::constants::SDRAM_SIZE_F32;
use crate::globby::*;

#[link_section = ".sdram_bss"]
pub static SDRAM_BUFFER_BUFFER: [f32; SDRAM_SIZE_F32] = [0.0; SDRAM_SIZE_F32];
pub static SDRAM_BUFFER: Globby<&[f32]> = Globby::new(&SDRAM_BUFFER_BUFFER);

