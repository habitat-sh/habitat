// Copyright (c) 2016-2018 Chef Software Inc. and/or applicable contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::{common::{self,
                     command::package::install::{InstallHookMode,
                                                 InstallMode,
                                                 InstallSource,
                                                 LocalPackageUsage},
                     ui::{Status,
                          UIWriter,
                          UI},
                     PROGRAM_NAME},
            error::Result,
            hcore::{fs::{cache_artifact_path,
                         cache_key_path,
                         CACHE_ARTIFACT_PATH,
                         CACHE_KEY_PATH},
                    package::PackageIdent,
                    ChannelIdent}};
use clap;
#[cfg(unix)]
use std::os::unix::fs::symlink;
#[cfg(windows)]
use std::os::windows::fs::symlink_dir as symlink;
use std::{fs as stdfs,
          path::Path};
use tempfile::TempDir;

use super::{BUSYBOX_IDENT,
            VERSION};

use crate::rootfs;

// Much of this functionality is duplicated (or slightly modified)
// from the Docker exporter. This needs to be abstacted out in
// the future for use with further exporters.
// https://github.com/habitat-sh/habitat/issues/4522
const DEFAULT_HAB_IDENT: &str = "core/hab";
const DEFAULT_LAUNCHER_IDENT: &str = "core/hab-launcher";
const DEFAULT_SUP_IDENT: &str = "core/hab-sup";

/// The specification for creating a temporary file system build root, based on Habitat packages.
///
/// When a `BuildSpec` is created, a `BuildRoot` is returned which can be used to produce exported
/// images, archives, etc.
#[derive(Debug)]
pub struct BuildSpec<'a> {
    /// A string representation of a Habitat Package Identifer for the Habitat CLI package.
    pub hab: &'a str,
    /// A string representation of a Habitat Package Identifer for the Habitat Launcher package.
    pub hab_launcher: &'a str,
    /// A string representation of a Habitat Package Identifer for the Habitat Supervisor package.
    pub hab_sup: &'a str,
    /// The Builder URL which is used to install all service and extra Habitat packages.
    pub url: &'a str,
    /// The Habitat release channel which is used to install all service and extra Habitat
    /// packages.
    pub channel: ChannelIdent,
    /// The Builder URL which is used to install all base Habitat packages.
    pub base_pkgs_url: &'a str,
    /// The Habitat release channel which is used to install all base Habitat packages.
    pub base_pkgs_channel: ChannelIdent,
    /// A Habitat Package Identifer or local path to a Habitat Artifact file which
    /// will be installed.
    pub ident_or_archive: &'a str,
}

impl<'a> BuildSpec<'a> {
    /// Creates a `BuildSpec` from cli arguments.
    pub fn new_from_cli_matches(m: &'a clap::ArgMatches<'_>, default_url: &'a str) -> Self {
        BuildSpec { hab:               m.value_of("HAB_PKG").unwrap_or(DEFAULT_HAB_IDENT),
                    hab_launcher:      m.value_of("HAB_LAUNCHER_PKG")
                                        .unwrap_or(DEFAULT_LAUNCHER_IDENT),
                    hab_sup:           m.value_of("HAB_SUP_PKG").unwrap_or(DEFAULT_SUP_IDENT),
                    url:               m.value_of("BLDR_URL").unwrap_or(&default_url),
                    channel:           m.value_of("CHANNEL")
                                        .map(ChannelIdent::from)
                                        .unwrap_or_default(),
                    base_pkgs_url:     m.value_of("BASE_PKGS_BLDR_URL").unwrap_or(&default_url),
                    base_pkgs_channel: m.value_of("BASE_PKGS_CHANNEL")
                                        .map(ChannelIdent::from)
                                        .unwrap_or_default(),
                    ident_or_archive:  m.value_of("PKG_IDENT_OR_ARTIFACT").unwrap(), }
    }

    /// Creates a `BuildRoot` for the given specification.
    ///
    /// # Errors
    ///
    /// * If a temporary directory cannot be created
    /// * If the root file system cannot be created
    /// * If the `BuildRootContext` cannot be created
    pub fn create(self, ui: &mut UI) -> Result<(TempDir, PackageIdent)> {
        let workdir = TempDir::new()?;
        let rootfs = workdir.path().join("rootfs");

        ui.status(Status::Creating,
                  format!("build root in {}", workdir.path().display()))?;

        let created_ident = self.prepare_rootfs(ui, &rootfs)?;

        Ok((workdir, created_ident))
    }

    fn prepare_rootfs<P: AsRef<Path>>(&self, ui: &mut UI, rootfs: P) -> Result<(PackageIdent)> {
        ui.status(Status::Creating, "root filesystem")?;
        rootfs::create(&rootfs)?;
        self.create_symlink_to_artifact_cache(ui, &rootfs)?;
        self.create_symlink_to_key_cache(ui, &rootfs)?;
        self.install_base_pkgs(ui, &rootfs)?;
        let ident = self.install_user_pkg(ui, self.ident_or_archive, &rootfs)?;
        self.remove_symlink_to_key_cache(ui, &rootfs)?;
        self.remove_symlink_to_artifact_cache(ui, &rootfs)?;

        Ok(ident)
    }

    fn create_symlink_to_artifact_cache<P: AsRef<Path>>(&self,
                                                        ui: &mut UI,
                                                        rootfs: P)
                                                        -> Result<()> {
        ui.status(Status::Creating, "artifact cache symlink")?;
        let src = cache_artifact_path(None::<P>);
        let dst = rootfs.as_ref().join(CACHE_ARTIFACT_PATH);
        stdfs::create_dir_all(dst.parent().expect("parent directory exists"))?;
        debug!("Symlinking src: {} to dst: {}",
               src.display(),
               dst.display());

        symlink(src, dst)?;
        Ok(())
    }

    fn create_symlink_to_key_cache<P: AsRef<Path>>(&self, ui: &mut UI, rootfs: P) -> Result<()> {
        ui.status(Status::Creating, "key cache symlink")?;
        let src = cache_key_path(None::<P>);
        let dst = rootfs.as_ref().join(CACHE_KEY_PATH);
        stdfs::create_dir_all(dst.parent().expect("parent directory exists"))?;
        debug!("Symlinking src: {} to dst: {}",
               src.display(),
               dst.display());

        symlink(src, dst)?;
        Ok(())
    }

    fn install_base_pkgs<P: AsRef<Path>>(&self, ui: &mut UI, rootfs: P) -> Result<BasePkgIdents> {
        let hab = self.install_base_pkg(ui, self.hab, &rootfs)?;
        let sup = self.install_base_pkg(ui, self.hab_sup, &rootfs)?;
        let launcher = self.install_base_pkg(ui, self.hab_launcher, &rootfs)?;
        let busybox = if cfg!(target_os = "linux") {
            Some(self.install_base_pkg(ui, BUSYBOX_IDENT, &rootfs)?)
        } else {
            None
        };

        Ok(BasePkgIdents { hab,
                           sup,
                           launcher,
                           busybox })
    }

    fn install_base_pkg<P: AsRef<Path>>(&self,
                                        ui: &mut UI,
                                        ident_or_archive: &str,
                                        fs_root_path: P)
                                        -> Result<PackageIdent> {
        self.install(ui,
                     ident_or_archive,
                     self.base_pkgs_url,
                     &self.base_pkgs_channel,
                     fs_root_path)
    }

    fn install_user_pkg<P: AsRef<Path>>(&self,
                                        ui: &mut UI,
                                        ident_or_archive: &str,
                                        fs_root_path: P)
                                        -> Result<PackageIdent> {
        self.install(ui, ident_or_archive, self.url, &self.channel, fs_root_path)
    }

    fn install<P: AsRef<Path>>(&self,
                               ui: &mut UI,
                               ident_or_archive: &str,
                               url: &str,
                               channel: &ChannelIdent,
                               fs_root_path: P)
                               -> Result<PackageIdent> {
        let install_source: InstallSource = ident_or_archive.parse()?;
        let package_install =
            common::command::package::install::start(ui,
                                                     url,
                                                     channel,
                                                     &install_source,
                                                     &*PROGRAM_NAME,
                                                     VERSION,
                                                     &fs_root_path,
                                                     &cache_artifact_path(Some(&fs_root_path)),
                                                     None,
                                                     // TODO fn: pass through and enable offline
                                                     // install mode
                                                     &InstallMode::default(),
                                                     // TODO (CM): pass through and enable
                                                     // ignore-local mode
                                                     &LocalPackageUsage::default(),
                                                     InstallHookMode::Ignore)?;
        Ok(package_install.into())
    }

    fn remove_symlink_to_artifact_cache<P: AsRef<Path>>(&self,
                                                        ui: &mut UI,
                                                        rootfs: P)
                                                        -> Result<()> {
        ui.status(Status::Deleting, "artifact cache symlink")?;
        stdfs::remove_dir_all(rootfs.as_ref().join(CACHE_ARTIFACT_PATH))?;
        Ok(())
    }

    fn remove_symlink_to_key_cache<P: AsRef<Path>>(&self, ui: &mut UI, rootfs: P) -> Result<()> {
        ui.status(Status::Deleting, "artifact key symlink")?;
        stdfs::remove_dir_all(rootfs.as_ref().join(CACHE_KEY_PATH))?;

        Ok(())
    }
}

/// The package identifiers for installed base packages.
#[derive(Debug)]
struct BasePkgIdents {
    /// Installed package identifer for the Habitat CLI package.
    pub hab: PackageIdent,
    /// Installed package identifer for the Supervisor package.
    pub sup: PackageIdent,
    /// Installed package identifer for the Launcher package.
    pub launcher: PackageIdent,
    /// Installed package identifer for the Busybox package.
    pub busybox: Option<PackageIdent>,
}
