use crate::{error::{Error,
                    Result},
            hcore::{config::ConfigFile,
                    fs::{am_i_root,
                         FS_ROOT_PATH}},
            CTL_SECRET_ENVVAR};
use habitat_core::{env as henv,
                   origin::Origin};
use habitat_sup_client::SrvClient;
use std::{fs::{self,
               File},
          io::Write,
          path::PathBuf};

const CLI_CONFIG_PATH: &str = "hab/etc/cli.toml";

lazy_static::lazy_static! {
    /// A cached reading of the config file. This avoids the need to continually read from disk.
    /// However, it also means changes to the file will not be picked up after the program has
    /// started. Ideally, we would repopulate this struct on file change or on some configured
    /// interval.
    /// https://github.com/habitat-sh/habitat/issues/7243
    pub static ref CACHED: Config = load().unwrap_or_default();
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Config {
    pub auth_token: Option<String>,
    pub origin:     Option<Origin>,
    pub ctl_secret: Option<String>,
    pub bldr_url:   Option<String>,
}

impl ConfigFile for Config {
    type Error = Error;
}

impl Default for Config {
    fn default() -> Self {
        Config { auth_token: None,
                 origin:     None,
                 ctl_secret: None,
                 bldr_url:   None, }
    }
}

pub fn load() -> Result<Config> {
    let cli_config_path = cli_config_path();
    if cli_config_path.exists() {
        debug!("Loading CLI config from {}", cli_config_path.display());
        Ok(Config::from_file(&cli_config_path)?)
    } else {
        debug!("No CLI config found, loading defaults");
        Ok(Config::default())
    }
}

pub fn save(config: &Config) -> Result<()> {
    let config_path = cli_config_path();
    let parent_path = match config_path.parent() {
        Some(p) => p,
        None => {
            return Err(Error::FileNotFound(config_path.to_string_lossy().into_owned()));
        }
    };
    fs::create_dir_all(&parent_path)?;
    let raw = toml::ser::to_string(config)?;
    debug!("Raw config toml:\n---\n{}\n---", &raw);
    let mut file = File::create(&config_path)?;
    file.write_all(raw.as_bytes())?;
    Ok(())
}

/// Check if the HAB_CTL_SECRET env var. If not, check the CLI config to see if there is a ctl
/// secret set and return a copy of that value.
pub fn ctl_secret_key(config: &Config) -> Result<String> {
    match henv::var(CTL_SECRET_ENVVAR) {
        Ok(v) => Ok(v),
        Err(_) => {
            match config.ctl_secret {
                Some(ref v) => Ok(v.to_string()),
                None => SrvClient::read_secret_key().map_err(Error::from),
            }
        }
    }
}

fn cli_config_path() -> PathBuf {
    if !am_i_root() {
        if let Some(home) = dirs::home_dir() {
            return home.join(format!(".{}", CLI_CONFIG_PATH));
        }
    }
    PathBuf::from(&*FS_ROOT_PATH).join(CLI_CONFIG_PATH)
}
