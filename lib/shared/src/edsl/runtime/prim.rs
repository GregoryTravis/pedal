use crate::edsl::runtime::cursor::Cursor;

pub fn pass_thru<const PI: usize, const FI: usize, const BI: usize, const TI: usize, const PO: usize,
                 const FO: usize, const BO: usize, const TO: usize>(i: usize, inc: &Cursor<PI, FI, BI, TI>, outc: &mut Cursor<PO, FO, BO, TO>) {
    outc.write(i, inc.read(i as isize));
}

pub fn add<const PI: usize, const FI: usize, const BI: usize, const TI: usize,
           const PI2: usize, const FI2: usize, const BI2: usize, const TI2: usize,
           const PO: usize, const FO: usize, const BO: usize, const TO: usize>(i: usize, inc: &Cursor<PI, FI, BI, TI>, in2c: &Cursor<PI2, FI2, BI2, TI2>, outc: &mut Cursor<PO, FO, BO, TO>) {
    outc.write(i, inc.read(i as isize) + in2c.read(i as isize));
}
