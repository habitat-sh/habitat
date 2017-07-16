// Copyright (c) 2017 Chef Software Inc. and/or applicable contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::str;

use butterfly::rumor::service::SysInfo;
use hcore;
#[cfg(any(target_os = "linux", target_os = "macos"))]
use libc;

use VERSION;
use config::GossipListenAddr;
use error::{Error, Result};
use http_gateway;

static LOGKEY: &'static str = "SY";

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Sys {
    pub version: String,
    pub member_id: String,
    pub ip: IpAddr,
    pub hostname: String,
    pub gossip_ip: IpAddr,
    pub gossip_port: u16,
    pub http_gateway_ip: IpAddr,
    pub http_gateway_port: u16,
    pub permanent: bool,
}

impl Sys {
    pub fn new(permanent: bool, gossip: GossipListenAddr, http: http_gateway::ListenAddr) -> Sys {
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
        Sys {
            version: VERSION.to_string(),
            member_id: "unloaded".to_string(),
            ip: ip,
            hostname: host,
            gossip_ip: gossip.ip(),
            gossip_port: gossip.port(),
            http_gateway_ip: http.ip(),
            http_gateway_port: http.port(),
            permanent: permanent,
        }
    }

    pub fn as_sys_info(&self) -> SysInfo {
        let mut sys_info = SysInfo::new();
        sys_info.set_ip(self.ip.to_string());
        sys_info.set_hostname(self.hostname.clone());
        sys_info.set_gossip_ip(self.gossip_ip.to_string());
        sys_info.set_gossip_port(self.gossip_port as u32);
        sys_info.set_http_gateway_ip(self.http_gateway_ip.to_string());
        sys_info.set_http_gateway_port(self.http_gateway_port as u32);
        sys_info
    }

    pub fn gossip_listen(&self) -> SocketAddr {
        SocketAddr::new(self.gossip_ip, self.gossip_port)
    }

    pub fn http_listen(&self) -> http_gateway::ListenAddr {
        http_gateway::ListenAddr::new(self.http_gateway_ip, self.http_gateway_port)
    }
}

pub fn lookup_ip() -> Result<IpAddr> {
    match hcore::util::sys::ip() {
        Ok(s) => Ok(s),
        Err(e) => Err(sup_error!(Error::HabitatCore(e))),
    }
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
pub fn lookup_hostname() -> Result<String> {
    use std::ffi::CStr;

    let len = 255;
    let mut buf = Vec::<u8>::with_capacity(len);
    let ptr = buf.as_mut_slice().as_mut_ptr();
    let err = unsafe { gethostname(ptr as *mut libc::c_char, len as libc::size_t) };
    match err {
        0 => {
            let slice = unsafe { CStr::from_ptr(ptr as *const libc::c_char) };
            let s = slice.to_str()?;
            Ok(s.to_string())
        }
        n => {
            error!("gethostname failure: {}", n);
            Err(sup_error!(Error::IPFailed))
        }
    }
}

#[cfg(target_os = "windows")]
pub fn lookup_hostname() -> Result<String> {
    use std::env;

    match env::var("COMPUTERNAME") {
        Ok(computername) => Ok(computername),
        Err(_) => Err(sup_error!(Error::IPFailed)),
    }
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
extern "C" {
    pub fn gethostname(name: *mut libc::c_char, size: libc::size_t) -> libc::c_int;
}
