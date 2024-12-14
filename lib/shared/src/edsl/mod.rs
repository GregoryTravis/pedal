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

    // Convert all "as" to tries
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

// Primitive
pub fn pass_thru<const PI: usize, const FI: usize, const BI: usize, const TI: usize, const PO: usize, const FO: usize, const BO: usize, const TO: usize>(i: usize, inc: &Cursor<PI, FI, BI, TI>, outc: &mut Cursor<PO, FO, BO, TO>) {
    outc.write(i, inc.read(i as isize));
}

pub fn add<const PI: usize, const FI: usize, const BI: usize, const TI: usize, const PI2: usize, const FI2: usize, const BI2: usize, const TI2: usize, const PO: usize, const FO: usize, const BO: usize, const TO: usize>(i: usize, inc: &Cursor<PI, FI, BI, TI>, in2c: &Cursor<PI2, FI2, BI2, TI2>, outc: &mut Cursor<PO, FO, BO, TO>) {
    outc.write(i, inc.read(i as isize) + in2c.read(i as isize));
}
