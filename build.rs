use cmake;
use std::env;
use std::path::PathBuf;

fn main() {
    let out_path_lib = PathBuf::from(env::var("OUT_DIR").unwrap())
        .join("lib");

    cmake::build("td");

    println!("cargo:rustc-link-search=native={}", out_path_lib.display());
    println!("cargo:rustc-link-lib=static=tdjson_static");
}
