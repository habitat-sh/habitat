// Copyright (c) 2016-2017 Chef Software Inc. and/or applicable contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

use dirs;

use crate::hcore::config::ConfigFile;
use crate::hcore::fs::{am_i_root, FS_ROOT_PATH};
use toml;

use crate::error::{Error, Result};

const CLI_CONFIG_PATH: &'static str = "hab/etc/cli.toml";

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Config {
    pub auth_token: Option<String>,
    pub origin: Option<String>,
    pub ctl_secret: Option<String>,
    pub bldr_url: Option<String>,
}

impl ConfigFile for Config {
    type Error = Error;
}

impl Default for Config {
    fn default() -> Self {
        Config {
            auth_token: None,
            origin: None,
            ctl_secret: None,
            bldr_url: None,
        }
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
            return Err(Error::FileNotFound(
                config_path.to_string_lossy().into_owned(),
            ))
        }
    };
    fs::create_dir_all(&parent_path)?;
    let raw = toml::ser::to_string(config)?;
    debug!("Raw config toml:\n---\n{}\n---", &raw);
    let mut file = File::create(&config_path)?;
    file.write_all(raw.as_bytes())?;
    Ok(())
}

fn cli_config_path() -> PathBuf {
    if !am_i_root() {
        if let Some(home) = dirs::home_dir() {
            return home.join(format!(".{}", CLI_CONFIG_PATH));
        }
    }
    PathBuf::from(&*FS_ROOT_PATH).join(CLI_CONFIG_PATH)
}
