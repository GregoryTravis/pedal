fn main() {
    println!("cargo:rustc-link-lib=static=high_pass");
    println!("cargo:rustc-link-lib=static=daisy");
    println!("cargo:rustc-link-search=native=/Users/gmt/pedal/high_pass/build");
    println!("cargo:rustc-link-search=native=/Users/gmt/DaisyExamples/libDaisy/build");

    // println!("cargo:rustc-link-lib=static=nosys");
    // println!("cargo:rustc-link-search=native=/Users/gmt/DaisyExamples/libDaisy/build");
    // println!("cargo:rustc-link-arg=--specs=nosys.specs");
}
