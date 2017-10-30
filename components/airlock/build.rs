use std::env;
use std::fs::File;
use std::path::Path;
use std::process::Command;
use std::io::Write;

const VERSION_ENVVAR: &'static str = "PLAN_VERSION";

fn main() {
    write_out_dir_file("VERSION", version());
}

pub fn version() -> String {
    match env::var(VERSION_ENVVAR) {
        Ok(ver) => ver,
        _ => read_git_version(),
    }
}

fn read_git_version() -> String {
    let child = Command::new("git")
        .arg("rev-parse")
        .arg("HEAD")
        .output()
        .expect("Failed to spawn child");
    String::from_utf8_lossy(&child.stdout).into_owned()
}

fn write_out_dir_file<P, S>(filename: P, content: S)
where
    P: AsRef<Path>,
    S: AsRef<str>,
{
    let mut f = File::create(
        Path::new(&env::var("OUT_DIR").expect(
            "Failed to read OUT_DIR environment variable",
        )).join(filename),
    ).expect("Failed to create OUT_DIR file");
    f.write_all(content.as_ref().trim().as_bytes()).expect(
        "Failed to write to OUT_DIR file",
    );
}
