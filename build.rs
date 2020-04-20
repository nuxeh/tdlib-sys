use cmake::Config;
use std::env;
use std::path::PathBuf;

fn main() {
    let dst = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Output is in build since we don't run the install target
    let dst_build = dst.join("build");

    let mut cfg = Config::new("td");

    // Trim down targets to only build tdjson_static
    cfg.build_target("tdjson_static");

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

    println!("cargo:rustc-link-search=native={}", dst_build.display());
    println!("cargo:rustc-link-lib=static=tdjson_static");
}
