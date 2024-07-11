#![allow(dead_code)]

#[cfg(any(all(target_os = "linux",
                  any(target_arch = "x86_64", target_arch = "aarch64")),
              all(target_os = "windows", target_arch = "x86_64"),))]
use super::util::ExternalCommandArgs;
use super::util::{self,
                  AuthToken,
                  BldrUrl,
                  CacheKeyPath,
                  ExternalCommandArgsWithHelpAndVersion,
                  FullyQualifiedPkgIdent,
                  PkgIdent};
use crate::cli::{dir_exists,
                 file_exists,
                 valid_ident_or_toml_file,
                 valid_origin};
use habitat_common::{cli::{BINLINK_DIR_ENVVAR,
                           DEFAULT_BINLINK_DIR,
                           PACKAGE_TARGET_ENVVAR},
                     FeatureFlag,
                     FEATURE_FLAGS};
use habitat_core::{env::Config,
                   package::{PackageIdent,
                             PackageTarget},
                   ChannelIdent};
use std::path::PathBuf;

use clap::Parser;

/// List all versions of installed packages
#[derive(Parser, Debug)]
pub struct PkgList {
    /// List all installed packages
    #[clap(name = "ALL", short = "a", long = "all", group = "prefix")]
    all: bool,

    /// An origin to list
    #[clap(name = "ORIGIN",
        short = "o",
        long = "origin",
        validator = valid_origin, group = "prefix")]
    origin: Option<String>,

    /// A package identifier (ex: core/redis, core/busybox-static/1.42.2)
    #[clap(name = "PKG_IDENT", group = "prefix")]
    pkg_ident: Option<PackageIdent>,
}

/// Prints the path to a specific installed release of a package
#[derive(Parser)]
pub struct PkgPath {
    #[command(flatten)]
    pkg_ident: PkgIdent,
}

/// Displays the binds for a service
#[derive(Parser)]
pub struct PkgBinds {
    #[command(flatten)]
    pkg_ident: PkgIdent,
}

#[derive(Parser)]
#[allow(clippy::large_enum_variant)]
/// Commands relating to Habitat packages
pub enum Pkg {
    Binds(PkgBinds),
    Binlink(PkgBinlink),
    Build(PkgBuild),
    Bulkupload(PkgBulkupload),
    Channels(PkgChannels),
    Config(PkgConfig),
    Delete(PkgDelete),
    Demote(PkgDemote),
    Dependencies(PkgDependencies),
    Download(PkgDownload),
    Env(PkgEnv),
    Exec(PkgExec),
    #[cfg(any(all(target_os = "linux",
                  any(target_arch = "x86_64", target_arch = "aarch64")),
              all(target_os = "windows", target_arch = "x86_64"),))]
    Export(ExportCommand),
    Hash(PkgHash),
    Header(PkgHeader),
    Info(PkgInfo),
    Install(PkgInstall),
    List(PkgList),
    Path(PkgPath),
    Promote(PkgPromote),
    Provides(PkgProvides),
    Search(PkgSearch),
    Sign(PkgSign),
    Uninstall(PkgUninstall),
    Upload(PkgUpload),
    Verify(PkgVerify),
}

/// Removes a package from Builder
#[derive(Parser)]
pub struct PkgDelete {
    #[command(flatten)]
    bldr_url: BldrUrl,

    #[command(flatten)]
    pkg_ident: FullyQualifiedPkgIdent,

    /// A package target (ex: x86_64-windows) (default: system appropriate target)
    #[clap(name = "PKG_TARGET", env = PACKAGE_TARGET_ENVVAR)]
    pkg_target: Option<PackageTarget>,

    #[command(flatten)]
    auth_token: AuthToken,
}

/// Demote a package from a specified channel
#[derive(Parser)]
pub struct PkgDemote {
    #[command(flatten)]
    bldr_url: BldrUrl,

    #[command(flatten)]
    pkg_ident: FullyQualifiedPkgIdent,

    /// Demote from the specified release channel
    #[clap(name = "CHANNEL")]
    channel: String,

    /// A package target (ex: x86_64-windows) (default: system appropriate target)
    #[clap(name = "PKG_TARGET", env = PACKAGE_TARGET_ENVVAR)]
    pkg_target: Option<PackageTarget>,

    #[command(flatten)]
    auth_token: AuthToken,
}

/// Promote a package to a specified channel
#[derive(Parser)]
pub struct PkgPromote {
    #[command(flatten)]
    bldr_url: BldrUrl,

    #[command(flatten)]
    pkg_ident:  FullyQualifiedPkgIdent,
    /// Promote to the specified release channel
    #[structopt(name = "CHANNEL")]
    channel:    String,
    /// A package target (ex: x86_64-windows) (default: system appropriate target)
    #[structopt(name = "PKG_TARGET", env = PACKAGE_TARGET_ENVVAR)]
    pkg_target: Option<PackageTarget>,
    #[structopt(flatten)]
    auth_token: AuthToken,
}

/// Find out what channels a package belongs to
#[derive(Parser)]
pub struct PkgChannels {
    #[command(flatten)]
    bldr_url: BldrUrl,

    #[command(flatten)]
    pkg_ident: FullyQualifiedPkgIdent,

    /// A package target (ex: x86_64-windows) (default: system appropriate target)
    #[clap(name = "PKG_TARGET", env = PACKAGE_TARGET_ENVVAR)]
    pkg_target: Option<PackageTarget>,

    #[command(flatten)]
    auth_token: AuthToken,
}

/// Displays the default configuration options for a service
#[derive(Parser)]
pub struct PkgConfig {
    #[command(flatten)]
    pkg_ident: PkgIdent,
}

/// Search installed Habitat packages for a given file
#[derive(Parser)]
pub struct PkgProvides {
    /// File name to find
    #[clap(name = "FILE")]
    file: String,

    /// Show fully qualified package names (ex: core/busybox-static/1.24.2/20160708162350)
    #[clap(name = "FULL_RELEASES", short = "r")]
    full_releases: bool,

    /// Show full path to file
    #[clap(name = "FULL_PATHS", short = "p")]
    full_paths: bool,
}

/// Search for a package in Builder
#[derive(Parser)]
pub struct PkgSearch {
    /// Search term
    #[clap(name = "SEARCH_TERM")]
    search_term: String,

    #[command(flatten)]
    bldr_url: BldrUrl,

    #[command(flatten)]
    auth_token: AuthToken,

    /// Limit how many packages to retrieve
    #[clap(name = "LIMIT", short = "l", long = "limit", default_value = "50")]
    limit: usize,
}

/// Signs an archive with an origin key, generating a Habitat Artifact
#[derive(Parser)]
pub struct PkgSign {
    /// Origin key used to create signature
    #[clap(name = "ORIGIN",
        long = "origin",
        validator = valid_origin)]
    origin: Option<String>,

    /// A path to a source archive file (ex: /home/acme-redis-3.0.7-21120102031201.tar.xz)
    #[clap(name = "SOURCE",
                validator = file_exists)]
    source: PathBuf,

    /// The destination path to the signed Habitat Artifact (ex:
    /// /home/acme-redis-3.0.7-21120102031201-x86_64-linux.hart)
    #[clap(name = "DEST")]
    dest: PathBuf,

    #[command(flatten)]
    cache_key_path: CacheKeyPath,
}

/// Safely uninstall a package and dependencies from the local filesystem
#[derive(Parser)]
pub struct PkgUninstall {
    #[command(flatten)]
    pkg_ident: PkgIdent,

    /// Just show what would be uninstalled, don't actually do it
    #[clap(name = "DRYRUN", short = "d", long = "dryrun")]
    dryrun: bool,

    /// Only keep this number of latest packages uninstalling all others.
    #[clap(name = "KEEP_LATEST", long = "keep-latest")]
    keep_latest: Option<usize>,

    /// Identifier of one or more packages that should not be uninstalled. (ex: core/redis,
    /// core/busybox-static/1.42.2/21120102031201)
    #[clap(name = "EXCLUDE", long = "exclude")]
    exclude: Vec<PackageIdent>,

    /// Don't uninstall dependencies
    #[clap(name = "NO_DEPS", long = "no-deps")]
    no_deps:               bool,
    /// Do not run any uninstall hooks
    #[clap(name = "IGNORE_UNINSTALL_HOOK", long = "ignore-uninstall-hook")]
    ignore_uninstall_hook: bool,
}

/// Uploads a local Habitat Artifact to Builder
#[derive(Parser)]
pub struct PkgUpload {
    #[command(flatten)]
    bldr_url: BldrUrl,

    #[command(flatten)]
    auth_token: AuthToken,

    /// Optional additional release channel to upload package to. Packages are always uploaded
    /// to `unstable`, regardless of the value of this option
    #[clap(name = "CHANNEL", short = "c", long = "channel")]
    channel: Option<String>,

    /// Skips checking availability of package and force uploads, potentially overwriting a
    /// stored copy of a package. (default: false)
    #[clap(name = "FORCE", long = "force")]
    force: bool,

    /// Disable auto-build for all packages in this upload
    #[clap(name = "NO_BUILD", long = "no-build")]
    no_builde: bool,

    /// One or more filepaths to a Habitat Artifact (ex:
    /// /home/acme-redis-3.0.7-21120102031201-x86_64-linux.hart)
    #[clap(name = "HART_FILE", required = true, validator = file_exists)]
    hart_file: Vec<PathBuf>,

    #[command(flatten)]
    cache_key_path: CacheKeyPath,
}

/// Verifies a Habitat Artifact with an origin key
#[derive(Parser)]
pub struct PkgVerify {
    /// A path to a Habitat Artifact (ex:
    /// /home/acme-redis-3.0.7-21120102031201-x86_64-linux.hart)
    #[clap(name = "SOURCE", validator = file_exists)]
    source: PathBuf,

    #[command(flatten)]
    cache_key_path: CacheKeyPath,
}

/// Creates a binlink for a package binary in a common 'PATH' location
#[derive(Parser)]
pub struct PkgBinlink {
    #[command(flatten)]
    pkg_ident: PkgIdent,

    /// The command to binlink (ex: bash)
    #[clap(name = "BINARY")]
    binary: Option<String>,

    /// Sets the destination directory
    #[clap(name = "DEST_DIR",
                short = "d",
                long = "dest",
                env = BINLINK_DIR_ENVVAR,
                default_value = DEFAULT_BINLINK_DIR)]
    dest_dir: PathBuf,

    /// Overwrite existing binlinks
    #[clap(name = "FORCE", short = "f", long = "force")]
    force: bool,
}

/// Builds a Plan using a Studio
#[derive(Parser)]
pub struct PkgBuild {
    /// Installs secret origin keys (ex: "unicorn", "acme,other,acme-ops")
    #[clap(name = "HAB_ORIGIN_KEYS", short = "k", long = "keys")]
    hab_origin_keys: Option<String>,

    /// Sets the Studio root (default: /hab/studios/<DIR_NAME>)
    #[clap(name = "HAB_STUDIO_ROOT", short = "r", long = "root")]
    hab_studio_root: Option<PathBuf>,

    /// Sets the source path (default: $PWD)
    #[clap(name = "SRC_PATH", short = "s", long = "src")]
    src_path: Option<PathBuf>,

    /// A directory containing a plan file or a `habitat/` directory which contains the plan
    /// file
    #[clap(name = "PLAN_CONTEXT")]
    plan_context: PathBuf,

    #[command(flatten)]
    cache_key_path: CacheKeyPath,

    /// Build a native package on the host system without a studio
    #[cfg(target_os = "linux")]
    #[clap(name = "NATIVE_PACKAGE", short = "N", long = "native-package", conflicts_with_all = &["REUSE", "DOCKER"])]
    native_package: bool,

    #[cfg(any(target_os = "linux", target_os = "windows"))]
    /// Reuses a previous Studio for the build (default: clean up before building)
    // Only a truly native/local Studio can be reused--the Docker implementation will always be
    // ephemeral
    #[clap(name = "REUSE", short = "R", long = "reuse")]
    reuse: bool,

    #[cfg(any(target_os = "linux", target_os = "windows"))]
    /// Uses a Dockerized Studio for the build
    #[clap(name = "DOCKER", short = "D", long = "docker")]
    docker: bool,
}

/// Bulk Uploads Habitat Artifacts to a Depot from a local directory
#[derive(Parser)]
pub struct PkgBulkupload {
    #[command(flatten)]
    bldr_url: BldrUrl,

    #[command(flatten)]
    auth_token: AuthToken,

    /// Optional additional release channel to upload package to. Packages are always uploaded
    /// to `unstable`, regardless of the value of this option
    #[clap(name = "CHANNEL", short = "c", long = "channel")]
    channel: Option<String>,

    /// Skip checking availability of package and force uploads, potentially overwriting a
    /// stored copy of a package
    #[clap(name = "FORCE", long = "force")]
    force: bool,

    /// Enable auto-build for all packages in this upload. Only applicable to SaaS Builder
    #[clap(name = "AUTO_BUILD", long = "auto-build")]
    auto_build: bool,

    /// Skip the confirmation prompt and automatically create origins that do not exist in the
    /// target Builder
    #[clap(name = "AUTO_CREATE_ORIGINS", long = "auto-create-origins")]
    auto_create_channels: bool,

    /// Directory Path from which artifacts will be uploaded
    #[clap(name = "UPLOAD_DIRECTORY", validator = dir_exists)]
    upload_directory: PathBuf,
}

/// Returns the Habitat Artifact dependencies. By default it will return the direct
/// dependencies of the package
#[derive(Parser)]
pub struct PkgDependencies {
    #[command(flatten)]
    pkg_ident: PkgIdent,

    /// Show transitive dependencies
    #[clap(name = "TRANSITIVE", short = "t", long = "transitive")]
    transitive: bool,

    /// Show packages which are dependant on this one
    #[clap(name = "REVERSE", short = "r", long = "reverse")]
    reverse: bool,
}

/// Download Habitat artifacts (including dependencies and keys) from Builder
#[derive(Parser)]
pub struct PkgDownload {
    #[command(flatten)]
    auth_token: AuthToken,

    #[command(flatten)]
    bldr_url: BldrUrl,

    /// Download from the specified release channel. Overridden if channel is specified in toml
    /// file
    #[clap(name = "CHANNEL",
                short = "c",
                long = "channel",
                default_value = "stable",
                env = ChannelIdent::ENVVAR)]
    channel: String,

    /// The path to store downloaded artifacts
    #[clap(name = "DOWNLOAD_DIRECTORY", long = "download-directory")]
    download_directory: Option<PathBuf>,

    /// File with newline separated package identifiers, or TOML file (ending with .toml
    /// extension)
    #[clap(name = "PKG_IDENT_FILE", long = "file", validator = valid_ident_or_toml_file)]
    pkg_ident_file: Vec<String>,

    /// One or more Habitat package identifiers (ex: acme/redis)
    #[clap(name = "PKG_IDENT")]
    pkg_ident: Vec<PackageIdent>,

    /// Target architecture to fetch. E.g. x86_64-linux. Overridden if architecture is
    /// specified in toml file
    #[clap(name = "PKG_TARGET", short = "t", long = "target")]
    pkg_target: Option<PackageTarget>,

    /// Verify package integrity after download (Warning: this can be slow)
    #[clap(name = "VERIFY", long = "verify")]
    verify: bool,

    /// Ignore packages specified that are not present on the target Builder
    #[clap(name = "IGNORE_MISSING_SEEDS", long = "ignore-missing-seeds")]
    ignore_missing_seed: bool,
}

/// Executes a command using the 'PATH' context of an installed package
#[derive(Parser)]
pub struct PkgExec {
    #[command(flatten)]
    pub pkg_ident: PkgIdent,

    /// The command to execute (ex: ls)
    #[clap(long)]
    pub cmd: PathBuf,

    #[command(flatten)]
    pub args: ExternalCommandArgsWithHelpAndVersion,
}

/// Prints the runtime environment of a specific installed package
#[derive(Parser)]
pub struct PkgEnv {
    #[command(flatten)]
    pkg_ident: PkgIdent,
}

/// Generates a blake2b hashsum from a target at any given filepath
#[derive(Parser)]
pub struct PkgHash {
    /// A filepath of the target
    #[clap(name = "SOURCE", validator = file_exists)]
    source: Option<PathBuf>,
}

/// Returns the Habitat Artifact header
#[derive(Parser)]
pub struct PkgHeader {
    /// A path to a Habitat Artifact (ex:
    /// /home/acme-redis-3.0.7-21120102031201-x86_64-linux.hart)
    #[clap(name = "SOURCE", validator = file_exists)]
    source: PathBuf,
}

/// Returns the Habitat Artifact information
#[derive(Parser)]
pub struct PkgInfo {
    /// Output will be rendered in json. (Includes extended metadata)
    #[clap(name = "TO_JSON", short = "j", long = "json")]
    to_json: bool,

    /// A path to a Habitat Artifact (ex:
    /// /home/acme-redis-3.0.7-21120102031201-x86_64-linux.hart)
    #[clap(name = "SOURCE", validator = file_exists)]
    source: PathBuf,
}

/// Installs a Habitat package from Builder or locally from a Habitat Artifact
#[derive(Parser)]
pub struct PkgInstall {
    #[command(flatten)]
    bldr_url: BldrUrl,

    /// Install from the specified release channel
    #[clap(short = "c",
                long = "channel",
                default_value = "stable",
                env = ChannelIdent::ENVVAR)]
    channel: String,

    /// One or more Habitat package identifiers (ex: acme/redis) and/or filepaths to a Habitat
    /// Artifact (ex: /home/acme-redis-3.0.7-21120102031201-x86_64-linux.hart)
    #[clap(required = true)]
    pkg_ident_or_artifact: Vec<String>,

    /// Binlink all binaries from installed package(s) into BINLINK_DIR
    #[clap(short = "b", long = "binlink")]
    binlink: bool,

    /// Binlink all binaries from installed package(s) into BINLINK_DIR
    #[clap(long = "binlink-dir",
                default_value = DEFAULT_BINLINK_DIR,
                validator = util::non_empty,
                env = BINLINK_DIR_ENVVAR)]
    binlink_dir: PathBuf,

    /// Overwrite existing binlinks
    #[clap(short = "f", long = "force")]
    force: bool,

    #[command(flatten)]
    auth_token: AuthToken,

    /// Do not run any install hooks
    #[clap(long = "ignore-install-hook")]
    ignore_install_hook: bool,

    /// Install packages in offline mode
    #[clap(long = "offline",
                hidden = !FEATURE_FLAGS.contains(FeatureFlag::OFFLINE_INSTALL))]
    offline: bool,

    /// Do not use locally-installed packages when a corresponding package cannot be installed
    /// from Builder
    #[clap(long = "ignore-local",
                hidden = !FEATURE_FLAGS.contains(FeatureFlag::IGNORE_LOCAL))]
    ignore_local: bool,
}

/// Exports the package to the specified format
#[cfg(any(all(target_os = "linux",
              any(target_arch = "x86_64", target_arch = "aarch64")),
          all(target_os = "windows", target_arch = "x86_64"),))]
#[derive(Parser)]
pub enum ExportCommand {
    #[cfg(target_os = "linux")]
    /// Cloud Foundry exporter
    Cf(ExternalCommandArgs),

    #[cfg(any(target_os = "linux", target_os = "windows"))]
    /// Container exporter
    Container(ExternalCommandArgs),

    #[cfg(any(target_os = "linux", target_os = "windows"))]
    // #[structopt(settings = &[AppSettings::Hidden])]
    Docker(ExternalCommandArgs),

    /// Mesos exporter
    #[cfg(target_os = "linux")]
    Mesos(ExternalCommandArgs),

    /// Tar exporter
    #[cfg(any(target_os = "linux", target_os = "windows"))]
    Tar(ExternalCommandArgs),
}
