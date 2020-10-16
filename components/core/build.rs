#[cfg(windows)]
fn main() {
    use std::{env,
              fs::File,
              io::prelude::*,
              path::Path};

    cc::Build::new().file("./src/os/users/admincheck.c")
                    .compile("libadmincheck.a");
    let mut file =
        File::create(Path::new(&env::var("OUT_DIR").unwrap()).join("hab-crypt")).unwrap();
    if let Ok(key) = env::var("HAB_CRYPTO_KEY") {
        file.write_all(&base64::decode(&key).unwrap()).unwrap();
    }
}

#[cfg(not(windows))]
fn main() {}
