#![allow(dead_code)]

use super::util::{AuthToken,
                  BldrUrl,
                  PkgIdent};
use crate::cli::valid_origin;
use habitat_common::cli::PACKAGE_TARGET_ENVVAR;
use habitat_core::package::PackageTarget;

use clap::{Parser,
           Subcommand};

#[derive(Subcommand)]
/// Commands relating to Habitat Builder
pub enum Bldr {
    /// Commands relating to Habitat Builder channels
    Channel(Channel),

    /// Commands relating to Habitat Builder jobs
    Job(Job),
}

#[derive(Subcommand)]
pub enum Channel {
    Create(ChannelCreate),
    Demote(ChannelDemote),
    Destroy(ChannelDestroy),
    List(ChannelList),
    Promote(ChannelPromote),
}

#[derive(Parser, Debug)]
pub struct BldrJobStatusSourceGroup {
    /// The job group id that was returned from "hab bldr job start" (ex: 771100000000000000)
    #[clap(name = "GROUP_ID", group = "status")]
    group_id: Option<String>,

    /// Show the status of recent job groups created in this origin (default: 10 most recent)
    #[clap(name = "ORIGIN",
            short = "o",
            long = "origin",
            validator = valid_origin,
            group = "status")]
    origin: Option<String>,
}

#[derive(Subcommand)]
pub enum Job {
    /// Cancel a build job group and any in-progress builds
    Cancel(JobCancel),

    Demote(JobDemote),
    Promote(JobPromote),
    Start(JobStart),
    Status(JobStatus),
}

/// Get the status of one or more job groups
#[derive(Parser)]
pub struct JobStatus {
    #[command(flatten)]
    source:    BldrJobStatusSourceGroup,
    /// Limit how many job groups to retrieve, ordered by most recent (default: 10)
    #[clap(name = "LIMIT", short = "l", long = "limit")]
    limit:     Option<usize>,
    /// Show the status of all build jobs for a retrieved job group
    #[clap(name = "SHOW_JOBS", short = "s", long = "showjobs")]
    show_jobs: bool,
    #[command(flatten)]
    bldr_url:  BldrUrl,
}

#[derive(Parser)]
pub struct JobCancel {
    /// The job group id that was returned from "hab bldr job start" (ex: 771100000000000000)
    #[clap(name = "GROUP_ID")]
    group_id: String,

    #[command(flatten)]
    bldr_url: BldrUrl,

    /// Don't prompt for confirmation
    #[clap(name = "FORCE", short = "f", long = "force")]
    force: bool,

    #[command(flatten)]
    auth_token: AuthToken,
}

/// Demote packages from a completed build job from a specified channel
#[derive(Parser)]
#[command(name = "demote")]
pub struct JobDemote {
    /// The job group id that was returned from "hab bldr job start" (ex: 771100000000000000)
    #[clap(name = "GROUP_ID")]
    group_id: String,

    /// The name of the channel to demote from
    #[clap(name = "CHANNEL")]
    channel: String,

    /// Limit the demotable packages to the specified origin
    #[clap(name = "ORIGIN",
            short = "o",
            long = "origin",
            validator = valid_origin)]
    origin: Option<String>,

    /// Allow editing the list of demotable packages
    #[clap(name = "INTERACTIVE", short = "i", long = "interactive")]
    interactive: bool,

    #[command(flatten)]
    bldr_url: BldrUrl,

    #[command(flatten)]
    auth_token: AuthToken,
}

/// Promote packages from a completed build job to a specified channel
#[derive(Parser)]
#[command(name = "promote")]
pub struct JobPromote {
    /// The job group id that was returned from "hab bldr job start" (ex: 771100000000000000)
    #[clap(name = "GROUP_ID")]
    group_id:    String,
    /// The target channel name
    #[clap(name = "CHANNEL")]
    channel:     String,
    /// Limit the promotable packages to the specified origin
    #[clap(name = "ORIGIN",
            short = "o",
            long = "origin",
            validator = valid_origin)]
    origin:      Option<String>,
    /// Allow editing the list of promotable packages
    #[clap(name = "INTERACTIVE", short = "i", long = "interactive")]
    interactive: bool,
    #[command(flatten)]
    bldr_url:    BldrUrl,
    #[command(flatten)]
    auth_token:  AuthToken,
}

/// Schedule a build job or group of jobs
#[derive(Parser)]
#[command(name = "start")]
pub struct JobStart {
    #[command(flatten)]
    pkg_ident:  PkgIdent,
    /// A package target (ex: x86_64-windows) (default: system appropriate target)
    #[clap(name = "PKG_TARGET", env = PACKAGE_TARGET_ENVVAR)]
    pkg_target: Option<PackageTarget>,

    #[command(flatten)]
    bldr_url: BldrUrl,

    #[command(flatten)]
    auth_token: AuthToken,

    /// Schedule jobs for this package and all of its reverse dependencies
    #[clap(name = "GROUP", short = "g", long = "group")]
    group: bool,
}

/// Creates a new channel
#[derive(Parser)]
#[command(name = "create")]
pub struct ChannelCreate {
    #[command(flatten)]
    bldr_url: BldrUrl,

    /// The channel name
    #[clap(name = "CHANNEL")]
    channel: String,

    /// Sets the origin to which the channel will belong. Default is from 'HAB_ORIGIN' or
    /// cli.toml
    #[structopt(name = "ORIGIN",
                short = "o",
                long = "origin",
                validator = valid_origin)]
    origin: Option<String>,
}

/// Lists origin channels
#[derive(Parser)]
#[command(name = "list")]
pub struct ChannelList {
    #[command(flatten)]
    bldr_url: BldrUrl,

    /// The origin for which channels will be listed. Default is from 'HAB_ORIGIN' or cli.toml
    #[clap(name = "ORIGIN", validator = valid_origin)]
    origin: Option<String>,

    /// Include sandbox channels for the origin
    #[clap(name = "SANDBOX", short = "s", long = "sandbox")]
    sandbox: bool,
}

/// Atomically promotes all packages in channel
#[derive(Parser)]
#[clap(name = "promote")]
pub struct ChannelPromote {
    #[command(flatten)]
    bldr_url:       BldrUrl,
    /// The origin for the channels. Default is from 'HAB_ORIGIN' or cli.toml
    #[clap(name = "ORIGIN",
                short = "o",
                long = "origin",
                validator = valid_origin)]
    origin:         String,
    /// The channel from which all packages will be selected for promotion
    #[clap(name = "SOURCE_CHANNEL")]
    source_channel: String,
    /// The channel to which packages will be promoted
    #[clap(name = "TARGET_CHANNEL")]
    target_channel: String,
    #[command(flatten)]
    auth_token:     AuthToken,
}

/// Atomically demotes selected packages in a target channel
#[derive(Parser)]
#[command(name = "demote")]
pub struct ChannelDemote {
    #[command(flatten)]
    bldr_url:       BldrUrl,
    /// The origin for the channels. Default is from 'HAB_ORIGIN' or cli.toml
    #[clap(name = "ORIGIN",
                short = "o",
                long = "origin",
                validator = valid_origin)]
    origin:         String,
    /// The channel from which all packages will be selected for demotion
    #[clap(name = "SOURCE_CHANNEL")]
    source_channel: String,
    /// The channel selected packages will be removed from
    #[clap(name = "TARGET_CHANNEL")]
    target_channel: String,
    #[command(flatten)]
    auth_token:     AuthToken,
}

/// Destroys a channel
#[derive(Parser)]
#[command(name = "destroy")]
pub struct ChannelDestroy {
    #[command(flatten)]
    bldr_url: BldrUrl,
    /// The channel name
    #[clap(name = "CHANNEL")]
    channel:  String,
    /// Sets the origin to which the channel belongs. Default is from 'HAB_ORIGIN' or cli.toml
    #[clap(name = "ORIGIN",
        short = "o",
        long = "origin",
        validator = valid_origin)]
    origin:   Option<String>,
}
