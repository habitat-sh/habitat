use clap_v4 as clap;
use clap::{Parser, parser::ValueSource, CommandFactory};
use std::convert::TryFrom;
use habitat_common::types::ResolvedListenCtlAddr;
use habitat_core::package::PackageIdent;
use crate::{error::Result as HabResult, command::config::sub_svc_config};


#[derive(Debug, Clone, Parser)]
#[command(
    arg_required_else_help = true,
    rename_all = "kebab-case",
    about = "Show the current config of a running service"
)]
pub struct ConfigShowOptions {
    #[arg(long)]
    pub remote_sup: Option<String>,

    #[arg(long)]
    pub ident: String,
}

impl ConfigShowOptions {
    pub async fn do_show(&self) -> HabResult<()> {
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

        let ident = self
            .ident
            .parse::<PackageIdent>()
            .expect("Invalid ident");

        sub_svc_config(ident, remote_sup).await
    }
}
