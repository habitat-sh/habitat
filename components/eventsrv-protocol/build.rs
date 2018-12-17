extern crate protoc;
extern crate protoc_rust;

use std::env;
use std::fs;

fn main() {
    if env::var("CARGO_FEATURE_PROTOCOLS").is_ok() {
        generate_protocols();
    }
}

fn generate_protocols() {
    let protocols = protocol_files();
    protoc_rust::run(protoc_rust::Args {
        out_dir: "src/message",
        input: protocols
            .iter()
            .map(AsRef::as_ref)
            .collect::<Vec<&str>>()
            .as_slice(),
        includes: &["protocols"],
    })
    .expect("protoc");
}

fn protocol_files() -> Vec<String> {
    let mut files = vec![];
    for entry in fs::read_dir("protocols").unwrap() {
        let file = entry.unwrap();
        // skip vim temp files
        if file.file_name().to_str().unwrap().starts_with(".") {
            continue;
        }
        if file.metadata().unwrap().is_file() {
            files.push(file.path().to_string_lossy().into_owned());
        }
    }
    files
}
