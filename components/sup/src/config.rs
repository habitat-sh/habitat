//! Configuration for the Supervisor.
//!
//! This module is populated from the CLI options in `main.rs`, and then passed through to the
//! [command](../command) modules. Check out the `config_from_args(..)` function there for more
//! details.
//!
//! See the [Config](struct.Config.html) struct for the specific options available.

use habitat_common::cli::{GOSSIP_DEFAULT_IP,
                          GOSSIP_DEFAULT_PORT,
                          GOSSIP_LISTEN_ADDRESS_ENVVAR};
use habitat_core::env::Config as EnvConfig;
use std::{fmt,
          io,
          net::{IpAddr,
                Ipv4Addr,
                SocketAddr,
                SocketAddrV4,
                ToSocketAddrs},
          ops::{Deref,
                DerefMut},
          option,
          result,
          str::FromStr};

use crate::error::{Result,
                   SupError};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct GossipListenAddr(SocketAddr);

impl GossipListenAddr {
    pub fn new(ip: Ipv4Addr, port: u16) -> Self {
        GossipListenAddr(SocketAddr::V4(SocketAddrV4::new(ip, port)))
    }

    /// Generate an address at which a server configured with this
    /// GossipListenAddr can communicate with itself.
    ///
    /// In particular, a server configured to listen on `0.0.0.0` vs
    /// `192.168.1.1` should be contacted via `127.0.0.1` in the
    /// former case, but `192.168.1.1` in the latter.
    pub fn local_addr(&self) -> Self {
        let mut addr = *self;
        if addr.0.ip().is_unspecified() {
            // TODO (CM): Use Ipv4Addr::loopback() when it's no longer experimental
            // TODO (CM): Support IPV6, once we do that more broadly
            addr.0.set_ip(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
        }
        addr
    }
}

impl Default for GossipListenAddr {
    fn default() -> GossipListenAddr {
        GossipListenAddr::new(GOSSIP_DEFAULT_IP.parse()
                                               .expect("GOSSIP_DEFAULT_IP can not be parsed."),
                              GOSSIP_DEFAULT_PORT)
    }
}

impl EnvConfig for GossipListenAddr {
    const ENVVAR: &'static str = GOSSIP_LISTEN_ADDRESS_ENVVAR;
}

impl Deref for GossipListenAddr {
    type Target = SocketAddr;

    fn deref(&self) -> &SocketAddr { &self.0 }
}

impl DerefMut for GossipListenAddr {
    fn deref_mut(&mut self) -> &mut SocketAddr { &mut self.0 }
}

impl FromStr for GossipListenAddr {
    type Err = SupError;

    fn from_str(val: &str) -> Result<Self> { Ok(GossipListenAddr(SocketAddr::from_str(val)?)) }
}

impl ToSocketAddrs for GossipListenAddr {
    type Iter = option::IntoIter<SocketAddr>;

    fn to_socket_addrs(&self) -> io::Result<Self::Iter> { self.0.to_socket_addrs() }
}

impl fmt::Display for GossipListenAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> result::Result<(), fmt::Error> {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod gossip_listen_addr {
        use super::*;
        #[test]
        fn local_addr_for_gossip_listen_addr_works_for_unspecified_address() {
            let listen_addr = GossipListenAddr::default();
            assert!(listen_addr.0.ip().is_unspecified());

            let local_addr = listen_addr.local_addr();
            assert!(local_addr.0.ip().is_loopback());
        }

        #[test]
        fn local_addr_for_gossip_listen_addr_returns_same_ip_for_a_specified_address() {
            let mut listen_addr = GossipListenAddr::default();
            listen_addr.0
                       .set_ip(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)));
            assert!(!listen_addr.0.ip().is_loopback());

            let local_addr = listen_addr.local_addr();
            assert_eq!(listen_addr, local_addr);
        }
    }

    mod env_config {
        use super::*;
        use habitat_common::locked_env_var;
        use std::{env,
                  num::ParseIntError,
                  result,
                  str::FromStr};

        #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq)]
        struct Thingie(u64);

        impl Default for Thingie {
            fn default() -> Self { Thingie(2112) }
        }

        impl FromStr for Thingie {
            type Err = ParseIntError;

            fn from_str(s: &str) -> result::Result<Self, Self::Err> {
                let raw = s.parse::<u64>()?;
                Ok(Thingie(raw))
            }
        }

        locked_env_var!(HAB_TESTING_THINGIE, lock_hab_testing_thingie);

        impl EnvConfig for Thingie {
            const ENVVAR: &'static str = "HAB_TESTING_THINGIE";
        }

        #[test]
        fn no_env_var_yields_default() {
            let _envvar = lock_hab_testing_thingie();
            assert!(env::var("HAB_TESTING_THINGIE").is_err()); // it's not set
            assert_eq!(Thingie::configured_value(), Thingie(2112));
            assert_eq!(Thingie::configured_value(), Thingie::default());
        }

        #[test]
        fn parsable_env_var_yields_parsed_value() {
            let envvar = lock_hab_testing_thingie();
            envvar.set("123");
            assert_eq!(Thingie::configured_value(), Thingie(123));
            assert_ne!(Thingie::configured_value(), Thingie::default());
        }

        #[test]
        fn unparsable_env_var_yields_default() {
            let envvar = lock_hab_testing_thingie();
            envvar.set("I'm not a number");
            assert_eq!(Thingie::configured_value(), Thingie::default());
        }
    }
}
