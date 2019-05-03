#[cfg(windows)]
fn main() {
    cc::Build::new().file("./src/obtain_sid.c")
                    .compile("libsid.a");
}

#[cfg(not(windows))]
fn main() {}
