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

pub mod upload {
    use std::path::Path;

    use common::gossip_file::GossipFile;
    use common::ui::{Status, UI};
    use hcore::crypto::{BoxKeyPair, SymKey};

    use error::Result;
    use gossip::{self, hab_gossip};

    pub fn start(ui: &mut UI,
                 peers: &Vec<String>,
                 ring_key: Option<&SymKey>,
                 user_pair: &BoxKeyPair,
                 service_pair: &BoxKeyPair,
                 number: u64,
                 file_path: &Path)
                 -> Result<()> {
        try!(ui.begin(format!("Uploading file {}", &file_path.display())));
        let file =
            try!(GossipFile::from_file_encrypt(&user_pair, &service_pair, file_path, number));

        let rumor = hab_gossip::Rumor::gossip_file(file);
        let mut list = hab_gossip::RumorList::new();
        list.add_rumor(rumor);
        if let Some(ring_key) = ring_key {
            try!(ui.status(Status::Encrypting,
                           format!("communication to \"{}\" ring with {}",
                                   &ring_key.name,
                                   &ring_key.name_with_rev())));
        }
        try!(ui.status(Status::Uploading,
                       format!("{} for {} into ring via {:?}",
                               &file_path.display(),
                               &service_pair.name,
                               &peers)));
        try!(gossip::send_rumors_to_peers(&peers, ring_key, &list));
        try!(ui.end(format!("Upload of {} complete.", &file_path.display())));
        Ok(())
    }
}
