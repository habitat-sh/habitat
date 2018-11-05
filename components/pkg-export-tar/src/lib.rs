extern crate base64;
#[macro_use]
extern crate clap;
extern crate habitat_common as common;
extern crate habitat_core as hcore;
extern crate url;

extern crate hab;
extern crate handlebars;

extern crate failure;
#[macro_use]
extern crate failure_derive;
extern crate flate2;
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate mktemp;
extern crate serde_json;
extern crate tar;
extern crate tempfile;

mod build;
pub mod cli;
mod error;
mod rootfs;

pub use cli::Cli;
use common::ui::UI;
pub use error::{Error, Result};
use flate2::write::GzEncoder;
use flate2::Compression;
use hcore::channel;
use hcore::package::{PackageIdent, PackageInstall};
use hcore::url as hurl;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use tar::Builder;

pub use build::BuildSpec;

/// The version of this library and program when built.
pub const VERSION: &'static str = include_str!(concat!(env!("OUT_DIR"), "/VERSION"));
/// The Habitat Package Identifier string for a Busybox package.
const BUSYBOX_IDENT: &'static str = "core/busybox-static";

pub fn export_for_cli_matches(ui: &mut UI, matches: &clap::ArgMatches) -> Result<()> {
    let default_channel = channel::default();
    let default_url = hurl::default_bldr_url();
    let spec = BuildSpec::new_from_cli_matches(&matches, &default_channel, &default_url);
    export(ui, spec)?;

    Ok(())
}

pub fn export(ui: &mut UI, build_spec: BuildSpec) -> Result<()> {
    let hab_pkg = build_spec.hab;
    let build_result = build_spec.create(ui).unwrap();
    let builder_dir_path = build_result.0.path();
    let pkg_ident = build_result.1;

    tar_command(builder_dir_path, pkg_ident, hab_pkg);
    Ok(())
}

#[allow(unused_must_use)]
fn tar_command(temp_dir_path: &Path, pkg_ident: PackageIdent, hab_pkg: &str) {
    let tarball_name = format_tar_name(pkg_ident);

    let tarball = File::create(tarball_name).unwrap();
    let enc = GzEncoder::new(tarball, Compression::default());
    let mut tar_builder = Builder::new(enc);
    tar_builder.follow_symlinks(false);

    let root_fs = temp_dir_path.clone().join("rootfs");
    let hab_pkgs_path = temp_dir_path.clone().join("rootfs/hab");

    // Although this line of code DOES work (it adds the required directories
    // and subdirectories to the tarball), it also returns an error
    // thread 'main' panicked at 'could not export.: "Is a directory (os error 21)"'
    // , /checkout/src/libcore/result.rs:906:4
    // An issue re: this error has been opened in the github repo of tar-rs
    // https://github.com/alexcrichton/tar-rs/issues/147
    // Until this is sorted out, I am not doing anything with the result
    // that is returned by this command -NSH
    tar_builder.append_dir_all("hab", hab_pkgs_path);

    // Find the path to the hab binary
    let mut hab_pkg_binary_path = hab_install_path(hab_package_ident(hab_pkg), root_fs);
    hab_pkg_binary_path.push("bin");

    // Append the hab binary to the tar ball
    tar_builder.append_dir_all("hab/bin", hab_pkg_binary_path);
}

fn format_tar_name(ident: PackageIdent) -> String {
    format!(
        "{}-{}-{}-{}.tar.gz",
        ident.origin,
        ident.name,
        ident.version.unwrap(),
        ident.release.unwrap()
    )
}

fn hab_package_ident(hab_pkg: &str) -> PackageIdent {
    PackageIdent::from_str(hab_pkg).unwrap()
}

fn hab_install_path(hab_ident: PackageIdent, root_fs_path: PathBuf) -> PathBuf {
    let root_fs_path = Path::new(&root_fs_path);
    PackageInstall::load(&hab_ident, Some(root_fs_path))
        .unwrap()
        .installed_path
}
