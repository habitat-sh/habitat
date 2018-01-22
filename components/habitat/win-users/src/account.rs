// Copyright (c) 2016-2017 Chef Software Inc. and/or applicable contributors
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

use std::ptr::null_mut;
use std::io::Error;

use widestring::WideCString;
use winapi::{LPCWSTR, BOOL, PSID, PSID_NAME_USE, LPDWORD, SID_NAME_USE};
use winapi::winerror::*;

use super::sid::Sid;

extern "system" {
    fn LookupAccountNameW(
        lpSystemName: LPCWSTR,
        lpAccountName: LPCWSTR,
        Sid: PSID,
        cbSid: LPDWORD,
        ReferencedDomainName: LPCWSTR,
        cchReferencedDomainName: LPDWORD,
        peUse: PSID_NAME_USE,
    ) -> BOOL;
}

pub struct Account {
    pub name: String,
    pub system_name: Option<String>,
    pub domain: String,
    pub account_type: SID_NAME_USE,
    pub sid: Sid,
}

impl Account {
    pub fn from_name(name: &str) -> Option<Account> {
        lookup_account(name, None)
    }

    pub fn from_name_and_system(name: &str, system_name: &str) -> Option<Account> {
        lookup_account(name, Some(system_name.to_string()))
    }
}

fn lookup_account(name: &str, system_name: Option<String>) -> Option<Account> {

    // if this is a machine account, strip the terminating '$'
    // LookupAccountName will return the sid of the computer account
    // given the computer name. Windows forbids usernames to match the
    // computer name
    let stripped_name = if name.ends_with("$") {
        &name[..name.len() - 1]
    } else {
        name
    };
    let mut sid_size: u32 = 0;
    let mut domain_size: u32 = 0;
    let wide = WideCString::from_str(stripped_name).unwrap();
    unsafe {
        LookupAccountNameW(
            null_mut(),
            wide.as_ptr(),
            null_mut(),
            &mut sid_size as LPDWORD,
            null_mut(),
            &mut domain_size as LPDWORD,
            null_mut(),
        )
    };
    match Error::last_os_error().raw_os_error().unwrap() as u32 {
        ERROR_INSUFFICIENT_BUFFER => {}
        ERROR_NONE_MAPPED => return None,
        _ => {
            panic!(
                "Error while looking up account for {}: {}",
                name,
                Error::last_os_error()
            )
        }
    }

    let mut sid: Vec<u8> = Vec::with_capacity(sid_size as usize);
    let mut domain: Vec<u16> = Vec::with_capacity(domain_size as usize);
    let mut sid_type = SID_NAME_USE(0);

    let ret = unsafe {
        LookupAccountNameW(
            null_mut(),
            wide.as_ptr(),
            sid.as_mut_ptr() as PSID,
            &mut sid_size as LPDWORD,
            domain.as_mut_ptr(),
            &mut domain_size as LPDWORD,
            &mut sid_type as PSID_NAME_USE,
        )
    };
    if ret == 0 {
        panic!(
            "Failed to retrieve SID for {}: {}",
            name,
            Error::last_os_error()
        );
    }
    unsafe {
        domain.set_len(domain_size as usize);
        sid.set_len(sid_size as usize);
    }
    let domain_str = WideCString::from_vec(domain).unwrap().to_string_lossy();
    Some(Account {
        name: name.to_string(),
        system_name: system_name,
        domain: domain_str,
        account_type: sid_type,
        sid: Sid { raw: sid },
    })
}

#[cfg(test)]
mod tests {
    use std::env;

    use winapi::winnt::{SidTypeUser, SidTypeWellKnownGroup};

    use super::*;

    #[test]
    fn real_account_returns_some() {
        assert_eq!(Account::from_name("Administrator").is_some(), true)
    }

    #[test]
    fn bogus_account_returns_none() {
        assert_eq!(Account::from_name("bogus").is_none(), true)
    }

    #[test]
    fn user_account_returns_user_type() {
        let acct_type = Account::from_name("Administrator").unwrap().account_type;
        assert_eq!(acct_type, SidTypeUser)
    }

    #[test]
    fn local_user_account_returns_local_machine_as_domain() {
        let acct_domain = Account::from_name("Administrator").unwrap().domain;
        assert_eq!(acct_domain, env::var("COMPUTERNAME").unwrap())
    }

    #[test]
    fn well_known_group_account_returns_correct_type() {
        let acct_type = Account::from_name("Everyone").unwrap().account_type;
        assert_eq!(acct_type, SidTypeWellKnownGroup)
    }

    #[test]
    fn well_known_group_account_has_well_known_sid() {
        let sid = Account::from_name("Everyone").unwrap().sid;
        assert_eq!(sid.to_string().unwrap(), "S-1-1-0")
    }

    #[test]
    fn mixing_case_returns_same_account() {
        let current_user = env::var("USERNAME").unwrap();
        let lower_sid = Account::from_name(current_user.to_lowercase().as_str())
            .unwrap()
            .sid;
        let upper_sid = Account::from_name(current_user.to_uppercase().as_str())
            .unwrap()
            .sid;
        assert_eq!(
            lower_sid.to_string().unwrap(),
            upper_sid.to_string().unwrap()
        )
    }
}
