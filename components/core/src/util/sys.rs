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

/// The technique used to determine the outgoing IP address is documented [here][1].
///
/// "A connected UDP socket can also be used to determine the outgoing interface
/// that will be used to a particular destination. This is because of a side effect of the connect
/// function when applied to a UDP socket: The kernel chooses the local IP address (assuming the
/// process has not already called bind to explicitly assign this). This local IP address is chosen
/// by searching the routing table for the destination IP address, and then using the primary IP
/// address for the resulting interface." From "Unix Network Programming v1.3" chapter 8
/// section 14.
///
/// [1]: http://www.masterraghu.com/subjects/np/introduction/unix_network_programming_v1.3/ch08lev1sec14.html
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
