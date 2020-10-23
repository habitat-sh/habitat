use std::{env,
          fs,
          path::Path};

#[cfg(windows)]
fn main() {
    use std::{fs::File,
              io::prelude::*};

    cc::Build::new().file("./src/os/users/admincheck.c")
                    .compile("libadmincheck.a");
    let mut file =
        File::create(Path::new(&env::var("OUT_DIR").unwrap()).join("hab-crypt")).unwrap();
    if let Ok(key) = env::var("HAB_CRYPTO_KEY") {
        file.write_all(&base64::decode(&key).unwrap()).unwrap();
    }

    populate_cacert();
}

#[cfg(not(windows))]
fn main() { populate_cacert(); }

pub fn populate_cacert() {
    let src = env::var("SSL_CERT_FILE").unwrap();
    let dst = Path::new(&env::var("OUT_DIR").unwrap()).join("cacert.pem");
    if !dst.exists() {
        fs::copy(src, dst).unwrap();
    }
}
