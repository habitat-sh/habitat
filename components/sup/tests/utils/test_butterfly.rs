//! Encapsulate a butterfly client and expose its functionality via a
//! test-focused API. Clients are configured for a specific service
//! group (namely, the one of the test package we are running).
//!
//! No ring key or encryption abilities are currently supported.

use anyhow::{Context,
             Result};
use habitat_butterfly::client::Client as ButterflyClient;
use habitat_core::service::ServiceGroup;
use std::{net::SocketAddr,
          time::{SystemTime,
                 UNIX_EPOCH}};

pub struct Client {
    butterfly_client: ButterflyClient,
}

impl Client {
    pub fn new(port: u16) -> Result<Client> {
        let gossip_addr =
            format!("127.0.0.1:{}", port).parse::<SocketAddr>()
                                         .context("Could not parse Butterfly gossip address!")?;
        let butterfly_client =
            ButterflyClient::new(&gossip_addr.to_string(), None).context("Could not create \
                                                                          Butterfly Client for \
                                                                          test!")?;
        Ok(Client { butterfly_client })
    }

    /// Apply the given configuration to the Supervisor. It will
    /// always be applied to the service group for which the client was
    /// initially configured.
    ///
    /// A time-based incarnation value is automatically used,
    /// resulting in less clutter in your tests.
    pub fn apply(&mut self,
                 package_name: &str,
                 service_group: &str,
                 applied_config: &str)
                 -> Result<()> {
        let config = applied_config.to_string();
        let config = config.as_bytes();

        // Validate the TOML, to save you from typos in your tests
        toml::de::from_slice::<toml::value::Value>(config).with_context(|| {
                                                              format!("Invalid TOML: {}",
                                                                      applied_config)
                                                          })?;

        let incarnation = Self::new_incarnation()?;
        self.butterfly_client
            .send_service_config(ServiceGroup::new(package_name, service_group, None).unwrap(),
                                 incarnation,
                                 config,
                                 false)
            .context("Cannot send the service configuration")?;
        Ok(())
    }

    /// Generate a new incarnation number using the number of seconds
    /// since the Unix Epoch. As a result, this is unique to within a
    /// second, so beware! Might need to incorporate nanoseconds as well.
    fn new_incarnation() -> Result<u64> {
        Ok(SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs())
    }
}
