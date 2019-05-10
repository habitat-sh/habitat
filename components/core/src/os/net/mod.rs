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

#[allow(unused_variables)]
#[cfg(windows)]
#[path = "windows.rs"]
mod imp;

#[cfg(not(windows))]
#[path = "unix.rs"]
mod imp;

pub use self::imp::*;

extern crate dns_lookup;

use dns_lookup::{getaddrinfo,
                 AddrInfoHints};
use std::io;

// lookup_fqdn returns a vector of fqdn that resolves the provided hostname.
//
// Since the underlying crate is platform agnostic, this function is as well,
// we only implement a single variable `flags` that has the right hint for
// the running operating system.
pub fn lookup_fqdn(hostname: &str) -> io::Result<String> {
    #[cfg(not(windows))]
    let flags = libc::AI_CANONNAME;

    #[cfg(windows)]
    let flags = winapi::shared::ws2def::AI_CANONNAME;

    let hints = AddrInfoHints { flags,
                                ..AddrInfoHints::default() };
    let addrinfos =
        getaddrinfo(Some(hostname), None, Some(hints))?.collect::<std::io::Result<Vec<_>>>()?;

    // If 'hints.ai_flags' includes the AI_CANONNAME flag, then the ai_canonname
    // field of the first of the addrinfo structures in the returned list is set
    // to point to the official name of the host.
    if !addrinfos.is_empty() {
        let addrinfo = addrinfos[0].clone();
        return Ok(addrinfo.canonname.unwrap_or_else(|| hostname.to_string()));
    }

    Ok(hostname.to_string())
}

// fqdn returns the fully qualified domain name of the running machine
pub fn fqdn() -> Option<String> {
    match hostname() {
        Ok(hostname) => lookup_fqdn(&hostname).ok(),
        Err(_) => None,
    }
}

#[test]
#[ignore]
fn test_fqdn() {
    // @afiune This test is ignore because it is testing the actual
    // fqdn of the running machine, mine has 'afiune-ubuntu-vb.lala.com'
    assert_eq!(fqdn().unwrap(),
               String::from("afiune-ubuntu-vb.lala.com"),
               "should match with the configured fqdn in the running machine");
}

#[cfg(not(windows))]
#[test]
fn test_fqdn_lookup() {
    let fqdn = lookup_fqdn("localhost");
    assert!(fqdn.is_ok());
    assert_eq!(fqdn.unwrap(),
               String::from("localhost"),
               "the fqdn of localhost should be localhost");
}

#[test]
fn test_fqdn_lookup_err() {
    let fqdn = lookup_fqdn("");
    assert!(fqdn.is_err(), "Should be an Err()");
    assert_eq!(format!("{}", fqdn.unwrap_err()),
               "failed to lookup address information: Name or service not known");
}
