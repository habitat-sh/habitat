use crate::{error::{Error,
                    Result},
            http_gateway,
            VERSION};
use habitat_butterfly::rumor::service::SysInfo;
use habitat_common::{outputln,
                     types::{GossipListenAddr,
                             ListenCtlAddr}};
use habitat_core;
use std::{net::{IpAddr,
                Ipv4Addr,
                SocketAddr},
          str};

static LOGKEY: &'static str = "SY";

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Sys {
    pub version:           String,
    pub member_id:         String,
    pub ip:                IpAddr,
    pub hostname:          String,
    pub gossip_ip:         IpAddr,
    pub gossip_port:       u16,
    pub ctl_gateway_ip:    IpAddr,
    pub ctl_gateway_port:  u16,
    pub http_gateway_ip:   IpAddr,
    pub http_gateway_port: u16,
    pub permanent:         bool,
}

impl Sys {
    pub fn new(permanent: bool,
               gossip: GossipListenAddr,
               ctl: ListenCtlAddr,
               http: http_gateway::ListenAddr)
               -> Sys {
        let ip = match lookup_ip() {
            Ok(ip) => ip,
            Err(e) => {
                let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
                outputln!("IP Address lookup failed; using fallback of {} ({})", ip, e);
                ip
            }
        };
        let host = match lookup_hostname() {
            Ok(host) => host,
            Err(e) => {
                let host = String::from("localhost");
                outputln!("Hostname lookup failed; using fallback of {} ({})", host, e);
                host
            }
        };
        Sys { version: VERSION.to_string(),
              member_id: "unloaded".to_string(),
              ip,
              hostname: host,
              gossip_ip: gossip.ip(),
              gossip_port: gossip.port(),
              ctl_gateway_ip: ctl.ip(),
              ctl_gateway_port: ctl.port(),
              http_gateway_ip: http.ip(),
              http_gateway_port: http.port(),
              permanent }
    }

    pub fn as_sys_info(&self) -> SysInfo {
        let mut sys_info = SysInfo::default();
        sys_info.ip = self.ip.to_string();
        sys_info.hostname = self.hostname.clone();
        sys_info.gossip_ip = self.gossip_ip.to_string();
        sys_info.gossip_port = u32::from(self.gossip_port);
        sys_info.ctl_gateway_ip = self.ctl_gateway_ip.to_string();
        sys_info.ctl_gateway_port = u32::from(self.ctl_gateway_port);
        sys_info.http_gateway_ip = self.http_gateway_ip.to_string();
        sys_info.http_gateway_port = u32::from(self.http_gateway_port);
        sys_info
    }

    pub fn ctl_listen(&self) -> SocketAddr {
        SocketAddr::new(self.ctl_gateway_ip, self.ctl_gateway_port)
    }

    pub fn gossip_listen(&self) -> SocketAddr { SocketAddr::new(self.gossip_ip, self.gossip_port) }

    pub fn http_listen(&self) -> http_gateway::ListenAddr {
        http_gateway::ListenAddr::new(self.http_gateway_ip, self.http_gateway_port)
    }
}

pub fn lookup_ip() -> Result<IpAddr> {
    match habitat_core::util::sys::ip() {
        Ok(s) => Ok(s),
        Err(e) => Err(Error::HabitatCore(e)),
    }
}

pub fn lookup_hostname() -> Result<String> {
    match habitat_core::os::net::hostname() {
        Ok(hostname) => Ok(hostname),
        Err(_) => Err(Error::IPFailed),
    }
}
