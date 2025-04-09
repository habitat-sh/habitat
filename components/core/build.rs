use std::{env,
          fs,
          path::Path};

#[cfg(windows)]
fn main() {
    use base64::{engine::general_purpose::STANDARD,
                 Engine};
    use std::{fs::File,
              io::prelude::*};

    cc::Build::new().file("./src/os/users/admincheck.c")
                    .compile("libadmincheck.a");
    let mut file =
        File::create(Path::new(&env::var("OUT_DIR").unwrap()).join("hab-crypt")).unwrap();
    if let Ok(key) = env::var("HAB_CRYPTO_KEY") {
        file.write_all(&STANDARD.decode(key).unwrap()).unwrap();
    }

    populate_cacert();
}

#[cfg(not(windows))]
fn main() { populate_cacert(); }

pub fn populate_cacert() {
    if let Ok(src) = env::var("SSL_CERT_FILE") {
        let dst = Path::new(&env::var("OUT_DIR").unwrap()).join("cacert.pem");
        // Verify the certificate data
        let cert_data =
            fs::read(&src).unwrap_or_else(|_| panic!("Failed to read SSL_CERT_FILE at {}", src));
        pem::parse_many(cert_data).unwrap_or_else(|_| {
                                      panic!("The SSL_CERT_FILE {} contains one or more invalid \
                                              certificates",
                                             src)
                                  });
        if !dst.exists() {
            fs::copy(&src, &dst).unwrap_or_else(|_| {
                                    panic!("Failed to copy CA certificates from '{}' to '{}' for \
                                            compiliation.",
                                           src,
                                           dst.display())
                                });
        }
    } else if env::var("PROFILE").unwrap() == "release" {
        panic!("SSL_CERT_FILE environment variable must contain path to minimal CA certificates \
                files to be used by Habitat in environments where core/cacerts package or native \
                platform CA certificates are not available.");
    } else {
        let dst = Path::new(&env::var("OUT_DIR").unwrap()).join("cacert.pem");
        fs::write(&dst, "").unwrap_or_else(|_| {
                               panic!("Failed to write empty CA certificates file at '{}' for \
                                       compiliation.",
                                      dst.display())
                           });
        println!("cargo:warning=SSL_CERT_FILE environment variable is not specified. Habitat \
                  will be built without a minimal set of CA root certificates. This may cause it \
                  to fail on https requests in environments where core/cacerts package or native \
                  platform CA certificates are not available.");
    }
}
