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

// lookup_fqdn returns a vector of fqdn that resolves the provided hostname
pub fn lookup_fqdn(hostname: String) -> io::Result<Vec<String>> {
  let hints = AddrInfoHints {
      flags: libc::AI_CANONNAME,
      .. AddrInfoHints::default()
  };
  let addrinfos =
      getaddrinfo(Some(&hostname), None, Some(hints))?
      .collect::<std::io::Result<Vec<_>>>()?;

  let canonnames = addrinfos
      .into_iter()
      .filter_map(|info| info.canonname)
      .collect();

  Ok(canonnames)
}

// fqdn returns the fully qualified domain name of the running machine
pub fn fqdn() -> Option<String> {
    // Implementation 1 - Using match statements
    match hostname() {
        Ok(hostname) => {
            // we clone the hostname since we need it to find the
            // right fqdn that the lookup fn returns
            let host_to_lookup = hostname.clone();

            match lookup_fqdn(host_to_lookup) {
                Ok(fqdns) => {
                    fqdns.into_iter()
                        .find(| ref h| h.contains(&hostname))
                }
                // @afiune if the lookup_fqdn returns an Err(), should we
                // return the hostname instead of None?
                Err(_) => None
            }
        },
        Err(_) => None,
    }
}

#[test]
#[ignore]
fn test_fqdn() {
    // @afiune This test is ignore because it is testing the actual
    // fqdn of the running machine, mine has 'afiune-ubuntu-vb.lala.com'
    assert_eq!(
        fqdn().unwrap(),
        String::from("afiune-ubuntu-vb.lala.com"),
        "should match with the configured fqdn in the running machine");
}

#[test]
fn test_fqdn_lookup() {
    let fqdn = lookup_fqdn(String::from("localhost"));
    assert!(fqdn.is_ok());
    assert_eq!(fqdn.unwrap(),
       vec![String::from("localhost")],
       "the fqdn of localhost should be localhost");
}

#[test]
fn test_fqdn_lookup_err() {
    let fqdn = lookup_fqdn(String::from(""));
    assert!(fqdn.is_err(), "Should be an Err()");
    assert_eq!(
        format!("{}",fqdn.unwrap_err()),
        "failed to lookup address information: Name or service not known");
}
