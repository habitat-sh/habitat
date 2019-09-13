pub mod pkg;

use crate::error::{Error,
                   Result};
use habitat_core;
use pnet::datalink;
use std::net::IpAddr;

fn ip_address_from_network_interface_or_parsed(s: &str) -> Result<IpAddr> {
    for interface in datalink::interfaces() {
        if interface.name == s {
            // Check we have no more than one IPv4 address
            let ipv4_interfaces = interface.ips
                                           .iter()
                                           .filter(|network| network.is_ipv4())
                                           .count();
            if ipv4_interfaces > 1 {
                return Err(Error::NetworkInterfaceIpv4AddressAmbiguity(interface.name));
            }
            // Try to find an IPv4 address
            return interface.ips
                            .iter()
                            .find_map(|network| {
                                if network.is_ipv4() {
                                    Some(network.ip())
                                } else {
                                    None
                                }
                            })
                            .ok_or(Error::NetworkInterfaceHasNoIpv4Address(interface.name));
        }
    }
    Ok(s.parse()?)
}

/// Determine a sys IP address with the following order of precedence:
///
/// 1. Try to dynamically determine an outgoing IP address using the routing table trick.
/// 2. If `maybe_unparsed_sys_ip` matches a network interface name do the following:
///     - If there is more than one IPv4 address, there is ambiguity and we produce an error. We do
///       not support multiple `sys.ip`s. Note, we do not support IPv6 so it is ok if there are
///       multiple IPv6 addresses.
///     - If there is no IPv4 address, produce an error.
///     - Otherwise, use the network interface's IPv4 address.
/// 3. Try to parse `maybe_unparsed_sys_ip` as an IP address.
pub fn determine_sys_ip_address(maybe_unparsed_sys_ip: Option<&str>) -> Result<IpAddr> {
    match habitat_core::util::sys::ip() {
        Ok(ip) => Ok(ip),
        Err(e) => {
            if let Some(val) = maybe_unparsed_sys_ip {
                ip_address_from_network_interface_or_parsed(val)
            } else {
                Err(e.into())
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::net::Ipv4Addr;

    #[test]
    fn test_ip_address_from_network_interface_or_parsed() {
        #[cfg(unix)]
        assert_eq!(ip_address_from_network_interface_or_parsed("lo").unwrap(),
                   Ipv4Addr::LOCALHOST);
        assert_eq!(ip_address_from_network_interface_or_parsed("127.0.0.1").unwrap(),
                   Ipv4Addr::LOCALHOST);
        assert!(ip_address_from_network_interface_or_parsed("this is an invalid address").is_err());
    }
}
