use crate::event::{Error,
                   EventStreamConfig,
                   Result};
use futures::{channel::{mpsc as futures_mpsc,
                        mpsc::UnboundedSender},
              stream::StreamExt};
use nats::{self,
           asynk::{Connection,
                   Options}};
use std::{path::PathBuf,
          sync::Arc};
use tokio::{sync::Mutex,
            time};
use std::time::Duration;
use std::thread;

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
struct NatsClient(Arc<Mutex<Option<Connection>>>);

impl NatsClient {
    fn new() -> NatsClient {
        NatsClient(Arc::new(Mutex::new(None)))
    }

    // Connect to the server. If a timeout was set, we want to ensure we establish a connection
    // before exiting the function. If we do not connect within the timeout we return an error.
    // If we do not have a timeout, we don't care if we can immediately connect. Instead we spawn
    // a future that will resolve when a connection is possible. Once we establish a
    // connection, this client will handle reconnecting if necessary.
    async fn connect(self, supervisor_id: String, config: EventStreamConfig) -> Result<()> {        
        match config.connect_method.into() {
            Some(timeout) => {
                time::timeout(timeout, 
                              self.connect_impl(supervisor_id, &config)).await??;
            } 
            None => {
                tokio::spawn(async move { 
                    self.connect_impl(supervisor_id, &config)
                        .await
                });
            }
        }
        Ok(())
    }

    async fn connect_impl(self, supervisor_id: String, config: &EventStreamConfig) -> Result<()> {
        while self.0.lock().await.is_none() {
            match Self::options_from_config(&supervisor_id, config)?
                .connect(&config.url.to_string())
                .await {
                    Ok(conn) => *self.0.lock().await = Some(conn),
                    Err(e) => {
                        trace!("Failed to connect to NATS server: {}", e);
                        thread::sleep(Duration::from_millis(1000));
                    }
                }
        }
        Ok(())
    }

    fn options_from_config(supervisor_id: &str, config: &EventStreamConfig) -> Result<Options> {
        let name = format!("hab_client_{}", supervisor_id);
        let ca_certs = habitat_core::tls::native_tls_wrapper::installed_cacerts(None)?;
        let mut options = Options::with_token(&config.token.to_string())
            .with_name(&name)
            .add_root_certificate(ca_certs.expect("No core/cacerts installed"))
            .max_reconnects(None);
        
        if let Some(ref cert_path) = config.server_certificate {
            let cert_path: PathBuf = cert_path.clone().into(); 
            options = options.add_root_certificate(cert_path);
        }
        Ok(options)
    }

    async fn publish(&self, subject: &str, msg: impl AsRef<[u8]>) -> Result<()> {
        if let Some(conn) = &*self.0.lock().await {
            conn.publish(subject, msg)
                .await
                .map_err(|_| Error::NotConnected)
        }
        else {
            Err(Error::NotConnected)
        }
    }
}

/// A lightweight handle for the NATS message stream. All events are converted into a NatsMessage
/// and sent into this stream to be published.
///
/// An UnboundedSender should be ok here. Messages are continously processed even if the client is
/// not currently connected.
pub struct NatsMessageStream(pub(super) UnboundedSender<NatsMessage>);

impl NatsMessageStream {
    pub async fn new(supervisor_id: &str, config: EventStreamConfig) -> Result<NatsMessageStream> {

        let client = NatsClient::new();
        client.clone()
            .connect(supervisor_id.to_string(), config)
            .await?;

        let (tx, mut rx) = futures_mpsc::unbounded::<NatsMessage>();

        // Spawn a task to handle publishing received messages
        tokio::spawn(async move {
            while let Some(packet) = rx.next().await {
                if let Err(e) = client.publish(packet.subject, packet.payload()).await {
                    error!("Failed to publish message to subject '{}', err: {}", packet.subject, e);
                }
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

