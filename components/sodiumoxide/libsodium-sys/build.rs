use std::env;
extern crate pkg_config;

fn main() {

    if let Ok(lib_dir) = env::var("SODIUM_LIB_DIR") {

    	println!("cargo:rustc-flags=-L native={}", lib_dir);

        let mode = match env::var_os("SODIUM_STATIC") {
            Some(_) => "static",
            None => "dylib"
        };
        println!("cargo:rustc-flags=-l {0}=sodium", mode);

    } else {

        pkg_config::find_library("libsodium").unwrap();

    }

}
