use core::default::Default;

use crate::edsl::runtime::{range::Range, signal::Signal};

pub struct Window<'a, T: Default + Copy> {
    signal: &'a Signal<T>,
    range: Range,
}

impl <'a, T: Default + Copy> Window<'a, T> {
    pub fn new(signal: &'a Signal<T>, range: Range) -> Window<'a, T> {
        Window {
            signal: signal,
            range: range,
        }
    }

    #[inline(always)]
    pub fn read(&self, i: isize) -> T {
        assert!(self.range.contains(i));
        self.signal.read(i)
    }

    #[inline(always)]
    pub fn range(&self) -> Range {
        self.range
    }
}
