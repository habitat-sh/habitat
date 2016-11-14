extern crate serde_codegen;

use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    write_version_file();
    codegen();
}

fn read_version() -> String {
    let ver_file = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap())
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("VERSION");
    let f = File::open(ver_file).unwrap();
    let mut reader = BufReader::new(f);
    let mut ver = String::new();
    reader.read_line(&mut ver).unwrap();
    ver
}

fn codegen() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let src = Path::new("src/serde_types.in.rs");
    let dst = Path::new(&out_dir).join("serde_types.rs");
    serde_codegen::expand(&src, &dst).unwrap();
}

fn write_version_file() {
    let version = match env::var("PLAN_VERSION") {
        Ok(ver) => ver,
        _ => read_version(),
    };
    let mut f = File::create(Path::new(&env::var("OUT_DIR").unwrap()).join("VERSION")).unwrap();
    f.write_all(version.trim().as_bytes()).unwrap();
}
