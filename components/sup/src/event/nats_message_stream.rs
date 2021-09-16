use crate::event::{EventStreamConfig,
                   Result};
use futures::{channel::{mpsc as futures_mpsc,
                        mpsc::UnboundedSender},
              stream::StreamExt};
use nats::{self,
           asynk::{Options, Connection}};
use tokio::{time, sync::Mutex};
use std::{
    io,
    path::PathBuf,
    sync::Arc,
    time::Duration};

/// The subject and payload of a NATS message.
#[derive(Debug)]
pub struct NatsMessage {
    subject: &'static str,
    payload: Vec<u8>,
}

impl NatsMessage {
    pub fn new(subject: &'static str, payload: Vec<u8>) -> Self {
        NatsMessage { subject, payload }
    }

    pub fn payload(&self) -> &[u8] { self.payload.as_slice() }
}

#[derive(Clone)]
pub struct NatsClient(Option<Connection>);

impl NatsClient {
    pub async fn connect(this: Arc<Mutex<Self>>, supervisor_id: String, config: EventStreamConfig) {        
        match config.connect_method.into() {
            Some(timeout) => {
                trace!("Timeout used -> {:?}", timeout);
                if let Err(e) = time::timeout(timeout, this.lock().await.connect_impl_with_reconnect(supervisor_id, &config))
                    .await {
                    trace!("Error connecting: {}", e);
                    //  The process does not exit.  We should shut down if timeout specified and we cannot connect.
                    //std::process::exit(1);
                }
            } 
            None => {
                trace!("Timeout not used");
                tokio::spawn(async move { this.lock().await.connect_impl(supervisor_id, &config).await });
            }
        };
    }

    async fn connect_impl_with_reconnect(&mut self, supervisor_id: String, config: &EventStreamConfig) {        
        while self.0.is_none() {
            match Self::options_from_config(&supervisor_id, config)
                .with_retry_first_connect()
                .reconnect_delay_callback(move |t|
                {
                    Duration::from_millis(std::cmp::min((t*100) as u64, 8000))
                })
                .connect(&config.url.to_string()).await {

                Ok(conn) => self.0 = Some(conn),
                Err(e) => {
                    error!("Failed to connect to NATS server: {}", e);
                }
            }
       }
    }

    async fn connect_impl(&mut self, supervisor_id: String, config: &EventStreamConfig) {        
        while self.0.is_none() {
            match Self::options_from_config(&supervisor_id, config)
                .with_retry_first_connect()
                .connect(&config.url.to_string()).await {
                Ok(conn) => self.0 = Some(conn),
                Err(e) => {
                    error!("Failed to connect to NATS server: {}", e);
                }
            }
       }
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
            }
            None => {
                Options::with_token(&config.token.to_string()) 
                    .with_name(&name)
                    .add_root_certificate(ca_certs.unwrap())
            }
        }
    }

    pub async fn publish(&self, subject: &str, msg: impl AsRef<[u8]>) -> io::Result<()> {
        if let Some(conn) = &self.0 {
            conn.publish(subject, msg).await
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "Not connected to NATS server!"))
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
    pub async fn new(supervisor_id: String, config: EventStreamConfig) -> Result<NatsMessageStream> {
        // Connect to the server. If a timeout was set, we want to ensure we establish a connection
        // before exiting the function. If we do not connect within the timeout we return an error.
        // If we do not have a timeout, we dont care if we can immediately connect. Instead we spawn
        // a future that will resolve when a connection is possible. Once we establish a
        // connection, the client will handle reconnecting if necessary.
        let client = Arc::new(Mutex::new(NatsClient(None)));
        NatsClient::connect(client.clone(), supervisor_id, config).await;

        let (tx, mut rx) = futures_mpsc::unbounded::<NatsMessage>();

        // Spawn a task to handle publishing received messages
        tokio::spawn(async move {
            while let Some(packet) = rx.next().await {
                let res = client.lock().await.publish(packet.subject, packet.payload()).await;
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

