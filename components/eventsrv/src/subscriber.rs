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

// NOTE: The sole purpose of this subscriber is testing and debugging. It's not
// required for normal operation.

use env_logger;

use habitat_eventsrv_protocol as protocol;
#[macro_use]
extern crate log;

use std::env;
use std::io::Read;

use crate::protocol::{EventEnvelope, EventEnvelope_Type, ServiceUpdate as ServiceUpdateProto};
use byteorder::{ByteOrder, LittleEndian};
use protobuf::parse_from_bytes;
use zmq::{Context, SUB};

fn main() {
    env_logger::init();
    let ctx = Context::new();
    let socket = ctx.socket(SUB).unwrap();

    let mut args: Vec<_> = env::args().collect();
    args.remove(0); // drop the binary name

    for p in args {
        let sub_connect = format!("tcp://localhost:{}", p);
        debug!("EventSrvSubscriber connecting to {}", sub_connect);
        assert!(socket.connect(&sub_connect).is_ok());
    }
    assert!(socket.set_subscribe(b"").is_ok()); // Subscribe to everything

    loop {
        match socket.recv_bytes(0) {
            Ok(bytes) => {
                let event = parse_from_bytes::<EventEnvelope>(&bytes).unwrap();
                let mut bytes_read = 0;
                let mut payload_buf: Vec<u8> = vec![];
                let mut size_buf = [0; 8];
                let current_payload = event.get_payload().to_vec();
                let mut payload_slice: &[u8] = &current_payload[..];
                let current_num_bytes = current_payload.len() as u64;

                let member_id = event.get_member_id();
                let timestamp = event.get_timestamp();
                let service = event.get_service();

                println!("SUBSCRIBER: Timestamp {}", timestamp);
                println!("SUBSCRIBER: Member ID {}", member_id);
                println!("SUBSCRIBER: Service {}", service);

                loop {
                    if bytes_read >= current_num_bytes {
                        break;
                    }

                    payload_slice.read_exact(&mut size_buf).unwrap();
                    let payload_size = LittleEndian::read_u64(&size_buf);
                    payload_buf.resize(payload_size as usize, 0);
                    payload_slice.read_exact(&mut payload_buf).unwrap();

                    match event.get_field_type() {
                        EventEnvelope_Type::ProtoBuf => {
                            let data =
                                parse_from_bytes::<ServiceUpdateProto>(&payload_buf).unwrap();
                            println!(
                                "SUBSCRIBER: Service Update Member ID {}",
                                data.get_member_id()
                            );
                            let cfg = data.get_cfg().to_vec();
                            let cfg_str = String::from_utf8(cfg).unwrap();
                            println!("SUBSCRIBER: Census Entry Config {}", cfg_str);
                        }
                        EventEnvelope_Type::JSON | EventEnvelope_Type::TOML => {
                            let data = String::from_utf8(payload_buf.clone()).unwrap();
                            println!("Data: {}", data);
                        }
                    };

                    bytes_read += size_buf.len() as u64 + payload_size;
                }
            }
            Err(e) => panic!("zeromq socket error: {:?}", e),
        }
    }
}
