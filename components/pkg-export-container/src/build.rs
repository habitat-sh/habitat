// TODO: agadgil: This module is cluttered too much with `cfg(unix)` and `cfg(windows)` like config
// conditionals, possibly better to have a `build_windows` and `build_linux` kind of modules. Will
// make the code readable and will remove this cluttering

use crate::{error::Error,
            graph::Graph,
            util,
            BUSYBOX_IDENT,
            CACERTS_IDENT,
            VERSION};

use anyhow::Result;
use clap::{self,
           ArgMatches};
use hab::license;
use habitat_common::{cfg_unix,
                     cfg_windows,
                     command::package::install::{InstallHookMode,
                                                 InstallMode,
                                                 InstallSource,
                                                 LocalPackageUsage},
                     ui::{Status,
                          UIWriter,
                          UI},
                     PROGRAM_NAME};

cfg_unix! {
    use crate::rootfs;

    use crate::accounts::{EtcGroupEntry,
                      EtcPasswdEntry};

    use anyhow::anyhow;

    use std::os::unix::fs::symlink;
}

cfg_windows! {
    use habitat_core::util::docker;

    use std::os::windows::fs::symlink_dir as symlink;
}

use habitat_core::{env,
                   fs::{cache_artifact_path,
                        CACHE_ARTIFACT_PATH,
                        CACHE_KEY_PATH,
                        CACHE_KEY_PATH_POSTFIX},
                   package::{FullyQualifiedPackageIdent,
                             PackageArchive,
                             PackageIdent,
                             PackageInstall},
                   ChannelIdent};
use log::debug;

use std::{collections::HashMap,
          convert::TryFrom,
          fs as stdfs,
          path::{Path,
                 PathBuf},
          str::FromStr};
use tempfile::TempDir;

// Much of this functionality is duplicated (or slightly modified)
// in the tar exporter. This needs to be abstacted out in
// the future for use with further exporters.
// https://github.com/habitat-sh/habitat/issues/4522

#[allow(clippy::unnecessary_wraps)]
fn default_base_image() -> Result<String> {
    #[cfg(unix)]
    {
        Ok(super::DEFAULT_BASE_IMAGE.to_string())
    }

    #[cfg(windows)]
    {
        Ok(format!("{}:{}",
                   super::DEFAULT_BASE_IMAGE,
                   docker::default_base_tag_for_host()?))
    }
}

/// The specification for creating a temporary file system build root, based on Habitat packages.
///
/// When a `BuildSpec` is created, a `BuildRoot` is returned which can be used to produce exported
/// images, archives, etc.
#[derive(Debug)]
pub(crate) struct BuildSpec {
    /// A string representation of a Habitat Package Identifer for the Habitat CLI package.
    hab: String,

    /// A string representation of a Habitat Package Identifer for the Habitat Launcher package.
    hab_launcher: String,

    /// A string representation of a Habitat Package Identifer for the Habitat Supervisor package.
    hab_sup: String,

    /// The Builder URL which is used to install all service and extra Habitat packages.
    url: String,

    /// The Habitat release channel which is used to install all service and extra Habitat
    /// packages.
    channel: ChannelIdent,

    /// The Builder URL which is used to install all base Habitat packages.
    base_pkgs_url: String,

    /// The Habitat release channel which is used to install all base Habitat packages.
    base_pkgs_channel: ChannelIdent,

    // TODO: This pub(crate) can potentially be removed
    /// A list of either Habitat Package Identifiers or local paths to Habitat Artifact files which
    /// will be installed.
    pub(crate) idents_or_archives: Vec<String>,

    /// The Builder Auth Token to use in the request
    auth: Option<String>,

    /// Base image used in Dockerfile
    base_image: String,

    /// Whether or not to create an image with a single layer for each
    /// Habitat package.
    multi_layer: bool,
}

impl TryFrom<&ArgMatches> for BuildSpec {
    type Error = crate::error::Error;

    fn try_from(m: &ArgMatches) -> std::result::Result<Self, Self::Error> {
        let base_image = match m.try_contains_id("BASE_IMAGE") {
            Ok(_) => {
                m.get_one::<String>("BASE_IMAGE")
                 .map(ToString::to_string)
                 .unwrap_or_else(|| default_base_image().expect("No base image supported"))
            }
            Err(_) => default_base_image().expect("No base image supported"),
        };

        // TODO (CM): incorporate this into our CLAP definition better
        Ok(BuildSpec { hab: m.get_one::<String>("HAB_PKG").unwrap().to_string(),
                       hab_launcher: m.get_one::<String>("HAB_LAUNCHER_PKG")
                                      .unwrap()
                                      .to_string(),
                       hab_sup: m.get_one::<String>("HAB_SUP_PKG").unwrap().to_string(),
                       url: m.get_one::<String>("BLDR_URL").unwrap().to_string(),
                       channel: m.get_one::<String>("CHANNEL")
                                 .unwrap_or(&"stable".to_string())
                                 .to_string()
                                 .into(),
                       base_pkgs_url: m.get_one::<String>("BASE_PKGS_BLDR_URL")
                                       .unwrap()
                                       .to_string(),
                       base_pkgs_channel: m.get_one::<String>("BASE_PKGS_CHANNEL")
                                           .unwrap_or(&"stable".to_string())
                                           .to_string()
                                           .into(),
                       auth: m.get_one::<String>("BLDR_AUTH_TOKEN")
                              .map(ToString::to_string),
                       idents_or_archives: m.get_many::<String>("PKG_IDENT_OR_ARTIFACT")
                                            .unwrap()
                                            .map(ToString::to_string)
                                            .collect(),
                       base_image,
                       multi_layer: m.get_flag("MULTI_LAYER") })
    }
}

impl BuildSpec {
    cfg_unix! {
        async fn prepare_rootfs(&self, ui: &mut UI, rootfs: &Path) -> Result<Graph> {
            ui.status(Status::Creating, "root filesystem")?;
            rootfs::create(rootfs)?;
            self.create_symlink_to_artifact_cache(ui, rootfs)?;
            self.create_symlink_to_key_cache(ui, rootfs)?;
            let base_pkgs = self.install_base_pkgs(ui, rootfs).await?;
            let user_pkgs = self.install_user_pkgs(ui, rootfs).await?;
            self.link_binaries(ui, rootfs, &base_pkgs)?;
            self.link_cacerts(ui, rootfs, &base_pkgs)?;
            self.link_user_pkgs(ui, rootfs, &user_pkgs)?;
            self.remove_symlink_to_key_cache(ui, rootfs)?;
            self.remove_symlink_to_artifact_cache(ui, rootfs)?;

            let graph = Graph::from_packages(base_pkgs, user_pkgs, rootfs)?;

            Ok(graph)
        }

        fn link_user_pkgs(&self,
                          ui: &mut UI,
                          rootfs: &Path,
                          user_pkgs: &[FullyQualifiedPackageIdent])
                          -> Result<()> {
            let dst = util::bin_path();
            for pkg in user_pkgs.iter() {
                hab::command::pkg::binlink::binlink_all_in_pkg(ui, pkg.as_ref(), dst, rootfs, true)
                    .map_err(|err| anyhow!("{}", err))?;
            }
            Ok(())
        }

        fn link_binaries(&self, ui: &mut UI, rootfs: &Path, base_pkgs: &BasePkgIdents) -> Result<()> {
            let dst = util::bin_path();

            hab::command::pkg::binlink::binlink_all_in_pkg(ui,
                                                           base_pkgs.busybox
                                                                    .as_ref()
                                                                    .expect("No busybox in idents")
                                                                    .as_ref(),
                                                           dst,
                                                           rootfs,
                                                           true).map_err(|err| anyhow!("{}", err))?;
            hab::command::pkg::binlink::start(ui, base_pkgs.hab.as_ref(), "hab", dst, rootfs, true)
                .map_err(|err| anyhow!("{}", err))?;
            Ok(())
        }

        fn link_cacerts(&self, ui: &mut UI, rootfs: &Path, base_pkgs: &BasePkgIdents) -> Result<()> {
            ui.status(Status::Creating, "cacerts symlink into /etc")?;
            let src = util::pkg_path_for(base_pkgs.cacerts.as_ref(), rootfs)?.join("ssl");
            let dst = rootfs.join("etc").join("ssl");
            stdfs::create_dir_all(dst.parent().expect("parent directory exists"))?;
            debug!("Symlinking src: {} to dst: {}",
                   src.display(),
                   dst.display());
            symlink(src, dst)?;

            Ok(())
        }
    }

    /// Creates a `BuildRoot` for the given specification.
    ///
    /// # Errors
    ///
    /// * If a temporary directory cannot be created
    /// * If the root file system cannot be created
    /// * If the `BuildRootContext` cannot be created
    pub(crate) async fn create(self, ui: &mut UI) -> Result<BuildRoot> {
        debug!("Creating BuildRoot from {:?}", &self);
        let workdir = TempDir::new()?;
        let rootfs = workdir.path().join("rootfs");
        ui.status(Status::Creating,
                  format!("build root in {}", workdir.path().display()))?;
        let graph = self.prepare_rootfs(ui, &rootfs).await?;
        Ok(BuildRoot { workdir,
                       ctx: BuildRootContext::from_spec(&self, &rootfs)?,
                       graph })
    }

    #[cfg(windows)]
    async fn prepare_rootfs(&self, ui: &mut UI, rootfs: &Path) -> Result<Graph> {
        ui.status(Status::Creating, "root filesystem")?;
        self.create_symlink_to_artifact_cache(ui, rootfs)?;
        self.create_symlink_to_key_cache(ui, rootfs)?;
        let base_pkgs = self.install_base_pkgs(ui, rootfs).await?;
        let user_pkgs = self.install_user_pkgs(ui, rootfs).await?;
        self.remove_symlink_to_key_cache(ui, rootfs)?;
        self.remove_symlink_to_artifact_cache(ui, rootfs)?;

        let graph = Graph::from_packages(base_pkgs, user_pkgs, rootfs)?;

        Ok(graph)
    }

    fn create_symlink_to_artifact_cache(&self, ui: &mut UI, rootfs: &Path) -> Result<()> {
        ui.status(Status::Creating, "artifact cache symlink")?;
        let src = cache_artifact_path(None::<&Path>);
        let dst = rootfs.join(CACHE_ARTIFACT_PATH);
        stdfs::create_dir_all(dst.parent().expect("parent directory exists"))?;
        debug!("Symlinking src: {} to dst: {}",
               src.display(),
               dst.display());

        symlink(src, dst)?;
        Ok(())
    }

    fn create_symlink_to_key_cache(&self, ui: &mut UI, rootfs: &Path) -> Result<()> {
        ui.status(Status::Creating, "key cache symlink")?;
        let src = &*CACHE_KEY_PATH;
        let dst = rootfs.join(CACHE_KEY_PATH_POSTFIX);
        stdfs::create_dir_all(dst.parent().expect("parent directory exists"))?;
        debug!("Symlinking src: {} to dst: {}",
               src.display(),
               dst.display());

        symlink(src, dst)?;
        Ok(())
    }

    async fn install_base_pkgs(&self, ui: &mut UI, rootfs: &Path) -> Result<BasePkgIdents> {
        let hab = self.install_base_pkg(ui, &self.hab, rootfs).await?;
        let sup = self.install_base_pkg(ui, &self.hab_sup, rootfs).await?;
        let launcher = self.install_base_pkg(ui, &self.hab_launcher, rootfs)
                           .await?;

        // TODO (CM): at some point these should be considered as
        // something other than "base" packages... replacing busybox
        // and cacerts isn't really something that's going to need to
        // be done
        let busybox = if cfg!(target_os = "linux") {
            Some(self.install_stable_pkg(ui, BUSYBOX_IDENT, rootfs).await?)
        } else {
            None
        };
        let cacerts = self.install_stable_pkg(ui, CACERTS_IDENT, rootfs).await?;

        Ok(BasePkgIdents { hab,
                           sup,
                           launcher,
                           busybox,
                           cacerts })
    }

    async fn install_user_pkgs(&self,
                               ui: &mut UI,
                               rootfs: &Path)
                               -> Result<Vec<FullyQualifiedPackageIdent>> {
        let mut idents = Vec::new();
        for ioa in self.idents_or_archives.iter() {
            idents.push(self.install_user_pkg(ui, ioa, rootfs).await?);
        }

        Ok(idents)
    }

    fn remove_symlink_to_artifact_cache(&self, ui: &mut UI, rootfs: &Path) -> Result<()> {
        ui.status(Status::Deleting, "artifact cache symlink")?;
        stdfs::remove_dir_all(rootfs.join(CACHE_ARTIFACT_PATH))?;
        Ok(())
    }

    fn remove_symlink_to_key_cache(&self, ui: &mut UI, rootfs: &Path) -> Result<()> {
        ui.status(Status::Deleting, "artifact key symlink")?;
        stdfs::remove_dir_all(rootfs.join(CACHE_KEY_PATH_POSTFIX))?;

        Ok(())
    }

    async fn install_base_pkg(&self,
                              ui: &mut UI,
                              ident_or_archive: &str,
                              fs_root_path: &Path)
                              -> Result<FullyQualifiedPackageIdent> {
        self.install(ui,
                     ident_or_archive,
                     &self.base_pkgs_url,
                     &self.base_pkgs_channel,
                     fs_root_path,
                     None)
            .await
    }

    async fn install_stable_pkg(&self,
                                ui: &mut UI,
                                ident_or_archive: &str,
                                fs_root_path: &Path)
                                -> Result<FullyQualifiedPackageIdent> {
        self.install(ui,
                     ident_or_archive,
                     &self.base_pkgs_url,
                     &ChannelIdent::stable(),
                     fs_root_path,
                     None)
            .await
    }

    async fn install_user_pkg(&self,
                              ui: &mut UI,
                              ident_or_archive: &str,
                              fs_root_path: &Path)
                              -> Result<FullyQualifiedPackageIdent> {
        self.install(ui,
                     ident_or_archive,
                     &self.url,
                     &self.channel,
                     fs_root_path,
                     self.auth.as_deref())
            .await
    }

    async fn install(&self,
                     ui: &mut UI,
                     ident_or_archive: &str,
                     url: &str,
                     channel: &ChannelIdent,
                     fs_root_path: &Path,
                     token: Option<&str>)
                     -> Result<FullyQualifiedPackageIdent> {
        let install_source: InstallSource = ident_or_archive.parse()?;
        let package_install =
            habitat_common::command::package::install::start(ui,
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

        // TODO (CM): Ideally, the typing of PackageInstall would be
        // such that we'd automatically get a
        // FullyQualifiedPackageIdent, but that's a change that will
        // have larger impact on the code base. For now, we can handle
        // the conversion locally, and then remove that once the
        // broader refactoring has occurred.
        let ident: PackageIdent = package_install.into();
        Ok(FullyQualifiedPackageIdent::try_from(ident).expect("should always be a \
                                                               fully-qualified identifier"))
    }
}

/// A temporary file system build root, based on Habitat packages.
pub(crate) struct BuildRoot {
    /// The temporary directory under which all root file system and other related files and
    /// directories will be created.
    workdir: TempDir,
    /// The build root context containing information about Habitat packages, `PATH` info, etc.
    ctx:     BuildRootContext,
    /// Dependency graph of the Habitat packages installed in the
    /// build root
    graph:   Graph,
}

impl BuildRoot {
    /// Returns the temporary work directory under which a root file system has been created.
    pub(crate) fn workdir(&self) -> &Path { self.workdir.path() }

    /// Returns the `BuildRootContext` for this build root.
    pub(crate) fn ctx(&self) -> &BuildRootContext { &self.ctx }

    pub(crate) fn graph(&self) -> &Graph { &self.graph }

    /// Destroys the temporary build root.
    ///
    /// Note that the `BuildRoot` will automatically destroy itself when it falls out of scope, so
    /// a call to this method is not required, but calling this will provide more user-facing
    /// progress and error reporting.
    ///
    /// # Errors
    ///
    /// * If the temporary work directory cannot be removed
    pub(crate) fn destroy(self, ui: &mut UI) -> Result<()> {
        ui.status(Status::Deleting, "temporary files")?;
        self.workdir.close()?;

        Ok(())
    }
}

/// The file system contents, location, Habitat packages, and other context for a build root.
#[derive(Debug)]
pub(crate) struct BuildRootContext {
    /// A list of all Habitat service and library packages which were determined from the original
    /// list in a `BuildSpec`.
    idents: Vec<PkgIdentType>,

    /// List of environment variables that can overload configuration.
    pub(crate) environment: HashMap<String, String>,

    #[cfg(unix)]
    /// The `bin` path which will be used for all program symlinking.
    bin_path: PathBuf,

    /// A string representation of the build root's `PATH` environment variable value (i.e. a
    /// colon-delimited `PATH` string).
    env_path: String,

    /// The channel name which was used to install all user-provided Habitat service and library
    /// packages.
    channel: ChannelIdent,

    /// The path to the root of the file system.
    rootfs: PathBuf,

    /// Base image used in Dockerfile
    base_image: String,

    /// Whether or not to create an image with a single layer for each
    /// Habitat package.
    multi_layer: bool,
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
    fn from_spec<P: Into<PathBuf>>(spec: &BuildSpec, rootfs: P) -> Result<Self> {
        let rootfs = rootfs.into();
        let mut idents = Vec::new();
        let mut tdeps = Vec::new();
        for ident_or_archive in &spec.idents_or_archives {
            let ident = if Path::new(ident_or_archive).is_file() {
                // We're going to use the `$pkg_origin/$pkg_name`, fuzzy form of a package
                // identifier to ensure that update strategies will work if desired
                let mut archive_ident = PackageArchive::new(ident_or_archive)?.ident()?;
                archive_ident.version = None;
                archive_ident.release = None;
                archive_ident
            } else {
                PackageIdent::from_str(ident_or_archive)?
            };
            let pkg_install = PackageInstall::load(&ident, Some(&rootfs))?;
            tdeps.push(ident.name.clone());
            for dependency in pkg_install.tdeps()? {
                tdeps.push(dependency.name);
            }
            if pkg_install.is_runnable() {
                idents.push(PkgIdentType::Svc(SvcIdent { ident,
                                                         exposes: pkg_install.exposes()? }));
            } else {
                idents.push(PkgIdentType::Lib(ident));
            }
        }

        tdeps.dedup();
        let mut environment: HashMap<String, String> =
            tdeps.into_iter()
                 .filter_map(|dep| {
                     let key = format!("HAB_{}", dep.to_uppercase());
                     env::var(&key).ok().map(|v| (key, v))
                 })
                 .collect();

        if license::check_for_license_acceptance().unwrap_or_default()
                                                  .accepted()
        {
            environment.insert(String::from("HAB_LICENSE"),
                               String::from("accept-no-persist"));
        }

        let bin_path = util::bin_path();

        let context = BuildRootContext { idents,
                                         environment,
                                         #[cfg(unix)]
                                         bin_path: bin_path.into(),
                                         env_path: bin_path.to_string_lossy().into_owned(),
                                         channel: spec.channel.clone(),
                                         rootfs,
                                         base_image: spec.base_image.clone(),
                                         multi_layer: spec.multi_layer };
        context.validate()?;

        Ok(context)
    }

    /// Returns a list of all provided Habitat packages which contain a runnable service.
    pub(crate) fn svc_idents(&self) -> Vec<&PackageIdent> {
        self.idents
            .iter()
            .filter_map(|t| {
                match *t {
                    PkgIdentType::Svc(ref svc) => Some(svc.ident.as_ref()),
                    _ => None,
                }
            })
            .collect()
    }

    /// Returns the first service package from the provided Habitat packages.
    pub(crate) fn primary_svc_ident(&self) -> &PackageIdent {
        self.svc_idents()
            .first()
            .expect("Primary service package was confirmed")
    }

    fn primary_svc(&self) -> Result<PackageInstall> {
        PackageInstall::load(self.primary_svc_ident(), Some(&self.rootfs)).map_err(From::from)
    }

    /// Returns the fully qualified Package Identifier for the first service package.
    ///
    /// # Errors
    ///
    /// * If the primary service package could not be loaded from disk
    pub(crate) fn installed_primary_svc_ident(&self) -> Result<FullyQualifiedPackageIdent> {
        let pkg_install = self.primary_svc()?;
        Ok(FullyQualifiedPackageIdent::try_from(
                pkg_install
                .ident())
            .expect("We should always have a fully-qualified \
                    package identifier at this point"))
    }

    /// Returns the list of package port exposes over all service packages.
    pub(crate) fn svc_exposes(&self) -> Vec<&str> {
        let mut exposes = Vec::new();
        for svc in self.idents.iter().filter_map(|t| {
                                         match *t {
                                             PkgIdentType::Svc(ref svc) => Some(svc),
                                             _ => None,
                                         }
                                     })
        {
            let pkg_exposes_vec: Vec<&str> = svc.exposes.iter().map(String::as_ref).collect();
            exposes.extend_from_slice(&pkg_exposes_vec);
        }
        exposes
    }

    /// Returns a tuple of users to be added to the image's passwd database and groups to be added
    /// to the image's group database.
    #[cfg(unix)]
    pub(crate) fn svc_users_and_groups(&self) -> Result<(Vec<EtcPasswdEntry>, Vec<EtcGroupEntry>)> {
        let mut users = Vec::new();
        let mut groups = Vec::new();
        let uid = super::DEFAULT_USER_AND_GROUP_ID;
        let gid = super::DEFAULT_USER_AND_GROUP_ID;

        let pkg = self.primary_svc()?;
        let user_name = pkg.svc_user()
                           .unwrap_or_default()
                           .unwrap_or_else(|| String::from("hab"));
        let group_name = pkg.svc_group()
                            .unwrap_or_default()
                            .unwrap_or_else(|| String::from("hab"));

        // TODO: In some cases, packages based on core/nginx and
        // core/httpd (and possibly others) will not work, because
        // they specify a SVC_USER of `root`, but implicitly rely on a
        // `hab` user being present for running lower-privileged
        // worker processes. Habitat currently doesn't have a way to
        // formally represent this, so until it does, we should make
        // sure that there is a `hab` user and group present, just in
        // case.
        //
        // With recent changes to the Supervisor, this hab user must
        // be in the hab group for these packages to function
        // properly, but only in the case that the `hab` user is
        // being used in this back-channel kind of way. In general,
        // there is no requirement that a user be in any specific
        // group. In particular, there is no requirement that
        // SVC_GROUP be the primary group of SVC_USER, or that
        // SVC_USER even need to be in SVC_GROUP at all.
        //
        // When we can represent this multi-user situation better, we
        // should be able to clean up some of this code (because it's
        // a bit gnarly!) and not have to add an implicit hab user or
        // group.
        //
        // NOTE: If this logic ever needs to get ANY more complex, it'd
        // probably be better to encapsulate user and group management
        // behind some "useradd" and "groupadd" facade functions that
        // manage some internal representation and render the
        // /etc/passwd and /etc/group files at the end, rather than
        // trying to directly manage those files' contents.

        // Since we're potentially going to have to create an extra
        // hab user and/or group, they're going to need
        // identifiers. If SVC_USER or SVC_GROUP is hab, then we'll
        // use the IDs given by the user. On the other hand, if we're
        // adding either one of those on top of SVC_USER/SVC_GROUP,
        // then we'll use a default, incremented by one on the off
        // chance that matches what the user specified on the command
        // line.
        let hab_uid = if uid == super::DEFAULT_HAB_UID {
            super::DEFAULT_HAB_UID + 1
        } else {
            super::DEFAULT_HAB_UID
        };
        let hab_gid = if gid == super::DEFAULT_HAB_GID {
            super::DEFAULT_HAB_GID + 1
        } else {
            super::DEFAULT_HAB_GID
        };

        match (user_name.as_ref(), group_name.as_ref()) {
            ("root", "root") => {
                // SVC_GROUP is SVC_USER's primary group (trivially)

                // Just create a hab user in a hab group for safety
                users.push(EtcPasswdEntry::new("hab", hab_uid, hab_gid));
                groups.push(EtcGroupEntry::group_with_users("hab", hab_gid, &["hab"]));
            }
            ("root", "hab") => {
                // SVC_GROUP is NOT SVC_USER's primary group

                // Currently, this is the anticipated case for nginx
                // and httpd packages... the lower-privileged hab user
                // needs to be in the hab group for things to work.
                users.push(EtcPasswdEntry::new("hab", hab_uid, gid));
                groups.push(EtcGroupEntry::group_with_users("hab", gid, &["hab"]));
            }
            ("root", _) => {
                // SVC_GROUP is NOT SVC_USER's primary group
                // (trivially)
                //
                // No user is in SVC_GROUP, actually
                groups.push(EtcGroupEntry::empty_group(&group_name, gid));

                // Just create a hab user in a hab group for safety
                users.push(EtcPasswdEntry::new("hab", hab_uid, hab_gid));
                groups.push(EtcGroupEntry::group_with_users("hab", hab_gid, &["hab"]));
            }
            ("hab", "hab") => {
                // If the user explicitly called for hab/hab, give it
                // to them.
                //
                // Strictly speaking, SVC_USER does not need to be in
                // SVC_GROUP, but if we're making a user, we need to
                // put them in *some* group.
                users.push(EtcPasswdEntry::new("hab", uid, gid));
                groups.push(EtcGroupEntry::group_with_users("hab", gid, &["hab"]));
            }
            ("hab", "root") => {
                // SVC_GROUP is NOT SVC_USER's primary group

                // To prevent having to edit the root group entry,
                // we'll just add the hab user to the hab group to put
                // them someplace.
                users.push(EtcPasswdEntry::new("hab", uid, hab_gid));
                groups.push(EtcGroupEntry::group_with_users("hab", hab_gid, &["hab"]));
            }
            ("hab", _) => {
                // SVC_GROUP IS SVC_USER's primary group, and there is
                // NO hab group

                // Again, sticking the hab user into the group because
                // it needs to go somewhere
                users.push(EtcPasswdEntry::new("hab", uid, gid));
                groups.push(EtcGroupEntry::group_with_users(&group_name, gid, &["hab"]));
            }
            (..) => {
                // SVC_GROUP IS SVC_USER's primary group, because it
                // has to go somewhere
                users.push(EtcPasswdEntry::new(&user_name, uid, gid));
                groups.push(EtcGroupEntry::group_with_users(&group_name, gid, &[&user_name]));

                // Just create a hab user in a hab group for safety
                users.push(EtcPasswdEntry::new("hab", hab_uid, hab_gid));
                groups.push(EtcGroupEntry::group_with_users("hab", hab_gid, &["hab"]));
            }
        }

        // TODO fn: add remaining missing users and groups from service packages

        Ok((users, groups))
    }

    /// Returns the `bin` path which is used for all program symlinking.
    #[cfg(unix)]
    pub(crate) fn bin_path(&self) -> &Path { self.bin_path.as_ref() }

    /// Returns a colon-delimited `PATH` string containing all important program paths.
    pub(crate) fn env_path(&self) -> &str { self.env_path.as_str() }

    /// Returns the release channel name used to install all provided Habitat packages.
    pub(crate) fn channel(&self) -> &ChannelIdent { &self.channel }

    /// Returns the root file system which is used to export an image.
    pub(crate) fn rootfs(&self) -> &Path { self.rootfs.as_ref() }

    /// Returns the base image used in the Dockerfile
    pub(crate) fn base_image(&self) -> &str { self.base_image.as_str() }

    pub(crate) fn multi_layer(&self) -> bool { self.multi_layer }

    fn validate(&self) -> Result<()> {
        // A valid context for a build root will contain at least one service package, called the
        // primary service package.
        if self.svc_idents().first().is_none() {
            return Err(Error::PrimaryServicePackageNotFound(self.idents
                                                                .iter()
                                                                .map(|e| e.ident().to_string())
                                                                .collect()).into());
        }

        Ok(())
    }
}

/// The package identifiers for installed base packages.
#[derive(Debug)]
pub(crate) struct BasePkgIdents {
    /// Installed package identifer for the Habitat CLI package.
    pub(crate) hab: FullyQualifiedPackageIdent,

    /// Installed package identifer for the Supervisor package.
    pub(crate) sup: FullyQualifiedPackageIdent,

    /// Installed package identifer for the Launcher package.
    pub(crate) launcher: FullyQualifiedPackageIdent,

    /// Installed package identifer for the Busybox package.
    pub(crate) busybox: Option<FullyQualifiedPackageIdent>,

    /// Installed package identifer for the CA certs package.
    pub(crate) cacerts: FullyQualifiedPackageIdent,
}

/// A service identifier representing a Habitat package which contains a runnable service.
#[derive(Debug)]
struct SvcIdent {
    /// The Package Identifier.
    ident:   PackageIdent,
    /// A list of all port exposes for the package.
    exposes: Vec<String>,
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
    pub(crate) fn ident(&self) -> &PackageIdent {
        match *self {
            PkgIdentType::Svc(ref svc) => &svc.ident,
            PkgIdentType::Lib(ref ident) => ident,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    fn build_spec() -> BuildSpec {
        BuildSpec { hab:                "hab".to_string(),
                    hab_launcher:       "hab_launcher".to_string(),
                    hab_sup:            "hab_sup".to_string(),
                    url:                "url".to_string(),
                    channel:            ChannelIdent::from("channel"),
                    base_pkgs_url:      "base_pkgs_url".to_string(),
                    base_pkgs_channel:  ChannelIdent::from("base_pkgs_channel"),
                    idents_or_archives: Vec::new(),
                    auth:               Some("heresafakeauthtokenduh".to_string()),
                    base_image:         "scratch".to_string(),
                    multi_layer:        false, }
    }

    cfg_unix! {
        use habitat_core::{fs,
                       package::{FullyQualifiedPackageIdent,
                                 PackageTarget}};

        use clap::ArgMatches;

        /// Generate Clap ArgMatches for the exporter from a vector of arguments.
        fn arg_matches(args: &[&str]) -> ArgMatches {
            let app = crate::cli::cli();
            app.get_matches_from(args)
        }

        struct FakePkg {
            ident:     String,
            bins:      Vec<String>,
            is_svc:    bool,
            rootfs:    PathBuf,
            svc_user:  String,
            svc_group: String,
        }

        impl FakePkg {
            fn new<P>(ident: &str, rootfs: P) -> FakePkg
                where P: AsRef<Path>
            {
                FakePkg { ident:     ident.to_string(),
                          bins:      Vec::new(),
                          is_svc:    false,
                          rootfs:    rootfs.as_ref().to_path_buf(),
                          svc_user:  "my_user".to_string(),
                          svc_group: "my_group".to_string(), }
            }

            #[cfg(not(windows))]
            fn add_bin(&mut self, bin: &str) -> &mut FakePkg {
                self.bins.push(bin.to_string());
                self
            }

            fn set_svc(&mut self, is_svc: bool) -> &mut FakePkg {
                self.is_svc = is_svc;
                self
            }

            fn set_svc_user(&mut self, svc_user: &str) -> &mut FakePkg {
                self.svc_user = svc_user.to_string();
                self
            }

            fn set_svc_group(&mut self, svc_group: &str) -> &mut FakePkg {
                self.svc_group = svc_group.to_string();
                self
            }

            fn install(&self) -> FullyQualifiedPackageIdent {
                let mut ident = PackageIdent::from_str(&self.ident).unwrap();
                if ident.version.is_none() {
                    ident.version = Some("1.2.3".into());
                }
                if ident.release.is_none() {
                    ident.release = Some("21120102121200".into());
                }
                let prefix = fs::pkg_install_path(&ident, Some(self.rootfs.as_path()));
                util::write_file(prefix.join("IDENT"), &ident.to_string()).unwrap();
                util::write_file(prefix.join("TARGET"), &PackageTarget::active_target()).unwrap();

                util::write_file(prefix.join("SVC_USER"), &self.svc_user).unwrap();
                util::write_file(prefix.join("SVC_GROUP"), &self.svc_group).unwrap();

                if !self.bins.is_empty() {
                    util::write_file(prefix.join("PATH"),
                                     fs::pkg_install_path(&ident, None::<&Path>).join("bin")
                                                                                .to_string_lossy()
                                                                                .as_ref()).unwrap();
                    for bin in self.bins.iter() {
                        util::write_file(prefix.join("bin").join(bin), "").unwrap();
                    }
                }
                if self.is_svc {
                    util::write_file(prefix.join("run"), "").unwrap();
                }
                FullyQualifiedPackageIdent::try_from(ident).expect("this must always be \
                                                                    fully-qualified")
            }
        }
    }

    mod build_spec {
        use super::*;
        use habitat_common::ui::UI;
        use tempfile::TempDir;

        #[test]
        fn artifact_cache_symlink() {
            let rootfs = TempDir::new().unwrap();
            let mut ui = UI::with_sinks();
            build_spec().create_symlink_to_artifact_cache(&mut ui, rootfs.path())
                        .unwrap();
            let link = rootfs.path().join(CACHE_ARTIFACT_PATH);

            assert_eq!(cache_artifact_path(None::<&Path>),
                       link.read_link().unwrap());
        }

        #[test]
        fn key_cache_symlink() {
            let rootfs = TempDir::new().unwrap();
            let mut ui = UI::with_sinks();
            build_spec().create_symlink_to_key_cache(&mut ui, rootfs.path())
                        .unwrap();
            let link = rootfs.path().join(CACHE_KEY_PATH_POSTFIX);

            assert_eq!(*CACHE_KEY_PATH, link.read_link().unwrap());
        }

        cfg_unix! {
            #[test]
            fn link_binaries() {
                let rootfs = TempDir::new().unwrap();
                let mut ui = UI::with_sinks();
                let base_pkgs = base_pkgs(rootfs.path());
                build_spec().link_binaries(&mut ui, rootfs.path(), &base_pkgs)
                            .unwrap();

                assert_eq!(fs::pkg_install_path(base_pkgs.busybox.as_ref().unwrap().as_ref(),
                                                None::<&Path>).join("bin/busybox"),
                           rootfs.path().join("bin/busybox").read_link().unwrap(),
                           "busybox program is symlinked into /bin");
                assert_eq!(fs::pkg_install_path(base_pkgs.busybox.as_ref().unwrap().as_ref(),
                                                None::<&Path>).join("bin/sh"),
                           rootfs.path().join("bin/sh").read_link().unwrap(),
                           "busybox's sh program is symlinked into /bin");
                assert_eq!(fs::pkg_install_path(base_pkgs.hab.as_ref(), None::<&Path>).join("bin/hab"),
                           rootfs.path().join("bin/hab").read_link().unwrap(),
                           "hab program is symlinked into /bin");
            }

            #[test]
            fn link_cacerts() {
                let rootfs = TempDir::new().unwrap();
                let mut ui = UI::with_sinks();
                let base_pkgs = base_pkgs(rootfs.path());
                build_spec().link_cacerts(&mut ui, rootfs.path(), &base_pkgs)
                            .unwrap();

                assert_eq!(fs::pkg_install_path(base_pkgs.cacerts.as_ref(), None::<&Path>).join("ssl"),
                           rootfs.path().join("etc/ssl").read_link().unwrap(),
                           "cacerts are symlinked into /etc/ssl");
            }

            fn base_pkgs<P: AsRef<Path>>(rootfs: P) -> BasePkgIdents {
                BasePkgIdents { hab:      fake_hab_install(&rootfs),
                                sup:      fake_sup_install(&rootfs),
                                launcher: fake_launcher_install(&rootfs),
                                busybox:  Some(fake_busybox_install(&rootfs)),
                                cacerts:  fake_cacerts_install(&rootfs), }
            }

            fn fake_hab_install<P: AsRef<Path>>(rootfs: P) -> FullyQualifiedPackageIdent {
                FakePkg::new("acme/hab", rootfs.as_ref()).add_bin("hab")
                                                         .install()
            }

            fn fake_sup_install<P: AsRef<Path>>(rootfs: P) -> FullyQualifiedPackageIdent {
                FakePkg::new("acme/hab-sup", rootfs.as_ref()).add_bin("hab-sup")
                                                             .install()
            }

            fn fake_launcher_install<P: AsRef<Path>>(rootfs: P) -> FullyQualifiedPackageIdent {
                FakePkg::new("acme/hab-launcher", rootfs.as_ref()).add_bin("hab-launch")
                                                                  .install()
            }

            fn fake_busybox_install<P: AsRef<Path>>(rootfs: P) -> FullyQualifiedPackageIdent {
                FakePkg::new("acme/busybox", rootfs.as_ref()).add_bin("busybox")
                                                             .add_bin("sh")
                                                             .install()
            }

            fn fake_cacerts_install<P: AsRef<Path>>(rootfs: P) -> FullyQualifiedPackageIdent {
                let ident = FakePkg::new("acme/cacerts", rootfs.as_ref()).install();

                let prefix = fs::pkg_install_path(ident.as_ref(), Some(rootfs));
                util::write_file(prefix.join("ssl/cacert.pem"), "").unwrap();
                ident
            }
        }
    }

    mod build_root_context {
        // All the tests in this modules are only called for `unix` ish targets.
        #![cfg(unix)]
        // We run these tests only on Unix as such they cannot be run on windows.
        use super::*;
        use habitat_common::PROGRAM_NAME;
        use habitat_core::package::PackageIdent;
        use std::str::FromStr;

        #[test]
        fn build_context_from_a_spec() {
            let rootfs = TempDir::new().unwrap();
            let _ = FakePkg::new("acme/libby", rootfs.path()).install();

            // A couple service packages
            let runna_install_ident = FakePkg::new("acme/runna", rootfs.path()).set_svc(true)
                                                                               .install();
            let _ = FakePkg::new("acme/jogga", rootfs.path()).set_svc(true)
                                                             .install();

            let mut spec = build_spec();
            spec.idents_or_archives = vec!["acme/libby".to_string(),
                                           "acme/runna".to_string(),
                                           "acme/jogga".to_string()];
            let ctx = BuildRootContext::from_spec(&spec, rootfs.path()).unwrap();

            assert_eq!(vec![&PackageIdent::from_str("acme/runna").unwrap(),
                            &PackageIdent::from_str("acme/jogga").unwrap(),],
                       ctx.svc_idents());
            assert_eq!(&PackageIdent::from_str("acme/runna").unwrap(),
                       ctx.primary_svc_ident());
            assert_eq!(runna_install_ident,
                       ctx.installed_primary_svc_ident().unwrap());

            assert_eq!(Path::new("/bin"), ctx.bin_path());
            assert_eq!("/bin", ctx.env_path());
            assert_eq!(&spec.channel, ctx.channel());
            assert_eq!(rootfs.path(), ctx.rootfs());

            let (users, groups) = ctx.svc_users_and_groups().unwrap();
            assert_eq!(2, users.len());
            assert_eq!(users[0].name, "my_user");
            assert_eq!(users[1].name, "hab");
            assert_eq!(2, groups.len());
            assert_eq!(groups[0].name, "my_group");
            assert_eq!(groups[1].name, "hab");
            // TODO fn: check ctx.svc_exposes()
        }

        #[test]
        fn hab_user_and_group_are_created_even_if_not_explicitly_called_for() {
            let rootfs = TempDir::new().unwrap();

            let _my_package = FakePkg::new("acme/my_pkg", rootfs.path()).set_svc(true)
                                                                        .set_svc_user("root")
                                                                        .set_svc_group("root")
                                                                        .install();
            let matches = arg_matches(&[&*PROGRAM_NAME, "acme/my_pkg"]);

            let build_spec = BuildSpec::try_from(&matches).unwrap();
            let ctx = BuildRootContext::from_spec(&build_spec, rootfs.path()).unwrap();

            let (users, groups) = ctx.svc_users_and_groups().unwrap();
            assert_eq!(1, users.len());
            assert_eq!(users[0].name, "hab");
            assert_eq!(1, groups.len());
            assert_eq!(groups[0].name, "hab");
        }

        #[test]
        fn hab_user_and_group_are_created_along_with_non_root_users() {
            let rootfs = TempDir::new().unwrap();

            let _my_package =
                FakePkg::new("acme/my_pkg", rootfs.path()).set_svc(true)
                                                          .set_svc_user("somebody_else")
                                                          .set_svc_group("some_other_group")
                                                          .install();

            #[cfg(unix)]
            let matches = arg_matches(&[&*PROGRAM_NAME, "acme/my_pkg"]);
            #[cfg(windows)]
            let matches =
                arg_matches(&[&*PROGRAM_NAME, "acme/my_pkg", "--base-image", "some/image"]);

            let build_spec = BuildSpec::try_from(&matches).unwrap();
            let ctx = BuildRootContext::from_spec(&build_spec, rootfs.path()).unwrap();

            let (users, groups) = ctx.svc_users_and_groups().unwrap();
            assert_eq!(2, users.len());
            assert_eq!(users[0].name, "somebody_else");
            assert_eq!(users[1].name, "hab");
            assert_eq!(2, groups.len());
            assert_eq!(groups[0].name, "some_other_group");
            assert_eq!(groups[1].name, "hab");
        }
    }
}
