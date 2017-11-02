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

use hab_core::package::archive::PackageArchive;
use hab_core::channel::{STABLE_CHANNEL, UNSTABLE_CHANNEL};
use bldr_core::logger::Logger;

use super::{RETRIES, RETRY_WAIT};
use {PRODUCT, VERSION};
use depot_client;
use hyper::status::StatusCode;
use retry::retry;
use error::{Error, Result};

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct Publisher {
    pub enabled: bool,
    pub url: String,
    pub channel_opt: Option<String>,
}

impl Publisher {
    pub fn run(
        &mut self,
        archive: &mut PackageArchive,
        auth_token: &str,
        logger: &mut Logger,
    ) -> Result<()> {
        if !self.enabled {
            debug!("Publishing skipped (not enabled)");
            return Ok(());
        }
        debug!(
            "Publisher (url: {}, channel: {:?})",
            self.url,
            self.channel_opt
        );

        let client = depot_client::Client::new(&self.url, PRODUCT, VERSION, None).unwrap();
        let ident = archive.ident().unwrap();

        match retry(RETRIES, RETRY_WAIT, || client.x_put_package(archive, auth_token), |res| {
            match *res {
                Ok(_) |  // Conflict means package got uploaded earlier
                Err(depot_client::Error::APIError(StatusCode::Conflict, _)) => true,
                Err(_) => {
                    let msg = format!("Upload {}: {:?}", ident, res);
                    debug!("{}", msg);
                    logger.log(&msg);
                    false
                }
            }
        }) {
            Ok(_) => (),
            Err(err) => {
                let msg = format!("Failed to upload {} after {} retries", ident, RETRIES);
                warn!("{}", msg);
                logger.log(&msg);
                return Err(Error::Retry(err));
            }
        }

        if self.channel_opt.is_none() {
            debug!("Promotion skipped (no channel specified)");
        } else {
            let channel = match self.channel_opt {
                Some(ref c) => c.clone(),
                None => panic!("Expected channel"),
            };

            if channel != STABLE_CHANNEL && channel != UNSTABLE_CHANNEL {
                match retry(RETRIES,
                    RETRY_WAIT,
                    || client.create_channel(&ident.origin, &channel, auth_token),
                    |res| {
                        match *res {
                            Ok(_) |  // Conflict means channel got created earlier
                            Err(depot_client::Error::APIError(StatusCode::Conflict, _)) => true,
                            Err(_) => {
                                let msg = format!("Create channel {}: {:?}", channel, res);
                                debug!("{}", msg);
                                logger.log(&msg);
                                false
                            }
                        }
                    },
                ) {
                    Ok(_) => (),
                    Err(err) => {
                        let msg = format!("Failed to create channel {} after {} retries",
                            channel, RETRIES);
                        warn!("{}", msg);
                        logger.log(&msg);
                        return Err(Error::Retry(err));
                    }
                }
            }

            match retry(RETRIES,
                RETRY_WAIT,
                || client.promote_package(&ident, &channel, auth_token),
                |res| {
                    if res.is_err() {
                        let msg = format!("Promote {} to {}: {:?}", ident, channel, res);
                        debug!("{}", msg);
                        logger.log(&msg);
                    };
                    res.is_ok()
                },
            ) {
                Ok(_) => (),
                Err(err) => {
                    let msg = format!("Failed to promote {} to {} after {} retries",
                        ident, channel, RETRIES);
                    warn!("{}", msg);
                    logger.log(&msg);
                    return Err(Error::Retry(err));
                }
            }
        }
        Ok(())
    }
}
