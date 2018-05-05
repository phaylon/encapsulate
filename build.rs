
use std::env;

fn main() {
    println!("cargo:rustc-env=ENCAPSULATE_RUSTC={}", env::var("RUSTC").unwrap());
}
