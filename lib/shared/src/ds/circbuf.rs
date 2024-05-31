//#[macro_use]
use alloc::vec::Vec;

pub struct CircBuf<T> {
    size: usize,
    start_index: usize,
    values: Vec<T>,
}

impl<T: core::clone::Clone + Copy> CircBuf<T> {
    pub const fn new_empty() -> Self {
        CircBuf {
            size: 0,
            start_index: 0,
            values: vec![],
        }
    }

    pub fn new(size: usize, init: T) -> Self {
        CircBuf {
            size: size,
            start_index: 0,
            values: vec![init; size],
        }
    }

    // Add to "end"
    pub fn push(&mut self, x: T) {
        self.values[self.start_index] = x;
        self.start_index = (self.start_index + 1) % self.size;
        assert!(self.start_index < self.size);
    }

    pub fn get(&self, i: usize) -> T {
        assert!(i < self.size);
        self.values[(self.start_index + i) % self.size]
    }
}
