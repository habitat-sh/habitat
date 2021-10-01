use crate::event::{EventStreamConfig,
                   Result};
use futures::{channel::{mpsc as futures_mpsc,
                        mpsc::UnboundedSender},
              stream::StreamExt};
use nats::{self,
           asynk::{Connection,
                   Options}};
use std::{io,
          path::PathBuf,
          sync::Arc};
use tokio::{sync::Mutex,
            time};

/// The subject and payload of a NATS message.
#[derive(Debug)]
pub struct NatsMessage {
    subject: &'static str,
    payload: Vec<u8>,
}

impl NatsMessage {
    pub fn new(subject: &'static str, payload: Vec<u8>) -> Self { NatsMessage { subject, payload } }

    pub fn payload(&self) -> &[u8] { self.payload.as_slice() }
}

/// NatsClient is main accessor to connect to NATS Server and
/// to publish messages to NATS.
#[derive(Clone)]
struct NatsClient {
    connection: Arc<Mutex<Option<Connection>>>,
}

impl NatsClient {
    fn new() -> NatsClient {
        NatsClient { connection: Arc::new(Mutex::new(None)), }
    }

    async fn connect(self, supervisor_id: String, config: EventStreamConfig) -> io::Result<()> {        
        match config.connect_method.into() {
            Some(timeout) => {
                trace!("Timeout used -> {:?}", timeout);
                time::timeout(timeout, 
                              self.connect_impl(supervisor_id, &config)).await??;
            } 
            None => {
                trace!("Timeout not used");
                tokio::spawn(async move { 
                    self.connect_impl(supervisor_id, &config)
                        .await
                });
            }
        }
        Ok(())
    }

    async fn connect_impl(self, supervisor_id: String, config: &EventStreamConfig) -> io::Result<()> {
        if self.connection.lock().await.is_none() {
            match Self::options_from_config(&supervisor_id, config) 
                .connect(&config.url.to_string())
                .await {
                    Ok(conn) => {
                        *self.connection.lock().await = Some(conn);
                    }
                    Err(e) => {
                        return Err(e);
                    }
                }
        }
        Ok(())
    }

    fn options_from_config(supervisor_id: &str, config: &EventStreamConfig) -> Options {
        let name = format!("hab_client_{}", supervisor_id);
        let ca_certs = match habitat_core::tls::native_tls_wrapper::installed_cacerts(None) {
            Ok(ca_certs) => ca_certs,
            Err(_)         => None
        };
        match config.server_certificate {
            Some(ref nats_options) => {
                let cert_path: PathBuf = nats_options.clone().into(); 
                Options::with_token(&config.token.to_string())
                    .with_name(&name)
                    .add_root_certificate(cert_path)
                    .add_root_certificate(ca_certs.unwrap())
                    .with_retry_on_failed_connect()
            }
            None => {
                Options::with_token(&config.token.to_string()) 
                    .with_name(&name)
                    .add_root_certificate(ca_certs.unwrap())
                    .with_retry_on_failed_connect()
            }
        }
    }

    async fn publish(&self, subject: &str, msg: impl AsRef<[u8]>) -> io::Result<()> {
        if let Some(conn) = &*self.connection.lock().await {
            conn.publish(subject, msg).await
        }
        else {
            Err(io::Error::new(io::ErrorKind::Other, "Not connected to NATS server!"))
        }
    }
}

/// A lightweight handle for the NATS message stream. All events are converted into a NatsMessage
/// and sentinto this stream to be published.
///
/// An UnboundedSender should be ok here. Messages are continously processed even if the client is
/// not currently connected.
pub struct NatsMessageStream(pub(super) UnboundedSender<NatsMessage>);

impl NatsMessageStream {
    pub async fn new(supervisor_id: &str, config: EventStreamConfig) -> Result<NatsMessageStream> {
        // Connect to the server. If a timeout was set, we want to ensure we establish a connection
        // before exiting the function. If we do not connect within the timeout we return an error.
        // If we do not have a timeout, we dont care if we can immediately connect. Instead we spawn
        // a future that will resolve when a connection is possible. Once we establish a
        // connection, the client will handle reconnecting if necessary.
        //let mut client = NatsClient::new();
        let client = NatsClient::new();
        client.clone()
            .connect(supervisor_id.to_string(), config)
            .await?;

        let (tx, mut rx) = futures_mpsc::unbounded::<NatsMessage>();

        // Spawn a task to handle publishing received messages
        tokio::spawn(async move {
            while let Some(packet) = rx.next().await {
                let res = client.publish(packet.subject, packet.payload()).await;
                trace!("publish result: {:?}", res);
            }
        });

        Ok(NatsMessageStream(tx))
    }

    /// Queues a NATS message to be published
    pub fn send(&self, event_packet: NatsMessage) {
        trace!("Queueing message: {:?}", event_packet.subject);
        if let Err(e) = self.0.unbounded_send(event_packet) {
            error!("Failed to queue message, err: {}", e);
        }
    }
}

