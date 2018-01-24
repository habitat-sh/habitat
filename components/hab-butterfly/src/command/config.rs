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

pub mod apply {
    use std::path::Path;
    use std::io::{self, Read};
    use std::fs::File;
    use std::thread;
    use std::time;

    use butterfly::client::Client;
    use common::ui::{Status, UI, UIWriter};
    use hcore::crypto::{SymKey, BoxKeyPair};
    use hcore::service::ServiceGroup;
    use toml;

    use error::{Error, Result};

    pub fn start(
        ui: &mut UI,
        sg: &ServiceGroup,
        number: u64,
        file_path: Option<&Path>,
        peers: &Vec<String>,
        ring_key: Option<&SymKey>,
        user_pair: Option<&BoxKeyPair>,
        service_pair: Option<&BoxKeyPair>,
    ) -> Result<()> {
        ui.begin(
            format!("Applying configuration for {} incarnation {}", sg, number, ),
        )?;

        ui.status(
            Status::Creating,
            format!("service configuration"),
        )?;

        let mut body = Vec::new();

        match file_path {
            Some(p) => {
                let mut file = File::open(&p)?;
                file.read_to_end(&mut body)?;
            }
            None => {
                io::stdin().read_to_end(&mut body)?;
            }
        };

        match toml::de::from_slice::<toml::value::Value>(&body) {
            Ok(_) => {
                ui.status(
                    Status::Verified,
                    "this configuration is valid TOML",
                )?
            }
            Err(err) => {
                ui.fatal("Invalid TOML")?;
                ui.br()?;
                ui.warn(&err)?;
                ui.br()?;
                return Err(Error::TomlDeserializeError(err));
            }
        }

        let mut encrypted = false;
        if service_pair.is_some() && user_pair.is_some() {
            ui.status(
                Status::Encrypting,
                format!(
                    "TOML as {} for {}",
                    user_pair.unwrap().name_with_rev(),
                    service_pair.unwrap().name_with_rev()
                ),
            )?;
            body = user_pair.unwrap().encrypt(
                &body,
                Some(service_pair.unwrap()),
            )?;
            encrypted = true;
        }

        for peer in peers.iter() {
            ui.status(Status::Applying, format!("to peer {}", peer))?;
            let mut client = Client::new(peer, ring_key.map(|k| k.clone())).map_err(
                |e| {
                    Error::ButterflyError(format!("{}", e))
                },
            )?;
            client
                .send_service_config(sg.clone(), number, body.clone(), encrypted)
                .map_err(|e| Error::ButterflyError(format!("{}", e)))?;

            // please take a moment to weep over the following line
            // of code. We must sleep to allow messages to be sent
            // before freeing the socket to prevent loss.
            // see https://github.com/zeromq/libzmq/issues/1264
            thread::sleep(time::Duration::from_millis(100));
        }
        ui.end("Applied configuration")?;
        Ok(())
    }
}
