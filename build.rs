use std::env;
use std::path::PathBuf;
use std::fs;
use std::io;
use std::process;

fn main() {
    // This build script copies the main CLI definition to a JSON file
    // because the `clap` crate requires a JSON file for CLI definitions
    let out_dir = env::var("OUT_DIR").unwrap();
    let main_json_path = PathBuf::from(out_dir).join("main.json");
    let main_cli_def = serde_json::to_string(&clap::load_yaml!("src/main.json")).unwrap();
    fs::write(main_json_path, main_cli_def).unwrap();

    // This script assumes that the `cargo:rustc-link-lib` attribute is used
    // to link the `inotify` library for file system monitoring
    // This is necessary because the `inotify` library is not linked by default
    println!("cargo:rustc-link-lib=inotify");
}