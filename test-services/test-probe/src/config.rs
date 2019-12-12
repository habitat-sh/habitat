use crate::error::{Error,
                   Result};
use clap::ArgMatches;
use std::{fs,
          path::PathBuf};
use toml;

pub const DEFAULT_JSON_SOURCE: &str = "/hab/svc/test-probe/config/render_context_file.json";
pub const DEFAULT_HOST: &str = "0.0.0.0";
pub const DEFAULT_PORT: u16 = 8000;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    pub host:                String,
    pub port:                u16,
    pub render_context_file: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Config { host:                DEFAULT_HOST.to_string(),
                 port:                DEFAULT_PORT,
                 render_context_file: PathBuf::from(DEFAULT_JSON_SOURCE), }
    }
}

pub fn from_matches(args: &ArgMatches) -> Result<Config> {
    if let Some(path) = args.value_of("config") {
        let contents = fs::read_to_string(path)?;
        toml::from_str(&contents).map_err(Error::Toml)
    } else {
        Ok(Config::default())
    }
}
