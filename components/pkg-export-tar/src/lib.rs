use habitat_common as common;
use habitat_core as hcore;

mod build;
mod rootfs;

use crate::{common::ui::UI,
            hcore::package::{PackageIdent,
                             PackageInstall}};
use anyhow::Result;
use flate2::{Compression,
             write::GzEncoder};
use std::{fs::File,
          path::{Path,
                 PathBuf},
          str::FromStr};
use tar::Builder;

use crate::build::BuildSpec;

use clap::Parser;
mod cli;

/// The version of this library and program when built.
const VERSION: &str = include_str!(concat!(env!("OUT_DIR"), "/VERSION"));

/// The Habitat Package Identifier string for a Busybox package.
const BUSYBOX_IDENT: &str = "core/busybox-static";

/// cli_driver: Public API. This is used by the caller to use the CLI.
pub async fn cli_driver(ui: &mut UI) -> Result<()> {
    let cli = cli::Cli::parse();
    export_for_cli_matches(ui, &cli).await
}

async fn export_for_cli_matches(ui: &mut UI, cli: &cli::Cli) -> Result<()> {
    let spec = BuildSpec::new_from_cli(cli);
    export(ui, spec).await
}

async fn export(ui: &mut UI, build_spec: BuildSpec<'_>) -> Result<()> {
    let hab_pkg = build_spec.hab;
    let no_hab_bin = build_spec.no_hab_bin;
    let build_result = build_spec.create(ui).await?;
    let builder_dir_path = build_result.0.path();
    let pkg_ident = build_result.1;

    tar_command(builder_dir_path, pkg_ident, hab_pkg, no_hab_bin);
    Ok(())
}

#[allow(unused_must_use)]
fn tar_command(temp_dir_path: &Path, pkg_ident: PackageIdent, hab_pkg: &str, no_hab_bin: bool) {
    let tarball_name = format_tar_name(pkg_ident);

    let tarball = File::create(tarball_name).unwrap();
    let enc = GzEncoder::new(tarball, Compression::default());
    let mut tar_builder = Builder::new(enc);
    tar_builder.follow_symlinks(false);

    let root_fs = temp_dir_path.join("rootfs");
    let hab_pkgs_path = temp_dir_path.join("rootfs/hab");

    // Although this line of code DOES work (it adds the required directories
    // and subdirectories to the tarball), it also returns an error
    // thread 'main' panicked at 'could not export.: "Is a directory (os error 21)"'
    // , /checkout/src/libcore/result.rs:906:4
    // An issue re: this error has been opened in the github repo of tar-rs
    // https://github.com/alexcrichton/tar-rs/issues/147
    // Until this is sorted out, I am not doing anything with the result
    // that is returned by this command -NSH
    tar_builder.append_dir_all("hab", hab_pkgs_path);

    // Conditionally include the hab binary if not excluded
    if !no_hab_bin {
        // Find the path to the hab binary
        let mut hab_pkg_binary_path = hab_install_path(&hab_package_ident(hab_pkg), &root_fs);
        hab_pkg_binary_path.push("bin");

        // Append the hab binary to the tar ball
        tar_builder.append_dir_all("hab/bin", hab_pkg_binary_path);
    }
}

fn format_tar_name(ident: PackageIdent) -> String {
    format!("{}-{}-{}-{}.tar.gz",
            ident.origin,
            ident.name,
            ident.version.unwrap(),
            ident.release.unwrap())
}

fn hab_package_ident(hab_pkg: &str) -> PackageIdent { PackageIdent::from_str(hab_pkg).unwrap() }

fn hab_install_path(hab_ident: &PackageIdent, root_fs_path: &Path) -> PathBuf {
    let root_fs_path = Path::new(&root_fs_path);
    PackageInstall::load(hab_ident, Some(root_fs_path)).unwrap()
                                                       .installed_path
}
