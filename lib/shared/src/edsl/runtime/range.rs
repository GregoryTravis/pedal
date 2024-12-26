// Inclusive at both ends.

#[derive(Copy, Clone, Debug)]
pub struct Range(pub isize, pub isize);

impl Range {
    pub fn contains(&self, i: isize) -> bool {
        self.0 <= i && i <= self.1
    }

    pub fn empty() -> Range {
        Range(0, 0)
    }

    // TODO: Heaven help me
    pub fn translate(&mut self, x: isize) {
        self.0 += x;
        self.1 += x;
    }
}
