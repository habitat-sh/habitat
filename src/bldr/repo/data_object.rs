// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

use std::collections::HashSet;
use std::fmt;
use std::str::FromStr;

use rustc_serialize::{Encoder, Decoder, Encodable, Decodable};

use error::{BldrResult, ErrorKind};
use package;
use super::data_store::ToMdbValue;

static LOGKEY: &'static str = "DO";

pub trait DataObject : Encodable + Decodable {
    type Key: ToMdbValue + fmt::Display;
    fn ident(&self) -> &Self::Key;
}

#[repr(C)]
#[derive(PartialEq, Debug, Clone)]
pub struct PackageIdent(String);

impl PackageIdent {
    pub fn new(ident: String) -> Self {
        PackageIdent(ident)
    }

    pub fn deriv_idx(&self) -> String {
        format!("{}", self.parts()[0])
    }

    pub fn name_idx(&self) -> String {
        let vec: Vec<&str> = self.parts();
        format!("{}/{}", vec[0], vec[1])
    }

    pub fn version_idx(&self) -> String {
        let vec: Vec<&str> = self.parts();
        format!("{}/{}/{}", vec[0], vec[1], vec[2])
    }

    pub fn parts(&self) -> Vec<&str> {
        self.0.split("/").collect()
    }
}

impl fmt::Display for PackageIdent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Encodable for PackageIdent {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        let p = self.parts();
        try!(s.emit_struct("PackageIdent", p.len(), |s| {
            try!(s.emit_struct_field("derivation", 0, |s| p[0].encode(s)));
            try!(s.emit_struct_field("name", 1, |s| p[1].encode(s)));
            if p.len() > 2 {
                try!(s.emit_struct_field("version", 2, |s| p[2].encode(s)));
            }
            if p.len() > 3 {
                try!(s.emit_struct_field("release", 3, |s| p[3].encode(s)));
            }
            Ok(())
        }));
        Ok(())
    }
}

impl Decodable for PackageIdent {
    fn decode<D: Decoder>(d: &mut D) -> Result<Self, D::Error> {
        d.read_struct("PackageIdent", 4, |d| {
            let derivation: String = try!(d.read_struct_field("derivation", 0, |d| Decodable::decode(d)));
            let name: String = try!(d.read_struct_field("name", 1, |d| Decodable::decode(d)));
            let version: String = try!(d.read_struct_field("version", 2, |d| Decodable::decode(d)));
            let release: String = try!(d.read_struct_field("release", 3, |d| Decodable::decode(d)));
            Ok(PackageIdent::new(format!("{}/{}/{}/{}", derivation, name, version, release)))
        })
    }
}

impl DataObject for PackageIdent {
    type Key = String;

    fn ident<'a>(&'a self) -> &'a String {
        &self.0
    }
}

impl Into<package::PackageIdent> for PackageIdent {
    fn into(self) -> package::PackageIdent {
        FromStr::from_str(&self.0).unwrap()
    }
}

#[repr(C)]
#[derive(RustcEncodable, RustcDecodable, PartialEq, Debug)]
pub struct View {
    pub ident: String,
    pub packages: HashSet<<Package as DataObject>::Key>,
}

impl View {
    pub fn new(name: &str) -> Self {
        View {
            ident: String::from(name),
            packages: HashSet::new(),
        }
    }

    pub fn add_package(&mut self, package: <Package as DataObject>::Key) -> &mut Self {
        self.packages.insert(package);
        self
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
    pub views: HashSet<<View as DataObject>::Key>,
}

impl Package {
    pub fn from_archive(archive: &package::PackageArchive) -> BldrResult<Self> {
        let ident = match archive.ident() {
            Ok(value) => {
                if !value.fully_qualified() {
                    return Err(bldr_error!(ErrorKind::InvalidPackageIdent(value.to_string())));
                }
                PackageIdent::new(value.to_string())
            }
            Err(e) => return Err(e),
        };
        Ok(Package {
            ident: ident,
            manifest: try!(archive.manifest()),
            deps: try!(archive.deps()).iter().map(|d| PackageIdent::new(d.to_string())).collect(),
            tdeps: try!(archive.tdeps()).iter().map(|d| PackageIdent::new(d.to_string())).collect(),
            exposes: try!(archive.exposes()),
            config: try!(archive.config()),
            views: HashSet::new(),
        })
    }

    pub fn add_view(&mut self, view: <View as DataObject>::Key) -> &mut Self {
        self.views.insert(view);
        self
    }
}

impl Into<package::Package> for Package {
    fn into(self) -> package::Package {
        let ident = self.ident.parts();
        package::Package {
            derivation: ident[0].to_string(),
            name: ident[1].to_string(),
            version: ident[2].to_string(),
            release: ident[3].to_string(),
            deps: self.deps.into_iter().map(|d| d.into()).collect(),
            tdeps: self.tdeps.into_iter().map(|d| d.into()).collect(),
        }
    }
}

impl DataObject for Package {
    type Key = String;

    fn ident<'a>(&'a self) -> &'a String {
        &self.ident.0
    }
}
