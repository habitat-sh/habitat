use crate::event::{EventStreamConfig,
                   Result};
use futures::{channel::{mpsc as futures_mpsc,
                        mpsc::UnboundedSender},
              stream::StreamExt};
use habitat_http_client;
use rants::{error::Error as RantsError,
            native_tls::TlsConnector,
            Client,
            Subject};
use tokio::time;

/// The subject and payload of a NATS message.
#[derive(Debug)]
pub struct NatsMessage {
    subject: &'static Subject,
    payload: Vec<u8>,
}

impl NatsMessage {
    pub fn new(subject: &'static Subject, payload: Vec<u8>) -> Self {
        NatsMessage { subject, payload }
    }

    pub fn payload(&self) -> &[u8] { self.payload.as_slice() }
}

/// A lightweight handle for the NATS message stream. All events are converted into a NatsMessage
/// and sent into this stream to be published.
///
/// An UnboundedSender should be ok here. Messages are continously processed even if the client is
/// not currently connected.
pub struct NatsMessageStream(pub(super) UnboundedSender<NatsMessage>);

impl NatsMessageStream {
    pub async fn new(supervisor_id: &str, config: EventStreamConfig) -> Result<NatsMessageStream> {
        let EventStreamConfig { url,
                                token,
                                connect_method,
                                server_certificate,
                                .. } = config;

        // TODO: validate in cli arg parsing
        let address = url.parse()?;
        let mut client = Client::new(vec![address]);

        // Configure the client connect message
        client.connect_mut()
              .await
              .name(format!("hab_client_{}", supervisor_id))
              .verbose(true)
              .token(token.to_string());

        // Configure the tls connector
        let mut tls_connector = TlsConnector::builder();
        for certificate in habitat_http_client::certificates(None)? {
            tls_connector.add_root_certificate(certificate);
        }
        if let Some(certificate) = server_certificate {
            tls_connector.add_root_certificate(certificate.into());
        }
        let tls_connector = tls_connector.build()?;
        client.set_tls_connector(tls_connector).await;

        // Connect to the server. If a timeout was set, we want to ensure we establish a connection
        // before exiting the function. If we do not connect within the timeout we return an error.
        // If we do not have a timeout, we dont care if we can immediately connect. Instead we spawn
        // a future that will resolve when a connection is possible. Once we establish a
        // connection, the client will handle reconnecting if necessary.
        if let Some(timeout) = connect_method.into() {
            time::timeout(timeout, client.connect()).await?;
        } else {
            let client = Client::clone(&client);
            tokio::spawn(async move { client.connect().await });
        }

        let (tx, mut rx) = futures_mpsc::unbounded::<NatsMessage>();

        // Spawn a task to handle publishing received messages
        tokio::spawn(async move {
            while let Some(packet) = rx.next().await {
                if let Err(e) = client.publish(packet.subject, packet.payload()).await {
                    // We do not retry any messages. If we are not connected when the message is
                    // processed or there is an error in publishing the message, the message will
                    // never be sent.
                    if let RantsError::NotConnected = e {
                        trace!("Failed to publish message to subject '{}' because the client is \
                                not connected",
                               packet.subject);
                    } else {
                        error!("Failed to publish message to subject '{}', err: {}",
                               packet.subject, e);
                    }
                }
            }
        });

        Ok(NatsMessageStream(tx))
    }

    /// Queues a NATS message to be published
    pub fn send(&self, event_packet: NatsMessage) {
        trace!("Queueing message: {:?}", event_packet);
        if let Err(e) = self.0.unbounded_send(event_packet) {
            error!("Failed to queue message, err: {}", e);
        }
    }
}
