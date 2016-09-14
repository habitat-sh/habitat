// Copyright (c) 2016 Chef Software Inc. and/or applicable contributors
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

    use common::gossip_file::GossipFile;
    use common::ui::{Status, UI};
    use hcore::crypto::SymKey;
    use hcore::service::ServiceGroup;

    use error::Result;
    use gossip::{self, hab_gossip};

    pub fn start(ui: &mut UI,
                 peers: &Vec<String>,
                 ring_key: Option<&SymKey>,
                 sg: &ServiceGroup,
                 number: u64,
                 file_path: Option<&Path>)
                 -> Result<()> {
        try!(ui.begin("Applying configuration"));
        let file = match file_path {
            Some(p) => try!(GossipFile::from_file(sg.clone(), p, number)),
            None => {
                let mut body = String::new();
                try!(io::stdin().read_to_string(&mut body));
                try!(GossipFile::from_body(sg.clone(), body.into(), number))
            }
        };
        let rumor = hab_gossip::Rumor::gossip_file(file);

        let mut list = hab_gossip::RumorList::new();
        list.add_rumor(rumor);

        if let Some(ring_key) = ring_key {
            try!(ui.status(Status::Encrypting,
                           format!("communication to \"{}\" ring with {}",
                                   &ring_key.name,
                                   &ring_key.name_with_rev())));
        }
        try!(ui.status(Status::Applying,
                       format!("configuration for {} into ring via {:?}", &sg, &peers)));
        try!(gossip::send_rumors_to_peers(&peers, ring_key, &list));
        try!(ui.end("Applied configuration"));
        Ok(())
    }
}
