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
use tokio::time;
use std::io::Result as IOResult;
use habitat_core;

pub struct NatsClient<'a> {
    options: &'a Options,
    connect_url : String,
    connection : Option<Connection>,
}

impl <'a> NatsClient<'a> {
    pub fn new(mut opts: &'a Options, url: &str) -> NatsClient<'a> {
        NatsClient {options: opts, connect_url: url.to_string(), connection: None}
    }

    pub async fn connect(&mut self) -> Result<Connection> {
        let connection = self.options.connect(&self.connect_url).await;
        match &connection {
            Ok(connection) => {
                println!("nats_connect global function connection OK");
                connection.publish("habitat.event.healthcheck", "nats.rs client is UP").await;
                Ok(Connection::clone(&connection))
            },
            _ => {
                println!("nats_connect global function failed to connect and set connection");
                Err(error::Error::ConnectNatsServer)
            }
        }
    }

    pub fn clone(&self) -> NatsClient {
        if let Some(connection) = &self.connection {
                NatsClient{options: &self.options, connect_url: self.connect_url.to_string(), connection: Some(Connection::clone(&connection))}
        }
        else {
                NatsClient{options: &self.options, connect_url: self.connect_url.to_string(), connection: None}
        }
    }

    pub async fn publish(&mut self, subject: &str, payload: &[u8]) {
        if let Some(connection) =  &self.connection {
            connection.publish(subject, payload);
        }
        else {
            println!("Publish failed");
        }
    }
}

/// The subject and payload of a NATS message.
#[derive(Debug)]
pub struct NatsMessage {
    subject: &'static Subject,
    payload: Vec<u8>,
}

//g_connection: Option<Connection> = None;

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

        println!("NatsMessageStream::new at {}", config.url);

        let token_str = format!("{}", config.token);
        let name = format!("hab_client_{}", supervisor_id);
        let cert_path = format!("{}", config.server_certificate.unwrap());
        let config_url = format!("{}", config.url);

        let nats_options: Options = Options::with_token(&token_str) 
            .add_root_certificate(&cert_path);

        let mut nats_client: NatsClient = NatsClient::new(&nats_options, &config_url);

        let (tx, rx) = futures_mpsc::unbounded::<NatsMessage>();

        // Connect to the server. If a timeout was set, we want to ensure we establish a connection
        // before exiting the function. If we do not connect within the timeout we return an error.
        // If we do not have a timeout, we dont care if we can immediately connect. Instead we spawn
        // a future that will resolve when a connection is possible. Once we establish a
        // connection, the client will handle reconnecting if necessary.

        if let Some(timeout) = config.connect_method.into() {
            println!("About to connect with timeout");
            let res = time::timeout(timeout, nats_client.connect())
                .await
                .map_err(|_| Error::ConnectNatsServer)?;
            println!("result from timeout = {:?}", res);
        }
        else {
            println!("About to connect - no timeout specified");
            let nats_opts: Options = Options::with_token(&token_str) 
                .add_root_certificate(&cert_path);
            tokio::spawn(async move { nats_client.connect().await });
        }

/*
 *      This works on initial connection but gets moved so cannot be used later.
        let connection : IOResult<Connection> = nats_options.connect(&config_url).await;
        match &connection {
            Ok(connection) => {
                connection.publish("habitat.event.healthcheck", "nats.rs client is UP").await;
                println!("Connection = {:?}", connection);
            },
            Err(connection) => {
                println!("Nats client failed to connect");
            }
        }       
*/
        let (tx, mut rx) = futures_mpsc::unbounded::<NatsMessage>();

        // Spawn a task to handle publishing received messages
        tokio::spawn(async move {
            while let Some(packet) = rx.next().await {
                let subj = format!("{}", packet.subject);
                println!("Have packet to send for subject {}", subj);

                let res = nats_client.publish(&subj, packet.payload()).await;
                println!("Result from publish: {:?}", res);
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

//  Free function to connect to nats
pub async fn nats_connect(options: Options, config_url: &str) -> Result<Connection> {
    let conn : IOResult<Connection> = options.connect(&config_url).await;
    match conn {
        Ok(conn) => {
            println!("nats_connect global function connection OK");
            conn.publish("habitat.event.healthcheck", "nats.rs client is UP").await;
            Ok(conn)
        },
        _ => {
            println!("nats_connect global function failed to connect and set connection");
            Err(error::Error::ConnectNatsServer)
        }
    }
}
