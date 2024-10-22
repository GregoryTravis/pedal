use core::cell::UnsafeCell;

// TODO: don't hard-code to f32.

// UnsafeSyncCell and UnsafeCell have zero space overhead. Originally I had a mutex wrapped around
// this, but the mutex does not work on the board when it is in SDRAM.
//
// I am trusting the UnsafeCell docs which say that it "opts-out of the immutability guarantee for &T".

#[repr(transparent)]
pub struct UnsafeSyncCell<T: ?Sized>(pub  UnsafeCell<T>);
unsafe impl<T: ?Sized + Sync> Sync for UnsafeSyncCell<T> {}

pub struct StaticBuffer<const N: usize>(UnsafeSyncCell<[f32; N]>);

impl <const N: usize> StaticBuffer<N> {
    pub const fn new() -> Self {
        Self(UnsafeSyncCell(UnsafeCell::new([0.0; N])))
    }

    pub fn as_ptr(&self) -> *mut f32 {
        unsafe {
            core::mem::transmute::<*mut [f32; N], *mut f32>(self.0.0.get())
        }
    }

    pub fn len(&self) -> usize { N }
}
