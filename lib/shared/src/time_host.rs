extern crate std;

pub fn hw_relative_time() -> u128 {
    std::time::UNIX_EPOCH.elapsed().unwrap().as_millis()
}
