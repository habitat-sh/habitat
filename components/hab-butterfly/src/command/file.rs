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

pub mod upload {
    use std::path::Path;
    use std::io::Read;
    use std::fs::File;
    use std::thread;
    use std::time;

    use butterfly::client::Client;
    use common::ui::{Status, UI};
    use hcore::crypto::{SymKey, BoxKeyPair};
    use hcore::service::ServiceGroup;

    use error::{Error, Result};

    pub fn start(ui: &mut UI,
                 sg: &ServiceGroup,
                 number: u64,
                 file_path: &Path,
                 peers: &Vec<String>,
                 ring_key: Option<&SymKey>,
                 user_pair: Option<&BoxKeyPair>,
                 service_pair: Option<&BoxKeyPair>)
                 -> Result<()> {
        try!(ui.begin(format!("Uploading file {} to {} incarnation {}",
                              &file_path.display(),
                              sg,
                              number)));
        try!(ui.status(Status::Creating, format!("service file")));

        let mut body = Vec::new();
        let mut file = try!(File::open(&file_path));
        try!(file.read_to_end(&mut body));

        // Safe because clap checks that this is a real file that exists
        let filename = file_path
            .file_name()
            .unwrap()
            .to_string_lossy()
            .into_owned();

        let mut encrypted = false;
        if service_pair.is_some() && user_pair.is_some() {
            try!(ui.status(Status::Encrypting,
                           format!("file as {} for {}",
                                   user_pair.unwrap().name_with_rev(),
                                   service_pair.unwrap().name_with_rev())));
            body = try!(user_pair.unwrap().encrypt(&body, service_pair.unwrap()));
            encrypted = true;
        }

        for peer in peers.iter() {
            try!(ui.status(Status::Applying, format!("to peer {}", peer)));
            let mut client = try!(Client::new(peer, ring_key.map(|k| k.clone()))
                .map_err(|e| Error::ButterflyError(format!("{}", e))));
            try!(client
                     .send_service_file(sg.clone(),
                                        filename.clone(),
                                        number,
                                        body.clone(),
                                        encrypted)
                     .map_err(|e| Error::ButterflyError(format!("{}", e))));

            // please take a moment to weep over the following line
            // of code. We must sleep to allow messages to be sent
            // before freeing the socket to prevent loss.
            // see https://github.com/zeromq/libzmq/issues/1264
            thread::sleep(time::Duration::from_millis(100));
        }
        try!(ui.end("Uploaded file"));
        Ok(())
    }
}
