// Copyright (c) 2017 Chef Software Inc. and/or applicable contributors
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

//! Encapsulate a butterfly client and expose its functionality via a
//! test-focused API. Clients are configured for a specific service
//! group (namely, the one of the test package we are running).
//!
//! No ring key or encryption abilities are currently supported.

extern crate habitat_butterfly;
use self::habitat_butterfly::client::Client as ButterflyClient;

extern crate habitat_core;
use self::habitat_core::service::ServiceGroup;

extern crate toml;

use std::net::SocketAddr;
use std::time::{UNIX_EPOCH, SystemTime};

pub struct Client {
    butterfly_client: ButterflyClient,
    pub package_name: String,
    pub service_group: String,
}

impl Client {
    pub fn new<T, U>(package_name: T, service_group: U, port: u16) -> Client
    where
        T: ToString,
        U: ToString,
    {

        let gossip_addr = format!("127.0.0.1:{}", port).parse::<SocketAddr>().expect(
            "Could not parse Butterfly gossip address!",
        );
        let c = ButterflyClient::new(&gossip_addr, None).expect(
            "Could not create Butterfly Client for test!",
        );
        Client {
            butterfly_client: c,
            package_name: package_name.to_string(),
            service_group: service_group.to_string(),
        }
    }

    /// Apply the given configuration to the supervisor. It will
    /// always be applied to the service group for which the client was
    /// initially configured.
    ///
    /// A time-based incarnation value is automatically used,
    /// resulting in less clutter in your tests.
    pub fn apply<T>(&mut self, config: T)
    where
        T: ToString,
    {
        let config = config.to_string();

        // Validate the TOML, to save you from typos in your tests
        if let Err(err) = self::toml::de::from_slice::<self::toml::value::Value>(
            &config.as_bytes(),
        )
        {
            panic!("Invalid TOML! {:?} ==> {:?}", config, err);
        }

        let payload = Vec::from(config.as_bytes());
        let incarnation = Self::new_incarnation();
        self.butterfly_client
            .send_service_config(
                ServiceGroup::new(None, &self.package_name, &self.service_group, None).unwrap(),
                incarnation,
                payload,
                false,
            )
            .expect("Cannot send the service configuration");
    }

    /// Generate a new incarnation number using the number of seconds
    /// since the Unix Epoch. As a result, this is unique to within a
    /// second, so beware! Might need to incorporate nanoseconds as well.
    fn new_incarnation() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}
