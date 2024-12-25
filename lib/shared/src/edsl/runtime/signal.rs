use alloc::vec::Vec;

pub struct Signal<T: Copy> {
    vec: Vec<T>,
}

impl <T: Copy> Signal<T> {
    pub fn new() -> Signal<T> {
        Signal {
            vec: vec![],
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
