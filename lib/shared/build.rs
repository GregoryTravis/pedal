use cc;

fn main() {
    //std::env::set_var("TARGET", "arm64-apple-darwin");
    cc::Build::new()
        .file("src/yin/Yin.c")
        .compile("yin");
    println!("cargo:rustc-link-lib=yin");
}
