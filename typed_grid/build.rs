use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    // Repo root README file
    let source = PathBuf::from("../README.md");
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("env variable `OUT_DIR` not found"));
    let dest = out_dir.join("README.md");

    // Copy file (overwrite if exists)
    fs::copy(&source, &dest).expect("failed to copy README.md");
}
