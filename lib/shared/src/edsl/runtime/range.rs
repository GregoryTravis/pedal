// Inclusive at both ends.

#[derive(Copy, Clone)]
pub struct Range(pub isize, pub isize);

impl Range {
    pub fn contains(&self, i: isize) -> bool {
        self.0 <= i && i <= self.1
    }
}
