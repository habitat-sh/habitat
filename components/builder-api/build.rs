use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    let version = env::var("PLAN_VERSION").unwrap_or(env::var("CARGO_PKG_VERSION").unwrap());
    let mut f = File::create(Path::new(&env::var("OUT_DIR").unwrap()).join("VERSION")).unwrap();
    f.write_all(version.as_bytes()).unwrap();
}
