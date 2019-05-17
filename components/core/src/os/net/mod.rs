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
