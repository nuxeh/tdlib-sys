use cmake::Config;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use glob::glob;

fn main() {
    let dst = PathBuf::from(env::var("OUT_DIR").unwrap());
    let dst_lib = dst.join("lib");
    let dst_build = dst.join("build");

    let gperf_path = env::var("TDLIB_GPERF_PATH")
        .map(PathBuf::from);

    let mut cfg = Config::new("td");

    // Trim down targets to only build tdjson_static
    cfg.build_target("tdjson_static");

    // Register cargo-built dependecies
    cfg.register_dep("openssl");
    cfg.register_dep("z");

    // Above isn't enough to build on Windows
    if let Ok(path) = env::var("DEP_Z_INCLUDE") {
        cfg.define("ZLIB_INCLUDE_DIR", path);
    }

    // Specify path to gperf if specified in the environment
    if let Ok(path) = gperf_path {
        cfg.define("GPERF_EXECUTABLE:FILEPATH", path);
    }

    // Build
    cfg.build();

    // Copy required files out of the source tree. You might think this is
    // possible by running a Cmake install target after a partial build, but
    // no, that causes a complete build of all targets
    install(&dst_build, &dst_lib, "tdjson_static");

    export_headers(&dst);

    // Static linking instructions
    println!("cargo:rustc-link-search=native={}", dst_lib.display());
    println!("cargo:rustc-link-lib=static=tdjson_static");

    clean();
}

/// Search for a file and copy it, since we don't necessarily know the file
/// extension of the library we want on any given platform.
fn install(src: &Path, dst: &Path, name: &str) {
    let glob_string = format!("{}/**/{}*", src.display(), name);

    glob(&glob_string)
        .expect("bad glob pattern")
        .filter_map(Result::ok)
        .filter(|p| p.is_file())
	.for_each(|found_path| {
            let file_name = found_path.file_name().expect("can't get file name");
            fs::copy(&found_path, dst.join(&file_name)).unwrap();
        });
}

/// Expose headers to any dependent crates
fn export_headers(dst: &Path) {
    let dst_include = dst.join("include/td/telegram");

    fs::create_dir_all(&dst_include).unwrap();

    fs::copy(
        "td/td/telegram/td_json_client.h",
        dst.join("include/td_json_client.h")
    ).expect("failed to copy header");
    fs::copy(
        dst.join("build/td/telegram/tdjson_export.h"),
        dst_include.join("tdjson_export.h")
    ).expect("failed to copy header");

    println!("cargo:root={}", dst.to_str().unwrap());
    println!("cargo:include={}", dst.join("include").display());
}

/// Clean the source tree, otherwise the tarball fails Cargo's validation.
fn clean() {
    Config::new("td")
        .build_target("clean")
        .build();
}
