use core::cell::UnsafeCell;

use crate::constants::SDRAM_SIZE_F32;

#[repr(transparent)]
pub struct UnsafeSyncCell<T: ?Sized>(pub  UnsafeCell<T>);
unsafe impl<T: ?Sized + Sync> Sync for UnsafeSyncCell<T> {}

pub static WRAPPED: UnsafeSyncCell<[f32; SDRAM_SIZE_F32]> = UnsafeSyncCell(UnsafeCell::new([0.0; SDRAM_SIZE_F32]));
