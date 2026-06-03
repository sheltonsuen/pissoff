use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("memory.x");

    fs::copy("../../memory.x", &dest_path).unwrap();

    println!("cargo:rustc-link-search={}", out_dir);
    println!("cargo:rerun-if-changed=../../memory.x");
}
