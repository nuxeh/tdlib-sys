use cmake::Config;
use std::env;
use std::path::PathBuf;

fn main() {
    let dst = PathBuf::from(env::var("OUT_DIR").unwrap());
    let dst_lib = dst.join("lib");

    let mut cfg = Config::new("td");

    if let Some(path) = env::var_os("DEP_OPENSSL_INCLUDE") {
        if let Some(path) = env::split_paths(&path).next() {
            if let Some(path) = path.to_str() {
                if path.len() > 0 {
                    cfg.define("OPENSSL_INCLUDE_DIR", path);
                }
            }
        }
    }

    if let Ok(path) = env::var("DEP_Z_INCLUDE") {
        cfg.define("ZLIB_INCLUDE_DIR", path);
    }

    cfg.build();

    println!("cargo:rustc-link-search=native={}", dst_lib.display());
    println!("cargo:rustc-link-lib=static=tdjson_static");
}
