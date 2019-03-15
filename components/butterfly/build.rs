extern crate pkg_config;
extern crate prost;
extern crate prost_build;

use std::{env,
          fs,
          path::PathBuf};

fn main() {
    if env::var("CARGO_FEATURE_PROTOCOLS").is_ok() {
        generate_protocols();
    }
}

fn generate_protocols() {
    let mut config = prost_build::Config::new();
    config.type_attribute(".butterfly.newscast.Rumor.payload",
                          "#[allow(clippy::large_enum_variant)]");
    config.type_attribute(".", "#[derive(Serialize, Deserialize)]");
    config.compile_protos(&protocol_files(), &protocol_includes())
          .expect("Error compiling protobuf definitions");
    for file in generated_files() {
        fs::copy(&file,
                 // NB: src/generated is presumed to exist; if you delete
                 // it, this'll fail.
                 format!("src/generated/{}",
                         file.file_name().unwrap().to_string_lossy())).expect("Could not copy \
                                                                               generated code to \
                                                                               src/generated");
    }
}

fn generated_files() -> Vec<PathBuf> {
    let mut files = vec![];
    for entry in fs::read_dir(env::var("OUT_DIR").unwrap()).unwrap() {
        let file = entry.unwrap();
        if file.file_name().to_str().unwrap().ends_with(".rs") {
            if file.metadata().unwrap().is_file() {
                files.push(file.path());
            }
        }
    }
    files
}

fn protocol_includes() -> Vec<String> { vec!["protocols".to_string()] }

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
