use std::net::{IpAddr,
               UdpSocket};

use crate::error::Result;

pub use crate::os::system::{uname,
                            Uname};

static GOOGLE_DNS: &'static str = "8.8.8.8:53";

pub fn ip() -> Result<IpAddr> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.connect(GOOGLE_DNS)?;
    let addr = socket.local_addr()?;
    Ok(addr.ip())
}
