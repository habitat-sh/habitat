// Inline common build behavior
include!("../../libbuild.rs");

use std::env;

fn main() {
    habitat::common();
    write_docker_pkg_ident();
}

fn write_docker_pkg_ident() {
    let ident = match env::var("PLAN_DOCKER_PKG_IDENT") {
        // Use the value provided by the build system if present
        Ok(ident) => ident,
        // Use the latest installed package as a default for development
        _ => String::from("core/docker"),
    };
    util::write_out_dir_file("DOCKER_PKG_IDENT", ident);
}
