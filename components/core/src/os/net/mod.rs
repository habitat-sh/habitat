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

use std::io;

/// Returns the fqdn from the provided hostname.
pub fn lookup_fqdn(hostname: &str) -> io::Result<String> {
    #[cfg(not(windows))]
    let flags = libc::AI_CANONNAME;

    #[cfg(windows)]
    let flags = winapi::shared::ws2def::AI_CANONNAME;

    let hints = dns_lookup::AddrInfoHints { flags,
                                            ..dns_lookup::AddrInfoHints::default() };

    // If 'hints.flags' includes the AI_CANONNAME flag, then the ai_canonname
    // field of the first of the addrinfo structures in the returned list is set
    // to point to the official name of the host.
    if let Some(first_result) = dns_lookup::getaddrinfo(Some(hostname), None, Some(hints))?.next() {
        match first_result {
            Ok(f) => Ok(f.canonname.expect("Some(canonname) if requested")),
            Err(e) => {
                debug!("lookup_fqdn() was unable to lookup the machine fqdn. {:?}",
                       e);
                Ok(hostname.to_string())
            }
        }
    } else {
        Ok(hostname.to_string())
    }
}

/// Returns the fully qualified domain name of the running machine.
pub fn fqdn() -> Option<String> {
    let result = dns_lookup::get_hostname().and_then(|hostname| lookup_fqdn(&hostname));
    if let Err(ref e) = result {
        debug!("fqdn() was unable to lookup the machine fqdn. {:?}", e);
    }
    result.ok()
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

#[cfg(not(windows))]
#[test]
fn test_fqdn_lookup_err() {
    let fqdn = lookup_fqdn("");
    assert!(fqdn.is_err(), "Should be an Err()");
    assert_eq!(format!("{}", fqdn.unwrap_err()),
               "failed to lookup address information: Name or service not known");
}
