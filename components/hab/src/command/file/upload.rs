// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

use std::path::Path;

use hcore::crypto::{Context, SymKey};
use hcore::service::ServiceGroup;
use common::gossip_file::GossipFile;

use error::Result;
use gossip::{self, hab_gossip};

pub fn start(peers: &Vec<String>,
             ring_key: Option<SymKey>,
             sg: ServiceGroup,
             number: u64,
             file_path: &Path,
             user: &str)
             -> Result<()> {
    let sg1 = sg.clone();
    let crypto_ctx = Context::default();
    let file = try!(GossipFile::from_file_encrypt(&crypto_ctx, sg, file_path, number, user));

    println!("Uploading file {} to {}", &file, &sg1);
    let rumor = hab_gossip::Rumor::gossip_file(file);

    let mut list = hab_gossip::RumorList::new();
    list.add_rumor(rumor);

    try!(gossip::send_rumors_to_peers(&peers, ring_key, &list));
    println!("Finished uploading file");
    Ok(())
}
