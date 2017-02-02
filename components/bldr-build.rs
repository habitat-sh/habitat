use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;

fn main() {
    write_version_file();
}

fn read_version() -> String {
    let child = Command::new("git")
        .arg("rev-parse")
        .arg("HEAD")
        .output()
        .expect("failed to spawn child");
    String::from_utf8_lossy(&child.stdout).into_owned()
}

fn write_version_file() {
    let version = match env::var("PLAN_VERSION") {
        Ok(ver) => ver,
        _ => read_version(),
    };
    let mut f = File::create(Path::new(&env::var("OUT_DIR").unwrap()).join("VERSION")).unwrap();
    f.write_all(version.trim().as_bytes()).unwrap();
}
