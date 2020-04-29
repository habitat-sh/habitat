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
    /// Creates a new channel
    Create {
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
    },
    /// Atomically demotes selected packages in a target channel
    Demote {
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
    },
    /// Destroys a channel
    Destroy {
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
    },
    /// Lists origin channels
    List {
        #[structopt(flatten)]
        bldr_url: BldrUrl,
        /// The origin for which channels will be listed. Default is from 'HAB_ORIGIN' or cli.toml
        #[structopt(name = "ORIGIN", validator = valid_origin)]
        origin:   Option<String>,
    },
    /// Atomically promotes all packages in channel
    Promote {
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
    },
}

#[derive(ConfigOpt, StructOpt, Debug)]
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
/// Commands relating to Habitat Builder jobs
pub enum Job {
    /// Cancel a build job group and any in-progress builds
    Cancel {
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
    },
    /// Demote packages from a completed build job from a specified channel
    Demote {
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
    },
    /// Promote packages from a completed build job to a specified channel
    Promote {
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
    },
    /// Schedule a build job or group of jobs
    Start {
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
    },
    /// Get the status of one or more job groups
    Status {
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
    },
}
