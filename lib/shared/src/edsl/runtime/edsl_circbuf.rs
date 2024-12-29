use alloc::vec::Vec;
use core::default::Default;

// TODO remove derive debugs for prod board?
#[derive(Debug)]
pub struct EdslCircbuf<T: Default + Copy> {
    size: usize,
    arr: Vec<T>,
    cursor: usize,
}

impl <T: Default + Copy> EdslCircbuf<T> {
    pub fn new(size: usize) -> EdslCircbuf<T> {
        EdslCircbuf {
            size: size,
            arr: vec![Default::default(); size],
            cursor: 0,
        }
    }

    pub fn write(&mut self, t: T) {
        self.cursor = self.cursor + 1;
        if self.cursor == self.size {
            self.cursor = 0;
        }

        self.arr[self.cursor] = t;
    }

    // Valid range [-(size-1)..0] (inclusive)
    pub fn read(&self, i: isize) -> T {
        // TODO remove
        //assert!(self.index_ok(i));

        // TODO this isn't the most useful special case
        if i == 0 {
            self.arr[self.cursor]
        } else {
            let mut ii: isize = self.cursor as isize + i;
            if ii < 0 {
                // TODO make self.size isize to avoid conversion?
                ii += self.size as isize;
            }
            // TODO remove runtime conversion check?
            self.arr[ii as usize]
        }
    }

    /*
    fn index_ok(&self, i: isize) -> bool {
        -(self.size as isize) < i && i <= 0
    }
    */
}
