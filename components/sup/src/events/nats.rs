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

//! Implements a NATS Streaming-based event stream.

use super::EventStream;
use crate::error::Result;
use futures::{sync::mpsc::unbounded,
              Future,
              Stream};
use nitox::{commands::ConnectCommand,
            streaming::{client::NatsStreamingClient,
                        error::NatsStreamingError},
            NatsClient,
            NatsClientOptions};
use std::{sync::mpsc::sync_channel,
          thread};
use tokio::{executor,
            runtime::current_thread::Runtime as ThreadRuntime};

/// All messages are published under this subject.
static HABITAT_SUBJECT: &'static str = "habitat";

/// All the information needed to establish a connection to a NATS
/// Streaming server.
// TODO: This will change as we firm up what the interaction between
// Habitat and A2 looks like.
pub struct EventConnectionInfo {
    pub name:        String,
    pub verbose:     bool,
    pub cluster_uri: String,
    pub cluster_id:  String,
}

/// Defines default connection information for a NATS Streaming server
/// running on localhost.
// TODO: As we become clear on the interaction between Habitat and A2,
// this implementation *may* disappear. It's useful for testing and
// prototyping, though.
impl Default for EventConnectionInfo {
    fn default() -> Self {
        EventConnectionInfo { name:        String::from("habitat"),
                              verbose:     true,
                              cluster_uri: String::from("127.0.0.1:4223"),
                              cluster_id:  String::from("test-cluster"), }
    }
}

pub(super) fn init_stream(conn_info: EventConnectionInfo) -> Result<EventStream> {
    // TODO (CM): Investigate back-pressure scenarios
    let (event_tx, event_rx) = unbounded();
    let (sync_tx, sync_rx) = sync_channel(0); // rendezvous channel

    // TODO (CM): We could theoretically create this future and spawn
    // it in the Supervisor's Tokio runtime, but there's currently a
    // bug: https://github.com/YellowInnovation/nitox/issues/24

    thread::Builder::new().name("events".to_string())
                          .spawn(move || {
                              let EventConnectionInfo { name,
                                                        verbose,
                                                        cluster_uri,
                                                        cluster_id, } = conn_info;

                              let cc = ConnectCommand::builder()
                // .user(Some("nats".to_string()))
                // .pass(Some("S3Cr3TP@5w0rD".to_string()))
                .name(Some(name))
                .verbose(verbose)
                .build()
                .unwrap();
                              let opts =
                                  NatsClientOptions::builder().connect_command(cc)
                                                              .cluster_uri(cluster_uri.as_str())
                                                              .build()
                                                              .unwrap();

                              let publisher = NatsClient::from_options(opts)
                .map_err(Into::<NatsStreamingError>::into)
                .and_then(|client| {
                    NatsStreamingClient::from(client)
                        .cluster_id(cluster_id)
                        .connect()
                })
                .map_err(|streaming_error| error!("{}", streaming_error))
                .and_then(move |client| {
                    sync_tx.send(()).expect("Couldn't synchronize!");
                    event_rx.for_each(move |event: Vec<u8>| {
                        let publish_event = client
                            .publish(HABITAT_SUBJECT.into(), event.into())
                            .map_err(|e| {
                                error!("Error publishing event: {:?}", e);
                            });
                        executor::spawn(publish_event);
                        Ok(())
                    })
                });

                              ThreadRuntime::new().expect("Couldn't create event stream runtime!")
                                                  .spawn(publisher)
                                                  .run()
                                                  .expect("something seriously wrong has occurred");
                          })
                          .expect("Couldn't start events thread!");

    sync_rx.recv()?; // TODO (CM): nicer error message
    Ok(EventStream(event_tx))
}
