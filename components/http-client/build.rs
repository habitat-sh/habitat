fn main() { inner::main() }

mod inner {
    use std::{env,
              fs,
              path::Path};

    pub fn main() {
        let src = env::var("SSL_CERT_FILE").unwrap();
        let dst = Path::new(&env::var("OUT_DIR").unwrap()).join("cacert.pem");
        if !dst.exists() {
            fs::copy(src, dst).unwrap();
        }
    }
}
