use core::marker::Send;

pub trait Patch: Send {
  fn rust_process_audio(&mut self,
                        input_slice: &[f32],
                        output_slice: &mut [f32],
                        time_in_seconds: f64);
}
