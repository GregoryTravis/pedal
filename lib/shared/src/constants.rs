// TODO: generate these from a shared config.
// TODO: use sample rate enum
pub const SAMPLE_RATE: u32 = 48000;
pub const BLOCK_SIZE: usize = 48;
pub const KSHEP: bool = true;
pub const PROD: bool = false;

pub const SDRAM_SIZE_BYTES: usize = 64 * 1024 * 1024;
pub const SDRAM_SIZE_F32: usize = SDRAM_SIZE_BYTES / core::mem::size_of::<f32>();

pub const FFT_SIZE: usize = 512;
