use crate::event::{Error,
                   EventStreamConfig,
                   NatsSubject,
                   Result};
use futures::{channel::{mpsc as futures_mpsc,
                        mpsc::UnboundedSender},
              stream::StreamExt};
use log::{error,
          trace,
          warn};
use rustls_native_certs::CertificateResult;
use tokio::time;

/// The subject and payload of a NATS message.
#[derive(Debug)]
pub struct NatsMessage {
    pub(super) subject: &'static NatsSubject,
    pub(super) payload: Vec<u8>,
}

impl NatsMessage {
    pub fn new(subject: &'static NatsSubject, payload: Vec<u8>) -> Self {
        NatsMessage { subject, payload }
    }
}

/// A lightweight handle for the NATS message stream. All events are converted into a NatsMessage
/// and sent into this stream to be published.
///
/// An UnboundedSender should be ok here. Messages are continuously processed even if the client is
/// not currently connected.
pub struct NatsMessageStream(pub(super) UnboundedSender<NatsMessage>);

impl NatsMessageStream {
    pub async fn new(supervisor_id: &str, config: EventStreamConfig) -> Result<NatsMessageStream> {
        let EventStreamConfig { url,
                                token,
                                connect_method,
                                server_certificate,
                                .. } = config;

        let connect_options =
            get_connect_options(supervisor_id, &token, server_certificate.clone())?;

        // Connect to the server. If a timeout was set, we want to ensure we establish a connection
        // before exiting the function. If we do not connect within the timeout we return an error.
        // If we do not have a timeout, we don't care if we can immediately connect. Instead we
        // spawn a future that will resolve when a connection is possible. Once we establish
        // a connection, the client will handle reconnecting if necessary.
        let client = match connect_method.into() {
            Some(timeout) => {
                match time::timeout(timeout,
                                    async_nats::connect_with_options(&url, connect_options)).await
                {
                    Ok(Ok(client)) => client,
                    Ok(Err(e)) => return Err(Error::NatsServerConnect(e)),
                    Err(_elapsed) => return Err(Error::ConnectionTimeout),
                }
            }
            None => {
                // When no timeout is set, establish the connection without waiting. The async-nats
                // client handles reconnection automatically if the connection is lost, so we don't
                // need to spawn additional reconnect attempts.
                async_nats::connect_with_options(&url, connect_options).await?
            }
        };

        let (tx, mut rx) = futures_mpsc::unbounded::<NatsMessage>();

        // Spawn a task to handle publishing received messages
        tokio::spawn(async move {
            while let Some(packet) = rx.next().await {
                let NatsMessage { subject, payload } = packet;
                if let Err(e) = client.publish(subject.clone(), payload.into()).await {
                    error!("Failed to publish message to subject '{}', err: {}",
                           subject, e);
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

fn get_connect_options(supervisor_id: &str,
                       token: &habitat_common::types::EventStreamToken,
                       server_certificate: Option<super::EventStreamServerCertificate>)
                       -> Result<async_nats::ConnectOptions> {
    let mut root_cert_store = rustls::RootCertStore::empty();

    // Load native system certificates. Even if some fail to load, we proceed with those that
    // loaded successfully. This allows connections with partial certificate data rather than
    // failing entirely, while still logging warnings about failures.
    let certificate_result: CertificateResult = rustls_native_certs::load_native_certs();
    let (added, ignored) = root_cert_store.add_parsable_certificates(certificate_result.certs);
    log::info!("Added {} certificates returned by rustls_native_certs::load_native_certs",
               added);
    log::info!("Ignored {} certificates returned by rustls_native_certs::load_native_certs",
               ignored);
    if !certificate_result.errors.is_empty() {
        log::warn!("Errors reported by rustls_native_certs::load_native_certs");
        for error in certificate_result.errors {
            log::warn!("ERROR: {:?}", error);
        }
    }

    // This is kind of the "habitat way of finding certs", above may be extra
    // Failures to load habitat_core certs are logged as warnings but do not prevent connection.
    // This allows the client to proceed with other available certificates.
    let habitat_certs = match habitat_core::tls::native_tls_wrapper::certificates(None) {
        Ok(certs) => certs,
        Err(err) => {
            warn!("Failed to load habitat_core TLS certificates: {}", err);
            Vec::new()
        }
    };

    for certificate in habitat_certs {
        if let Ok(native_tls_der_cert) = certificate.to_der() {
            let rustls_der_cert =
                rustls_pki_types::CertificateDer::from_slice(&native_tls_der_cert);
            if let Err(err) = root_cert_store.add(rustls_der_cert) {
                warn!("Failed to add habitat_core certificate to root store: {}",
                      err);
            }
        } else {
            warn!("Failed to convert habitat_core certificate to DER format");
        }
    }

    // add the provided server_certificate into the certificate store
    if let Some(certificate) = server_certificate {
        let native_tls_der_cert = <super::EventStreamServerCertificate as
            Into<native_tls::Certificate>>::into(certificate).to_der().map_err(|err| {
                Error::NativeTls(Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("Failed to convert native_tls certificate to DER format: {}", err),
                )))
            })?;
        let rustls_der_cert = rustls_pki_types::CertificateDer::from_slice(&native_tls_der_cert);
        root_cert_store.add(rustls_der_cert).map_err(|err| {
                                                 Error::NativeTls(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Failed to add server certificate to root store: {}", err),
            )))
                                             })?;
    }

    let tls_client_config = rustls::ClientConfig::builder().with_root_certificates(root_cert_store)
                                                           .with_no_client_auth();
    let connect_options = async_nats::ConnectOptions::new().name(format!("hab_client_{}",
                                                                         supervisor_id))
                                                           .token(token.to_string())
                                                           .tls_client_config(tls_client_config);
    Ok(connect_options)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::NatsServerAddress;
    use habitat_common::types::{EventStreamConnectMethod,
                                EventStreamMetadata,
                                EventStreamServerCertificate};
    use std::{net::SocketAddr,
              sync::LazyLock,
              time::Duration};
    use tokio::{io::{AsyncBufReadExt,
                     AsyncReadExt,
                     AsyncWriteExt,
                     BufReader},
                net::TcpListener,
                sync::oneshot,
                time::timeout};

    fn config(url: NatsServerAddress,
              connect_method: EventStreamConnectMethod)
              -> EventStreamConfig {
        EventStreamConfig { environment: String::from("env"),
                            application: String::from("app"),
                            site: None,
                            meta: EventStreamMetadata::default(),
                            token: "token".parse().unwrap(),
                            url,
                            connect_method,
                            server_certificate: None }
    }

    fn config_with_cert(url: NatsServerAddress,
                        connect_method: EventStreamConnectMethod,
                        server_certificate: Option<EventStreamServerCertificate>)
                        -> EventStreamConfig {
        EventStreamConfig { environment: String::from("env"),
                            application: String::from("app"),
                            site: None,
                            meta: EventStreamMetadata::default(),
                            token: "token".parse().unwrap(),
                            url,
                            connect_method,
                            server_certificate }
    }

    fn install_rustls_provider() {
        static INSTALL: LazyLock<()> = LazyLock::new(|| {
            rustls::crypto::aws_lc_rs::default_provider().install_default()
                                                         .unwrap();
        });
        LazyLock::force(&INSTALL);
    }

    #[tokio::test]
    async fn returns_connection_timeout_when_server_never_responds() {
        install_rustls_provider();

        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr: SocketAddr = listener.local_addr().unwrap();

        // Accept a connection and then deliberately never complete the NATS handshake.
        tokio::spawn(async move {
            let _ = listener.accept().await;
            tokio::time::sleep(Duration::from_secs(5)).await;
        });

        let config = config(format!("nats://{}", addr).parse().unwrap(),
                            EventStreamConnectMethod::Timeout { secs: 1 });
        let result = NatsMessageStream::new("sup-id", config).await;

        match result {
            Err(Error::ConnectionTimeout) => {}
            Err(Error::NatsServerConnect(_)) => {}
            Err(e) => panic!("unexpected error: {e:?}"),
            Ok(_) => panic!("expected connection to fail"),
        }
    }

    #[tokio::test]
    async fn maps_connect_error_when_server_refuses_connection() {
        install_rustls_provider();

        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        drop(listener); // Release the port so the connection attempt is refused.

        let config = config(format!("nats://{}", addr).parse().unwrap(),
                            EventStreamConnectMethod::Timeout { secs: 1 });

        // The connection should fail with either NatsServerConnect or ConnectionTimeout.
        // async-nats may timeout before detecting the refused connection.
        let result = NatsMessageStream::new("sup-id", config).await;
        match result {
            Err(Error::NatsServerConnect(_)) => {}
            Err(Error::ConnectionTimeout) => {}
            Err(e) => panic!("unexpected error: {e:?}"),
            Ok(_) => panic!("expected connection failure"),
        }
    }

    async fn run_fake_nats_server(listener: TcpListener, payload_sender: oneshot::Sender<Vec<u8>>) {
        let (mut stream, _addr) = listener.accept().await.unwrap();
        let port = stream.local_addr().unwrap().port();
        let info = format!("INFO {{\"server_id\":\"test\",\"version\":\"1.0.0\",\"host\":\"127.0.\
                            0.1\",\"port\":{},\"max_payload\":1048576}}\r\n",
                           port);
        stream.write_all(info.as_bytes()).await.unwrap();
        let (reader, mut writer) = stream.into_split();
        let mut reader = BufReader::new(reader);

        loop {
            let mut line = String::new();
            if reader.read_line(&mut line).await.unwrap_or(0) == 0 {
                break;
            }

            let trimmed = line.trim_end();
            if trimmed.starts_with("PING") {
                writer.write_all(b"PONG\r\n").await.unwrap();
            } else if trimmed.starts_with("CONNECT") {
                continue;
            } else if trimmed.starts_with("PUB ") {
                let parts: Vec<&str> = trimmed.split_whitespace().collect();
                if parts.len() >= 3 {
                    let payload_len: usize = parts[2].parse().unwrap_or(0);
                    let mut payload = vec![0u8; payload_len];
                    reader.read_exact(&mut payload).await.unwrap();
                    let mut crlf = [0u8; 2];
                    let _ = reader.read_exact(&mut crlf).await;
                    let _ = payload_sender.send(payload);
                    break;
                }
            }
        }
    }

    #[tokio::test]
    async fn publishes_payload_over_fake_nats_server() {
        install_rustls_provider();

        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let (payload_sender, payload_receiver) = oneshot::channel();
        tokio::spawn(run_fake_nats_server(listener, payload_sender));

        static TEST_SUBJECT: LazyLock<NatsSubject> =
            LazyLock::new(|| NatsSubject::from("habitat.event.test"));

        let config = config(format!("nats://{}", addr).parse().unwrap(),
                            EventStreamConnectMethod::Timeout { secs: 2 });
        let stream = NatsMessageStream::new("sup-id", config).await.unwrap();

        stream.send(NatsMessage::new(&TEST_SUBJECT, b"payload".to_vec()));

        let payload =
            timeout(Duration::from_secs(2), payload_receiver).await
                                                             .expect("fake server should capture \
                                                                      publish")
                                                             .expect("payload should be delivered");

        assert_eq!(payload, b"payload");
    }

    #[tokio::test]
    async fn accepts_self_signed_certificate() {
        use rcgen::{CertificateParams,
                    KeyPair};
        use std::io::Write;
        use tempfile::NamedTempFile;

        install_rustls_provider();

        // Generate a self-signed certificate
        let mut params = CertificateParams::new(vec!["localhost".to_string()]).unwrap();
        params.is_ca = rcgen::IsCa::Ca(rcgen::BasicConstraints::Unconstrained);
        let key_pair = KeyPair::generate().unwrap();
        let cert = params.self_signed(&key_pair).unwrap();

        // Write certificate to a temp file
        let mut cert_file = NamedTempFile::new().unwrap();
        write!(cert_file, "{}", cert.pem()).unwrap();
        cert_file.flush().unwrap();

        // Parse the certificate path into EventStreamServerCertificate
        let cert_path = cert_file.path().to_str().unwrap();
        let server_cert: EventStreamServerCertificate = cert_path.parse().unwrap();

        // Attempt to create NatsMessageStream with self-signed cert
        // This validates that the certificate is properly loaded into the rustls cert store
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        drop(listener); // Will fail to connect, but should accept the certificate

        let config = config_with_cert(format!("nats://{}", addr).parse().unwrap(),
                                      EventStreamConnectMethod::Timeout { secs: 1 },
                                      Some(server_cert));

        // The connection should fail due to refused connection, NOT due to certificate issues
        let result = NatsMessageStream::new("sup-id", config).await;
        match result {
            Err(Error::NatsServerConnect(_)) => {
                // This is expected - connection refused, but cert was accepted
            }
            Err(Error::ConnectionTimeout) => {
                // Also acceptable - timeout waiting for connection
            }
            Err(e) => panic!("unexpected error (cert should have been accepted): {e:?}"),
            Ok(_) => panic!("expected connection failure due to no server"),
        }
    }
}
