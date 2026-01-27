//! Event subsystem-specific error handling

use std::result;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Could not establish connection to NATS server")]
    NatsServerConnect(#[from] async_nats::ConnectError),

    #[error(transparent)]
    HabitatCore(#[from] habitat_core::Error),

    #[error(transparent)]
    Nats(#[from] async_nats::Error),

    #[error(transparent)]
    NatsSubscribeError(#[from] async_nats::SubscribeError),

    #[error(transparent)]
    TlsCertLoadError(#[from] std::io::Error),

    #[error("Connection to NATS server timed out")]
    ConnectionTimeout,

    #[error("Error from habitat's native_tls_wrapper: {0}")]
    NativeTls(Box<dyn std::error::Error + Send + Sync>),
}
