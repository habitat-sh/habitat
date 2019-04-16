#[cfg(windows)]
fn main() {
    extern crate base64;
    extern crate gcc;

    use std::{env,
              fs::File,
              io::prelude::*,
              path::Path};

    gcc::compile_library("libadmincheck.a", &["./src/os/users/admincheck.c"]);
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
