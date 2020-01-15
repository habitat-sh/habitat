//! Encapsulate a butterfly client and expose its functionality via a
//! test-focused API. Clients are configured for a specific service
//! group (namely, the one of the test package we are running).
//!
//! No ring key or encryption abilities are currently supported.

use self::habitat_butterfly::client::Client as ButterflyClient;
use habitat_butterfly;

use self::habitat_core::service::ServiceGroup;
use habitat_core;

use toml;

use std::{net::SocketAddr,
          time::{SystemTime,
                 UNIX_EPOCH}};

pub struct Client {
    butterfly_client:  ButterflyClient,
    pub package_name:  String,
    pub service_group: String,
}

impl Client {
    pub fn new(package_name: &str, service_group: &str, port: u16) -> Client {
        let gossip_addr =
            format!("127.0.0.1:{}", port).parse::<SocketAddr>()
                                         .expect("Could not parse Butterfly gossip address!");
        let c = ButterflyClient::new(&gossip_addr.to_string(), None).expect("Could not create \
                                                                             Butterfly Client \
                                                                             for test!");
        Client { butterfly_client: c,
                 package_name:     package_name.to_string(),
                 service_group:    service_group.to_string(), }
    }

    /// Apply the given configuration to the Supervisor. It will
    /// always be applied to the service group for which the client was
    /// initially configured.
    ///
    /// A time-based incarnation value is automatically used,
    /// resulting in less clutter in your tests.
    pub fn apply(&mut self, config: &str) {
        let config = config.to_string();
        let config = config.as_bytes();

        // Validate the TOML, to save you from typos in your tests
        if let Err(err) = self::toml::de::from_slice::<self::toml::value::Value>(&config) {
            panic!("Invalid TOML! {:?} ==> {:?}", config, err);
        }

        let incarnation = Self::new_incarnation();
        self.butterfly_client
            .send_service_config(ServiceGroup::new(&self.package_name,
                                                   &self.service_group,
                                                   None).unwrap(),
                                 incarnation,
                                 config,
                                 false)
            .expect("Cannot send the service configuration");
    }

    /// Generate a new incarnation number using the number of seconds
    /// since the Unix Epoch. As a result, this is unique to within a
    /// second, so beware! Might need to incorporate nanoseconds as well.
    fn new_incarnation() -> u64 {
        SystemTime::now().duration_since(UNIX_EPOCH)
                         .unwrap()
                         .as_secs()
    }
}
