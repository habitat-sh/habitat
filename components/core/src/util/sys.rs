use crate::error::{Error,
                   Result};
pub use crate::os::system::{uname,
                            Uname};
use std::net::{IpAddr,
               UdpSocket};

static GOOGLE_DNS: &str = "8.8.8.8:53";

pub fn ip() -> Result<IpAddr> {
    let socket = UdpSocket::bind("0.0.0.0:0").map_err(Error::IpLookupFailed)?;
    socket.connect(GOOGLE_DNS).map_err(Error::IpLookupFailed)?;
    let addr = socket.local_addr().map_err(Error::IpLookupFailed)?;
    Ok(addr.ip())
}
