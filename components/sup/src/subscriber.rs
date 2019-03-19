// Copyright (c) 2019 Chef Software Inc. and/or applicable contributors
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

extern crate futures;
extern crate nitox;
extern crate tokio;

use futures::{future::{ok,
                       Future},
              Stream};
use nitox::{commands::ConnectCommand,
            streaming::client::{NatsStreamingClient,
                                SubscribeOptionsBuilder},
            NatsClient,
            NatsClientOptions};

fn main() {
    println!("Welcome to the testing subscriber for the Automate / Habitat Event Streaming \
              Prototype");
    println!("Press '^C' to end");
    let connect_cmd = ConnectCommand::builder().build().unwrap();
    let options = NatsClientOptions::builder().connect_command(connect_cmd)
                                              .cluster_uri("127.0.0.1:4223")
                                              .build()
                                              .unwrap();

    let listener =
        NatsClient::from_options(options).map_err(Into::into)
                                         .and_then(|client| {
                                             NatsStreamingClient::from(client)
                .cluster_id("test-cluster".to_string())
                .connect()
                                         })
                                         .and_then(|client| {
                                             let opts = SubscribeOptionsBuilder::default().build()
                                                                                          .unwrap();
                                             let topic = "habitat".to_string();
                                             println!("Subscribed to topic: '{}'", topic);
                                             println!("===================");
                                             client.subscribe(topic, opts)
                                                   .and_then(move |message_stream| {
                                                       message_stream.for_each(|msg| {
                        println!("Message: {:#?}", String::from_utf8_lossy(&msg.proto.data));
                        ok(())
                    })
                                                   })
                                         })
                                         .map_err(|e| println!("ERROR: {:?}", e));

    tokio::runtime::Runtime::new().unwrap()
                                  .block_on(listener)
                                  .unwrap();
}
