use alloc::vec::Vec;
use core::default::Default;

use crate::edsl::runtime::edsl_circbuf::EdslCircbuf;

pub type Signal<T> = EdslCircbufSignal<T>;
//pub type Signal<T> = VecSignal<T>;

#[derive(Debug)]
pub struct VecSignal<T: Default + Copy> {
    vec: Vec<T>,
}

impl <T: Default + Copy> VecSignal<T> {
    pub fn new(size: usize) -> VecSignal<T> {
        VecSignal {
            vec: vec![Default::default(); size],
        }
    }

    pub fn write(&mut self, x: T) {
        self.vec.push(x);
    }

    pub fn read(&self, i: isize) -> T {
        //assert!(i <= 0);
        // TODO switch to positive?
        self.vec[self.to_index(i)]
    }

    fn to_index(&self, i: isize) -> usize {
        (self.vec.len() as isize - 1 + i) as usize
    }
}

#[derive(Debug)]
pub struct EdslCircbufSignal<T: Default + Copy> {
    buf: EdslCircbuf<T>,
}

impl <T: Default + Copy> EdslCircbufSignal<T> {
    pub fn new(size: usize) -> EdslCircbufSignal<T> {
        EdslCircbufSignal {
            buf: EdslCircbuf::new(size),
        }
    }

    pub fn write(&mut self, x: T) {
        self.buf.write(x);
    }

    pub fn read(&self, i: isize) -> T {
        self.buf.read(i)
    }
}
