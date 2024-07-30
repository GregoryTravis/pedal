extern crate std;

use std::{thread, time};

pub fn hw_delay(delay_ms: u32) {
    thread::sleep(time::Duration::from_millis(delay_ms.into()));
}
