// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

pub mod apply {
    use std::path::Path;
    use std::io::{self, Read};

    use hcore::crypto::SymKey;
    use hcore::service::ServiceGroup;
    use common::gossip_file::GossipFile;

    use error::Result;
    use gossip::{self, hab_gossip};

    pub fn start(peers: &Vec<String>,
                 ring_key: Option<&SymKey>,
                 sg: &ServiceGroup,
                 number: u64,
                 file_path: Option<&Path>,
                 as_gossip: bool)
                 -> Result<()> {
        let sg1 = sg.clone();
        let file = match file_path {
            Some(p) => try!(GossipFile::from_file(sg.clone(), p, number, as_gossip)),
            None => {
                let mut body = vec![0; 1024];
                try!(io::stdin().read_to_end(&mut body));
                try!(GossipFile::from_body(sg.clone(), "config.toml".to_string(), body, number))
            }
        };
        println!("Applying configuration {} to {}", &file, &sg1);
        let rumor = hab_gossip::Rumor::gossip_file(file);

        let mut list = hab_gossip::RumorList::new();
        list.add_rumor(rumor);

        try!(gossip::send_rumors_to_peers(&peers, ring_key, &list));
        println!("Finished applying configuration");
        Ok(())
    }
}
