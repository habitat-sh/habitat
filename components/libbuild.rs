#[allow(dead_code)]
const VERSION_ENVVAR: &'static str = "PLAN_VERSION";

#[allow(dead_code)]
mod builder {
    use std::env;
    use std::process::Command;

    use super::VERSION_ENVVAR;

    pub fn common() {
        super::version::write_file(version());
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
}

#[allow(dead_code)]
mod habitat {
    use std::env;
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use std::path::Path;

    use super::VERSION_ENVVAR;

    pub fn common() {
        super::version::write_file(version());
    }

    pub fn version() -> String {
        match env::var(VERSION_ENVVAR) {
            Ok(ver) => ver,
            _ => read_common_version(),
        }
    }

    fn read_common_version() -> String {
        let ver_file = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap())
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .join("VERSION");
        let f = File::open(ver_file).expect("Failed to open version file");
        let mut reader = BufReader::new(f);
        let mut ver = String::new();
        reader.read_line(&mut ver).expect(
            "Failed to read line from version file",
        );
        ver
    }
}

#[allow(dead_code)]
mod util {
    use std::env;
    use std::fs::File;
    use std::io::Write;
    use std::path::Path;

    pub fn write_out_dir_file<P, S>(filename: P, content: S)
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
}

#[allow(dead_code)]
mod version {
    pub fn write_file<S: AsRef<str>>(version: S) {
        super::util::write_out_dir_file("VERSION", version);
    }
}
