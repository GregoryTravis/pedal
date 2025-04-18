use crate::constants::*;

// This is a goofy usage pattern, but I have two very different ffts to unify.
pub trait FFT {
    fn get_input(&mut self) -> &mut [f32; FFT_SIZE];
    fn run(&mut self) -> &[f32; FFT_SIZE];
}
