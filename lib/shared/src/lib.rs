#![cfg_attr(not(for_host), no_std)]

pub mod filter;

use core::marker::Send;

pub trait Patch: Send {
  fn rust_process_audio(&mut self, left_input_slice: &[f32], right_input_slice: &[f32],
                        left_output_slice: &mut [f32], right_output_slice: &mut [f32],
                        size: usize);
}
