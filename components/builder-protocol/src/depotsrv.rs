// Copyright (c) 2016 Chef Software Inc. and/or applicable contributors
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

use std::collections::BTreeMap;
use std::fmt;
use std::result;

use hab_core;
use hab_core::package::{self, Identifiable, FromArchive, PackageArchive};
use rustc_serialize::{Decoder, Decodable, Encoder, Encodable};
use rustc_serialize::json::{Json, ToJson};
use protobuf;

use message::Persistable;

pub use message::depotsrv::*;

impl Decodable for OriginKeyIdent {
    fn decode<D: Decoder>(d: &mut D) -> result::Result<Self, D::Error> {
        d.read_struct("OriginKeyIdent", 3, |d| {
            let mut ident = OriginKeyIdent::new();
            ident.set_origin(try!(d.read_struct_field("origin", 0, |d| Decodable::decode(d))));
            ident.set_revision(try!(d.read_struct_field("revision", 1, |d| Decodable::decode(d))));
            ident.set_location(try!(d.read_struct_field("location", 2, |d| Decodable::decode(d))));
            Ok(ident)
        })
    }
}

impl Decodable for Package {
    fn decode<D: Decoder>(d: &mut D) -> result::Result<Self, D::Error> {
        d.read_struct("Package", 7, |d| {
            let mut package = Package::new();
            package.set_ident(try!(d.read_struct_field("ident", 0, |d| Decodable::decode(d))));
            package.set_checksum(try!(d.read_struct_field("checksum", 1, |d| Decodable::decode(d))));
            package.set_manifest(try!(d.read_struct_field("manifest", 2, |d| Decodable::decode(d))));
            let deps: Vec<PackageIdent> = try!(d.read_struct_field("deps", 3, |d| Decodable::decode(d)));
            package.set_deps(protobuf::RepeatedField::from_vec(deps));
            let tdeps: Vec<PackageIdent> = try!(d.read_struct_field("tdeps", 4, |d| Decodable::decode(d)));
            package.set_tdeps(protobuf::RepeatedField::from_vec(tdeps));
            package.set_exposes(try!(d.read_struct_field("exposes", 5, |d| Decodable::decode(d))));
            if let Some(cfg) = try!(d.read_struct_field("config", 6, |d| Ok(Decodable::decode(d).ok()))) {
                package.set_config(cfg);
            }
            Ok(package)
        })
    }
}

impl Decodable for PackageIdent {
    fn decode<D: Decoder>(d: &mut D) -> result::Result<Self, D::Error> {
        d.read_struct("PackageIdent", 4, |d| {
            let mut ident = PackageIdent::new();
            ident.set_origin(try!(d.read_struct_field("origin", 0, |d| Decodable::decode(d))));
            ident.set_name(try!(d.read_struct_field("name", 1, |d| Decodable::decode(d))));
            ident.set_version(try!(d.read_struct_field("version", 2, |d| Decodable::decode(d))));
            ident.set_release(try!(d.read_struct_field("release", 3, |d| Decodable::decode(d))));
            Ok(ident)
        })
    }
}

impl Encodable for PackageIdent {
    fn encode<S: Encoder>(&self, s: &mut S) -> result::Result<(), S::Error> {
        try!(s.emit_struct("PackageIdent", 4, |s| {
            try!(s.emit_struct_field("origin", 0, |s| self.get_origin().encode(s)));
            try!(s.emit_struct_field("name", 1, |s| self.get_name().encode(s)));
            if !self.get_version().is_empty() {
                try!(s.emit_struct_field("version", 2, |s| self.get_version().encode(s)));
            }
            if !self.get_release().is_empty() {
                try!(s.emit_struct_field("release", 3, |s| self.get_release().encode(s)));
            }
            Ok(())
        }));
        Ok(())
    }
}

impl Into<package::PackageIdent> for PackageIdent {
    fn into(self) -> package::PackageIdent {
        package::PackageIdent::new(self.get_origin(),
                                   self.get_name(),
                                   Some(self.get_version()),
                                   Some(self.get_release()))
    }
}

impl Into<package::PackageIdent> for Package {
    fn into(self) -> package::PackageIdent {
        self.get_ident().clone().into()
    }
}

impl fmt::Display for PackageIdent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if !self.get_version().is_empty() && !self.get_release().is_empty() {
            write!(f,
                   "{}/{}/{}/{}",
                   self.get_origin(),
                   self.get_name(),
                   self.get_version(),
                   self.get_release())
        } else if !self.get_version().is_empty() {
            write!(f,
                   "{}/{}/{}",
                   self.get_origin(),
                   self.get_name(),
                   self.get_version())
        } else {
            write!(f, "{}/{}", self.get_origin(), self.get_name())
        }
    }
}

impl fmt::Display for Package {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.get_ident().fmt(f)
    }
}

impl Persistable for Package {
    type Key = PackageIdent;

    fn primary_key(&self) -> Self::Key {
        self.get_ident().clone()
    }

    fn set_primary_key(&mut self, value: Self::Key) {
        self.set_ident(value);
    }
}

impl Persistable for View {
    type Key = String;

    fn primary_key(&self) -> Self::Key {
        self.get_name().to_string()
    }

    fn set_primary_key(&mut self, value: Self::Key) {
        self.set_name(value);
    }
}

impl FromArchive for Package {
    type Error = hab_core::Error;

    fn from_archive(archive: &mut PackageArchive) -> hab_core::Result<Self> {
        let ident = match archive.ident() {
            Ok(value) => PackageIdent::from(value),
            Err(e) => return Err(hab_core::Error::from(e)),
        };
        let manifest = try!(archive.manifest());
        let deps = try!(archive.deps()).into_iter().map(|d| d.into()).collect();
        let tdeps = try!(archive.tdeps()).into_iter().map(|d| d.into()).collect();
        let exposes = try!(archive.exposes()).into_iter().map(|d| d as u32).collect();
        let config = try!(archive.config());
        let checksum = try!(archive.checksum());

        let mut package = Package::new();
        package.set_ident(ident);
        package.set_manifest(manifest);
        package.set_deps(deps);
        package.set_tdeps(tdeps);
        package.set_exposes(exposes);
        if let Some(cfg) = config {
            package.set_config(cfg);
        }
        package.set_checksum(checksum);
        Ok(package)
    }
}

impl From<hab_core::package::PackageIdent> for PackageIdent {
    fn from(value: hab_core::package::PackageIdent) -> PackageIdent {
        let mut ident = PackageIdent::new();
        ident.set_origin(value.origin);
        ident.set_name(value.name);
        if let Some(ver) = value.version {
            ident.set_version(ver);
        }
        if let Some(rel) = value.release {
            ident.set_release(rel);
        }
        ident
    }
}

impl Identifiable for Package {
    fn origin(&self) -> &str {
        self.get_ident().get_origin()
    }

    fn name(&self) -> &str {
        self.get_ident().get_name()
    }

    fn version(&self) -> Option<&str> {
        let ver = self.get_ident().get_version();
        if ver.is_empty() {
            None
        } else {
            Some(ver)
        }
    }

    fn release(&self) -> Option<&str> {
        let rel = self.get_ident().get_release();
        if rel.is_empty() {
            None
        } else {
            Some(rel)
        }
    }
}

impl Identifiable for PackageIdent {
    fn origin(&self) -> &str {
        self.get_origin()
    }

    fn name(&self) -> &str {
        self.get_name()
    }

    fn version(&self) -> Option<&str> {
        let ver = self.get_version();
        if ver.is_empty() {
            None
        } else {
            Some(ver)
        }
    }

    fn release(&self) -> Option<&str> {
        let rel = self.get_release();
        if rel.is_empty() {
            None
        } else {
            Some(rel)
        }
    }
}

impl ToJson for OriginKeyIdent {
    fn to_json(&self) -> Json {
        let mut m = BTreeMap::new();
        m.insert("origin".to_string(), self.get_origin().to_json());
        m.insert("revision".to_string(), self.get_revision().to_json());
        m.insert("location".to_string(), self.get_location().to_json());
        Json::Object(m)
    }
}

impl ToJson for PackageIdent {
    fn to_json(&self) -> Json {
        let mut m = BTreeMap::new();
        let ver = self.get_version();
        let rel = self.get_release();
        m.insert("origin".to_string(), self.get_origin().to_json());
        m.insert("name".to_string(), self.get_name().to_json());
        if !ver.is_empty() {
            m.insert("version".to_string(), ver.to_json());
        }
        if !rel.is_empty() {
            m.insert("release".to_string(), rel.to_json());
        }
        Json::Object(m)
    }
}

impl ToJson for Package {
    fn to_json(&self) -> Json {
        let mut m = BTreeMap::new();
        m.insert("ident".to_string(), self.get_ident().to_json());
        m.insert("checksum".to_string(), self.get_checksum().to_json());
        m.insert("manifest".to_string(), self.get_manifest().to_json());
        m.insert("deps".to_string(), self.get_deps().to_vec().to_json());
        m.insert("tdeps".to_string(), self.get_tdeps().to_vec().to_json());
        m.insert("exposes".to_string(), self.get_exposes().to_json());
        m.insert("config".to_string(), self.get_config().to_json());
        Json::Object(m)
    }
}
