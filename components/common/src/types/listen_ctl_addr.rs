use std::{fmt,
          net::{IpAddr,
                Ipv4Addr,
                SocketAddr,
                SocketAddrV4},
          result};

habitat_core::env_config_socketaddr!(#[derive(Clone, Copy, PartialEq, Eq, Debug)],
                                     pub ListenCtlAddr,
                                     HAB_LISTEN_CTL,
                                     SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, Self::DEFAULT_PORT)));

impl ListenCtlAddr {
    pub const DEFAULT_PORT: u16 = 9632;

    pub fn new(ip: Ipv4Addr, port: u16) -> Self {
        ListenCtlAddr(SocketAddr::V4(SocketAddrV4::new(ip, port)))
    }

    pub fn ip(&self) -> IpAddr { self.0.ip() }

    pub fn port(&self) -> u16 { self.0.port() }
}

impl fmt::Display for ListenCtlAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> result::Result<(), fmt::Error> {
        write!(f, "{}", self.0)
    }
}

impl From<SocketAddr> for ListenCtlAddr {
    fn from(socket_addr: SocketAddr) -> Self { ListenCtlAddr(socket_addr) }
}

impl AsRef<SocketAddr> for ListenCtlAddr {
    fn as_ref(&self) -> &SocketAddr { &self.0 }
}
