use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;
use std::process::Command;

fn main() {
    let version = match env::var("PLAN_VERSION") {
        Ok(ver) => ver,
        _ => read_version(),
    };
    generate_apidocs();
    let mut f = File::create(Path::new(&env::var("OUT_DIR").unwrap()).join("VERSION")).unwrap();
    f.write_all(version.trim().as_bytes()).unwrap();
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
