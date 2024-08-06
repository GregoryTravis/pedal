extern crate libm;

use core::f64::consts::PI;
use std::println;

use shared::filter::sine_table::*;

// The table returned is 1 larger than `table_size`.
fn generate_sine_table(table_size: usize) {
    println!("const TABLE: &'static [f32] = &[");

    for i in 0..table_size+1 {
        let x = ((i as f64) * 2.0 * PI) / (table_size as f64);
        println!("{:?},", libm::sin(x) as f32);
    }

    println!("];");
}

pub fn main() {
    generate_sine_table(256);
    let tpio3 = 2.094 + (2.0 * 3.14159);
    println!("{} {}", tpio3, table_sin(tpio3));
    /*
    println!("{:?}", libm::remquo(8.75, 10.0));
    println!("{:?}", libm::remquo(9.75, 10.0));
    println!("{:?}", libm::remquo(8.99, 10.0));
    println!("{:?}", libm::remquo(9.99, 10.0));
    println!("{:?}", libm::remquo(1.51, 1.0));
    println!("{:?}", libm::remquo(1.5, 1.0));
    println!("{:?}", libm::remquo(1.49, 1.0));
    println!("{:?}", libm::remquo(1.01, 1.0));
    println!("{:?}", libm::remquo(1.00, 1.0));
    println!("{:?}", libm::remquo(0.99, 1.0));
    */
}
