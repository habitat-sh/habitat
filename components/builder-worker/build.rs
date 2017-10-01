// Inline common build behavior
include!("../libbuild.rs");

use std::env;

fn main() {
    builder::common();
    write_studio_pkg_ident();
}

fn write_studio_pkg_ident() {
    let ident = match env::var("PLAN_STUDIO_PKG_IDENT") {
        // Use the value provided by the build system if present
        Ok(ident) => ident,
        // Use the latest installed package as a default for development
        _ => String::from("core/hab-studio"),
    };
    util::write_out_dir_file("STUDIO_PKG_IDENT", ident);
}
