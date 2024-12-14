// TODO make the compiler calculate T = P+F+B

// Overall index calc is: wrap(local i + current + batch_cursor)
pub struct Buffer<const P: usize, const F: usize, const B: usize, const T: usize> {
    current: usize,
    samples: [f32; T],
}

impl <const P: usize, const F: usize, const B: usize, const T: usize> Buffer<P, F, B, T> {
    pub const fn new() -> Buffer<P, F, B, T> {
        Buffer {
            current: 0,
            samples: [0.0; T],
        }
    }

    pub fn read(&self, i: isize) -> f32 {
        let ii = Buffer::<P, F, B, T>::wrap(self.current as isize + i);
        self.samples[ii]
    }

    pub fn write(&mut self, i: usize, x: f32) {
        self.samples[Buffer::<P, F, B, T>::wrap(i as isize)] = x;
    }

    pub fn advance(&mut self, n: usize) {
        self.current = Buffer::<P, F, B, T>::wrap((self.current + n) as isize);
    }

    fn wrap(i: isize) -> usize {
        // TODO make this and AND with a 2^n size
        let wrapped_i = (i + T as isize) % T as isize;
        assert!(wrapped_i >= 0 && wrapped_i < T as isize);
        wrapped_i as usize
    }
}
