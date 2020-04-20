use std::env;
use std::path::PathBuf;

fn main() {
    let tdlib_header = env::var("DEP_TDJSON_INCLUDE")
        .map(PathBuf::from)
        .expect("Can't access header from environment")
	.join("tdjson.h");

    let bindings = bindgen::Builder::default()
        .header(tdlib_header.to_string_lossy())
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
