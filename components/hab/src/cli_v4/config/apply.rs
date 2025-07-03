use clap_v4 as clap;
use clap::{Parser, parser::ValueSource, CommandFactory};
use std::{convert::TryFrom, path::PathBuf};
use habitat_common::{ui::UI, types::ResolvedListenCtlAddr};
use habitat_core::service::ServiceGroup;
use crate::{error::Result as HabResult, command::config::sub_svc_set};


#[derive(Debug, Clone, Parser)]
#[command(
    arg_required_else_help = true,
    rename_all = "kebab-case",
    about = "Apply a configuration to a running service"
)]
pub struct ConfigApplyOptions {
    #[arg(long)]
    pub remote_sup: Option<String>,

    #[arg(long)]
    pub group: String,

    #[arg(long)]
    pub file: PathBuf,

    #[arg(long, default_value_t = 0)]
    pub version: u64,

    #[arg(long)]
    pub user: Option<String>,
}

impl ConfigApplyOptions {
    pub async fn do_apply(&self, ui: &mut UI) -> HabResult<()> {
        let raw_args: Vec<_> = std::env::args_os().skip(2).collect();
        let matches = Self::command().get_matches_from(raw_args);
        let remote_sup = match matches.value_source("remote-sup") {
            Some(ValueSource::CommandLine) => {
                let s = matches.get_one::<String>("remote-sup").unwrap();
                Some(ResolvedListenCtlAddr::try_from(s.clone()).unwrap())
            }
            _ => self.remote_sup.as_ref()
                              .map(|s| ResolvedListenCtlAddr::try_from(s.clone()).unwrap()),
        };

        let service_group = self
            .group
            .parse::<ServiceGroup>()
            .expect("Invalid service group identifier");

        sub_svc_set(
            ui,
            service_group,
            &self.file,
            self.version,
            self.user.clone(),
            remote_sup,
        )
        .await
    }
}
