// Copyright (c) 2016-2017 Chef Software Inc. and/or applicable contributors
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

use clap;
use std::fs as stdfs;
#[cfg(target_os = "linux")]
use std::os::unix::fs::symlink;
#[cfg(target_os = "windows")]
use std::os::windows::fs::symlink_dir as symlink;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use common;
use common::command::package::install::InstallSource;
use common::ui::{UI, Status};
use hab;
use hcore::fs::{CACHE_ARTIFACT_PATH, CACHE_KEY_PATH, cache_artifact_path, cache_key_path};
use hcore::PROGRAM_NAME;
use hcore::package::{PackageArchive, PackageIdent, PackageInstall};
use tempdir::TempDir;

use error::{Error, Result};
use fs;
use rootfs;
use super::{VERSION, BUSYBOX_IDENT, CACERTS_IDENT};
use util;

const DEFAULT_HAB_IDENT: &'static str = "core/hab";
const DEFAULT_LAUNCHER_IDENT: &'static str = "core/hab-launcher";
const DEFAULT_SUP_IDENT: &'static str = "core/hab-sup";
const DEFAULT_USER_ID: u32 = 42;
const DEFAULT_GROUP_ID: u32 = DEFAULT_USER_ID;

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
    pub channel: &'a str,
    /// The Builder URL which is used to install all base Habitat packages.
    pub base_pkgs_url: &'a str,
    /// The Habitat release channel which is used to install all base Habitat packages.
    pub base_pkgs_channel: &'a str,
    /// A list of either Habitat Package Identifiers or local paths to Habitat Artifact files which
    /// will be installed.
    pub idents_or_archives: Vec<&'a str>,
    /// Numeric user ID of the user
    pub user_id: u32,
    /// Numeric user ID of the group
    pub group_id: u32,
    /// Run the container as a non-root user?
    pub non_root: bool,
}

impl<'a> BuildSpec<'a> {
    /// Creates a `BuildSpec` from cli arguments.
    pub fn new_from_cli_matches(
        m: &'a clap::ArgMatches,
        default_channel: &'a str,
        default_url: &'a str,
    ) -> Self {
        BuildSpec {
            hab: m.value_of("HAB_PKG").unwrap_or(DEFAULT_HAB_IDENT),
            hab_launcher: m.value_of("HAB_LAUNCHER_PKG").unwrap_or(
                DEFAULT_LAUNCHER_IDENT,
            ),
            hab_sup: m.value_of("HAB_SUP_PKG").unwrap_or(DEFAULT_SUP_IDENT),
            url: m.value_of("BLDR_URL").unwrap_or(&default_url),
            channel: m.value_of("CHANNEL").unwrap_or(&default_channel),
            base_pkgs_url: m.value_of("BASE_PKGS_BLDR_URL").unwrap_or(&default_url),
            base_pkgs_channel: m.value_of("BASE_PKGS_CHANNEL").unwrap_or(&default_channel),
            idents_or_archives: m.values_of("PKG_IDENT_OR_ARTIFACT")
                .expect("No package specified")
                .collect(),
            user_id: match m.value_of("USER_ID") {
                Some(i) => {
                    // unwrap OK because validation function ensures it
                    i.parse::<u32>().unwrap()
                }
                None => DEFAULT_USER_ID,
            },
            group_id: match m.value_of("GROUP_ID") {
                Some(i) => {
                    // unwrap OK because validation function ensures it
                    i.parse::<u32>().unwrap()
                }
                None => DEFAULT_GROUP_ID,
            },
            non_root: m.is_present("NON_ROOT"),
        }
    }

    /// Creates a `BuildRoot` for the given specification.
    ///
    /// # Errors
    ///
    /// * If a temporary directory cannot be created
    /// * If the root file system cannot be created
    /// * If the `BuildRootContext` cannot be created
    pub fn create(self, ui: &mut UI) -> Result<BuildRoot> {
        debug!("Creating BuildRoot from {:?}", &self);
        let workdir = TempDir::new(&*PROGRAM_NAME)?;
        let rootfs = workdir.path().join("rootfs");
        ui.status(
            Status::Creating,
            format!("build root in {}", workdir.path().display()),
        )?;
        self.prepare_rootfs(ui, &rootfs)?;
        let ctx = BuildRootContext::from_spec(&self, rootfs)?;

        Ok(BuildRoot {
            workdir: workdir,
            ctx: ctx,
        })
    }

    fn prepare_rootfs<P: AsRef<Path>>(&self, ui: &mut UI, rootfs: P) -> Result<()> {
        ui.status(Status::Creating, "root filesystem")?;
        if cfg!(target_os = "linux") {
            rootfs::create(&rootfs)?;
        }
        self.create_symlink_to_artifact_cache(ui, &rootfs)?;
        self.create_symlink_to_key_cache(ui, &rootfs)?;
        let base_pkgs = self.install_base_pkgs(ui, &rootfs)?;
        let user_pkgs = self.install_user_pkgs(ui, &rootfs)?;
        if cfg!(target_os = "linux") {
            self.link_binaries(ui, &rootfs, &base_pkgs)?;
            self.link_cacerts(ui, &rootfs, &base_pkgs)?;
            self.link_user_pkgs(ui, &rootfs, &user_pkgs)?;
        }
        self.remove_symlink_to_key_cache(ui, &rootfs)?;
        self.remove_symlink_to_artifact_cache(ui, &rootfs)?;

        Ok(())
    }

    fn create_symlink_to_artifact_cache<P: AsRef<Path>>(
        &self,
        ui: &mut UI,
        rootfs: P,
    ) -> Result<()> {
        ui.status(Status::Creating, "artifact cache symlink")?;
        let src = cache_artifact_path(None::<P>);
        let dst = rootfs.as_ref().join(CACHE_ARTIFACT_PATH);
        stdfs::create_dir_all(dst.parent().expect("parent directory exists"))?;
        debug!(
            "Symlinking src: {} to dst: {}",
            src.display(),
            dst.display()
        );

        Ok(symlink(src, dst)?)
    }

    fn create_symlink_to_key_cache<P: AsRef<Path>>(&self, ui: &mut UI, rootfs: P) -> Result<()> {
        ui.status(Status::Creating, "key cache symlink")?;
        let src = cache_key_path(None::<P>);
        let dst = rootfs.as_ref().join(CACHE_KEY_PATH);
        stdfs::create_dir_all(dst.parent().expect("parent directory exists"))?;
        debug!(
            "Symlinking src: {} to dst: {}",
            src.display(),
            dst.display()
        );

        Ok(symlink(src, dst)?)
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
        let cacerts = self.install_base_pkg(ui, CACERTS_IDENT, &rootfs)?;

        Ok(BasePkgIdents {
            hab,
            sup,
            launcher,
            busybox,
            cacerts,
        })
    }

    fn install_user_pkgs<P: AsRef<Path>>(
        &self,
        ui: &mut UI,
        rootfs: P,
    ) -> Result<Vec<PackageIdent>> {
        let mut idents = Vec::new();
        for ioa in self.idents_or_archives.iter() {
            idents.push(self.install_user_pkg(ui, ioa, &rootfs)?);
        }

        Ok(idents)
    }

    fn link_user_pkgs<P: AsRef<Path>>(
        &self,
        ui: &mut UI,
        rootfs: P,
        user_pkgs: &Vec<PackageIdent>,
    ) -> Result<()> {
        let dst = util::bin_path();
        for pkg in user_pkgs.iter() {
            hab::command::pkg::binlink::binlink_all_in_pkg(ui, &pkg, &dst, rootfs.as_ref(), true)?;
        }

        Ok(())
    }

    fn link_binaries<P: AsRef<Path>>(
        &self,
        ui: &mut UI,
        rootfs: P,
        base_pkgs: &BasePkgIdents,
    ) -> Result<()> {
        let dst = util::bin_path();
        hab::command::pkg::binlink::binlink_all_in_pkg(
            ui,
            &base_pkgs.busybox.clone().expect("No busybox in idents"),
            &dst,
            rootfs.as_ref(),
            true,
        )?;
        hab::command::pkg::binlink::start(ui, &base_pkgs.hab, "hab", &dst, rootfs.as_ref(), true)?;

        Ok(())
    }

    fn link_cacerts<P: AsRef<Path>>(
        &self,
        ui: &mut UI,
        rootfs: P,
        base_pkgs: &BasePkgIdents,
    ) -> Result<()> {
        ui.status(Status::Creating, "cacerts symlink into /etc")?;
        let src = util::pkg_path_for(&base_pkgs.cacerts, rootfs.as_ref())?
            .join("ssl");
        let dst = rootfs.as_ref().join("etc").join("ssl");
        stdfs::create_dir_all(dst.parent().expect("parent directory exists"))?;
        debug!(
            "Symlinking src: {} to dst: {}",
            src.display(),
            dst.display()
        );
        symlink(src, dst)?;

        Ok(())
    }

    fn remove_symlink_to_artifact_cache<P: AsRef<Path>>(
        &self,
        ui: &mut UI,
        rootfs: P,
    ) -> Result<()> {
        ui.status(Status::Deleting, "artifact cache symlink")?;
        stdfs::remove_dir_all(rootfs.as_ref().join(CACHE_ARTIFACT_PATH))?;
        Ok(())
    }

    fn remove_symlink_to_key_cache<P: AsRef<Path>>(&self, ui: &mut UI, rootfs: P) -> Result<()> {
        ui.status(Status::Deleting, "artifact key symlink")?;
        stdfs::remove_dir_all(rootfs.as_ref().join(CACHE_KEY_PATH))?;

        Ok(())
    }

    fn install_base_pkg<P: AsRef<Path>>(
        &self,
        ui: &mut UI,
        ident_or_archive: &str,
        fs_root_path: P,
    ) -> Result<PackageIdent> {
        self.install(
            ui,
            ident_or_archive,
            self.base_pkgs_url,
            self.base_pkgs_channel,
            fs_root_path,
        )
    }

    fn install_user_pkg<P: AsRef<Path>>(
        &self,
        ui: &mut UI,
        ident_or_archive: &str,
        fs_root_path: P,
    ) -> Result<PackageIdent> {
        self.install(ui, ident_or_archive, self.url, self.channel, fs_root_path)
    }

    fn install<P: AsRef<Path>>(
        &self,
        ui: &mut UI,
        ident_or_archive: &str,
        url: &str,
        channel: &str,
        fs_root_path: P,
    ) -> Result<PackageIdent> {

        let install_source: InstallSource = ident_or_archive.parse()?;
        let package_install = common::command::package::install::start(
            ui,
            url,
            Some(channel),
            &install_source,
            &*PROGRAM_NAME,
            VERSION,
            &fs_root_path,
            &cache_artifact_path(Some(&fs_root_path)),
            None,
        )?;
        Ok(package_install.into())
    }
}

/// A temporary file system build root, based on Habitat packages.
pub struct BuildRoot {
    /// The temporary directory under which all root file system and other related files and
    /// directories will be created.
    workdir: TempDir,
    /// The build root context containing information about Habitat packages, `PATH` info, etc.
    ctx: BuildRootContext,
}

impl BuildRoot {
    /// Returns the temporary work directory under which a root file system has been created.
    pub fn workdir(&self) -> &Path {
        self.workdir.path()
    }

    /// Returns the `BuildRootContext` for this build root.
    pub fn ctx(&self) -> &BuildRootContext {
        &self.ctx
    }

    /// Destroys the temporary build root.
    ///
    /// Note that the `BuildRoot` will automatically destroy itself when it falls out of scope, so
    /// a call to this method is not required, but calling this will provide more user-facing
    /// progress and error reporting.
    ///
    /// # Errors
    ///
    /// * If the temporary work directory cannot be removed
    pub fn destroy(self, ui: &mut UI) -> Result<()> {
        ui.status(Status::Deleting, "temporary files")?;
        self.workdir.close()?;

        Ok(())
    }
}

/// The file system contents, location, Habitat pacakges, and other context for a build root.
#[derive(Debug)]
pub struct BuildRootContext {
    /// A list of all Habitat service and library packages which were determined from the original
    /// list in a `BuildSpec`.
    idents: Vec<PkgIdentType>,
    /// The `bin` path which will be used for all program symlinking.
    bin_path: PathBuf,
    /// A string representation of the build root's `PATH` environment variable value (i.e. a
    /// colon-delimited `PATH` string).
    env_path: String,
    /// The channel name which was used to install all user-provided Habitat service and library
    /// packages.
    channel: String,
    /// The path to the root of the file system.
    rootfs: PathBuf,
    /// The user ID of the primary service user
    user_id: u32,
    /// The group ID of the primary service group
    group_id: u32,
    /// Whether or not the container should be tailored to run Habitat
    /// as a non-root user
    non_root: bool,
}

impl BuildRootContext {
    /// Creates a new `BuildRootContext` from a build spec.
    ///
    /// The root file system path will be used to inspect installed Habitat packages to populate
    /// metadata, determine primary service, etc.
    ///
    /// # Errors
    ///
    /// * If an artifact file cannot be read or if a Package Identifier cannot be determined
    /// * If a Package Identifier cannot be parsed from an string representation
    /// * If package metadata cannot be read
    pub fn from_spec<P: Into<PathBuf>>(spec: &BuildSpec, rootfs: P) -> Result<Self> {
        let rootfs = rootfs.into();
        let mut idents = Vec::new();
        for ident_or_archive in &spec.idents_or_archives {
            let ident = if Path::new(ident_or_archive).is_file() {
                // We're going to use the `$pkg_origin/$pkg_name`, fuzzy form of a package
                // identifier to ensure that update strategies will work if desired
                let mut archive_ident = PackageArchive::new(ident_or_archive).ident()?;
                archive_ident.version = None;
                archive_ident.release = None;
                archive_ident
            } else {
                PackageIdent::from_str(ident_or_archive)?
            };
            let pkg_install = PackageInstall::load(&ident, Some(&rootfs))?;
            if pkg_install.is_runnable() {
                idents.push(PkgIdentType::Svc(SvcIdent {
                    ident: ident,
                    exposes: pkg_install.exposes()?,
                    user: pkg_install.svc_user()?,
                    group: pkg_install.svc_group()?,
                }));
            } else {
                idents.push(PkgIdentType::Lib(ident));
            }
        }
        let bin_path = util::bin_path();

        let context = BuildRootContext {
            idents: idents,
            bin_path: bin_path.into(),
            env_path: bin_path.to_string_lossy().into_owned(),
            channel: spec.channel.into(),
            rootfs: rootfs,
            user_id: spec.user_id,
            group_id: spec.group_id,
            non_root: spec.non_root,
        };
        context.validate()?;

        Ok(context)
    }

    /// Returns a list of all provided Habitat packages which contain a runnable service.
    pub fn svc_idents(&self) -> Vec<&PackageIdent> {
        self.idents
            .iter()
            .filter_map(|t| match *t {
                PkgIdentType::Svc(ref svc) => Some(svc.ident.as_ref()),
                _ => None,
            })
            .collect()
    }

    /// Returns the first service package from the provided Habitat packages.
    pub fn primary_svc_ident(&self) -> &PackageIdent {
        self.svc_idents().first().map(|e| *e).expect(
            "Primary service package was confirmed",
        )
    }

    fn primary_svc(&self) -> Result<PackageInstall> {
        PackageInstall::load(self.primary_svc_ident(), Some(&self.rootfs)).map_err(From::from)
    }

    /// Returns the fully qualified Package Identifier for the first service package.
    ///
    /// # Errors
    ///
    /// * If the primary service package could not be loaded from disk
    pub fn installed_primary_svc_ident(&self) -> Result<PackageIdent> {
        let pkg_install = self.primary_svc()?;
        Ok(pkg_install.ident().clone())
    }

    /// Returns the list of package port exposes over all service packages.
    pub fn svc_exposes(&self) -> Vec<&str> {
        let mut exposes = Vec::new();
        for svc in self.idents.iter().filter_map(|t| match *t {
            PkgIdentType::Svc(ref svc) => Some(svc),
            _ => None,
        })
        {
            let pkg_exposes_vec: Vec<&str> = svc.exposes.iter().map(|e| e.as_ref()).collect();
            exposes.extend_from_slice(&pkg_exposes_vec);
        }
        exposes
    }

    /// Returns the list of package volume mount paths over all service packages.
    pub fn svc_volumes(&self) -> Vec<String> {
        let mut vols = Vec::new();
        for svc in self.svc_idents() {
            vols.push(fs::svc_data_path(&svc.name).to_string_lossy().replace(
                "\\",
                "/",
            ));
            vols.push(fs::svc_config_path(&svc.name).to_string_lossy().replace(
                "\\",
                "/",
            ));
        }
        vols
    }

    /// Returns a tuple of users to be added to the image's passwd database and groups to be added
    /// to the image's group database.
    pub fn svc_users_and_groups(&self) -> Result<(Vec<String>, Vec<String>)> {
        let mut users = Vec::new();
        let mut groups = Vec::new();
        let uid = self.user_id;
        let gid = self.group_id;

        let pkg = self.primary_svc()?;
        let user_name = pkg.svc_user().unwrap_or(Some(String::from("hab"))).unwrap();
        let group_name = pkg.svc_group()
            .unwrap_or(Some(String::from("hab")))
            .unwrap();
        if user_name != "root" {
            users.push(format!(
                "{name}:x:{uid}:{gid}:{name} User:/:/bin/false\n",
                name = user_name,
                uid = uid,
                gid = gid
            ));
            groups.push(format!(
                "{name}:x:{gid}:{user_name}\n",
                name = group_name,
                gid = gid,
                user_name = user_name
            ));
        }
        // TODO fn: add remaining missing users and groups from service packages

        Ok((users, groups))
    }

    /// Returns the `bin` path which is used for all program symlinking.
    pub fn bin_path(&self) -> &Path {
        self.bin_path.as_ref()
    }

    /// Returns a colon-delimited `PATH` string containing all important program paths.
    pub fn env_path(&self) -> &str {
        self.env_path.as_str()
    }

    /// Returns the release channel name used to install all provided Habitat packages.
    pub fn channel(&self) -> &str {
        self.channel.as_str()
    }

    /// Returns the root file system which is used to export an image.
    pub fn rootfs(&self) -> &Path {
        self.rootfs.as_ref()
    }

    pub fn primary_user_id(&self) -> u32 {
        if self.non_root { self.user_id } else { 0 }
    }

    pub fn primary_group_id(&self) -> u32 {
        if self.non_root { self.group_id } else { 0 }
    }

    fn validate(&self) -> Result<()> {
        // A valid context for a build root will contain at least one service package, called the
        // primary service package.
        if let None = self.svc_idents().first().map(|e| *e) {
            return Err(Error::PrimaryServicePackageNotFound(
                self.idents.iter().map(|e| e.ident().to_string()).collect(),
            ))?;
        }

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
    /// Installed package identifer for the CA certs package.
    pub cacerts: PackageIdent,
}

/// A service identifier representing a Habitat package which contains a runnable service.
#[derive(Debug)]
struct SvcIdent {
    /// The Package Identifier.
    pub ident: PackageIdent,
    /// A list of all port exposes for the package.
    pub exposes: Vec<String>,
    /// The service user which is required, if provided in the package
    pub user: Option<String>,
    /// The service group which is required, if provided in the package
    pub group: Option<String>,
}

/// An enum of service and library Habitat packages.
///
/// A package is considered a service package if it contains a runnable service, via a `run` hook.
#[derive(Debug)]
enum PkgIdentType {
    /// A service package which contains a runnable service.
    Svc(SvcIdent),
    /// A library package which does not contain a runnable service.
    Lib(PackageIdent),
}

impl PkgIdentType {
    /// Returns the Package Identifier for the package type.
    pub fn ident(&self) -> &PackageIdent {
        match *self {
            PkgIdentType::Svc(ref svc) => &svc.ident,
            PkgIdentType::Lib(ref ident) => &ident,
        }
    }
}

#[cfg(test)]
mod test {
    use hcore;
    use hcore::package::PackageTarget;

    use super::*;

    fn build_spec<'a>() -> BuildSpec<'a> {
        BuildSpec {
            hab: "hab",
            hab_launcher: "hab_launcher",
            hab_sup: "hab_sup",
            url: "url",
            channel: "channel",
            base_pkgs_url: "base_pkgs_url",
            base_pkgs_channel: "base_pkgs_channel",
            idents_or_archives: Vec::new(),
            user_id: 42,
            group_id: 2112,
            non_root: false,
        }
    }

    fn fake_pkg_install<P: AsRef<Path>>(
        ident: &str,
        bins: Option<Vec<&str>>,
        is_svc: bool,
        rootfs: P,
    ) -> PackageIdent {
        let mut ident = PackageIdent::from_str(ident).unwrap();
        if let None = ident.version {
            ident.version = Some("1.2.3".into());
        }
        if let None = ident.release {
            ident.release = Some("21120102121200".into());
        }
        let prefix = hcore::fs::pkg_install_path(&ident, Some(rootfs));
        util::write_file(prefix.join("IDENT"), &ident.to_string()).unwrap();
        util::write_file(prefix.join("TARGET"), &PackageTarget::default().to_string()).unwrap();

        util::write_file(prefix.join("SVC_USER"), "my_user").unwrap();
        util::write_file(prefix.join("SVC_GROUP"), "my_group").unwrap();

        if let Some(bins) = bins {
            util::write_file(
                prefix.join("PATH"),
                hcore::fs::pkg_install_path(&ident, None::<&Path>)
                    .join("bin")
                    .to_string_lossy()
                    .as_ref(),
            ).unwrap();
            for bin in bins {
                util::write_file(prefix.join("bin").join(bin), "").unwrap();
            }
        }
        if is_svc {
            util::write_file(prefix.join("run"), "").unwrap();
        }
        ident
    }

    mod build_spec {
        use std::io::{self, Cursor, Write};
        use std::sync::{Arc, RwLock};

        use common::ui::{Coloring, UI};
        use hcore;
        use tempdir::TempDir;

        use super::super::*;
        use super::*;

        #[test]
        fn artifact_cache_symlink() {
            let rootfs = TempDir::new("rootfs").unwrap();
            let (mut ui, _, _) = ui();
            build_spec()
                .create_symlink_to_artifact_cache(&mut ui, rootfs.path())
                .unwrap();
            let link = rootfs.path().join(CACHE_ARTIFACT_PATH);

            assert_eq!(
                cache_artifact_path(None::<&Path>),
                link.read_link().unwrap()
            );
        }

        #[test]
        fn key_cache_symlink() {
            let rootfs = TempDir::new("rootfs").unwrap();
            let (mut ui, _, _) = ui();
            build_spec()
                .create_symlink_to_key_cache(&mut ui, rootfs.path())
                .unwrap();
            let link = rootfs.path().join(CACHE_KEY_PATH);

            assert_eq!(cache_key_path(None::<&Path>), link.read_link().unwrap());
        }

        #[cfg(target_os = "linux")]
        #[test]
        fn link_binaries() {
            let rootfs = TempDir::new("rootfs").unwrap();
            let (mut ui, _, _) = ui();
            let base_pkgs = base_pkgs(rootfs.path());
            build_spec()
                .link_binaries(&mut ui, rootfs.path(), &base_pkgs)
                .unwrap();

            assert_eq!(
                hcore::fs::pkg_install_path(base_pkgs.busybox.as_ref().unwrap(), None::<&Path>)
                    .join("bin/busybox"),
                rootfs.path().join("bin/busybox").read_link().unwrap(),
                "busybox program is symlinked into /bin"
            );
            assert_eq!(
                hcore::fs::pkg_install_path(&base_pkgs.busybox.unwrap(), None::<&Path>)
                    .join("bin/sh"),
                rootfs.path().join("bin/sh").read_link().unwrap(),
                "busybox's sh program is symlinked into /bin"
            );
            assert_eq!(
                hcore::fs::pkg_install_path(&base_pkgs.hab, None::<&Path>).join("bin/hab"),
                rootfs.path().join("bin/hab").read_link().unwrap(),
                "hab program is symlinked into /bin"
            );
        }

        #[test]
        fn link_cacerts() {
            let rootfs = TempDir::new("rootfs").unwrap();
            let (mut ui, _, _) = ui();
            let base_pkgs = base_pkgs(rootfs.path());
            build_spec()
                .link_cacerts(&mut ui, rootfs.path(), &base_pkgs)
                .unwrap();

            assert_eq!(
                hcore::fs::pkg_install_path(&base_pkgs.cacerts, None::<&Path>).join("ssl"),
                rootfs.path().join("etc/ssl").read_link().unwrap(),
                "cacerts are symlinked into /etc/ssl"
            );
        }

        fn ui() -> (UI, OutputBuffer, OutputBuffer) {
            let stdout_buf = OutputBuffer::new();
            let stderr_buf = OutputBuffer::new();

            let ui = UI::with_streams(
                Box::new(io::empty()),
                || Box::new(stdout_buf.clone()),
                || Box::new(stderr_buf.clone()),
                Coloring::Never,
                false,
            );

            (ui, stdout_buf, stderr_buf)
        }

        fn base_pkgs<P: AsRef<Path>>(rootfs: P) -> BasePkgIdents {
            BasePkgIdents {
                hab: fake_hab_install(&rootfs),
                sup: fake_sup_install(&rootfs),
                launcher: fake_launcher_install(&rootfs),
                busybox: Some(fake_busybox_install(&rootfs)),
                cacerts: fake_cacerts_install(&rootfs),
            }
        }

        fn fake_hab_install<P: AsRef<Path>>(rootfs: P) -> PackageIdent {
            fake_pkg_install("acme/hab", Some(vec!["hab"]), false, &rootfs)
        }

        fn fake_sup_install<P: AsRef<Path>>(rootfs: P) -> PackageIdent {
            fake_pkg_install("acme/hab-sup", Some(vec!["hab-sup"]), false, &rootfs)
        }

        fn fake_launcher_install<P: AsRef<Path>>(rootfs: P) -> PackageIdent {
            fake_pkg_install(
                "acme/hab-launcher",
                Some(vec!["hab-launch"]),
                false,
                &rootfs,
            )
        }

        fn fake_busybox_install<P: AsRef<Path>>(rootfs: P) -> PackageIdent {
            fake_pkg_install("acme/busybox", Some(vec!["busybox", "sh"]), false, &rootfs)
        }

        fn fake_cacerts_install<P: AsRef<Path>>(rootfs: P) -> PackageIdent {
            let ident = fake_pkg_install("acme/cacerts", None, false, &rootfs);
            let prefix = hcore::fs::pkg_install_path(&ident, Some(rootfs));
            util::write_file(prefix.join("ssl/cacert.pem"), "").unwrap();
            ident
        }

        #[derive(Clone)]
        pub struct OutputBuffer {
            pub cursor: Arc<RwLock<Cursor<Vec<u8>>>>,
        }

        impl OutputBuffer {
            fn new() -> Self {
                OutputBuffer { cursor: Arc::new(RwLock::new(Cursor::new(Vec::new()))) }
            }
        }

        impl Write for OutputBuffer {
            fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
                self.cursor
                    .write()
                    .expect("Cursor lock is poisoned")
                    .write(buf)
            }

            fn flush(&mut self) -> io::Result<()> {
                self.cursor
                    .write()
                    .expect("Cursor lock is poisoned")
                    .flush()
            }
        }
    }

    mod build_root_context {
        use std::collections::HashSet;
        use std::iter::FromIterator;
        use std::str::FromStr;

        use hcore::package::PackageIdent;
        use hcore::fs::FS_ROOT_PATH;

        use super::super::*;
        use super::*;

        #[test]
        fn build_context_from_a_spec() {
            let rootfs = TempDir::new("rootfs").unwrap();
            // A library-only package
            let _ = fake_pkg_install("acme/libby", None, false, rootfs.path());
            // A couple service packages
            let runna_install_ident = fake_pkg_install("acme/runna", None, true, rootfs.path());
            let _ = fake_pkg_install("acme/jogga", None, true, rootfs.path());
            let mut spec = build_spec();
            spec.idents_or_archives = vec!["acme/libby", "acme/runna", "acme/jogga"];
            let ctx = BuildRootContext::from_spec(&spec, rootfs.path()).unwrap();

            assert_eq!(
                vec![
                    &PackageIdent::from_str("acme/runna").unwrap(),
                    &PackageIdent::from_str("acme/jogga").unwrap(),
                ],
                ctx.svc_idents()
            );
            assert_eq!(
                &PackageIdent::from_str("acme/runna").unwrap(),
                ctx.primary_svc_ident()
            );
            assert_eq!(
                runna_install_ident,
                ctx.installed_primary_svc_ident().unwrap()
            );
            assert_eq!(Path::new("/bin"), ctx.bin_path());
            assert_eq!("/bin", ctx.env_path());
            assert_eq!(spec.channel, ctx.channel());
            assert_eq!(rootfs.path(), ctx.rootfs());

            // Order of paths should not matter, this is why we set-compare
            let vol_paths = vec![
                (&*FS_ROOT_PATH)
                    .join("hab/svc/jogga/config")
                    .to_string_lossy()
                    .to_string(),
                (&*FS_ROOT_PATH)
                    .join("hab/svc/jogga/data")
                    .to_string_lossy()
                    .to_string(),
                (&*FS_ROOT_PATH)
                    .join("hab/svc/runna/config")
                    .to_string_lossy()
                    .to_string(),
                (&*FS_ROOT_PATH)
                    .join("hab/svc/runna/data")
                    .to_string_lossy()
                    .to_string(),
            ];
            let vol_paths: HashSet<String> = HashSet::from_iter(vol_paths.iter().cloned());
            assert_eq!(
                vol_paths,
                HashSet::from_iter(ctx.svc_volumes().iter().cloned())
            );

            let (users, groups) = ctx.svc_users_and_groups().unwrap();
            assert_eq!(1, users.len());
            assert!(users[0].starts_with("my_user:"));
            assert_eq!(1, groups.len());
            assert!(groups[0].starts_with("my_group:"));
            // TODO fn: check ctx.svc_exposes()
        }
    }
}
