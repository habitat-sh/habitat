use gcc;

#[cfg(windows)]
fn main() { gcc::compile_library("libsid.a", &["./src/obtain_sid.c"]); }

#[cfg(not(windows))]
fn main() {}
