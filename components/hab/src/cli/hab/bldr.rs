use super::util::{AuthToken,
                  BldrUrl,
                  ConfigOptAuthToken,
                  ConfigOptBldrUrl,
                  ConfigOptPkgIdent,
                  PkgIdent};
use crate::cli::valid_origin;
use configopt::ConfigOpt;
use habitat_common::cli::PACKAGE_TARGET_ENVVAR;
use habitat_core::package::PackageTarget;
use serde::Serialize;
use structopt::{clap::ArgGroup,
                StructOpt};

#[derive(ConfigOpt, StructOpt)]
#[structopt(no_version)]
/// Commands relating to Habitat Builder
pub enum Bldr {
    #[structopt(no_version)]
    Channel(Channel),
    #[structopt(no_version)]
    Job(Job),
}

#[derive(ConfigOpt, StructOpt)]
#[structopt(no_version)]
/// Commands relating to Habitat Builder channels
pub enum Channel {
    Create(ChannelCreate),
    Demote(ChannelDemote),
    Destroy(ChannelDestroy),
    List(ChannelList),
    Promote(ChannelPromote),
}

#[derive(ConfigOpt, StructOpt, Debug)]
#[configopt(derive(Serialize))]
#[structopt(group = ArgGroup::with_name("status").required(true), no_version)]
pub struct BldrJobStatusSourceGroup {
    /// The job group id that was returned from "hab bldr job start" (ex: 771100000000000000)
    #[structopt(name = "GROUP_ID", group = "status")]
    group_id: Option<String>,
    /// Show the status of recent job groups created in this origin (default: 10 most recent)
    #[structopt(name = "ORIGIN",
            short = "o",
            long = "origin",
            validator = valid_origin,
            group = "status")]
    origin:   Option<String>,
}

#[derive(ConfigOpt, StructOpt)]
#[structopt(no_version)]
/// REMOVED: Commands relating to Habitat Builder jobs
///
/// NOTICE: Public Builder Build Functions are no longer supported.
/// Please reach out to your account team if you were using this feature.
pub enum Job {
    Cancel(JobCancel),
    Demote(JobDemote),
    Promote(JobPromote),
    Start(JobStart),
    Status(JobStatus),
}

/// REMOVED: Get the status of one or more job groups
///
/// NOTICE: Public Builder Build Functions are no longer supported.
/// Please reach out to your account team if you were using this feature.
#[derive(ConfigOpt, StructOpt)]
#[structopt(name = "status", no_version)]
pub struct JobStatus {
    #[structopt(flatten)]
    source:    BldrJobStatusSourceGroup,
    /// Limit how many job groups to retrieve, ordered by most recent (default: 10)
    #[structopt(name = "LIMIT", short = "l", long = "limit")]
    limit:     Option<usize>,
    /// Show the status of all build jobs for a retrieved job group
    #[structopt(name = "SHOW_JOBS", short = "s", long = "showjobs")]
    show_jobs: bool,
    #[structopt(flatten)]
    bldr_url:  BldrUrl,
}

/// REMOVED: Cancel a build job group and any in-progress builds
///
/// NOTICE: Public Builder Build Functions are no longer supported.
/// Please reach out to your account team if you were using this feature.
#[derive(ConfigOpt, StructOpt)]
#[structopt(name = "cancel", no_version)]
pub struct JobCancel {
    /// The job group id that was returned from "hab bldr job start" (ex: 771100000000000000)
    #[structopt(name = "GROUP_ID")]
    group_id:   String,
    #[structopt(flatten)]
    bldr_url:   BldrUrl,
    /// Don't prompt for confirmation
    #[structopt(name = "FORCE", short = "f", long = "force")]
    force:      bool,
    #[structopt(flatten)]
    auth_token: AuthToken,
}

/// REMOVED: Demote packages from a completed build job from a specified channel
///
/// NOTICE: Public Builder Build Functions are no longer supported.
/// Please reach out to your account team if you were using this feature.
#[derive(ConfigOpt, StructOpt)]
#[structopt(name = "demote", no_version)]
pub struct JobDemote {
    /// The job group id that was returned from "hab bldr job start" (ex: 771100000000000000)
    #[structopt(name = "GROUP_ID")]
    group_id:    String,
    /// The name of the channel to demote from
    #[structopt(name = "CHANNEL")]
    channel:     String,
    /// Limit the demotable packages to the specified origin
    #[structopt(name = "ORIGIN",
            short = "o",
            long = "origin",
            validator = valid_origin)]
    origin:      Option<String>,
    /// Allow editing the list of demotable packages
    #[structopt(name = "INTERACTIVE", short = "i", long = "interactive")]
    interactive: bool,
    #[structopt(flatten)]
    bldr_url:    BldrUrl,
    #[structopt(flatten)]
    auth_token:  AuthToken,
}

/// REMOVED: Promote packages from a completed build job to a specified channel
///
/// NOTICE: Public Builder Build Functions are no longer supported.
/// Please reach out to your account team if you were using this feature.
#[derive(ConfigOpt, StructOpt)]
#[structopt(name = "promote", no_version)]
pub struct JobPromote {
    /// The job group id that was returned from "hab bldr job start" (ex: 771100000000000000)
    #[structopt(name = "GROUP_ID")]
    group_id:    String,
    /// The target channel name
    #[structopt(name = "CHANNEL")]
    channel:     String,
    /// Limit the promotable packages to the specified origin
    #[structopt(name = "ORIGIN",
            short = "o",
            long = "origin",
            validator = valid_origin)]
    origin:      Option<String>,
    /// Allow editing the list of promotable packages
    #[structopt(name = "INTERACTIVE", short = "i", long = "interactive")]
    interactive: bool,
    #[structopt(flatten)]
    bldr_url:    BldrUrl,
    #[structopt(flatten)]
    auth_token:  AuthToken,
}

/// REMOVED: Schedule a build job or group of jobs
///
/// NOTICE: Public Builder Build Functions are no longer supported.
/// Please reach out to your account team if you were using this feature.
#[derive(ConfigOpt, StructOpt)]
#[structopt(name = "start", no_version)]
pub struct JobStart {
    #[structopt(flatten)]
    pkg_ident:  PkgIdent,
    /// A package target (ex: x86_64-windows) (default: system appropriate target)
    #[structopt(name = "PKG_TARGET", env = PACKAGE_TARGET_ENVVAR)]
    pkg_target: Option<PackageTarget>,
    #[structopt(flatten)]
    bldr_url:   BldrUrl,
    #[structopt(flatten)]
    auth_token: AuthToken,
    /// Schedule jobs for this package and all of its reverse dependencies
    #[structopt(name = "GROUP", short = "g", long = "group")]
    group:      bool,
}

/// Creates a new channel
#[derive(ConfigOpt, StructOpt)]
#[structopt(name = "create", no_version)]
pub struct ChannelCreate {
    #[structopt(flatten)]
    bldr_url: BldrUrl,
    /// The channel name
    #[structopt(name = "CHANNEL")]
    channel:  String,
    /// Sets the origin to which the channel will belong. Default is from 'HAB_ORIGIN' or
    /// cli.toml
    #[structopt(name = "ORIGIN",
                short = "o",
                long = "origin",
                validator = valid_origin)]
    origin:   Option<String>,
}

/// Lists origin channels
#[derive(ConfigOpt, StructOpt)]
#[structopt(name = "list", no_version)]
pub struct ChannelList {
    #[structopt(flatten)]
    bldr_url: BldrUrl,
    /// The origin for which channels will be listed. Default is from 'HAB_ORIGIN' or cli.toml
    #[structopt(name = "ORIGIN", validator = valid_origin)]
    origin:   Option<String>,
    /// Include sandbox channels for the origin
    #[structopt(name = "SANDBOX", short = "s", long = "sandbox")]
    sandbox:  bool,
}

/// Atomically promotes all packages in channel
#[derive(ConfigOpt, StructOpt)]
#[structopt(name = "promote", no_version)]
pub struct ChannelPromote {
    #[structopt(flatten)]
    bldr_url:       BldrUrl,
    /// The origin for the channels. Default is from 'HAB_ORIGIN' or cli.toml
    #[structopt(name = "ORIGIN",
                short = "o",
                long = "origin",
                validator = valid_origin)]
    origin:         String,
    /// The channel from which all packages will be selected for promotion
    #[structopt(name = "SOURCE_CHANNEL")]
    source_channel: String,
    /// The channel to which packages will be promoted
    #[structopt(name = "TARGET_CHANNEL")]
    target_channel: String,
    #[structopt(flatten)]
    auth_token:     AuthToken,
}

/// Atomically demotes selected packages in a target channel
#[derive(ConfigOpt, StructOpt)]
#[structopt(name = "demote", no_version)]
pub struct ChannelDemote {
    #[structopt(flatten)]
    bldr_url:       BldrUrl,
    /// The origin for the channels. Default is from 'HAB_ORIGIN' or cli.toml
    #[structopt(name = "ORIGIN",
                short = "o",
                long = "origin",
                validator = valid_origin)]
    origin:         String,
    /// The channel from which all packages will be selected for demotion
    #[structopt(name = "SOURCE_CHANNEL")]
    source_channel: String,
    /// The channel selected packages will be removed from
    #[structopt(name = "TARGET_CHANNEL")]
    target_channel: String,
    #[structopt(flatten)]
    auth_token:     AuthToken,
}

/// Destroys a channel
#[derive(ConfigOpt, StructOpt)]
#[structopt(name = "destroy", no_version)]
pub struct ChannelDestroy {
    #[structopt(flatten)]
    bldr_url: BldrUrl,
    /// The channel name
    #[structopt(name = "CHANNEL")]
    channel:  String,
    /// Sets the origin to which the channel belongs. Default is from 'HAB_ORIGIN' or cli.toml
    #[structopt(name = "ORIGIN",
        short = "o",
        long = "origin",
        validator = valid_origin)]
    origin:   Option<String>,
}
