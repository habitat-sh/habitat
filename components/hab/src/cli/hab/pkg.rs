use super::util::{AuthToken,
                  BldrUrl,
                  CacheKeyPath,
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
use structopt::{clap::{AppSettings,
                       ArgGroup},
                StructOpt};

#[derive(StructOpt, Debug)]
#[structopt(group = ArgGroup::with_name("prefix").required(true), no_version)]
pub struct List {
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

#[derive(StructOpt)]
#[structopt(no_version)]
#[allow(clippy::large_enum_variant)]
/// Commands relating to Habitat packages
pub enum Pkg {
    /// Displays the binds for a service
    Binds {
        #[structopt(flatten)]
        pkg_ident: PkgIdent,
    },
    /// Creates a binlink for a package binary in a common 'PATH' location
    Binlink {
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
    },
    /// Builds a Plan using a Studio
    Build {
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
    },
    /// Bulk Uploads Habitat Artifacts to a Depot from a local directory
    Bulkupload {
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
    },
    /// Find out what channels a package belongs to
    Channels {
        #[structopt(flatten)]
        bldr_url:   BldrUrl,
        #[structopt(flatten)]
        pkg_ident:  FullyQualifiedPkgIdent,
        /// A package target (ex: x86_64-windows) (default: system appropriate target)
        #[structopt(name = "PKG_TARGET", env = PACKAGE_TARGET_ENVVAR)]
        pkg_target: Option<PackageTarget>,
        #[structopt(flatten)]
        auth_token: AuthToken,
    },
    /// Displays the default configuration options for a service
    Config {
        #[structopt(flatten)]
        pkg_ident: PkgIdent,
    },
    /// Removes a package from Builder
    Delete {
        #[structopt(flatten)]
        bldr_url:   BldrUrl,
        #[structopt(flatten)]
        pkg_ident:  FullyQualifiedPkgIdent,
        /// A package target (ex: x86_64-windows) (default: system appropriate target)
        #[structopt(name = "PKG_TARGET", env = PACKAGE_TARGET_ENVVAR)]
        pkg_target: Option<PackageTarget>,
        #[structopt(flatten)]
        auth_token: AuthToken,
    },
    /// Demote a package from a specified channel
    Demote {
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
    },
    /// Returns the Habitat Artifact dependencies. By default it will return the direct
    /// dependencies of the package
    Dependencies {
        #[structopt(flatten)]
        pkg_ident:  PkgIdent,
        /// Show transitive dependencies
        #[structopt(name = "TRANSITIVE", short = "t", long = "transitive")]
        transitive: bool,
        /// Show packages which are dependant on this one
        #[structopt(name = "REVERSE", short = "r", long = "reverse")]
        reverse:    bool,
    },
    /// Download Habitat artifacts (including dependencies and keys) from Builder
    Download {
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
    },
    /// Prints the runtime or build-time environment of specific installed packages
    Env {
        /// A package identifier (ex: core/redis, core/busybox-static/1.42.2)
        #[structopt(name = "PKG_IDENT", required = true)]
        pkg_ident:       Vec<PackageIdent>,
        /// Include common build-time environment variables in addition to runtime variables
        #[structopt(name = "BUILD", short = "b", long = "build")]
        build:           bool,
        /// Append existing values to environment variables when printing
        #[structopt(name = "APPEND_EXISTING", short = "a", long = "append-existing")]
        append_existing: bool,
    },
    /// Executes a command using the 'PATH' context of an installed package
    Exec {
        #[structopt(flatten)]
        pkg_ident: PkgIdent,
        /// The command to execute (ex: ls)
        #[structopt(name = "CMD")]
        cmd:       String,
        /// Arguments to the command (ex: -l /tmp)
        #[structopt(name = "ARGS")]
        args:      Vec<String>,
    },
    /// Exports the package to the specified format
    Export {
        /// The export format (ex: cf, docker, mesos, or tar)
        #[structopt(name = "FORMAT")]
        format:    String,
        /// A package identifier (ex: core/redis, core/busybox-static/1.42.2) or filepath to a
        /// Habitat Artifact (ex: /home/acme-redis-3.0.7-21120102031201-x86_64-linux.hart)
        #[structopt(name = "PKG_IDENT")]
        pkg_ident: PackageIdent,
        #[structopt(flatten)]
        bldr_url:  BldrUrl,
        /// Retrieve the container's package from the specified release channel
        #[structopt(name = "CHANNEL",
            short = "c",
            long = "channel",
            default_value = "stable",
            env = ChannelIdent::ENVVAR)]
        channel:   String,
    },
    /// Generates a blake2b hashsum from a target at any given filepath
    Hash {
        /// A filepath of the target
        #[structopt(name = "SOURCE", validator = file_exists)]
        source: Option<PathBuf>,
    },
    /// Returns the Habitat Artifact header
    #[structopt(settings = &[AppSettings::Hidden])]
    Header {
        /// A path to a Habitat Artifact (ex:
        /// /home/acme-redis-3.0.7-21120102031201-x86_64-linux.hart)
        #[structopt(name = "SOURCE", validator = file_exists)]
        source: PathBuf,
    },
    /// Returns the Habitat Artifact information
    Info {
        /// Output will be rendered in json
        #[structopt(name = "TO_JSON", short = "j", long = "json")]
        to_json: bool,
        /// A path to a Habitat Artifact (ex:
        /// /home/acme-redis-3.0.7-21120102031201-x86_64-linux.hart)
        #[structopt(name = "SOURCE", validator = file_exists)]
        source:  PathBuf,
    },
    /// Installs a Habitat package from Builder or locally from a Habitat Artifact
    Install {
        #[structopt(flatten)]
        bldr_url:              BldrUrl,
        /// Install from the specified release channel
        #[structopt(name = "CHANNEL",
                    short = "c",
                    long = "channel",
                    default_value = "stable",
                    env = ChannelIdent::ENVVAR)]
        channel:               String,
        /// One or more Habitat package identifiers (ex: acme/redis) and/or filepaths to a Habitat
        /// Artifact (ex: /home/acme-redis-3.0.7-21120102031201-x86_64-linux.hart)
        #[structopt(name = "PKG_IDENT_OR_ARTIFACT", required = true)]
        pkg_ident_or_artifact: Vec<String>,
        /// Binlink all binaries from installed package(s) into BINLINK_DIR
        #[structopt(name = "BINLINK", short = "b", long = "binlink")]
        binlink:               bool,
        /// Binlink all binaries from installed package(s) into BINLINK_DIR
        #[structopt(name = "BINLINK_DIR",
                    long = "binlink-dir",
                    default_value = DEFAULT_BINLINK_DIR,
                    env = BINLINK_DIR_ENVVAR)]
        binlink_dir:           PathBuf,
        /// Overwrite existing binlinks
        #[structopt(name = "FORCE", short = "f", long = "force")]
        force:                 bool,
        #[structopt(flatten)]
        auth_token:            AuthToken,
        /// Do not run any install hooks
        #[structopt(name = "IGNORE_INSTALL_HOOK", long = "ignore-install-hook")]
        ignore_install_hook:   bool,
        /// Install packages in offline mode
        #[structopt(name = "OFFLINE", long = "offline",
                    hidden = !FEATURE_FLAGS.contains(FeatureFlag::OFFLINE_INSTALL))]
        offline:               bool,
        /// Do not use locally-installed packages when a corresponding package cannot be installed
        /// from Builder
        #[structopt(name = "IGNORE_LOCAL", long = "ignore-local",
                    hidden = !FEATURE_FLAGS.contains(FeatureFlag::IGNORE_LOCAL))]
        ignore_local:          bool,
    },
    /// List all versions of installed packages
    List(List),
    /// Prints the path to a specific installed release of a package
    Path {
        #[structopt(flatten)]
        pkg_ident: PkgIdent,
    },
    /// Promote a package to a specified channel
    Promote {
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
    },
    /// Search installed Habitat packages for a given file
    Provides {
        /// File name to find
        #[structopt(name = "FILE")]
        file:          String,
        /// Show fully qualified package names (ex: core/busybox-static/1.24.2/20160708162350)
        #[structopt(name = "FULL_RELEASES", short = "r")]
        full_releases: bool,
        /// Show full path to file
        #[structopt(name = "FULL_PATHS", short = "p")]
        full_paths:    bool,
    },
    /// Search for a package in Builder
    Search {
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
    },
    /// Signs an archive with an origin key, generating a Habitat Artifact
    Sign {
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
    },
    /// Safely uninstall a package and dependencies from the local filesystem
    Uninstall {
        #[structopt(flatten)]
        pkg_ident:   PkgIdent,
        /// Just show what would be uninstalled, don't actually do it
        #[structopt(name = "DRYRUN", short = "d", long = "dryrun")]
        dryrun:      bool,
        /// Only keep this number of latest packages uninstalling all others.
        #[structopt(name = "KEEP_LATEST", long = "keep-latest")]
        keep_latest: Option<usize>,
        /// Identifier of one or more packages that should not be uninstalled. (ex: core/redis,
        /// core/busybox-static/1.42.2/21120102031201)
        #[structopt(name = "EXCLUDE", long = "exclude")]
        exclude:     Vec<PackageIdent>,
        /// Don't uninstall dependencies
        #[structopt(name = "NO_DEPS", long = "no-deps")]
        no_deps:     bool,
    },
    /// Uploads a local Habitat Artifact to Builder
    Upload {
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
    },
    /// Verifies a Habitat Artifact with an origin key
    Verify {
        /// A path to a Habitat Artifact (ex:
        /// /home/acme-redis-3.0.7-21120102031201-x86_64-linux.hart)
        #[structopt(name = "SOURCE", validator = file_exists)]
        source:         PathBuf,
        #[structopt(flatten)]
        cache_key_path: CacheKeyPath,
    },
}
