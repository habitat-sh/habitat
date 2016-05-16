// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

pub mod apply {
    use std::path::Path;
    use std::io::{self, Read};

    use ansi_term::Colour::{Blue, Green, Yellow};
    use hcore::crypto::SymKey;
    use hcore::service::ServiceGroup;
    use common::gossip_file::GossipFile;

    use error::Result;
    use gossip::{self, hab_gossip};

    pub fn start(peers: &Vec<String>,
                 ring_key: Option<&SymKey>,
                 sg: &ServiceGroup,
                 number: u64,
                 file_path: Option<&Path>)
                 -> Result<()> {
        println!("{}",
                 Yellow.bold().paint(format!("» Applying configuration")));
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
            println!("{} communication to \"{}\" ring with {}",
                     Green.bold().paint("☛ Encrypting"),
                     &ring_key.name,
                     &ring_key.name_with_rev());

        }
        println!("{} configuration for {} into ring via {:?}",
                 Green.bold().paint("↑ Applying"),
                 &sg,
                 &peers);
        try!(gossip::send_rumors_to_peers(&peers, ring_key, &list));
        println!("{}", Blue.paint(format!("★ Applied configuration.")));
        Ok(())
    }
}
