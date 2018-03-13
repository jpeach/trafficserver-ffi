extern crate bindgen;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use std::env;
use std::path::PathBuf;
use std::process::{Command, Stdio};

#[derive(Serialize, Deserialize, Debug)]
struct Layout
{
    #[serde(rename = "BINDIR")]
    bin_dir: String,

    #[serde(rename = "INCLUDEDIR")]
    include_dir: String,

    #[serde(rename = "PLUGINDIR")]
    plugin_dir: String,

}

// Exec `traffic_layout` and extract the Layout from the JSON result.
fn get_layout() -> Layout
{
    let result = Command::new("traffic_layout")
        .args(&["--layout", "--json"])
        .stdin(Stdio::null())
        .output()
        .expect("failed to spawn \"traffic_layout\"");

    if !result.status.success() {
        panic!("traffic_layout -l -j failed: {}", String::from_utf8(result.stdout).unwrap());
    }

    let s = String::from_utf8(result.stdout).unwrap();

    let l: Layout = serde_json::from_str(&s.as_str()).expect("invalid layout");
    return l
}

fn main()
{
    let l = get_layout();

    let bindings = bindgen::Builder::default()
        .header("trafficserver.h")
        .clang_arg(format!("-I{}", l.include_dir))
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
