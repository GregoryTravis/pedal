pub struct PassThruFilter {}

impl PassThruFilter {
    pub fn new() -> PassThruFilter {
        PassThruFilter {}
    }

    pub fn filter(&mut self, input_slice: &[f32], output_slice: &mut [f32], size: usize) {
        for i in 0..size {
            output_slice[i] = input_slice[i];
        }
    }
}
