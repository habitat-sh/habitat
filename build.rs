#[cfg(windows)]
fn main() {
    use gcc;
    gcc::compile_library("libsid.a", &["./src/obtain_sid.c"]);
}

#[cfg(not(windows))]
fn main() {}
