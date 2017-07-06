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

use std::path::{Path, PathBuf};

use hab_core::package::archive::PackageArchive;
use hab_core::config::ConfigFile;

use super::workspace::Workspace;
use {PRODUCT, VERSION};
use config::Config;
use depot_client;
use error::{Error, Result};

/// Postprocessing config file name
const CONFIG_FILE: &'static str = "builder.toml";

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct Publish {
    pub enabled: bool,
    pub url: String,
    pub channel: String,
}

impl Publish {
    pub fn run(&mut self, archive: &mut PackageArchive, auth_token: &str) -> bool {
        if !self.enabled {
            return true;
        }
        debug!(
            "post process: publish (url: {}, channel: {})",
            self.url,
            self.channel
        );
        let client = depot_client::Client::new(&self.url, PRODUCT, VERSION, None).unwrap();
        if let Some(err) = client.x_put_package(archive, auth_token).err() {
            error!("post processing error uploading package, ERR={:?}", err);
            return false;
        };
        let ident = archive.ident().unwrap();
        if let Some(err) = client
            .promote_package(&ident, &self.channel, auth_token)
            .err()
        {
            error!("post processing error promoting package, ERR={:?}", err);
            return false;
        };
        true
    }
}

#[derive(Default, Deserialize)]
#[serde(default)]
struct PublishBuilder {
    /// Whether publish is enabled
    enabled: Option<bool>,
    /// URL to Depot API
    url: Option<String>,
    /// Channel to publish to
    channel: Option<String>,
}

impl PublishBuilder {
    fn new(config_path: &Path) -> Result<Self> {
        let builder = if config_path.exists() {
            debug!(
                "using post processing config from {}",
                config_path.display()
            );
            PublishBuilder::from_file(config_path)?
        } else {
            debug!("no post processing config - using defaults");
            PublishBuilder::default()
        };
        Ok(builder)
    }

    fn build(self, config: &Config) -> Publish {
        Publish {
            enabled: self.enabled.unwrap_or(config.auto_publish),
            url: self.url.unwrap_or(config.depot_url.clone()),
            channel: self.channel.unwrap_or(config.depot_channel.clone()),
        }
    }
}

impl ConfigFile for PublishBuilder {
    type Error = Error;
}

pub struct PostProcessor {
    config_path: PathBuf,
}

impl PostProcessor {
    pub fn new(workspace: &Workspace) -> Self {
        let parent_path = Path::new(workspace.job.get_project().get_plan_path())
            .parent()
            .unwrap();
        let file_path = workspace.src().join(parent_path.join(CONFIG_FILE));
        PostProcessor { config_path: file_path }
    }

    pub fn run(&mut self, archive: &mut PackageArchive, config: &Config) -> bool {
        let mut publisher = match PublishBuilder::new(&self.config_path) {
            Ok(builder) => builder.build(config),
            Err(err) => {
                warn!("Failed to parse builder config, {}", err);
                return false;
            }
        };
        debug!("starting post processing");
        publisher.run(archive, &config.auth_token)
    }
}

#[cfg(test)]
mod tests {
    use hab_core::config::ConfigFile;
    use super::*;
    use config::Config;

    #[test]
    fn test_publish_config_from_toml() {
        let toml = r#"
        enabled = false
        url = "https://willem.habitat.sh/v1/depot"
        channel = "unstable"
        "#;

        let config = Config::default();
        let cfg = PublishBuilder::from_raw(toml).unwrap().build(&config);
        assert_eq!("https://willem.habitat.sh/v1/depot", cfg.url);
        assert_eq!(false, cfg.enabled);
        assert_eq!("unstable", cfg.channel);
    }
}
