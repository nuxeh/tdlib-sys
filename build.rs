use cmake::Config;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use glob::glob;
use walkdir::WalkDir;

fn main() {
    let dst = PathBuf::from(env::var("OUT_DIR").unwrap());
    let dst_build = dst.join("build");
    let dst_include = dst.join("include");
    let dst_lib = dst.join("lib");

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

    // Create output directories
    fs::create_dir_all(&dst_lib).unwrap();
    fs::create_dir_all(&dst_include).unwrap();

    // Copy required files out of the source tree. You might think this is
    // possible by running a Cmake install target after a partial build, but
    // no, that causes a complete build of all targets
    install(&PathBuf::from("td/td/telegram/"), &dst_include, "td_json_client.h");
    install(&dst_build, &dst_include, "tdjson_export.h");

    // Static linking instructions
    println!("cargo:rustc-link-search=native={}", dst_build.display());
    println!("cargo:rustc-link-lib=static=tdjson_static");

    // Root and include instrucitons for accessing headers in dependent libs
    println!("cargo:root={}", dst.to_str().unwrap());
    println!("cargo:include={}", dst_include.display());

    clean();
}

/// Search for a file and copy it
fn install(src: &Path, dst: &Path, name: &str) {
    let glob_string = format!("{}/{}", src.display(), name);

    glob(&glob_string)
        .expect("bad glob pattern")
        .filter_map(Result::ok)
        .filter(|p| p.is_file())
        .for_each(|found_path| {
            println!("copying {:?}", found_path);
            let file_name = found_path.file_name().expect("can't get file name");
            fs::copy(&found_path, dst.join(&file_name)).unwrap();
        });
}

/// Clean the source tree, otherwise the tarball fails Cargo's validation.
fn clean() {
    WalkDir::new("td")
        .into_iter()
        .filter_map(Result::ok)
        .map(|e| e.into_path())
        .filter(|p| p.is_dir() && p.ends_with("auto"))
        .for_each(|dir| {
            println!("cleaning {}", dir.display());
            fs::remove_dir_all(&dir)
                .expect("error deleting file");
        });
}
