use crate::event::{Error,
                   EventStream,
                   EventStreamConnectionInfo,
                   Result};
use futures::sync::mpsc as futures_mpsc;
use nats::Client;
use std::{sync::mpsc as std_mpsc,
          thread,
          time::Duration};
use tokio::{prelude::Stream,
            runtime::current_thread::Runtime};

/// All messages are published under this subject.
const HABITAT_SUBJECT: &str = "habitat";

pub(super) fn init_stream(conn_info: EventStreamConnectionInfo) -> Result<EventStream> {
    // TODO (DM): This cannot be unbounded. We need backpressure. If the connection is down when we
    // try to publish we try to reconnect this can be time consuming so we can easily get
    // behind.
    let (event_tx, event_rx) = futures_mpsc::unbounded();
    let (sync_tx, sync_rx) = std_mpsc::sync_channel(0); // rendezvous channel

    let EventStreamConnectionInfo { name,
                                    verbose,
                                    cluster_uri,
                                    // TODO (DM): The nats client we are using does not support
                                    // auth tokens and will need to be patched
                                    auth_token: _,
                                    connect_method, } = conn_info;
    let connection_is_timeout = connect_method.is_timeout();

    // Note: With the way we are using the client, we will not respond to pings from the server
    // (https://nats-io.github.io/docs/nats_protocol/nats-protocol.html#pingpong).
    // Instead, we rely on publishing events to keep us connected to the server. This is completely
    // valid according to the protocol, and in fact, the server will not send pings if there is
    // other traffic from a client. If we have no events for an extended period of time, we will
    // be automatically disconnected (because we do not respond to pings). When the next event comes
    // in we will try to reconnect.
    let mut client = Client::new(cluster_uri)?;
    client.set_name(&name);
    client.set_synchronous(verbose);
    let closure = move || {
        // Try to establish an intial connection to the NATS server.
        loop {
            if let Err(e) = client.connect() {
                if connection_is_timeout {
                    error!("Failed to connect to NATS server '{}'. Retrying...", e);
                } else {
                    warn!("Failed to connect to NATS server '{}'.", e);
                    break;
                }
            } else {
                if connection_is_timeout {
                    sync_tx.send(())
                           .expect("Couldn't synchronize event thread!");
                }
                break;
            }
            thread::sleep(Duration::from_secs(1))
        }

        let event_handler = event_rx.for_each(move |event: Vec<u8>| {
                                        if let Err(e) = client.publish(HABITAT_SUBJECT, &event) {
                                            error!("Failed to publish event, '{}'", e);
                                        }
                                        Ok(())
                                    });

        Runtime::new().expect("Couldn't create event stream runtime!")
                      .spawn(event_handler)
                      .run()
                      .expect("something seriously wrong has occurred");
    };

    thread::Builder::new().name("events".to_string())
                          .spawn(closure)
                          .map_err(Error::SpawnEventThreadError)?;

    if let Some(timeout) = connect_method.into() {
        sync_rx.recv_timeout(timeout)?;
    }
    Ok(EventStream(event_tx))
}
