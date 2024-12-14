use crate::edsl::buffer::Buffer;

pub struct Cursor<'a, const P: usize, const F: usize, const B: usize, const T: usize> {
    buffer: &'a mut Buffer<P, F, B, T>,
    index: usize,
}

impl <'a, const P: usize, const F: usize, const B: usize, const T: usize> Cursor<'a, P, F, B, T> {
    pub fn new(buffer: &'a mut Buffer<P, F, B, T>) -> Cursor<'a, P, F, B, T> {
        Cursor {
            buffer: buffer,
            index: 0,
        }
    }

    pub fn read(&self, i: isize) -> f32 {
        self.buffer.read(self.index as isize + i)
    }

    pub fn write(&mut self, i: usize, x: f32) {
        self.buffer.write(self.index + i, x);
    }
}
