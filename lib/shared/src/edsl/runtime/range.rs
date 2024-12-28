extern crate std;

use core::cmp::{Eq, PartialEq};
use std::hash::Hash;

// Inclusive at both ends.

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Range(pub isize, pub isize);

impl Range {
    pub fn contains(&self, i: isize) -> bool {
        self.0 <= i && i <= self.1
    }

    pub fn empty() -> Range {
        Range(0, 0)
    }

    pub fn translate(&self, x: isize) -> Range {
        Range(self.0+x, self.1+x)
    }
}
