fn main() {
    inner::main()
}

#[cfg(target_os = "linux")]
mod inner {
    use std::env;
    use std::fs;
    use std::path::Path;

    pub fn main() {
        let src = env::var("SSL_CERT_FILE").unwrap();
        let dst = Path::new(&env::var("OUT_DIR").unwrap()).join("cacert.pem");
        if !dst.exists() {
            fs::copy(src, dst).unwrap();
        }
    }
}

#[cfg(not(target_os = "linux"))]
mod inner {
    pub fn main() {}
}
