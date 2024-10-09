use crate::constants::SDRAM_SIZE_F32;
use crate::globby::*;

pub static SDRAM_BUFFER: Globby<[f32; SDRAM_SIZE_F32]> = Globby::new([0.0; SDRAM_SIZE_F32]);
