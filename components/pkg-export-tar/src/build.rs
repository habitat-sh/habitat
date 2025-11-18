use super::{BUSYBOX_IDENT,
            VERSION};
use crate::{common::{self,
                     PROGRAM_NAME,
                     command::package::install::{InstallHookMode,
                                                 InstallMode,
                                                 InstallSource,
                                                 LocalPackageUsage},
                     ui::{Status,
                          UI,
                          UIWriter}},
            hcore::{ChannelIdent,
                    fs::{CACHE_ARTIFACT_PATH,
                         CACHE_KEY_PATH,
                         CACHE_KEY_PATH_POSTFIX,
                         cache_artifact_path},
                    package::PackageIdent},
            rootfs};
use anyhow::Result;
use log::debug;
#[cfg(unix)]
use std::os::unix::fs::symlink;
#[cfg(windows)]
use std::os::windows::fs::symlink_dir as symlink;
use std::{fs as stdfs,
          path::Path};
use tempfile::TempDir;

// Much of this functionality is duplicated (or slightly modified)
// from the Docker exporter. This needs to be abstacted out in
// the future for use with further exporters.
// https://github.com/habitat-sh/habitat/issues/4522

/// The specification for creating a temporary file system build root, based on Habitat packages.
///
/// When a `BuildSpec` is created, a `BuildRoot` is returned which can be used to produce exported
/// images, archives, etc.
#[derive(Debug)]
pub(crate) struct BuildSpec<'a> {
    /// A string representation of a Habitat Package Identifer for the Habitat CLI package.
    pub(crate) hab: &'a str,

    /// A string representation of a Habitat Package Identifer for the Habitat Launcher package.
    hab_launcher: &'a str,

    /// A string representation of a Habitat Package Identifer for the Habitat Supervisor package.
    hab_sup: &'a str,

    /// The Builder URL which is used to install all service and extra Habitat packages.
    url: &'a str,

    /// The Habitat release channel which is used to install all service and extra Habitat
    /// packages.
    channel: ChannelIdent,

    /// The Builder URL which is used to install all base Habitat packages.
    base_pkgs_url: &'a str,

    /// The Habitat release channel which is used to install all base Habitat packages.
    base_pkgs_channel: ChannelIdent,

    /// A Habitat Package Identifer or local path to a Habitat Artifact file which
    /// will be installed.
    ident_or_archive: &'a str,

    /// The Builder Auth Token to use in the request
    auth: Option<&'a str>,

    /// Whether to exclude the hab bin directory from the final bundle
    pub(crate) no_hab_bin: bool,

    /// Excludes supervisor and launcher packages (`chef/hab-sup` and `chef/hab-launcher`)
    /// from the tar archive. These packages work together to provide service management.
    no_hab_sup: bool,
}

impl<'a> BuildSpec<'a> {
    /// Creates a `BuildSpec` from cli arguments.
    pub(crate) fn new_from_cli(cli: &'a crate::cli::Cli) -> Self {
        BuildSpec { hab: cli.hab_pkg.as_str(),

                    hab_launcher: cli.hab_launcher_pkg.as_str(),

                    hab_sup: cli.hab_sup_pkg.as_str(),

                    url: cli.bldr_url.as_str(),

                    channel: cli.channel.as_str().into(),

                    base_pkgs_url: cli.base_pkgs_url.as_str(),

                    base_pkgs_channel: cli.base_pkgs_channel.as_str().into(),

                    auth: cli.bldr_auth_token.as_deref(),

                    ident_or_archive: cli.pkg_ident.as_str(),

                    no_hab_bin: cli.no_hab_bin,

                    no_hab_sup: cli.no_hab_sup, }
    }

    /// Creates a `BuildRoot` for the given specification.
    ///
    /// # Errors
    ///
    /// * If a temporary directory cannot be created
    /// * If the root file system cannot be created
    /// * If the `BuildRootContext` cannot be created
    pub(crate) async fn create(self, ui: &mut UI) -> Result<(TempDir, PackageIdent)> {
        let workdir = TempDir::new()?;
        let rootfs = workdir.path().join("rootfs");

        ui.status(Status::Creating,
                  format!("build root in {}", workdir.path().display()))?;

        let created_ident = self.prepare_rootfs(ui, &rootfs).await?;

        Ok((workdir, created_ident))
    }

    async fn prepare_rootfs(&self, ui: &mut UI, rootfs: &Path) -> Result<PackageIdent> {
        ui.status(Status::Creating, "root filesystem")?;
        rootfs::create(rootfs)?;
        self.create_symlink_to_artifact_cache(ui, rootfs)?;
        self.create_symlink_to_key_cache(ui, rootfs)?;
        self.install_base_pkgs(ui, rootfs).await?;
        let ident = self.install_user_pkg(ui, self.ident_or_archive, rootfs)
                        .await?;
        self.remove_symlink_to_key_cache(ui, rootfs)?;
        self.remove_symlink_to_artifact_cache(ui, rootfs)?;

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
        let src = &*CACHE_KEY_PATH;
        let dst = rootfs.as_ref().join(CACHE_KEY_PATH_POSTFIX);
        stdfs::create_dir_all(dst.parent().expect("parent directory exists"))?;
        debug!("Symlinking src: {} to dst: {}",
               src.display(),
               dst.display());

        symlink(src, dst)?;
        Ok(())
    }

    async fn install_base_pkgs(&self, ui: &mut UI, rootfs: &Path) -> Result<BasePkgIdents> {
        let hab = self.install_base_pkg(ui, self.hab, rootfs).await?;

        let (sup, launcher) = if self.no_hab_sup {
            (None, None)
        } else {
            let sup = self.install_base_pkg(ui, self.hab_sup, rootfs).await?;
            let launcher = self.install_base_pkg(ui, self.hab_launcher, rootfs).await?;
            (Some(sup), Some(launcher))
        };

        // TODO (CM): at some point this should be considered as
        // something other than a "base" package... replacing busybox
        // isn't really something that's going to need to be done.
        let busybox = if cfg!(target_os = "linux") {
            Some(self.install_stable_pkg(ui, BUSYBOX_IDENT, rootfs).await?)
        } else {
            None
        };

        Ok(BasePkgIdents { hab,
                           sup,
                           launcher,
                           busybox })
    }

    async fn install_base_pkg(&self,
                              ui: &mut UI,
                              ident_or_archive: &str,
                              fs_root_path: &Path)
                              -> Result<PackageIdent> {
        self.install(ui,
                     ident_or_archive,
                     self.base_pkgs_url,
                     &self.base_pkgs_channel,
                     fs_root_path,
                     self.auth)
            .await
    }

    async fn install_stable_pkg(&self,
                                ui: &mut UI,
                                ident_or_archive: &str,
                                fs_root_path: &Path)
                                -> Result<PackageIdent> {
        self.install(ui,
                     ident_or_archive,
                     self.base_pkgs_url,
                     &ChannelIdent::stable(),
                     fs_root_path,
                     self.auth)
            .await
    }

    async fn install_user_pkg(&self,
                              ui: &mut UI,
                              ident_or_archive: &str,
                              fs_root_path: &Path)
                              -> Result<PackageIdent> {
        self.install(ui,
                     ident_or_archive,
                     self.url,
                     &self.channel,
                     fs_root_path,
                     self.auth)
            .await
    }

    async fn install(&self,
                     ui: &mut UI,
                     ident_or_archive: &str,
                     url: &str,
                     channel: &ChannelIdent,
                     fs_root_path: &Path,
                     token: Option<&str>)
                     -> Result<PackageIdent> {
        let install_source: InstallSource = ident_or_archive.parse()?;
        let package_install =
            common::command::package::install::start(ui,
                                                     url,
                                                     channel,
                                                     &install_source,
                                                     &PROGRAM_NAME,
                                                     VERSION,
                                                     fs_root_path,
                                                     &cache_artifact_path(Some(&fs_root_path)),
                                                     token,
                                                     // TODO fn: pass through and enable offline
                                                     // install mode
                                                     &InstallMode::default(),
                                                     // TODO (CM): pass through and enable
                                                     // ignore-local mode
                                                     &LocalPackageUsage::default(),
                                                     InstallHookMode::Ignore).await?;
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
        stdfs::remove_dir_all(rootfs.as_ref().join(CACHE_KEY_PATH_POSTFIX))?;

        Ok(())
    }
}

/// The package identifiers for installed base packages.
#[derive(Debug)]
#[allow(dead_code)]
struct BasePkgIdents {
    /// Installed package identifer for the Habitat CLI package.
    hab:      PackageIdent,
    /// Installed package identifer for the Supervisor package.
    sup:      Option<PackageIdent>,
    /// Installed package identifer for the Launcher package.
    launcher: Option<PackageIdent>,
    /// Installed package identifer for the Busybox package.
    busybox:  Option<PackageIdent>,
}
