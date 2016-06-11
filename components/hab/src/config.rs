// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

use hcore::config::{ConfigFile, ParseInto};
use hcore::fs::{am_i_root, FS_ROOT_PATH};
use toml;

use error::{Error, Result};

const CLI_CONFIG_PATH: &'static str = "hab/etc/cli.toml";

pub fn load() -> Result<Config> {
    let cli_config_path = cli_config_path();
    if cli_config_path.exists() {
        debug!("Loading CLI config from {}", cli_config_path.display());
        Ok(try!(Config::from_file(&cli_config_path)))
    } else {
        debug!("No CLI config found, loading defaults");
        Ok(Config::default())
    }
}

pub fn save(config: &Config) -> Result<()> {
    let config_path = cli_config_path();
    let parent_path = match config_path.parent() {
        Some(p) => p,
        None => return Err(Error::FileNotFound(config_path.to_string_lossy().into_owned())),
    };
    try!(fs::create_dir_all(&parent_path));
    let raw = toml::encode_str(config);
    debug!("Raw config toml:\n---\n{}\n---", &raw);
    let mut file = try!(File::create(&config_path));
    try!(file.write_all(raw.as_bytes()));
    Ok(())
}

fn cli_config_path() -> PathBuf {
    match am_i_root() {
        true => PathBuf::from(FS_ROOT_PATH).join(CLI_CONFIG_PATH),
        _ => {
            match env::home_dir() {
                Some(home) => home.join(format!(".{}", CLI_CONFIG_PATH)),
                None => PathBuf::from(FS_ROOT_PATH).join(CLI_CONFIG_PATH),
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, RustcEncodable)]
pub struct Config {
    pub auth_token: Option<String>,
    pub origin: Option<String>,
}

impl ConfigFile for Config {
    type Error = Error;

    fn from_toml(toml: toml::Value) -> Result<Self> {
        let mut cfg = Config::default();
        try!(toml.parse_into("auth_token", &mut cfg.auth_token));
        try!(toml.parse_into("origin", &mut cfg.origin));
        Ok(cfg)
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            auth_token: None,
            origin: None,
        }
    }
}
