use crate::{error::{Error as HabError,
                    Result},
            hcore::fs::{am_i_root,
                        FS_ROOT_PATH},
            CTL_SECRET_ENVVAR};
use habitat_core::{env as henv,
                   origin::Origin};
use habitat_sup_client::SrvClient;
use std::{fs::{self,
               File},
          io::{self,
               Write},
          path::{Path,
                 PathBuf}};

const CLI_CONFIG_PATH_POSTFIX: &str = "hab/etc/cli.toml";

lazy_static::lazy_static! {
    pub static ref CLI_CONFIG_PATH: PathBuf = cli_config_path();
    pub static ref CLI_CONFIG_PATH_PARENT: &'static Path = CLI_CONFIG_PATH
                                                    .parent()
                                                    .expect("cli config path parent");

    /// A cached reading of the config file. This avoids the need to continually read from disk.
    /// However, it also means changes to the file will not be picked up after the program has
    /// started. Ideally, we would repopulate this struct on file change or on some configured
    /// interval.
    /// https://github.com/habitat-sh/habitat/issues/7243
    pub static ref CACHED: Config = load().unwrap_or_default();
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("reading '{}' failed, err: {0}", CLI_CONFIG_PATH.display())]
    File(#[from] io::Error),
    #[error("parsing '{}' failed, err: {0}", CLI_CONFIG_PATH.display())]
    Toml(#[from] toml::de::Error),
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
pub struct Config {
    pub auth_token: Option<String>,
    pub origin:     Option<Origin>,
    pub ctl_secret: Option<String>,
    pub bldr_url:   Option<String>,
}

impl Config {
    fn from_file<T: AsRef<Path>>(path: T) -> std::result::Result<Self, Error> {
        let raw = fs::read_to_string(path)?;
        Ok(toml::from_str(&raw)?)
    }
}

pub fn load() -> std::result::Result<Config, Error> {
    if CLI_CONFIG_PATH.exists() {
        debug!("Loading CLI config from {}", CLI_CONFIG_PATH.display());
        Ok(Config::from_file(&*CLI_CONFIG_PATH)?)
    } else {
        debug!("No CLI config found, loading defaults");
        Ok(Config::default())
    }
}

pub fn save(config: &Config) -> Result<()> {
    fs::create_dir_all(&*CLI_CONFIG_PATH_PARENT)?;
    let raw = toml::ser::to_string(config)?;
    debug!("Raw config toml:\n---\n{}\n---", &raw);
    let mut file = File::create(&*CLI_CONFIG_PATH)?;
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
