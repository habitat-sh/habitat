use crate::{error::{Error as HabError,
                    Result as HabResult},
            hcore::fs::{am_i_root,
                        FS_ROOT_PATH},
            CTL_SECRET_ENVVAR};
use habitat_core::{env as henv,
                   origin::Origin};
use habitat_sup_client::SrvClient;
use std::{fs,
          io,
          path::{Path,
                 PathBuf}};

const CLI_CONFIG_PATH_POSTFIX: &str = "hab/etc/cli.toml";

lazy_static::lazy_static! {
    static ref CLI_CONFIG_PATH: PathBuf = cli_config_path();
    static ref CLI_CONFIG_PATH_PARENT: &'static Path = CLI_CONFIG_PATH
                                                    .parent()
                                                    .expect("cli config path parent");
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("'{}' io failure, err: {0}", CLI_CONFIG_PATH.display())]
    Io(#[from] io::Error),
    #[error("deserializing '{}' failed, err: {0}", CLI_CONFIG_PATH.display())]
    Deserialize(#[from] toml::de::Error),
    #[error("serializing '{}' failed, err: {0}", CLI_CONFIG_PATH.display())]
    Serialize(#[from] toml::ser::Error),
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
pub struct Config {
    pub auth_token: Option<String>,
    pub origin:     Option<Origin>,
    pub ctl_secret: Option<String>,
    pub bldr_url:   Option<String>,
}

impl Config {
    fn from_file<T: AsRef<Path>>(path: T) -> Result<Self, Error> {
        let raw = fs::read_to_string(path)?;
        Ok(toml::from_str(&raw)?)
    }

    pub fn load() -> Result<Self, Error> {
        if CLI_CONFIG_PATH.exists() {
            debug!("Loading CLI config from {}", CLI_CONFIG_PATH.display());
            Ok(Config::from_file(&*CLI_CONFIG_PATH)?)
        } else {
            debug!("No CLI config found, loading defaults");
            Ok(Config::default())
        }
    }

    pub fn save(&self) -> Result<(), Error> {
        fs::create_dir_all(&*CLI_CONFIG_PATH_PARENT)?;
        let raw = toml::ser::to_string(self)?;
        debug!("Raw config toml:\n---\n{}\n---", &raw);
        fs::write(&*CLI_CONFIG_PATH, raw)?;
        Ok(())
    }
}

/// Check if the HAB_CTL_SECRET env var. If not, check the CLI config to see if there is a ctl
/// secret set and return a copy of that value.
pub fn ctl_secret_key(config: &Config) -> HabResult<String> {
    match henv::var(CTL_SECRET_ENVVAR) {
        Ok(v) => Ok(v),
        Err(_) => {
            match config.ctl_secret {
                Some(ref v) => Ok(v.to_string()),
                None => SrvClient::read_secret_key().map_err(HabError::from),
            }
        }
    }
}

fn cli_config_path() -> PathBuf {
    if !am_i_root() {
        if let Some(home) = dirs::home_dir() {
            return home.join(format!(".{}", CLI_CONFIG_PATH_POSTFIX));
        }
    }
    PathBuf::from(&*FS_ROOT_PATH).join(CLI_CONFIG_PATH_POSTFIX)
}
