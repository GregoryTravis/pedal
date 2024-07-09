extern crate std;

//use alloc::string::String;
use std::println;

pub fn test_dump_as_source(var: &str, a: &[f32]) {
    println!("lazy_static! {{");
    println!("pub static ref {}: Vec<f32> = vec![", var);

    for i in 0..a.len() {
        println!("{:?},", a[i]);
    }

    println!("];");
    println!("}}");
}

