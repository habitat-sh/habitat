use crate::{event::{Error,
                    EventStream,
                    EventStreamConnectionInfo,
                    Result},
            sup_futures};
use futures::{future::Future,
              sync::mpsc as futures_mpsc};
use nats::Client;
use std::{thread,
          time::{Duration,
                 Instant}};
use tokio::{prelude::Stream,
            runtime::Runtime};

/// All messages are published under this subject.
const HABITAT_SUBJECT: &str = "habitat.event.healthcheck";
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

pub(super) fn init_stream(conn_info: EventStreamConnectionInfo,
                          runtime: &mut Runtime)
                          -> Result<EventStream> {
    let (event_tx, event_rx) = futures_mpsc::channel(EVENT_CHANNEL_SIZE);

    let EventStreamConnectionInfo { name,
                                    verbose,
                                    cluster_uri,
                                    auth_token,
                                    connect_method, } = conn_info;
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

    // Try to establish an intial connection to the NATS server.
    let start = Instant::now();
    let maybe_timeout = connect_method.into();
    while let Err(e) = client.connect() {
        if let Some(timeout) = maybe_timeout {
            error!("Failed to connect to NATS server '{}'. Retrying...", e);
            if Instant::now() > start + timeout {
                return Err(Error::ConnectEventServerError);
            }
        } else {
            warn!("Failed to connect to NATS server '{}'.", e);
            break;
        }
        thread::sleep(Duration::from_secs(1));
    }

    let (handle, event_handler) =
        sup_futures::cancelable_future(event_rx.for_each(move |event: Vec<u8>| {
                                                   if let Err(e) =
                                                       client.publish(HABITAT_SUBJECT, &event)
                                                   {
                                                       error!("Failed to publish event, '{}'", e);
                                                   }
                                                   Ok(())
                                               }));
    // Convert the error type so `spawn` can handle it.
    let event_handler = event_handler.map_err(|_| ());
    runtime.spawn(event_handler);

    Ok(EventStream::new(event_tx, handle))
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
