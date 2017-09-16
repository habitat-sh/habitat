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

use {PRODUCT, VERSION};
use depot_client;
use hyper::status::StatusCode;
use retry::retry;
use super::{RETRIES, RETRY_WAIT};

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
    ) -> bool {
        if !self.enabled {
            debug!("Publishing skipped (not enabled)");
            return true;
        }
        debug!(
            "Publisher (url: {}, channel: {:?})",
            self.url,
            self.channel_opt
        );

        let client = depot_client::Client::new(&self.url, PRODUCT, VERSION, None).unwrap();

        match retry(RETRIES, RETRY_WAIT, || client.x_put_package(archive, auth_token), |res| {
            let msg = format!("Upload status: {:?}", res);
            debug!("{}", msg);
            logger.log(&msg);
            match *res {
                Ok(_) |  // Conflict means package got uploaded earlier
                Err(depot_client::Error::APIError(StatusCode::Conflict, _)) => true,
                Err(_) => false
            }
        }) {
            Ok(_) => (),
            Err(_) => {
                let msg = format!("Publisher failed uploading package after {} retries", RETRIES);
                error!("{}", msg);
                logger.log(&msg);
                return false;
            }
        }

        let ident = archive.ident().unwrap();

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
                        let msg = format!("Create channel status: {:?}", res);
                        debug!("{}", msg);
                        logger.log(&msg);
                        match *res {
                            Ok(_) |  // Conflict means channel got created earlier
                            Err(depot_client::Error::APIError(StatusCode::Conflict, _)) => true,
                            Err(_) => false
                        }
                    },
                ) {
                    Ok(_) => (),
                    Err(_) => {
                        let msg = format!("Publisher failed creating channel after {} retries", RETRIES);
                        error!("{}", msg);
                        logger.log(&msg);
                        return false;
                    }
                }
            }

            match retry(RETRIES,
                RETRY_WAIT,
                || client.promote_package(&ident, &channel, auth_token),
                |res| {
                    let msg = format!("Promote status: {:?}", res);
                    debug!("{}", msg);
                    logger.log(&msg);
                    res.is_ok()
                },
            ) {
                Ok(_) => (),
                Err(_) => {
                    let msg = format!("Publisher failed promoting package after {} retries", RETRIES);
                    error!("{}", msg);
                    logger.log(&msg);
                    return false;
                }
            }
        }
        true
    }
}
