use crate::edsl::runtime::cursor::Cursor;

pub fn pass_thru<const PI: usize, const FI: usize, const B: usize, const TI: usize, const PO: usize,
                 const FO: usize, const TO: usize>(i: usize, inc: &Cursor<PI, FI, B, TI>, outc: &mut Cursor<PO, FO, B, TO>) {
    outc.write(i, inc.read(i as isize));
}

pub fn add<const PI: usize, const FI: usize, const B: usize, const TI: usize,
           const PI2: usize, const FI2: usize, const TI2: usize,
           const PO: usize, const FO: usize, const TO: usize>(i: usize, inc: &Cursor<PI, FI, B, TI>, in2c: &Cursor<PI2, FI2, B, TI2>, outc: &mut Cursor<PO, FO, B, TO>) {
    outc.write(i, inc.read(i as isize) + in2c.read(i as isize));
}
