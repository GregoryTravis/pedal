pub struct MinMax<T>
where
    T: Copy + PartialOrd
{
    minmax: Option<(T, T)>,
}

impl <T> MinMax<T>
where
    T: Copy + PartialOrd
{
    pub fn new() -> MinMax<T> {
        MinMax {
            minmax: None,
        }
    }

    pub fn update(&mut self, value: T) {
        match &mut self.minmax {
            None => {
                self.minmax = Some((value, value))
            }
            Some((ref mut min, ref mut max)) => {
                if value < *min { *min = value; }
                if value > *max { *max = value; }
            }
        }
    }

    // Dangerous, but it's for debugging
    pub fn get(&self) -> (T, T) {
        self.minmax.unwrap()
    }
}

