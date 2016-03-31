// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

use std::fmt;
use std::result;
use std::slice;

use hcore::{package, Error, Result};
use libc::c_void;
use lmdb_sys;
use rustc_serialize::{Encoder, Decoder, Encodable, Decodable};

pub trait DataObject : Encodable + Decodable {
    type Key: ToMdbValue + FromMdbValue + fmt::Display;
    fn ident(&self) -> &Self::Key;
}

#[repr(C)]
#[derive(PartialEq, Debug, Clone)]
pub struct PackageIdent(package::PackageIdent, String);

impl PackageIdent {
    pub fn new(ident: package::PackageIdent) -> Self {
        let string_id = ident.to_string();
        PackageIdent(ident, string_id)
    }

    pub fn fully_qualified(&self) -> bool {
        self.0.fully_qualified()
    }

    pub fn len(&self) -> u8 {
        let mut len = 2;
        if self.0.version.is_some() {
            len += 1;
        }
        if self.0.release.is_some() {
            len += 1;
        }
        len
    }

    pub fn origin_idx(&self) -> String {
        format!("{}", self.0.origin)
    }

    pub fn name_idx(&self) -> String {
        format!("{}/{}", self.0.origin, self.0.name)
    }

    pub fn version_idx(&self) -> Option<String> {
        if self.0.version.is_some() {
            Some(format!("{}/{}/{}",
                         self.0.origin,
                         self.0.name,
                         self.0.version.as_ref().unwrap()))
        } else {
            None
        }
    }
}

impl fmt::Display for PackageIdent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<package::PackageIdent> for PackageIdent {
    fn as_ref(&self) -> &package::PackageIdent {
        &self.0
    }
}

impl AsRef<str> for PackageIdent {
    fn as_ref(&self) -> &str {
        &self.1
    }
}

impl Encodable for PackageIdent {
    fn encode<S: Encoder>(&self, s: &mut S) -> result::Result<(), S::Error> {
        try!(s.emit_struct("PackageIdent", self.len() as usize, |s| {
            try!(s.emit_struct_field("origin", 0, |s| self.0.origin.encode(s)));
            try!(s.emit_struct_field("name", 1, |s| self.0.name.encode(s)));
            if let Some(ref version) = self.0.version {
                try!(s.emit_struct_field("version", 2, |s| version.encode(s)));
            }
            if let Some(ref release) = self.0.release {
                try!(s.emit_struct_field("release", 3, |s| release.encode(s)));
            }
            Ok(())
        }));
        Ok(())
    }
}

impl Decodable for PackageIdent {
    fn decode<D: Decoder>(d: &mut D) -> result::Result<Self, D::Error> {
        d.read_struct("PackageIdent", 4, |d| {
            let origin: String = try!(d.read_struct_field("origin", 0, |d| Decodable::decode(d)));
            let name: String = try!(d.read_struct_field("name", 1, |d| Decodable::decode(d)));
            let version: String = try!(d.read_struct_field("version", 2, |d| Decodable::decode(d)));
            let release: String = try!(d.read_struct_field("release", 3, |d| Decodable::decode(d)));
            Ok(PackageIdent::new(package::PackageIdent::new(origin,
                                                            name,
                                                            Some(version),
                                                            Some(release))))
        })
    }
}

impl DataObject for PackageIdent {
    type Key = String;

    fn ident(&self) -> &String {
        &self.1
    }
}

impl Into<package::PackageIdent> for PackageIdent {
    fn into(self) -> package::PackageIdent {
        self.0
    }
}

impl From<package::PackageIdent> for PackageIdent {
    fn from(val: package::PackageIdent) -> PackageIdent {
        PackageIdent::new(val)
    }
}

#[repr(C)]
#[derive(RustcEncodable, RustcDecodable, PartialEq, Debug)]
pub struct View {
    pub ident: String,
}

impl View {
    pub fn new(name: &str) -> Self {
        View { ident: String::from(name) }
    }
}

impl DataObject for View {
    type Key = String;

    fn ident<'a>(&'a self) -> &'a String {
        &self.ident
    }
}

impl fmt::Display for View {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.ident)
    }
}

#[repr(C)]
#[derive(RustcEncodable, RustcDecodable, PartialEq, Debug)]
pub struct Package {
    pub ident: PackageIdent,
    pub manifest: String,
    pub deps: Vec<PackageIdent>,
    pub tdeps: Vec<PackageIdent>,
    pub exposes: Vec<u16>,
    pub config: Option<String>,
    pub checksum: String,
}

impl Package {
    pub fn from_archive(archive: &mut package::PackageArchive) -> Result<Self> {
        let ident = match archive.ident() {
            Ok(value) => {
                if !value.fully_qualified() {
                    return Err(Error::InvalidPackageIdent(value.to_string()));
                }
                PackageIdent::new(value)
            }
            Err(e) => return Err(Error::from(e)),
        };
        Ok(Package {
            ident: ident,
            manifest: try!(archive.manifest()),
            deps: try!(archive.deps()).into_iter().map(|d| d.into()).collect(),
            tdeps: try!(archive.tdeps()).into_iter().map(|d| d.into()).collect(),
            exposes: try!(archive.exposes()),
            config: try!(archive.config()),
            checksum: try!(archive.checksum()),
        })
    }
}

impl DataObject for Package {
    type Key = String;

    fn ident(&self) -> &String {
        &self.ident.1
    }
}

pub unsafe trait ToMdbValue {
    fn to_mdb_value(&self) -> lmdb_sys::MDB_val;
}

unsafe impl ToMdbValue for String {
    fn to_mdb_value(&self) -> lmdb_sys::MDB_val {
        let t: &str = self;
        lmdb_sys::MDB_val {
            mv_data: t.as_ptr() as *mut c_void,
            mv_size: t.len(),
        }
    }
}

pub unsafe trait FromMdbValue {
    unsafe fn from_mdb_value(value: &lmdb_sys::MDB_val) -> Self;
}

unsafe impl FromMdbValue for String {
    unsafe fn from_mdb_value(value: &lmdb_sys::MDB_val) -> Self {
        let bytes: Vec<u8> = slice::from_raw_parts(value.mv_data as *const u8, value.mv_size)
                                 .to_vec();
        String::from_utf8(bytes).unwrap()
    }
}
