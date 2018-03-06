extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let bindings = bindgen::Builder::default()
        .header("trafficserver.h")
        // TODO(jpeach) Figure out how to make this a build option
        // for the crate.
        .clang_arg("-I/opt/ats/include")
	    // Whitelist only TS* functions. This keeps the API surface
	    // area to something reasonable and suppresses most of the
	    // system header types that would otherwise be generated.
        .whitelist_function("TS.*")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

