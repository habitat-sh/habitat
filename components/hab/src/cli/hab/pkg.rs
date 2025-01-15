use super::util::{self,
                  AuthToken,
                  BldrUrl,
                  CacheKeyPath,
                  ConfigOptAuthToken,
                  ConfigOptBldrUrl,
                  ConfigOptCacheKeyPath,
                  ConfigOptExternalCommandArgsWithHelpAndVersion,
                  ConfigOptFullyQualifiedPkgIdent,
                  ConfigOptPkgIdent,
                  ExternalCommandArgsWithHelpAndVersion,
                  FullyQualifiedPkgIdent,
                  PkgIdent};
#[cfg(any(all(target_os = "linux",
                  any(target_arch = "x86_64", target_arch = "aarch64")),
              all(target_os = "windows", target_arch = "x86_64"),))]
use super::util::{ConfigOptExternalCommandArgs,
                  ExternalCommandArgs};
use crate::cli::{dir_exists,
                 file_exists,
                 valid_ident_or_toml_file,
                 valid_origin};
use configopt::ConfigOpt;
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
use structopt::{clap::{AppSettings,
                       ArgGroup},
                StructOpt};

/// List all versions of installed packages
#[derive(ConfigOpt, StructOpt, Debug)]
#[structopt(name = "list", group = ArgGroup::with_name("prefix").required(true), no_version)]
pub struct PkgList {
    /// List all installed packages
    #[structopt(name = "ALL", short = "a", long = "all", group = "prefix")]
    all:       bool,
    /// An origin to list
    #[structopt(name = "ORIGIN",
        short = "o",
        long = "origin",
        validator = valid_origin, group = "prefix")]
    origin:    Option<String>,
    /// A package identifier (ex: core/redis, core/busybox-static/1.42.2)
    #[structopt(name = "PKG_IDENT", group = "prefix")]
    pkg_ident: Option<PackageIdent>,
}

/// Prints the path to a specific installed release of a package
#[derive(ConfigOpt, StructOpt)]
#[structopt(name = "path", no_version)]
pub struct PkgPath {
    #[structopt(flatten)]
    pkg_ident: PkgIdent,
}

/// Displays the binds for a service
#[derive(ConfigOpt, StructOpt)]
#[structopt(name = "binds", no_version, settings = &[AppSettings::ArgRequiredElseHelp])]
pub struct PkgBinds {
    #[structopt(flatten)]
    pkg_ident: PkgIdent,
}

#[derive(ConfigOpt, StructOpt)]
#[structopt(no_version)]
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
#[derive(ConfigOpt, StructOpt)]
#[structopt(name = "delete", no_version)]
pub struct PkgDelete {
    #[structopt(flatten)]
    bldr_url:   BldrUrl,
    #[structopt(flatten)]
    pkg_ident:  FullyQualifiedPkgIdent,
    /// A package target (ex: x86_64-windows) (default: system appropriate target)
    #[structopt(name = "PKG_TARGET", env = PACKAGE_TARGET_ENVVAR)]
    pkg_target: Option<PackageTarget>,
    #[structopt(flatten)]
    auth_token: AuthToken,
}

/// Demote a package from a specified channel
#[derive(ConfigOpt, StructOpt)]
#[structopt(name = "demote", no_version)]
pub struct PkgDemote {
    #[structopt(flatten)]
    bldr_url:   BldrUrl,
    #[structopt(flatten)]
    pkg_ident:  FullyQualifiedPkgIdent,
    /// Demote from the specified release channel
    #[structopt(name = "CHANNEL")]
    channel:    String,
    /// A package target (ex: x86_64-windows) (default: system appropriate target)
    #[structopt(name = "PKG_TARGET", env = PACKAGE_TARGET_ENVVAR)]
    pkg_target: Option<PackageTarget>,
    #[structopt(flatten)]
    auth_token: AuthToken,
}

/// Promote a package to a specified channel
#[derive(ConfigOpt, StructOpt)]
#[structopt(name = "promote", no_version)]
pub struct PkgPromote {
    #[structopt(flatten)]
    bldr_url:   BldrUrl,
    #[structopt(flatten)]
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
#[derive(ConfigOpt, StructOpt)]
#[structopt(name = "channels", no_version)]
pub struct PkgChannels {
    #[structopt(flatten)]
    bldr_url:   BldrUrl,
    #[structopt(flatten)]
    pkg_ident:  FullyQualifiedPkgIdent,
    /// A package target (ex: x86_64-windows) (default: system appropriate target)
    #[structopt(name = "PKG_TARGET", env = PACKAGE_TARGET_ENVVAR)]
    pkg_target: Option<PackageTarget>,
    #[structopt(flatten)]
    auth_token: AuthToken,
}

/// Displays the default configuration options for a service
#[derive(ConfigOpt, StructOpt)]
#[structopt(name = "config", no_version)]
pub struct PkgConfig {
    #[structopt(flatten)]
    pkg_ident: PkgIdent,
}

/// Search installed Habitat packages for a given file
#[derive(ConfigOpt, StructOpt)]
#[structopt(name = "provides", no_version, rename_all = "screamingsnake")]
pub struct PkgProvides {
    /// File name to find
    #[structopt(name = "FILE")]
    file:          String,
    /// Show fully qualified package names (ex: core/busybox-static/1.24.2/20160708162350)
    #[structopt(name = "FULL_RELEASES", short = "r")]
    full_releases: bool,
    /// Show full path to file
    #[structopt(name = "FULL_PATHS", short = "p")]
    full_paths:    bool,
}

/// Search for a package in Builder
#[derive(ConfigOpt, StructOpt)]
#[structopt(name = "search", no_version, rename_all = "screamingsnake")]
pub struct PkgSearch {
    /// Search term
    #[structopt(name = "SEARCH_TERM")]
    search_term: String,
    #[structopt(flatten)]
    bldr_url:    BldrUrl,
    #[structopt(flatten)]
    auth_token:  AuthToken,
    /// Limit how many packages to retrieve
    #[structopt(name = "LIMIT", short = "l", long = "limit", default_value = "50")]
    limit:       usize,
}

/// Signs an archive with an origin key, generating a Habitat Artifact
#[derive(ConfigOpt, StructOpt)]
#[structopt(name = "sign", no_version, rename_all = "screamingsnake")]
pub struct PkgSign {
    /// Origin key used to create signature
    #[structopt(name = "ORIGIN",
        long = "origin",
        validator = valid_origin)]
    origin:         Option<String>,
    /// A path to a source archive file (ex: /home/acme-redis-3.0.7-21120102031201.tar.xz)
    #[structopt(name = "SOURCE",
                validator = file_exists)]
    source:         PathBuf,
    /// The destination path to the signed Habitat Artifact (ex:
    /// /home/acme-redis-3.0.7-21120102031201-x86_64-linux.hart)
    #[structopt(name = "DEST")]
    dest:           PathBuf,
    #[structopt(flatten)]
    cache_key_path: CacheKeyPath,
}

/// Safely uninstall a package and dependencies from the local filesystem
#[derive(ConfigOpt, StructOpt)]
#[structopt(name = "uninstall", no_version, rename_all = "screamingsnake")]
pub struct PkgUninstall {
    #[structopt(flatten)]
    pkg_ident:             PkgIdent,
    /// Just show what would be uninstalled, don't actually do it
    #[structopt(name = "DRYRUN", short = "d", long = "dryrun")]
    dryrun:                bool,
    /// Only keep this number of latest packages uninstalling all others.
    #[structopt(name = "KEEP_LATEST", long = "keep-latest")]
    keep_latest:           Option<usize>,
    /// Identifier of one or more packages that should not be uninstalled. (ex: core/redis,
    /// core/busybox-static/1.42.2/21120102031201)
    #[structopt(name = "EXCLUDE", long = "exclude")]
    exclude:               Vec<PackageIdent>,
    /// Don't uninstall dependencies
    #[structopt(name = "NO_DEPS", long = "no-deps")]
    no_deps:               bool,
    /// Do not run any uninstall hooks
    #[structopt(name = "IGNORE_UNINSTALL_HOOK", long = "ignore-uninstall-hook")]
    ignore_uninstall_hook: bool,
}

/// Uploads a local Habitat Artifact to Builder
#[derive(ConfigOpt, StructOpt)]
#[structopt(name = "upload", no_version, rename_all = "screamingsnake")]
pub struct PkgUpload {
    #[structopt(flatten)]
    bldr_url:       BldrUrl,
    #[structopt(flatten)]
    auth_token:     AuthToken,
    /// Optional additional release channel to upload package to. Packages are always uploaded
    /// to `unstable`, regardless of the value of this option
    #[structopt(name = "CHANNEL", short = "c", long = "channel")]
    channel:        Option<String>,
    /// Skips checking availability of package and force uploads, potentially overwriting a
    /// stored copy of a package. (default: false)
    #[structopt(name = "FORCE", long = "force")]
    force:          bool,
    /// Disable auto-build for all packages in this upload
    #[structopt(name = "NO_BUILD", long = "no-build")]
    no_builde:      bool,
    /// One or more filepaths to a Habitat Artifact (ex:
    /// /home/acme-redis-3.0.7-21120102031201-x86_64-linux.hart)
    #[structopt(name = "HART_FILE", required = true, validator = file_exists)]
    hart_file:      Vec<PathBuf>,
    #[structopt(flatten)]
    cache_key_path: CacheKeyPath,
}

/// Verifies a Habitat Artifact with an origin key
#[derive(ConfigOpt, StructOpt)]
#[structopt(name = "verify", no_version, rename_all = "screamingsnake")]
pub struct PkgVerify {
    /// A path to a Habitat Artifact (ex:
    /// /home/acme-redis-3.0.7-21120102031201-x86_64-linux.hart)
    #[structopt(name = "SOURCE", validator = file_exists)]
    source:         PathBuf,
    #[structopt(flatten)]
    cache_key_path: CacheKeyPath,
}

/// Creates a binlink for a package binary in a common 'PATH' location
#[derive(ConfigOpt, StructOpt)]
#[structopt(name = "binlink", no_version, rename_all = "screamingsnake")]
pub struct PkgBinlink {
    #[structopt(flatten)]
    pkg_ident: PkgIdent,
    /// The command to binlink (ex: bash)
    #[structopt(name = "BINARY")]
    binary:    Option<String>,
    /// Sets the destination directory
    #[structopt(name = "DEST_DIR",
                short = "d",
                long = "dest",
                env = BINLINK_DIR_ENVVAR,
                default_value = DEFAULT_BINLINK_DIR)]
    dest_dir:  PathBuf,
    /// Overwrite existing binlinks
    #[structopt(name = "FORCE", short = "f", long = "force")]
    force:     bool,
}

/// Builds a Plan using a Studio
#[derive(ConfigOpt, StructOpt)]
#[structopt(name = "build", no_version, rename_all = "screamingsnake")]
pub struct PkgBuild {
    /// Installs secret origin keys (ex: "unicorn", "acme,other,acme-ops")
    #[structopt(name = "HAB_ORIGIN_KEYS", short = "k", long = "keys")]
    hab_origin_keys: Option<String>,
    /// Sets the Studio root (default: /hab/studios/<DIR_NAME>)
    #[structopt(name = "HAB_STUDIO_ROOT", short = "r", long = "root")]
    hab_studio_root: Option<PathBuf>,
    /// Sets the source path (default: $PWD)
    #[structopt(name = "SRC_PATH", short = "s", long = "src")]
    src_path:        Option<PathBuf>,
    /// A directory containing a plan file or a `habitat/` directory which contains the plan
    /// file
    #[structopt(name = "PLAN_CONTEXT")]
    plan_context:    PathBuf,
    #[structopt(flatten)]
    cache_key_path:  CacheKeyPath,
    /// Build a native package on the host system without a studio
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    #[structopt(name = "NATIVE_PACKAGE", short = "N", long = "native-package", conflicts_with_all = &["REUSE", "DOCKER"])]
    native_package:  bool,
    #[cfg(any(target_os = "linux", target_os = "windows"))]
    /// Reuses a previous Studio for the build (default: clean up before building)
    // Only a truly native/local Studio can be reused--the Docker implementation will always be
    // ephemeral
    #[structopt(name = "REUSE", short = "R", long = "reuse")]
    reuse:           bool,
    #[cfg(any(target_os = "linux", target_os = "windows"))]
    /// Uses a Dockerized Studio for the build
    #[structopt(name = "DOCKER", short = "D", long = "docker")]
    docker:          bool,
    /// Channel used to retrieve plan dependencies for Chef supported origins
    #[structopt(name = "REFRESH_CHANNEL",
                short = "f",
                long = "refresh-channel",
                env = "HAB_REFRESH_CHANNEL",
                default_value = "stable")]
    refresh_channel: Option<String>,
}

/// Bulk Uploads Habitat Artifacts to a Depot from a local directory
#[derive(ConfigOpt, StructOpt)]
#[structopt(name = "bulkupload", no_version, rename_all = "screamingsnake")]
pub struct PkgBulkupload {
    #[structopt(flatten)]
    bldr_url:             BldrUrl,
    #[structopt(flatten)]
    auth_token:           AuthToken,
    /// Optional additional release channel to upload package to. Packages are always uploaded
    /// to `unstable`, regardless of the value of this option
    #[structopt(name = "CHANNEL", short = "c", long = "channel")]
    channel:              Option<String>,
    /// Skip checking availability of package and force uploads, potentially overwriting a
    /// stored copy of a package
    #[structopt(name = "FORCE", long = "force")]
    force:                bool,
    /// Enable auto-build for all packages in this upload. Only applicable to SaaS Builder
    #[structopt(name = "AUTO_BUILD", long = "auto-build")]
    auto_build:           bool,
    /// Skip the confirmation prompt and automatically create origins that do not exist in the
    /// target Builder
    #[structopt(name = "AUTO_CREATE_ORIGINS", long = "auto-create-origins")]
    auto_create_channels: bool,
    /// Directory Path from which artifacts will be uploaded
    #[structopt(name = "UPLOAD_DIRECTORY", validator = dir_exists)]
    upload_directory:     PathBuf,
}

/// Returns the Habitat Artifact dependencies. By default it will return the direct
/// dependencies of the package
#[derive(ConfigOpt, StructOpt)]
#[structopt(name = "dependencies", no_version, rename_all = "screamingsnake")]
pub struct PkgDependencies {
    #[structopt(flatten)]
    pkg_ident:  PkgIdent,
    /// Show transitive dependencies
    #[structopt(name = "TRANSITIVE", short = "t", long = "transitive")]
    transitive: bool,
    /// Show packages which are dependant on this one
    #[structopt(name = "REVERSE", short = "r", long = "reverse")]
    reverse:    bool,
}

/// Download Habitat artifacts (including dependencies and keys) from Builder
#[derive(ConfigOpt, StructOpt)]
#[structopt(name = "download", no_version, rename_all = "screamingsnake")]
pub struct PkgDownload {
    #[structopt(flatten)]
    auth_token:          AuthToken,
    #[structopt(flatten)]
    bldr_url:            BldrUrl,
    /// Download from the specified release channel. Overridden if channel is specified in toml
    /// file
    #[structopt(name = "CHANNEL",
                short = "c",
                long = "channel",
                default_value = "stable",
                env = ChannelIdent::ENVVAR)]
    channel:             String,
    /// The path to store downloaded artifacts
    #[structopt(name = "DOWNLOAD_DIRECTORY", long = "download-directory")]
    download_directory:  Option<PathBuf>,
    /// File with newline separated package identifiers, or TOML file (ending with .toml
    /// extension)
    #[structopt(name = "PKG_IDENT_FILE", long = "file", validator = valid_ident_or_toml_file)]
    pkg_ident_file:      Vec<String>,
    /// One or more Habitat package identifiers (ex: acme/redis)
    #[structopt(name = "PKG_IDENT")]
    pkg_ident:           Vec<PackageIdent>,
    /// Target architecture to fetch. E.g. x86_64-linux. Overridden if architecture is
    /// specified in toml file
    #[structopt(name = "PKG_TARGET", short = "t", long = "target")]
    pkg_target:          Option<PackageTarget>,
    /// Verify package integrity after download (Warning: this can be slow)
    #[structopt(name = "VERIFY", long = "verify")]
    verify:              bool,
    /// Ignore packages specified that are not present on the target Builder
    #[structopt(name = "IGNORE_MISSING_SEEDS", long = "ignore-missing-seeds")]
    ignore_missing_seed: bool,
}

/// Executes a command using the 'PATH' context of an installed package
#[derive(ConfigOpt, StructOpt)]
#[structopt(name = "exec", aliases = &["exe"], no_version, rename_all = "screamingsnake")]
pub struct PkgExec {
    #[structopt(flatten)]
    pub pkg_ident: PkgIdent,
    /// The command to execute (ex: ls)
    #[structopt()]
    pub cmd:       PathBuf,
    #[structopt(flatten)]
    pub args:      ExternalCommandArgsWithHelpAndVersion,
}

/// Prints the runtime environment of a specific installed package
#[derive(ConfigOpt, StructOpt)]
#[structopt(name = "env", no_version)]
pub struct PkgEnv {
    #[structopt(flatten)]
    pkg_ident: PkgIdent,
}

/// Generates a blake2b hashsum from a target at any given filepath
#[derive(ConfigOpt, StructOpt)]
#[structopt(name = "hash", no_version)]
pub struct PkgHash {
    /// A filepath of the target
    #[structopt(name = "SOURCE", validator = file_exists)]
    source: Option<PathBuf>,
}

/// Returns the Habitat Artifact header
#[derive(ConfigOpt, StructOpt)]
#[structopt(name = "header", no_version, settings = &[AppSettings::Hidden])]
pub struct PkgHeader {
    /// A path to a Habitat Artifact (ex:
    /// /home/acme-redis-3.0.7-21120102031201-x86_64-linux.hart)
    #[structopt(name = "SOURCE", validator = file_exists)]
    source: PathBuf,
}

/// Returns the Habitat Artifact information
#[derive(ConfigOpt, StructOpt)]
#[structopt(name = "info", no_version, rename_all = "screamingsnake")]
pub struct PkgInfo {
    /// Output will be rendered in json. (Includes extended metadata)
    #[structopt(name = "TO_JSON", short = "j", long = "json")]
    to_json: bool,
    /// A path to a Habitat Artifact (ex:
    /// /home/acme-redis-3.0.7-21120102031201-x86_64-linux.hart)
    #[structopt(name = "SOURCE", validator = file_exists)]
    source:  PathBuf,
}

/// Installs a Habitat package from Builder or locally from a Habitat Artifact
#[derive(ConfigOpt, StructOpt)]
#[structopt(name = "install", no_version, rename_all = "screamingsnake")]
pub struct PkgInstall {
    #[structopt(flatten)]
    bldr_url:              BldrUrl,
    /// Install from the specified release channel
    #[structopt(short = "c",
                long = "channel",
                default_value = "stable",
                env = ChannelIdent::ENVVAR)]
    channel:               String,
    /// One or more Habitat package identifiers (ex: acme/redis) and/or filepaths to a Habitat
    /// Artifact (ex: /home/acme-redis-3.0.7-21120102031201-x86_64-linux.hart)
    #[structopt(required = true)]
    pkg_ident_or_artifact: Vec<String>,
    /// Binlink all binaries from installed package(s) into BINLINK_DIR
    #[structopt(short = "b", long = "binlink")]
    binlink:               bool,
    /// Binlink all binaries from installed package(s) into BINLINK_DIR
    #[structopt(long = "binlink-dir",
                default_value = DEFAULT_BINLINK_DIR,
                validator = util::non_empty,
                env = BINLINK_DIR_ENVVAR)]
    binlink_dir:           PathBuf,
    /// Overwrite existing binlinks
    #[structopt(short = "f", long = "force")]
    force:                 bool,
    #[structopt(flatten)]
    auth_token:            AuthToken,
    /// Do not run any install hooks
    #[structopt(long = "ignore-install-hook")]
    ignore_install_hook:   bool,
    /// Install packages in offline mode
    #[structopt(long = "offline",
                hidden = !FEATURE_FLAGS.contains(FeatureFlag::OFFLINE_INSTALL))]
    offline:               bool,
    /// Do not use locally-installed packages when a corresponding package cannot be installed
    /// from Builder
    #[structopt(long = "ignore-local")]
    ignore_local:          bool,
}

/// Exports the package to the specified format
#[cfg(any(all(target_os = "linux",
              any(target_arch = "x86_64", target_arch = "aarch64")),
          all(target_os = "windows", target_arch = "x86_64"),))]
#[derive(ConfigOpt, StructOpt)]
#[structopt(name = "export", aliases = &["e", "ex", "exp", "expo", "expor"], no_version)]
pub enum ExportCommand {
    #[cfg(any(target_os = "linux", target_os = "windows"))]
    /// Container exporter
    Container(ExternalCommandArgs),
    #[cfg(any(target_os = "linux", target_os = "windows"))]
    #[structopt(settings = &[AppSettings::Hidden])]
    Docker(ExternalCommandArgs),
    /// Tar exporter
    #[cfg(any(target_os = "linux", target_os = "windows"))]
    Tar(ExternalCommandArgs),
}
