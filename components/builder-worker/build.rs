// Inline common build behavior
include!("../libbuild.rs");

use std::env;

fn main() {
    builder::common();
    write_studio_pkg_ident();
    write_docker_exporter_pkg_ident();
    write_docker_pkg_ident();
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

fn write_docker_exporter_pkg_ident() {
    let ident = match env::var("PLAN_DOCKER_EXPORTER_PKG_IDENT") {
        // Use the value provided by the build system if present
        Ok(ident) => ident,
        // Use the latest installed package as a default for development
        _ => String::from("core/hab-pkg-export-docker"),
    };
    util::write_out_dir_file("DOCKER_EXPORTER_PKG_IDENT", ident);
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
