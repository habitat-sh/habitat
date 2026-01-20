// Implementation of `hab sup secret`
/// Commands relating to a Habitat Supervisor's Control Gateway secret
use clap_v4 as clap;

use clap::Parser;

use crate::{cli_v4::utils::SubjectAlternativeName,
            error::Result as HabResult};
use habitat_core::fs::HAB_CTL_KEYS_CACHE;
use std::path::PathBuf;

#[cfg(not(target_os = "macos"))]
use habitat_common::ui::{self,
                         UIWriter};

#[cfg(not(target_os = "macos"))]
use habitat_sup_protocol as sup_proto;

#[cfg(not(target_os = "macos"))]
use habitat_core::tls::ctl_gateway as ctl_gateway_tls;

#[derive(Debug, Clone, Parser)]
#[command(arg_required_else_help = true,
          disable_version_flag = true,
          help_template = "{name} {version} {author-section} {about-section} \n{usage-heading} \
                           {usage}\n\n{all-args}\n")]
pub enum SupSecretOptions {
    /// Generate a secret key to use as a Supervisor's Control Gateway secret
    Generate,

    /// Generate a private key and certificate for the Supervisor's
    /// Control Gateway TLS connection
    GenerateTls {
        /// The DNS name to use in the certificates subject alternative name extension
        #[arg(long = "subject-alternative-name")]
        subject_alternative_name: SubjectAlternativeName,

        /// The directory to store the generated private key and certificate
        #[arg(name = "path", default_value = HAB_CTL_KEYS_CACHE)]
        path: PathBuf,
    },
}

impl SupSecretOptions {
    #[cfg(not(target_os = "macos"))]
    pub(super) async fn execute(&self) -> HabResult<()> {
        match self {
            SupSecretOptions::Generate => {
                let mut ui = ui::ui();
                let mut buf = String::new();
                sup_proto::generate_secret_key(&mut buf);
                ui.info(buf)?;
                Ok(())
            }

            SupSecretOptions::GenerateTls {subject_alternative_name, path} => {
                Ok(ctl_gateway_tls::generate_self_signed_certificate_and_key(&subject_alternative_name.dns_name()?, path)
                .map_err(habitat_core::Error::from)?)
            }
        }
    }

    #[cfg(target_os = "macos")]
    pub(super) async fn execute(&self) -> HabResult<()> { Ok(()) }
}
