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

// NOTE: This file is mostly used for testing purposes and isn't necessary for the main operation
// of EventSrvClient.

extern crate byteorder;
extern crate habitat_eventsrv;
extern crate habitat_eventsrv_client;

use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use byteorder::{ByteOrder, LittleEndian};

use habitat_eventsrv::message::event::{EventEnvelope, EventEnvelope_Type};
use habitat_eventsrv_client::EventSrvClient;

fn main() {
    let mut args: Vec<_> = env::args().collect();
    args.remove(0); // drop the binary name

    // First should be the path to a file
    // Everything else is a port number
    let file_arg = args.remove(0);
    let mut event = EventEnvelope::new();

    let path = Path::new(&file_arg);
    let display = path.display();
    let mut payload = String::new();

    let mut file = match File::open(&path) {
        Err(why) => panic!("Couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    match file.read_to_string(&mut payload) {
        Err(why) => panic!("Couldn't read {}: {}", display, why.description()),
        Ok(_) => (),
    }

    let field_type = match path.extension() {
        None => EventEnvelope_Type::ProtoBuf,
        Some(ext) => {
            match ext.to_str() {
                Some("json") => EventEnvelope_Type::JSON,
                Some("toml") => EventEnvelope_Type::TOML,
                _ => panic!("Unknown file type {:?}", ext),
            }
        }
    };

    event.set_field_type(field_type);

    // JB TODO: this is duplicated with the code in sup/manager/mod.rs
    let mut payload_buf: Vec<u8> = vec![];
    let mut proto_size = vec![0; 8];
    let mut bytes = payload.into_bytes();
    LittleEndian::write_u64(&mut proto_size, bytes.len() as u64);
    payload_buf.append(&mut proto_size);
    payload_buf.append(&mut bytes);

    event.set_payload(payload_buf);
    event.set_service(String::from("eventsrv-client command line"));

    let client = EventSrvClient::new(args);
    client.connect();
    let _ = client.send(event);
}
