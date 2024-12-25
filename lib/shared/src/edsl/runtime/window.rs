use crate::edsl::runtime::{range::Range, signal::Signal};

pub struct Window<'a, T: Copy> {
    signal: &'a Signal<T>,
    range: Range,
}

impl <'a, T: Copy> Window<'a, T> {
    pub fn new(signal: &'a Signal<T>, range: Range) -> Window<'a, T> {
        Window {
            signal: signal,
            range: range,
        }
    }

    pub fn read(&self, i: isize) -> T {
        assert!(self.range.contains(i));
        self.signal.read(i)
    }
}
