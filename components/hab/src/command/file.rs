// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

pub mod upload {
    use std::path::Path;

    use ansi_term::Colour::{Blue, Green, Yellow};
    use hcore::crypto::{BoxKeyPair, SymKey};
    use common::gossip_file::GossipFile;

    use error::Result;
    use gossip::{self, hab_gossip};

    pub fn start(peers: &Vec<String>,
                 ring_key: Option<&SymKey>,
                 user_pair: &BoxKeyPair,
                 service_pair: &BoxKeyPair,
                 number: u64,
                 file_path: &Path)
                 -> Result<()> {
        println!("{}",
                 Yellow.bold().paint(format!("» Uploading file {}", &file_path.display())));
        let file =
            try!(GossipFile::from_file_encrypt(&user_pair, &service_pair, file_path, number));

        let rumor = hab_gossip::Rumor::gossip_file(file);
        let mut list = hab_gossip::RumorList::new();
        list.add_rumor(rumor);
        if let Some(ring_key) = ring_key {
            println!("{} communication to \"{}\" ring with {}",
                     Green.bold().paint("☛ Encrypting"),
                     &ring_key.name,
                     &ring_key.name_with_rev());

        }
        println!("{} {} for {} into ring via {:?}",
                 Green.bold().paint("↑ Uploading"),
                 &file_path.display(),
                 &service_pair.name,
                 &peers);
        try!(gossip::send_rumors_to_peers(&peers, ring_key, &list));
        println!("{}",
                 Blue.paint(format!("★ Upload of {} complete.", &file_path.display())));
        Ok(())
    }
}
