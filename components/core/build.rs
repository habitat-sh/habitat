extern crate gcc;

#[cfg(windows)]
fn main() {
    gcc::compile_library("libadmincheck.a", &["./src/os/users/admincheck.c"]);
}

#[cfg(not(windows))]
fn main() {}
