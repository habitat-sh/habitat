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
const NATS_SCHEME: &str = "nats://";
const EVENT_CHANNEL_SIZE: usize = 1024;

fn nats_uri(uri: &str, auth_token: &str) -> String {
    // Unconditionally, remove the scheme. We will add it back.
    let uri = String::from(uri).replace(NATS_SCHEME, "");
    // If the uri contains credentials or the auth token is empty use the uri as is. Otherwise, add
    // the auth token.
    if uri.contains('@') || auth_token.is_empty() {
        format!("{}{}", NATS_SCHEME, uri)
    } else {
        format!("{}{}@{}", NATS_SCHEME, auth_token, uri)
    }
}

pub(super) fn init_stream(conn_info: EventStreamConnectionInfo) -> Result<EventStream> {
    let (event_tx, event_rx) = futures_mpsc::channel(EVENT_CHANNEL_SIZE);
    let (sync_tx, sync_rx) = std_mpsc::sync_channel(0); // rendezvous channel

    let EventStreamConnectionInfo { name,
                                    verbose,
                                    cluster_uri,
                                    auth_token,
                                    connect_method, } = conn_info;
    let connection_is_timeout = connect_method.is_timeout();
    let uri = nats_uri(&cluster_uri, &auth_token.to_string());

    // Note: With the way we are using the client, we will not respond to pings from the server
    // (https://nats-io.github.io/docs/nats_protocol/nats-protocol.html#pingpong).
    // Instead, we rely on publishing events to keep us connected to the server. This is completely
    // valid according to the protocol, and in fact, the server will not send pings if there is
    // other traffic from a client. If we have no events for an extended period of time, we will
    // be automatically disconnected (because we do not respond to pings). When the next event comes
    // in we will try to reconnect.
    let mut client = Client::new(uri.as_ref())?;
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

#[cfg(test)]
mod tests {
    use super::nats_uri;

    #[test]
    fn test_nats_uri() {
        assert_eq!(&nats_uri("nats://127.0.0.1:4222", ""),
                   "nats://127.0.0.1:4222");
        assert_eq!(&nats_uri("127.0.0.1:4222", ""), "nats://127.0.0.1:4222");
        assert_eq!(&nats_uri("username:password@127.0.0.1:4222", "some_token"),
                   "nats://username:password@127.0.0.1:4222");
        assert_eq!(&nats_uri("127.0.0.1:4222", "some_token"),
                   "nats://some_token@127.0.0.1:4222");
        assert_eq!(&nats_uri("nats://127.0.0.1:4222", "some_token"),
                   "nats://some_token@127.0.0.1:4222");
        assert_eq!(&nats_uri("nats://existing_token@127.0.0.1:4222", "not_used_token"),
                   "nats://existing_token@127.0.0.1:4222");
        assert_eq!(&nats_uri("existing_token@127.0.0.1:4222", "not_used_token"),
                   "nats://existing_token@127.0.0.1:4222");
    }
}
