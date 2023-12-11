extern "C" {
  pub fn load_spew();
}

pub fn show_load() {
  unsafe { load_spew(); }
}
