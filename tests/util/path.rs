use std::env;
use std::path::PathBuf;

pub fn exe_path() -> PathBuf {
    env::current_exe().unwrap()
}

pub fn root() -> PathBuf {
    exe_path().parent().unwrap().parent().unwrap().parent().unwrap().join("tests")
}

pub fn fixtures() -> PathBuf {
    root().join("fixtures")
}

pub fn fixture(name: &str) -> PathBuf {
    fixtures().join(name)
}

pub fn fixture_as_string(name: &str) -> String {
    let fixture_string = fixtures().join(name).to_string_lossy().into_owned();
    fixture_string
}

pub fn bldr_build() -> String {
    root().parent().unwrap().join("packages/bldr-build").to_string_lossy().into_owned()
}

pub fn bldr_package() -> String {
    root().parent().unwrap().join("packages/bldr").to_string_lossy().into_owned()
}

pub fn bldr() -> String {
    root().parent().unwrap().join("target/debug/bldr").to_string_lossy().into_owned()
}
