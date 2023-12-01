fn main() {
    println!("cargo:rustc-link-lib=static=high_pass");
    println!("cargo:rustc-link-search=native=/Users/gmt/pedal/high_pass/build"); // I believe this linker option is the problem
}
