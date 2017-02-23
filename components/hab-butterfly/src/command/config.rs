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
    use std::str;
    use std::path::Path;
    use std::io::{self, Read};
    use std::fs::File;
    use std::thread;
    use std::time;

    use butterfly::client::Client;
    use common::ui::{Status, UI};
    use hcore::crypto::{SymKey, BoxKeyPair};
    use hcore::service::ServiceGroup;
    use toml;

    use error::{Error, Result};

    pub fn start(ui: &mut UI,
                 sg: &ServiceGroup,
                 number: u64,
                 file_path: Option<&Path>,
                 peers: &Vec<String>,
                 ring_key: Option<&SymKey>,
                 user_pair: Option<&BoxKeyPair>,
                 service_pair: Option<&BoxKeyPair>)
                 -> Result<()> {
        try!(ui.begin(format!("Applying configuration for {} incarnation {}", sg, number,)));

        try!(ui.status(Status::Creating, format!("service configuration")));

        let mut body = Vec::new();

        match file_path {
            Some(p) => {
                let mut file = try!(File::open(&p));
                try!(file.read_to_end(&mut body));
            }
            None => {
                try!(io::stdin().read_to_end(&mut body));
            }
        };

        match toml::de::from_slice::<toml::value::Value>(&body) {
            Ok(_) => try!(ui.status(Status::Verified, "this configuration is valid TOML")),
            Err(err) => {
                try!(ui.fatal("Invalid TOML"));
                try!(ui.br());
                try!(ui.warn(&err));
                try!(ui.br());
                return Err(Error::TomlDeserializeError(err));
            }
        }

        let mut encrypted = false;
        if service_pair.is_some() && user_pair.is_some() {
            try!(ui.status(Status::Encrypting,
                           format!("TOML as {} for {}",
                                   user_pair.unwrap().name_with_rev(),
                                   service_pair.unwrap().name_with_rev())));
            body = try!(user_pair.unwrap().encrypt(&body, service_pair.unwrap()));
            encrypted = true;
        }

        for peer in peers.iter() {
            try!(ui.status(Status::Applying, format!("to peer {}", peer)));
            let mut client = try!(Client::new(peer, ring_key.map(|k| k.clone()))
                .map_err(|e| Error::ButterflyError(format!("{}", e))));
            try!(client.send_service_config(sg.clone(), number, body.clone(), encrypted)
                .map_err(|e| Error::ButterflyError(format!("{}", e))));

            // please take a moment to weep over the following line
            // of code. We must sleep to allow messages to be sent
            // before freeing the socket to prevent loss.
            // see https://github.com/zeromq/libzmq/issues/1264
            thread::sleep(time::Duration::from_millis(100));
        }
        try!(ui.end("Applied configuration"));
        Ok(())
    }
}
