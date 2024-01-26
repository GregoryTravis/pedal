pub fn sample_i16_to_f32(x: i16) -> f32 {
    (x as f32) / 32768.0
}

pub fn sample_f32_to_i16(x: f32) -> i16 {
    ((x * 32767.0) as i16).try_into().unwrap()
}
