use crate::event::{Error,
                   EventStreamConfig,
                   Result};
use futures::{channel::{mpsc as futures_mpsc,
                        mpsc::UnboundedSender},
              stream::StreamExt};
use habitat_http_client;
use nats::{native_tls::TlsConnector,
           Client};
use std::{thread,
          time::{Duration,
                 Instant}};

const NATS_SCHEME: &str = "nats://";

/// The subject and payload of an event message.
#[derive(Debug)]
pub struct NatsMessage {
    subject: &'static str,
    payload: Vec<u8>,
}

impl NatsMessage {
    pub fn new(subject: &'static str, payload: Vec<u8>) -> Self { NatsMessage { subject, payload } }

    pub fn payload(&self) -> &[u8] { self.payload.as_slice() }
}

/// A lightweight handle for the event stream. All events get to the
/// event stream through this.
pub struct NatsMessageStream(pub(super) UnboundedSender<NatsMessage>);

impl NatsMessageStream {
    pub async fn new(supervisor_id: &str, config: EventStreamConfig) -> Result<NatsMessageStream> {
        let (event_tx, mut event_rx) = futures_mpsc::unbounded::<NatsMessage>();

        let name = format!("hab_client_{}", supervisor_id);
        let EventStreamConfig { url,
                                token,
                                connect_method,
                                server_certificate,
                                .. } = config;
        let uri = nats_uri(&url, &token.to_string());

        let mut tls_config = TlsConnector::builder();
        for certificate in habitat_http_client::certificates(None)? {
            tls_config.add_root_certificate(certificate);
        }
        if let Some(certificate) = server_certificate {
            tls_config.add_root_certificate(certificate.into());
        }
        let tls_config = tls_config.build()?;

        // Note: With the way we are using the client, we will not respond to pings from the
        // server (https://nats-io.github.io/docs/nats_protocol/nats-protocol.html#pingpong).
        // Instead, we rely on publishing events to keep us connected to the server. This is
        // completely valid according to the protocol, and in fact, the server will
        // not send pings if there is other traffic from a client. If we have no
        // events for an extended period of time, we will be automatically
        // disconnected (because we do not respond to pings). When the next event comes
        // in we will try to reconnect.
        let mut client = Client::new(uri.as_ref())?;
        client.set_name(&name);
        client.set_synchronous(true);
        client.set_tls_config(tls_config);

        // Try to establish an intial connection to the NATS server.
        let start = Instant::now();
        let maybe_timeout = connect_method.into();
        while let Err(e) = client.connect() {
            if let Some(timeout) = maybe_timeout {
                if Instant::now() > start + timeout {
                    return Err(Error::ConnectEventServer);
                }
                error!("Failed to connect to NATS server '{}'. Retrying...", e);
            } else {
                warn!("Failed to connect to NATS server '{}'.", e);
                break;
            }
            thread::sleep(Duration::from_secs(1));
        }

        let event_handler = async move {
            while let Some(packet) = event_rx.next().await {
                if let Err(e) = client.publish(packet.subject, packet.payload()) {
                    error!("Failed to publish event to '{}', '{}'", packet.subject, e);
                }
            }
        };
        tokio::spawn(event_handler);

        Ok(NatsMessageStream(event_tx))
    }

    /// Queues an event to be sent out.
    pub fn send(&self, event_packet: NatsMessage) {
        trace!("About to queue an event: {:?}", event_packet);
        if let Err(e) = self.0.unbounded_send(event_packet) {
            error!("Failed to queue event: {}", e);
        }
    }
}

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
