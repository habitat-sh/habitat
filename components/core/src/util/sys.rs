use crate::error::{Error,
                   Result};
pub use crate::os::system::{uname,
                            Uname};
use std::{io,
          net::{IpAddr,
                Ipv4Addr,
                SocketAddr,
                ToSocketAddrs,
                UdpSocket}};

const UNSPECIFIED_SOCKET_ADDR: (Ipv4Addr, u16) = (Ipv4Addr::UNSPECIFIED, 0);

crate::env_config_socketaddr!(OutboundIpAddrLookupSocketAddr,
                              HAB_OUTBOUND_IP_ADDR_LOOKUP_SOCKET_ADDR,
                              // Use Google DNS as the default
                              8,
                              8,
                              8,
                              8,
                              53);

pub fn ip() -> Result<IpAddr> {
    let connect_addr = SocketAddr::from(OutboundIpAddrLookupSocketAddr::configured_value());
    ip_impl(connect_addr).map_err(Error::NoOutboundIpAddr)
}

fn ip_impl(connect_addr: impl ToSocketAddrs) -> io::Result<IpAddr> {
    let socket = UdpSocket::bind(UNSPECIFIED_SOCKET_ADDR)?;
    socket.connect(connect_addr)?;
    let addr = socket.local_addr()?;
    Ok(addr.ip())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ip_lookup() {
        assert!(ip_impl("an invalid socket addr").is_err());
        assert!(ip().is_ok());

        // We would prefer to use the `locked_env_var` macro here but currently, the `habitat_core`
        // crate does not depend on `habitat_common`. If this becomes an issue, some refactoring
        // will be needed.
        std::env::set_var(OutboundIpAddrLookupSocketAddr::ENVVAR, "127.0.0.1:80");
        // localhost should route back to localhost
        assert_eq!(ip().unwrap(), Ipv4Addr::LOCALHOST);
    }
}
