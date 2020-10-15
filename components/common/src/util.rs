pub mod path;

use crate::error::Error;
use std::{io,
          net::{SocketAddr,
                ToSocketAddrs}};

pub fn resolve_socket_addr_with_default_port<S: AsRef<str>>(
    addr: S,
    default_port: u16)
    -> Result<(String, SocketAddr), Error> {
    let addr = addr.as_ref();
    let domain;
    let mut addrs = if let Some(index) = addr.find(':') {
                        domain = &addr[..index];
                        addr.to_socket_addrs()
                    } else {
                        domain = addr;
                        (addr, default_port).to_socket_addrs()
                    }.map_err(|e| Error::ListenCtlResolutionError(addr.to_string(), e))?;
    addrs.find(std::net::SocketAddr::is_ipv4)
         .ok_or_else(|| {
             Error::ListenCtlResolutionError(addr.to_string(),
                                             io::Error::new(io::ErrorKind::InvalidInput,
                                                            "did not resolve to an ipv4 socket \
                                                             address"))
         })
         .map(|addr| (domain.to_string(), addr))
}

#[cfg(test)]
mod test {
    use super::resolve_socket_addr_with_default_port;

    #[test]
    fn test_socket_addr_with_default_port() {
        assert_eq!(resolve_socket_addr_with_default_port("127.0.0.1", 89).unwrap(),
                   ("127.0.0.1".to_string(), "127.0.0.1:89".parse().expect("")));
        assert_eq!(resolve_socket_addr_with_default_port("localhost", 89).unwrap(),
                   ("localhost".to_string(), "127.0.0.1:89".parse().expect("")));
        assert_eq!(resolve_socket_addr_with_default_port("1.2.3.4:1500", 89).unwrap(),
                   ("1.2.3.4".to_string(), "1.2.3.4:1500".parse().expect("")));
        assert!(resolve_socket_addr_with_default_port("an_invalid_address", 89).is_err());
    }
}
