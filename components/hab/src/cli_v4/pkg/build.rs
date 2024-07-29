// Implemenatation of `hab pkg build`
use clap_v4 as clap;

use std::path::PathBuf;

use clap::{ArgAction,
           Parser};

use habitat_common::ui::UI;

#[cfg(target_os = "linux")]
use habitat_common::FeatureFlag;

use habitat_core::{crypto,
                   crypto::keys::KeyCache,
                   origin::Origin};

use crate::{command::pkg::build,
            error::Result as HabResult};

use crate::cli_v4::utils::CacheKeyPath;

#[derive(Debug, Clone, Parser)]
#[command(arg_required_else_help = true)]
pub(crate) struct PkgBuildOptions {
    // TODO: Should multiple Origins be supported? The semantics looks like that but the original
    // v2 code does not look like supporting.
    /// Installs secret origin keys (ex: "unicorn", "acme,other,acme-ops")
    #[arg(name = "HAB_ORIGIN_KEYS", short = 'k', long = "keys", action = ArgAction::Append)]
    hab_origin_keys: Vec<Origin>,

    /// Sets the Studio root (default: /hab/studios/<DIR_NAME>)
    #[arg(name = "HAB_STUDIO_ROOT", short = 'r', long = "root")]
    hab_studio_root: Option<String>,

    /// Sets the source path [default: $PWD]
    #[structopt(name = "SRC_PATH", short = 's', long = "src")]
    src_path: Option<String>,

    /// A directory containing a plan file or a `habitat/` directory which contains the plan
    /// file
    #[arg(name = "PLAN_CONTEXT")]
    plan_context: String,

    #[command(flatten)]
    cache_key_path: CacheKeyPath,

    #[cfg(target_os = "linux")]
    /// Build a native package on the host system without a studio
    #[arg(name = "NATIVE_PACKAGE", short = 'N', long = "native-package", conflicts_with_all = &["REUSE", "DOCKER"])]
    native_package: bool,

    #[cfg(any(target_os = "linux", target_os = "windows"))]
    /// Reuses a previous Studio for the build (default: clean up before building)
    // Only a truly native/local Studio can be reused--the Docker implementation will always be
    // ephemeral
    #[arg(name = "REUSE", short = 'R', long = "reuse", action = ArgAction::SetTrue)]
    reuse: bool,

    #[cfg(any(target_os = "linux", target_os = "windows"))]
    /// Uses a Dockerized Studio for the build
    #[arg(name = "DOCKER", short = 'D', long = "docker", action = ArgAction::SetTrue)]
    docker: bool,
}

impl PkgBuildOptions {
    pub(super) async fn do_build(&self, ui: &mut UI) -> HabResult<()> {
        if !self.hab_origin_keys.is_empty() {
            crypto::init()?;
            let key_cache = KeyCache::new::<PathBuf>((&self.cache_key_path).into());
            for origin in self.hab_origin_keys.iter() {
                // Validate that a secret signing key is present on disk
                // for each origin.
                key_cache.latest_secret_origin_signing_key(origin)?;
            }
        }

        let native_package = false;

        #[cfg(target_os = "linux")]
        let native_package = if self.native_package {
            if !feature_flags.contains(FeatureFlag::NATIVE_PACKAGE_SUPPORT) {
                return Err(Error::ArgumentError(String::from("`--native-package` is \
                                                              only available when \
                                                              `HAB_FEAT_NATIVE_PACKAGE_SUPPORT` \
                                                              is set")));
            }
            true
        } else {
            false
        };
        #[cfg(target_family = "windows")]
        let native_package = false;

        let (reuse_flag, docker_flag) = (false, false);

        #[cfg(any(target_os = "linux", target_os = "windows"))]
        let (reuse_flag, docker_flag) = (self.reuse, self.docker);

        build::start(ui,
                     self.plan_context.as_ref(),
                     self.hab_studio_root.as_deref(),
                     self.src_path.as_deref(),
                     &self.hab_origin_keys,
                     native_package,
                     reuse_flag,
                     docker_flag).await
    }
}
