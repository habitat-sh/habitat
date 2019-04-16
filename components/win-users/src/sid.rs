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

#![allow(non_snake_case)]

use std::{io,
          mem,
          ptr::{copy,
                null_mut}};

use widestring::WideCString;
use winapi::{shared::{minwindef::{BOOL,
                                  BYTE,
                                  DWORD,
                                  FALSE,
                                  HLOCAL,
                                  LPBOOL,
                                  LPDWORD,
                                  LPVOID,
                                  TRUE,
                                  WORD},
                      ntdef::HANDLE,
                      winerror},
             um::{handleapi,
                  processthreadsapi,
                  winbase,
                  winnt::{ACCESS_MASK,
                          ACL,
                          DACL_SECURITY_INFORMATION,
                          LPCWSTR,
                          MAXDWORD,
                          PACL,
                          PHANDLE,
                          PSECURITY_DESCRIPTOR,
                          PSECURITY_INFORMATION,
                          PSID,
                          TOKEN_READ}}};

#[repr(C)]
struct ACL_SIZE_INFORMATION {
    aceCount:      DWORD,
    aclBytesInUse: DWORD,
    aclBytesFree:  DWORD,
}

#[repr(C)]
struct ACE_HEADER {
    aceType:  BYTE,
    aceFlags: BYTE,
    aceSize:  WORD,
}
type PaceHeader = *mut ACE_HEADER;

#[repr(C)]
struct ACCESS_ALLOWED_ACE {
    header:   ACE_HEADER,
    mask:     ACCESS_MASK,
    sidStart: DWORD,
}

extern "system" {
    fn ObtainSid(hToken: HANDLE, psid: *mut PSID) -> BOOL;
}

#[link(name = "user32")]
extern "system" {
    fn AddAccessAllowedAceEx(pAcl: PACL,
                             dwAceRevision: DWORD,
                             aceFlags: DWORD,
                             accessMask: DWORD,
                             pSid: PSID)
                             -> BOOL;
    fn AddAce(pAcl: PACL,
              dwAceRevision: DWORD,
              dwStartingAceIndex: DWORD,
              pAceList: LPVOID,
              nAceListLength: DWORD)
              -> BOOL;
    fn ConvertSidToStringSidW(Sid: PSID, StringSid: LPCWSTR) -> BOOL;
    fn GetAce(pAcl: PACL, dwAceIndex: DWORD, pAce: *mut LPVOID) -> BOOL;
    fn GetAclInformation(pAcl: PACL,
                         pAclInformation: LPVOID,
                         nAclInformationLength: DWORD,
                         dwAclInformationClass: DWORD)
                         -> BOOL;
    fn OpenProcessToken(processHandle: HANDLE, desiredAccess: DWORD, tokenHandle: PHANDLE) -> BOOL;
    fn GetLengthSid(pSid: PSID) -> DWORD;
    fn GetSecurityDescriptorDacl(pSecurityDescriptor: PSECURITY_DESCRIPTOR,
                                 lpbDaclPresent: LPBOOL,
                                 pDacl: *mut PACL,
                                 lpbDaclDefaulted: LPBOOL)
                                 -> BOOL;
    fn GetUserObjectSecurity(hObj: HANDLE,
                             pSIRequested: PSECURITY_INFORMATION,
                             pSD: PSECURITY_INFORMATION,
                             nLength: DWORD,
                             lpnLengthNeeded: LPDWORD)
                             -> BOOL;
    fn InitializeAcl(pAcl: PACL, nAclLength: DWORD, dwAclRevision: DWORD) -> BOOL;
    fn InitializeSecurityDescriptor(pSecurityDescriptor: PSECURITY_DESCRIPTOR,
                                    dwRevision: DWORD)
                                    -> BOOL;
    fn SetSecurityDescriptorDacl(pSecurityDescriptor: PSECURITY_DESCRIPTOR,
                                 bDaclPresent: BOOL,
                                 pDacl: PACL,
                                 bDaclDefaulted: BOOL)
                                 -> BOOL;
    fn SetUserObjectSecurity(hObj: HANDLE,
                             pSIRequested: PSECURITY_INFORMATION,
                             pSID: PSECURITY_DESCRIPTOR)
                             -> BOOL;
}

pub const GENERIC_READ: DWORD = 0x80000000;
pub const GENERIC_WRITE: DWORD = 0x40000000;
pub const GENERIC_EXECUTE: DWORD = 0x20000000;
pub const GENERIC_ALL: DWORD = 0x10000000;

pub const WINSTA_ALL_ACCESS: DWORD = 0x37F;
pub const DELETE: DWORD = 0x00010000;
pub const READ_CONTROL: DWORD = 0x00020000;
pub const WRITE_DAC: DWORD = 0x00040000;
pub const WRITE_OWNER: DWORD = 0x00080000;

pub const DESKTOP_CREATEMENU: DWORD = 0x0004;
pub const DESKTOP_CREATEWINDOW: DWORD = 0x0002;
pub const DESKTOP_ENUMERATE: DWORD = 0x0040;
pub const DESKTOP_HOOKCONTROL: DWORD = 0x0008;
pub const DESKTOP_JOURNALPLAYBACK: DWORD = 0x0020;
pub const DESKTOP_JOURNALRECORD: DWORD = 0x0010;
pub const DESKTOP_READOBJECTS: DWORD = 0x0001;
pub const DESKTOP_SWITCHDESKTOP: DWORD = 0x0100;
pub const DESKTOP_WRITEOBJECTS: DWORD = 0x0080;

pub const OBJECECT_INHERIT_ACE: DWORD = 0x1;
pub const CONTAINER_INHERIT_ACE: DWORD = 0x2;
pub const NO_PROPAGATE_INHERIT_ACE: DWORD = 0x4;
pub const INHERIT_ONLY_ACE: DWORD = 0x8;

pub struct Sid {
    pub raw: Vec<u8>,
}

impl Sid {
    pub fn from_current_user() -> io::Result<Self> {
        unsafe {
            let handle = processthreadsapi::GetCurrentProcess();
            let mut token = null_mut();
            cvt(OpenProcessToken(handle, TOKEN_READ, &mut token))?;
            let sid = Self::from_token(token);
            handleapi::CloseHandle(token);
            handleapi::CloseHandle(handle);
            Ok(sid?)
        }
    }

    pub fn from_token(token: HANDLE) -> io::Result<Self> {
        unsafe {
            let mut sid: PSID = null_mut();
            cvt(ObtainSid(token, &mut sid))?;

            let sz = GetLengthSid(sid) as usize;
            let mut buf: Vec<u8> = Vec::with_capacity(sz);
            copy(sid, buf.as_mut_ptr() as PSID, sz);
            Ok(Self { raw: buf })
        }
    }

    pub fn to_string(&self) -> io::Result<String> {
        let mut buffer: LPCWSTR = null_mut();
        unsafe {
            cvt(ConvertSidToStringSidW(self.raw.as_ptr() as PSID,
                                       (&mut buffer as *mut LPCWSTR)
                                       as LPCWSTR))?
        };

        let widestr = unsafe { WideCString::from_ptr_str(buffer) };
        unsafe { winbase::LocalFree(buffer as HLOCAL) };
        Ok(widestr.to_string_lossy())
    }

    // This code was adapted from much of the C++ code in
    // https://msdn.microsoft.com/en-us/library/windows/desktop/aa379608(v=vs.85).aspx
    pub fn add_to_user_object(&self,
                              handle: HANDLE,
                              ace_flags: DWORD,
                              access_mask: DWORD)
                              -> io::Result<()> {
        unsafe {
            let mut needed_len: u32 = 0;
            let mut sd: Vec<u8> = Vec::new();
            let mut sd_new: Vec<u8> = Vec::new();
            let mut dacl_present: BOOL = FALSE;
            let mut dacl_exist: BOOL = FALSE;
            let mut pacl: PACL = null_mut();

            if GetUserObjectSecurity(handle,
                                     &mut DACL_SECURITY_INFORMATION,
                                     null_mut(),
                                     0,
                                     &mut needed_len)
               == 0
            {
                match io::Error::last_os_error().raw_os_error() {
                    Some(error) => {
                        match error as u32 {
                            winerror::ERROR_INSUFFICIENT_BUFFER => {
                                sd = Vec::with_capacity((needed_len) as usize);
                                sd_new = Vec::with_capacity((needed_len) as usize);
                            }
                            _ => return Err(io::Error::last_os_error()),
                        }
                    }
                    None => {}
                }
            }

            cvt(GetUserObjectSecurity(handle,
                                      &mut DACL_SECURITY_INFORMATION,
                                      sd.as_mut_ptr()
                                      as PSECURITY_INFORMATION,
                                      needed_len,
                                      &mut needed_len))?;

            cvt(InitializeSecurityDescriptor(sd_new.as_mut_ptr()
                                             as PSECURITY_DESCRIPTOR,
                                             1))?; // SECURITY_DESCRIPTOR_REVISION

            let pd: PSECURITY_DESCRIPTOR = sd.as_mut_ptr() as PSECURITY_DESCRIPTOR;
            cvt(GetSecurityDescriptorDacl(pd,
                                          &mut dacl_present,
                                          &mut pacl,
                                          &mut dacl_exist))?;

            let mut size_info = ACL_SIZE_INFORMATION { aceCount:      0,
                                                       aclBytesInUse: mem::size_of::<ACL>()
                                                                      as DWORD,
                                                       aclBytesFree:  0, };
            if pacl != null_mut() {
                let mut acl_size_buf: Vec<u8> =
                    Vec::with_capacity(mem::size_of::<ACL_SIZE_INFORMATION>());
                cvt(GetAclInformation(pacl,
                                      acl_size_buf.as_mut_ptr() as LPVOID,
                                      mem::size_of::<ACL_SIZE_INFORMATION>()
                                      as DWORD,
                                      2 /* AclSizeInformation */))?;

                let psize_info = &mut *(acl_size_buf.as_mut_ptr() as *mut ACL_SIZE_INFORMATION);
                size_info.aceCount = (*psize_info).aceCount;
                size_info.aclBytesInUse = (*psize_info).aclBytesInUse;
                size_info.aclBytesFree = (*psize_info).aclBytesFree;
            }

            let psid_length = GetLengthSid(self.raw.as_ptr() as PSID);
            let new_acl_size = size_info.aclBytesInUse
                               + (2 * (mem::size_of::<ACCESS_ALLOWED_ACE>() as DWORD))
                               + (2 * psid_length)
                               - (2 * (mem::size_of::<DWORD>() as DWORD));
            let mut new_acl_buf: Vec<u8> = Vec::with_capacity(new_acl_size as usize);
            cvt(InitializeAcl(new_acl_buf.as_mut_ptr() as PACL,
                              new_acl_size,
                              2 /* ACL_REVISION */))?;

            if dacl_present == TRUE {
                for i in 0..size_info.aceCount {
                    let mut temp_acl: LPVOID = null_mut();
                    cvt(GetAce(pacl, i, &mut temp_acl))?;
                    cvt(AddAce(new_acl_buf.as_mut_ptr() as PACL,
                               2, // ACL_REVISION
                               MAXDWORD,
                               temp_acl,
                               (*(temp_acl as PaceHeader)).aceSize as DWORD))?;
                }
            }

            cvt(AddAccessAllowedAceEx(new_acl_buf.as_mut_ptr() as PACL,
                                      2, // ACL_REVISION
                                      ace_flags,
                                      access_mask,
                                      self.raw.as_ptr() as PSID))?;

            cvt(SetSecurityDescriptorDacl(sd_new.as_mut_ptr()
                                          as PSECURITY_DESCRIPTOR,
                                          TRUE,
                                          new_acl_buf.as_mut_ptr() as PACL,
                                          FALSE))?;
            cvt(SetUserObjectSecurity(handle,
                                      &mut DACL_SECURITY_INFORMATION,
                                      sd_new.as_mut_ptr()
                                      as PSECURITY_DESCRIPTOR))?;

            Ok(())
        }
    }
}

fn cvt(i: i32) -> io::Result<i32> {
    if i == 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(i)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn current_user_sid() { assert!(Sid::from_current_user().is_ok()) }
}
