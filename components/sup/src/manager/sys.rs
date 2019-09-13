use crate::VERSION;
use habitat_butterfly::rumor::service::SysInfo;
use habitat_common::{outputln,
                     types::{GossipListenAddr,
                             HttpListenAddr,
                             ListenCtlAddr}};
use habitat_core;
use std::{net::{IpAddr,
                SocketAddr},
          str};

static LOGKEY: &str = "SY";

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
               http: HttpListenAddr,
               ip: IpAddr)
               -> Self {
        let host = habitat_core::os::net::hostname().unwrap_or_else(|e| {
                                                        let host = String::from("localhost");
                                                        outputln!("Hostname lookup failed; using \
                                                                   fallback of {} ({})",
                                                                  host,
                                                                  e);
                                                        host
                                                    });
        Self { version: VERSION.to_string(),
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

    pub fn http_listen(&self) -> HttpListenAddr {
        HttpListenAddr::new(self.http_gateway_ip, self.http_gateway_port)
    }
}
