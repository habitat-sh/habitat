// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

use std::path::Path;

use config::Config;
use config_file::{ConfigFile, ServiceGroup};
use error::BldrResult;
use gossip::client::Client;
use gossip::rumor::{Rumor, RumorList};

static LOGKEY: &'static str = "IJ";

pub fn inject(config: &Config) -> BldrResult<()> {
    let sg = try!(ServiceGroup::from(&config.service_group()));
    let vn = *config.version_number();
    let cf = try!(ConfigFile::from_file(sg, Path::new(&config.file_path()), vn));
    let mut rumor_list = RumorList::new();
    let rumor = Rumor::config_file(cf);
    rumor_list.add_rumor(rumor);
    try!(initial_peers(config.gossip_peer(), &rumor_list));
    Ok(())
}

pub fn initial_peers(peer_listeners: &[String], rumor_list: &RumorList) -> BldrResult<()> {
    let fail_after = 10;
    let mut count = 0;

    if peer_listeners.len() > 0 {
        while count < fail_after {
            if try_peers(peer_listeners, rumor_list) {
                return Ok(());
            } else {
                count = count + 1;
                outputln!("Could not connect to any initial peers; attempt {} of {}.",
                          count,
                          fail_after);
            }
        }
    }
    Ok(())
}

fn try_peers(peer_listeners: &[String], rumor_list: &RumorList) -> bool {
    let mut initialized = false;
    for to in peer_listeners {
        outputln!("Joining gossip peer at {}", to);
        let mut c = match Client::new(&to[..]) {
            Ok(c) => c,
            Err(e) => {
                debug!("Error creating gossip client - {:?}", e);
                outputln!("Failed to create a gossip client for {}", to);
                continue;
            }
        };

        match c.inject(rumor_list.clone()) {
            Ok(_) => outputln!("Rumors injected at {}", to),
            Err(e) => {
                outputln!("Failed to ping {:?}: {:?}", to, e);
                continue;
            }
        }
        initialized = true;
    }
    initialized
}
