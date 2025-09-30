// Implemenatation of `hab pkg build`
use clap_v4 as clap;

use std::path::PathBuf;

use clap::{ArgAction,
           Parser};

use habitat_common::ui::UI;

use habitat_common::FeatureFlag;

use habitat_core::{crypto,
                   crypto::keys::KeyCache,
                   origin::Origin};

use crate::{command::pkg::build,
            error::Result as HabResult};

#[cfg(any(target_os = "linux", target_os = "macos"))]
use crate::error::Error as HabError;

use crate::cli_v4::utils::CacheKeyPath;

#[derive(Debug, Clone, Parser)]
#[command(arg_required_else_help = true,
          help_template = "{name} {version} {author-section} {about-section} \n{usage-heading} \
                           {usage}\n\n{all-args}\n")]
pub(crate) struct PkgBuildOptions {
    /// Installs secret origin keys (ex: "unicorn", "acme,other,acme-ops")
    #[arg(name = "HAB_ORIGIN_KEYS", short = 'k', long = "keys", action = ArgAction::Append)]
    hab_origin_keys: Vec<Origin>,

    /// Sets the Studio root (default: /hab/studios/<DIR_NAME>)
    #[arg(name = "HAB_STUDIO_ROOT", short = 'r', long = "root")]
    hab_studio_root: Option<String>,

    // TODO: Same as above
    /// Sets the source path [default: $PWD]
    #[arg(name = "SRC_PATH", short = 's', long = "src")]
    src_path: Option<String>,

    // TODO : Same as above
    /// A directory containing a plan file or a `habitat/` directory which contains the plan
    /// file
    #[arg(name = "PLAN_CONTEXT")]
    plan_context: String,

    #[command(flatten)]
    cache_key_path: CacheKeyPath,

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    /// Build a native package on the host system without a studio
    #[arg(name = "NATIVE_PACKAGE", short = 'N', long = "native-package")]
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

    /// Channel used to retrieve plan dependencies for Chef supported origins
    #[arg(name = "REFRESH_CHANNEL",
          short = 'f',
          long = "refresh-channel",
          env = "HAB_REFRESH_CHANNEL",
          default_value = "base")]
    refresh_channel: Option<String>,
}

impl PkgBuildOptions {
    // Required because of lot of `cfg`...
    #[allow(unused_variables)]
    pub(super) async fn do_build(&self, ui: &mut UI, feature_flags: FeatureFlag) -> HabResult<()> {
        // Validate conflicts between native package and studio options
        #[cfg(any(target_os = "linux", target_os = "macos"))]
        if self.native_package {
            #[cfg(target_os = "linux")]
            if self.reuse || self.docker {
                return Err(HabError::ArgumentError(String::from("--native-package \
                                                                 conflicts with \
                                                                 --reuse and \
                                                                 --docker")));
            }
        }

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

        let native_package = self.should_build_native_package(feature_flags)?;

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
                     docker_flag,
                     self.refresh_channel.as_deref()).await
    }

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    fn should_build_native_package(&self, feature_flags: FeatureFlag) -> HabResult<bool> {
        if self.native_package {
            if !feature_flags.contains(FeatureFlag::NATIVE_PACKAGE_SUPPORT) {
                return Err(HabError::ArgumentError(String::from("`--native-package` is only \
                                                                 available when \
                                                                 `HAB_FEAT_NATIVE_PACKAGE_SUPPORT` \
                                                                 is set")));
            }
            Ok(true)
        } else {
            Ok(false)
        }
    }

    #[cfg(not(any(target_os = "linux", target_os = "macos")))]
    fn should_build_native_package(&self, _: FeatureFlag) -> HabResult<bool> { Ok(false) }
}
