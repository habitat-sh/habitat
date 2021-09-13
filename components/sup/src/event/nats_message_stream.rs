use crate::event::{error,
                   Error,
                   EventStreamConfig,
                   Result};
use futures::{channel::{mpsc as futures_mpsc,
                        mpsc::UnboundedSender},
              stream::StreamExt};
use rants::Subject;
use nats;
use nats::asynk::{Options, Connection};
use tokio::{time, sync::Mutex};
use std::{
    io,
    path::PathBuf,
    sync::Arc,
    thread,
    time::Duration};
use habitat_core;

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

#[derive(Clone)]
pub struct NatsClient(Option<Connection>);

impl NatsClient {
    pub async fn connect(this: Arc<Mutex<Self>>, supervisor_id: String, config: EventStreamConfig) {        
        match config.connect_method.into() {
            Some(timeout) => {
                println!("Timeout used -> {:?}", timeout);
                time::timeout(timeout, this.lock().await.connect_impl(supervisor_id, &config))
                    .await
                    .map_err(|_| Error::ConnectNatsServer).unwrap()
            } 
            None => {
                println!("Timeout not used");
                tokio::spawn(async move { this.lock().await.connect_impl(supervisor_id, &config).await });
            }
        };
    }

    async fn connect_impl(&mut self, supervisor_id: String, config: &EventStreamConfig) {        
        while self.0.is_none() {
            match Self::options_from_config(&supervisor_id, config).connect(&config.url.to_string()).await {
                Ok(conn) => self.0 = Some(conn),
                Err(e) => {
                    error!("Failed to connect to NATS server: {}", e);
                    thread::sleep(Duration::from_millis(1000));
                }
            }
       }
    }

    fn options_from_config(supervisor_id: &str, config: &EventStreamConfig) -> Options {
        let name = format!("hab_client_{}", supervisor_id);
        match &config.server_certificate {
            Some(nats_options) => {
                let cert_path: PathBuf = nats_options.into(); 
                Options::with_token(&config.token.to_string())
                    .with_name(&name)
                    .add_root_certificate(cert_path)
            }
            None => {
                Options::with_token(&config.token.to_string()) 
                    .with_name(&name)
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
        // let client = NatsClient(None);
        // NatsClient::connect(Arc::new(Mutex::new(client)).clone(), supervisor_id, config).await;

        let (tx, mut rx) = futures_mpsc::unbounded::<NatsMessage>();

        // Spawn a task to handle publishing received messages
        tokio::spawn(async move {
            while let Some(packet) = rx.next().await {
                let subj = format!("{}", packet.subject);
                let res = client.lock().await.publish(&subj, packet.payload()).await;
                println!("publish result: {:?}", res);
            }
        });

        Ok(NatsMessageStream(tx))
    }

    /// Queues a NATS message to be published
    pub fn send(&self, event_packet: NatsMessage) {
        println!("Queueing message: {:?}", event_packet.subject);
        if let Err(e) = self.0.unbounded_send(event_packet) {
            error!("Failed to queue message, err: {}", e);
        }
    }
}
