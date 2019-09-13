pub mod pkg;

use crate::error::Result;
use habitat_core;
use std::net::IpAddr;

pub fn discover_outgoing_ip() -> Result<IpAddr> { Ok(habitat_core::util::sys::ip()?) }
