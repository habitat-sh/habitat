// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

pub mod convert;
pub mod path;
pub mod sys;
pub mod signals;

use std::net::Ipv4Addr;
use std::net::SocketAddrV4;
use std::str::FromStr;

use time;

use error::{Error, Result};
static LOGKEY: &'static str = "UT";

/// Gives us a time to stop for in seconds.
pub fn stop_time(duration: i64) -> time::Timespec {
    let current_time = time::now_utc().to_timespec();
    let wait_duration = time::Duration::seconds(duration as i64);
    let stop_time = current_time + wait_duration;
    stop_time
}

/// This function takes a string as an option.
/// - If you haven't sent a string, then we'll return (default_ip, default_port).
/// - If you sent Some(some_ip_without_port), we'll return (your IP, default_port).
/// - If you sent Some(some_ip_with_port), we'll just hand that back (your IP, your port).
pub fn parse_ip_port_with_defaults(s: Option<&str>,
                                   default_ip: &str,
                                   default_port: u16)
                                   -> Result<(String, u16)> {
    if let None = s {
        // return the default IP:Port combo
        return Ok((default_ip.to_string(), default_port));
    }

    let s = s.unwrap();

    // is it an IP + Port combo?
    match SocketAddrV4::from_str(s) {
        Ok(sa) => return Ok((sa.ip().to_string(), sa.port())),
        Err(_) => (),
    }

    // is it just an IP? if so, then append the default gossip port
    if Ipv4Addr::from_str(s).is_ok() {
        return Ok((s.to_string(), default_port));
    }

    return Err(sup_error!(Error::IPFailed));
}

#[cfg(test)]
mod tests {
    use super::parse_ip_port_with_defaults;

    #[test]
    fn parse_ip_port_with_defaults_test() {
        let default_ip = "192.168.1.11";
        let default_port = 1234;
        // no ip/port passed in, use default ip:port
        assert!(("192.168.1.11".to_string(), 1234) ==
                parse_ip_port_with_defaults(None, default_ip, default_port).unwrap());
        // only IP passed in
        assert!(("192.168.1.99".to_string(), 1234) ==
                parse_ip_port_with_defaults(Some("192.168.1.99"), default_ip, default_port)
            .unwrap());
        // pass in IP and port
        assert!(("192.168.1.99".to_string(), 5678) ==
                parse_ip_port_with_defaults(Some("192.168.1.99:5678"), default_ip, default_port)
            .unwrap());
        // pass in something unparseable
        assert!(parse_ip_port_with_defaults(Some("foo"), default_ip, default_port).is_err());
    }
}
