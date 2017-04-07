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

// NOTE: The sole purpose of this client is testing and debugging. It's not
// required for normal operation.

extern crate habitat_eventsrv;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate zmq;
extern crate protobuf;
extern crate time;
extern crate rand;

mod message;

use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;
use rand::{thread_rng, Rng};
use zmq::{Context, PUSH};
use protobuf::Message;

use message::event::{EventEnvelope, EventEnvelope_Type};

fn main() {

    let mut args: Vec<_> = env::args().collect();
    args.remove(0); // drop the binary name

    let file_arg = args.remove(0);

    let ctx = Context::new();
    let socket = ctx.socket(PUSH).unwrap();

    // Everything else is treated as a port number
    for p in args {
        let push_connect = format!("tcp://localhost:{}", p);
        println!("connecting to {}", push_connect);
        assert!(socket.connect(&push_connect).is_ok());
    }

    let path = Path::new(&file_arg);
    let display = path.display();
    let mut file = match File::open(&path) {
        Err(why) => panic!("Couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    let mut payload = String::new();
    match file.read_to_string(&mut payload) {
        Err(why) => panic!("Couldn't read {}: {}", display, why.description()),
        Ok(_) => debug!("{} contains:\n{}\n\n", display, payload),
    }

    // Create some fake-yet-plausible data for our messages; we'll
    // randomly select from these when creating our messages.
    //
    // The service "" is an error scenario; there should always be a
    // service.
    let members = [1, 2, 3, 4, 5];
    let services = ["frontend", "backend", "database", "ponies", ""];
    let mut rng = thread_rng();

    loop {
        let timestamp = current_time();
        let mut event = EventEnvelope::new();

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
        event.set_payload(payload.as_bytes().to_vec());
        event.set_timestamp(timestamp);

        let member_id = rng.choose(&members).unwrap();
        event.set_member_id(*member_id);

        let service_name = rng.choose(&services).unwrap();
        event.set_service(String::from(*service_name));

        println!("PUBLISHER: Timestamp {}", timestamp);
        println!("PUBLISHER: Member ID {}", member_id);
        println!("PUBLISHER: Service {}\n", service_name);

        socket
            .send(event.write_to_bytes().unwrap().as_slice(), 0)
            .unwrap();
        let sleep_time = Duration::from_secs(3);
        sleep(sleep_time);
    }
}

fn current_time() -> u64 {
    let timespec = time::get_time();
    let sec: u64 = timespec.sec as u64 * 1000;
    let nsec: u64 = timespec.nsec as u64 / 1000 / 1000;
    sec + nsec
}
