impl SDRAM {
  pub fn get_base_pointer() -> *mut f32 {
      let boxed = Box::new([0.0; SDRAM_SIZE_BYTES ]);
      boxed.as_ptr()
  }
}
