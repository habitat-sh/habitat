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

use std::ptr::{copy, null_mut};
use std::io;
use std::mem;

use kernel32::{self, LocalFree};
use widestring::WideCString;
use winapi;

#[repr(C)]
struct ACL_SIZE_INFORMATION {
    aceCount: winapi::DWORD,
    aclBytesInUse: winapi::DWORD,
    aclBytesFree: winapi::DWORD,
}

#[repr(C)]
struct ACE_HEADER {
    aceType: winapi::BYTE,
    aceFlags: winapi::BYTE,
    aceSize: winapi::WORD,
}
type PaceHeader = *mut ACE_HEADER;

#[repr(C)]
struct ACCESS_ALLOWED_ACE {
    header: ACE_HEADER,
    mask: winapi::ACCESS_MASK,
    sidStart: winapi::DWORD,
}

extern "system" {
    fn ObtainSid(hToken: winapi::HANDLE, psid: *mut winapi::PSID) -> winapi::BOOL;
}

#[link(name = "user32")]
extern "system" {
    fn AddAccessAllowedAceEx(
        pAcl: winapi::PACL,
        dwAceRevision: winapi::DWORD,
        aceFlags: winapi::DWORD,
        accessMask: winapi::DWORD,
        pSid: winapi::PSID,
    ) -> winapi::BOOL;
    fn AddAce(
        pAcl: winapi::PACL,
        dwAceRevision: winapi::DWORD,
        dwStartingAceIndex: winapi::DWORD,
        pAceList: winapi::LPVOID,
        nAceListLength: winapi::DWORD,
    ) -> winapi::BOOL;
    fn ConvertSidToStringSidW(Sid: winapi::PSID, StringSid: winapi::LPCWSTR) -> winapi::BOOL;
    fn GetAce(
        pAcl: winapi::PACL,
        dwAceIndex: winapi::DWORD,
        pAce: *mut winapi::LPVOID,
    ) -> winapi::BOOL;
    fn GetAclInformation(
        pAcl: winapi::PACL,
        pAclInformation: winapi::LPVOID,
        nAclInformationLength: winapi::DWORD,
        dwAclInformationClass: winapi::DWORD,
    ) -> winapi::BOOL;
    fn OpenProcessToken(
        processHandle: winapi::HANDLE,
        desiredAccess: winapi::DWORD,
        tokenHandle: winapi::PHANDLE,
    ) -> winapi::BOOL;
    fn GetLengthSid(pSid: winapi::PSID) -> winapi::DWORD;
    fn GetSecurityDescriptorDacl(
        pSecurityDescriptor: winapi::PSECURITY_DESCRIPTOR,
        lpbDaclPresent: winapi::LPBOOL,
        pDacl: *mut winapi::PACL,
        lpbDaclDefaulted: winapi::LPBOOL,
    ) -> winapi::BOOL;
    fn GetUserObjectSecurity(
        hObj: winapi::HANDLE,
        pSIRequested: winapi::PSECURITY_INFORMATION,
        pSD: winapi::PSECURITY_INFORMATION,
        nLength: winapi::DWORD,
        lpnLengthNeeded: winapi::LPDWORD,
    ) -> winapi::BOOL;
    fn InitializeAcl(
        pAcl: winapi::PACL,
        nAclLength: winapi::DWORD,
        dwAclRevision: winapi::DWORD,
    ) -> winapi::BOOL;
    fn InitializeSecurityDescriptor(
        pSecurityDescriptor: winapi::PSECURITY_DESCRIPTOR,
        dwRevision: winapi::DWORD,
    ) -> winapi::BOOL;
    fn SetSecurityDescriptorDacl(
        pSecurityDescriptor: winapi::PSECURITY_DESCRIPTOR,
        bDaclPresent: winapi::BOOL,
        pDacl: winapi::PACL,
        bDaclDefaulted: winapi::BOOL,
    ) -> winapi::BOOL;
    fn SetUserObjectSecurity(
        hObj: winapi::HANDLE,
        pSIRequested: winapi::PSECURITY_INFORMATION,
        pSID: winapi::PSECURITY_DESCRIPTOR,
    ) -> winapi::BOOL;
}

pub const GENERIC_READ: winapi::DWORD = 0x80000000;
pub const GENERIC_WRITE: winapi::DWORD = 0x40000000;
pub const GENERIC_EXECUTE: winapi::DWORD = 0x20000000;
pub const GENERIC_ALL: winapi::DWORD = 0x10000000;

pub const WINSTA_ALL_ACCESS: winapi::DWORD = 0x37F;
pub const DELETE: winapi::DWORD = 0x00010000;
pub const READ_CONTROL: winapi::DWORD = 0x00020000;
pub const WRITE_DAC: winapi::DWORD = 0x00040000;
pub const WRITE_OWNER: winapi::DWORD = 0x00080000;

pub const DESKTOP_CREATEMENU: winapi::DWORD = 0x0004;
pub const DESKTOP_CREATEWINDOW: winapi::DWORD = 0x0002;
pub const DESKTOP_ENUMERATE: winapi::DWORD = 0x0040;
pub const DESKTOP_HOOKCONTROL: winapi::DWORD = 0x0008;
pub const DESKTOP_JOURNALPLAYBACK: winapi::DWORD = 0x0020;
pub const DESKTOP_JOURNALRECORD: winapi::DWORD = 0x0010;
pub const DESKTOP_READOBJECTS: winapi::DWORD = 0x0001;
pub const DESKTOP_SWITCHDESKTOP: winapi::DWORD = 0x0100;
pub const DESKTOP_WRITEOBJECTS: winapi::DWORD = 0x0080;

pub const OBJECECT_INHERIT_ACE: winapi::DWORD = 0x1;
pub const CONTAINER_INHERIT_ACE: winapi::DWORD = 0x2;
pub const NO_PROPAGATE_INHERIT_ACE: winapi::DWORD = 0x4;
pub const INHERIT_ONLY_ACE: winapi::DWORD = 0x8;

pub struct Sid {
    pub raw: Vec<u8>,
}

impl Sid {
    pub fn from_current_user() -> io::Result<Self> {
        unsafe {
            let handle = kernel32::GetCurrentProcess();
            let mut token = null_mut();
            cvt(OpenProcessToken(handle, winapi::TOKEN_READ, &mut token))?;
            let sid = Self::from_token(token);
            kernel32::CloseHandle(token);
            kernel32::CloseHandle(handle);
            Ok(sid?)
        }
    }

    pub fn from_token(token: winapi::HANDLE) -> io::Result<Self> {
        unsafe {
            let mut sid: winapi::PSID = null_mut();
            cvt(ObtainSid(token, &mut sid))?;

            let sz = GetLengthSid(sid) as usize;
            let mut buf: Vec<u8> = Vec::with_capacity(sz);
            copy(sid, buf.as_mut_ptr() as winapi::PSID, sz);
            Ok(Self { raw: buf })
        }
    }

    pub fn to_string(&self) -> io::Result<String> {
        let mut buffer: winapi::LPCWSTR = null_mut();
        unsafe {
            cvt(ConvertSidToStringSidW(
                self.raw.as_ptr() as winapi::PSID,
                (&mut buffer as *mut winapi::LPCWSTR) as winapi::LPCWSTR,
            ))?
        };

        let widestr = unsafe { WideCString::from_ptr_str(buffer) };
        unsafe { LocalFree(buffer as winapi::HLOCAL) };
        Ok(widestr.to_string_lossy())
    }

    // This code was adapted from much of the C++ code in
    // https://msdn.microsoft.com/en-us/library/windows/desktop/aa379608(v=vs.85).aspx
    pub fn add_to_user_object(
        &self,
        handle: winapi::HANDLE,
        ace_flags: winapi::DWORD,
        access_mask: winapi::DWORD,
    ) -> io::Result<()> {
        unsafe {
            let mut needed_len: u32 = 0;
            let mut sd: Vec<u8> = Vec::new();
            let mut sd_new: Vec<u8> = Vec::new();
            let mut dacl_present: winapi::BOOL = winapi::FALSE;
            let mut dacl_exist: winapi::BOOL = winapi::FALSE;
            let mut pacl: winapi::PACL = null_mut();

            if GetUserObjectSecurity(
                handle,
                &mut winapi::DACL_SECURITY_INFORMATION,
                null_mut(),
                0,
                &mut needed_len,
            ) == 0
            {
                match io::Error::last_os_error().raw_os_error() {
                    Some(error) => {
                        match error as u32 {
                            winapi::winerror::ERROR_INSUFFICIENT_BUFFER => {
                                sd = Vec::with_capacity((needed_len) as usize);
                                sd_new = Vec::with_capacity((needed_len) as usize);
                            }
                            _ => return Err(io::Error::last_os_error()),
                        }
                    }
                    None => {}
                }
            }

            cvt(GetUserObjectSecurity(
                handle,
                &mut winapi::DACL_SECURITY_INFORMATION,
                sd.as_mut_ptr() as winapi::PSECURITY_INFORMATION,
                needed_len,
                &mut needed_len,
            ))?;

            cvt(InitializeSecurityDescriptor(
                sd_new.as_mut_ptr() as winapi::PSECURITY_DESCRIPTOR,
                1,
            ))?; // SECURITY_DESCRIPTOR_REVISION

            let pd: winapi::PSECURITY_DESCRIPTOR = sd.as_mut_ptr() as winapi::PSECURITY_DESCRIPTOR;
            cvt(GetSecurityDescriptorDacl(
                pd,
                &mut dacl_present,
                &mut pacl,
                &mut dacl_exist,
            ))?;

            let mut size_info = ACL_SIZE_INFORMATION {
                aceCount: 0,
                aclBytesInUse: mem::size_of::<winapi::ACL>() as winapi::DWORD,
                aclBytesFree: 0,
            };
            if pacl != null_mut() {
                let mut acl_size_buf: Vec<u8> =
                    Vec::with_capacity(mem::size_of::<ACL_SIZE_INFORMATION>());
                cvt(GetAclInformation(
                    pacl,
                    acl_size_buf.as_mut_ptr() as winapi::LPVOID,
                    mem::size_of::<ACL_SIZE_INFORMATION>() as winapi::DWORD,
                    2, // AclSizeInformation
                ))?;

                let psize_info = &mut *(acl_size_buf.as_mut_ptr() as *mut ACL_SIZE_INFORMATION);
                size_info.aceCount = (*psize_info).aceCount;
                size_info.aclBytesInUse = (*psize_info).aclBytesInUse;
                size_info.aclBytesFree = (*psize_info).aclBytesFree;
            }

            let psid_length = GetLengthSid(self.raw.as_ptr() as winapi::PSID);
            let new_acl_size = size_info.aclBytesInUse +
                (2 * (mem::size_of::<ACCESS_ALLOWED_ACE>() as winapi::DWORD)) +
                (2 * psid_length) -
                (2 * (mem::size_of::<winapi::DWORD>() as winapi::DWORD));
            let mut new_acl_buf: Vec<u8> = Vec::with_capacity(new_acl_size as usize);
            cvt(InitializeAcl(
                new_acl_buf.as_mut_ptr() as winapi::PACL,
                new_acl_size,
                2, // ACL_REVISION
            ))?;

            if dacl_present == winapi::TRUE {
                for i in 0..size_info.aceCount {
                    let mut temp_acl: winapi::LPVOID = null_mut();
                    cvt(GetAce(pacl, i, &mut temp_acl))?;
                    cvt(AddAce(
                        new_acl_buf.as_mut_ptr() as winapi::PACL,
                        2, // ACL_REVISION
                        winapi::MAXDWORD,
                        temp_acl,
                        (*(temp_acl as PaceHeader)).aceSize as winapi::DWORD,
                    ))?;
                }
            }

            cvt(AddAccessAllowedAceEx(
                new_acl_buf.as_mut_ptr() as winapi::PACL,
                2, // ACL_REVISION
                ace_flags,
                access_mask,
                self.raw.as_ptr() as winapi::PSID,
            ))?;

            cvt(SetSecurityDescriptorDacl(
                sd_new.as_mut_ptr() as winapi::PSECURITY_DESCRIPTOR,
                winapi::TRUE,
                new_acl_buf.as_mut_ptr() as winapi::PACL,
                winapi::FALSE,
            ))?;
            cvt(SetUserObjectSecurity(
                handle,
                &mut winapi::DACL_SECURITY_INFORMATION,
                sd_new.as_mut_ptr() as winapi::PSECURITY_DESCRIPTOR,
            ))?;

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
    fn current_user_sid() {
        assert!(Sid::from_current_user().is_ok())
    }
}
