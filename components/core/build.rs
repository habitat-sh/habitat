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
    match env::var("HAB_CRYPTO_KEY") {
        Ok(key) => {
            file.write_all(&base64::decode(&key).unwrap()).unwrap();
        }
        Err(_) => {}
    }
}

#[cfg(not(windows))]
fn main() {}
