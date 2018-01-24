// Inline common build behavior
include!("../libbuild.rs");

extern crate protoc;
extern crate protoc_rust;

use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::process::Command;

fn main() {
    habitat::common();
    generate_apidocs();
    if env::var("CARGO_FEATURE_PROTOCOLS").is_ok() {
        generate_protocols();
    }
}

fn generate_apidocs() {
    let dst = Path::new(&env::var("OUT_DIR").unwrap()).join("api.html");
    match env::var("CARGO_FEATURE_APIDOCS") {
        Ok(_) => {
            let src = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("doc/api.raml");
            let cmd = Command::new("raml2html")
                .arg("-i")
                .arg(src)
                .arg("-o")
                .arg(dst)
                .status()
                .expect("failed to compile html from raml");

            assert!(cmd.success());
        }
        Err(_) => {
            let mut file = File::create(dst).unwrap();
            file.write_all(b"No API docs provided at build").unwrap();
        }
    };
}

fn generate_protocols() {
    let protocols = protocol_files();
    protoc_rust::run(protoc_rust::Args {
        out_dir: "src/protocols/generated",
        input: protocols
            .iter()
            .map(AsRef::as_ref)
            .collect::<Vec<&str>>()
            .as_slice(),
        includes: &["protocols"],
    }).expect("protoc");
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
