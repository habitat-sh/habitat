use crate::error::{Error,
                   Result};
pub use crate::os::system::{uname,
                            Uname};
use std::{io,
          net::{IpAddr,
                Ipv4Addr,
                ToSocketAddrs,
                UdpSocket}};

const UNSPECIFIED_SOCKET_ADDR: (Ipv4Addr, u16) = (Ipv4Addr::UNSPECIFIED, 0);
const GOOGLE_DNS: &str = "8.8.8.8:53";

pub fn ip() -> Result<IpAddr> { ip_impl(GOOGLE_DNS).map_err(Error::IpLookupFailed) }

fn ip_impl(connect_addr: impl ToSocketAddrs) -> io::Result<IpAddr> {
    let socket = UdpSocket::bind(UNSPECIFIED_SOCKET_ADDR)?;
    socket.connect(connect_addr)?;
    let addr = socket.local_addr()?;
    Ok(addr.ip())
}

#[cfg(test)]
mod test {
    use super::{ip,
                ip_impl};

    #[test]
    fn ip_lookup() {
        assert!(ip_impl("an invalid socket addr").is_err());
        assert!(ip().is_ok());
    }
}
