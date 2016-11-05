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
    use std::net::SocketAddr;
    use std::path::Path;
    use std::io::{self, Read};

    use common::ui::{Status, UI};
    use hcore::crypto::SymKey;
    use hcore::service::ServiceGroup;
    // use butterfly::server::Server;
    // use butterfly::member::Member;
    // use butterfly::trace::Trace;
    // use butterfly::rumor::service_config::ServiceConfig;
    // use butterfly::server::timing::Timing;

    use error::Result;

    pub fn start(ui: &mut UI,
                 peers: &Vec<String>,
                 ring_key: Option<&SymKey>,
                 sg: &ServiceGroup,
                 number: u64,
                 file_path: Option<&Path>)
                 -> Result<()> {
        try!(ui.begin("Not applying configuration - rebuilding"));

        // Set up Butterfly
        //        let swim_addr: SocketAddr = try!(gconfig().swim_listen().parse());
        //        let gossip_addr: SocketAddr = try!(gconfig().gossip_listen().parse());
        //        let mut member = Member::new();
        //        member.set_swim_port(swim_addr.port() as i32);
        //        member.set_gossip_port(gossip_addr.port() as i32);
        //
        //        let server = try!(Server::new(gconfig().swim_listen(),
        //                                      gconfig().gossip_listen(),
        //                                      member,
        //                                      Trace::default(),
        //                                      ring_key.map(|k| k.clone()),
        //                                      None));
        //
        //        let body = match file_path {
        //            Some(p) => {
        //                let file = try!(File::open(&file_path));
        //                let mut body = String::new();
        //                try!(file.read_to_string(&mut body));
        //                body
        //            }
        //            None => {
        //                let mut body = String::new();
        //                try!(io::stdin().read_to_string(&mut body));
        //                body
        //            }
        //        };
        //
        //        let mut service_config =
        //            ServiceConfig::new(member.get_id(), sg.clone(), body.as_bytes().to_vec());
        //        service_config.set_incarnation(number);
        //
        //        server.insert_service_config(service_config);
        //
        //        if let Some(ring_key) = ring_key {
        //            try!(ui.status(Status::Encrypting,
        //                           format!("communication to \"{}\" ring with {}",
        //                                   &ring_key.name,
        //                                   &ring_key.name_with_rev())));
        //        }
        //        try!(ui.status(Status::Applying,
        //                       format!("configuration for {} into ring via {:?}", &sg, &peers)));
        //
        //        // try!(gossip::send_rumors_to_peers(&peers, ring_key, &list));
        //        try!(ui.end("Applied configuration"));
        Ok(())
    }
}
