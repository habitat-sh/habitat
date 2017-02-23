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

use std::fmt;
use std::result;

use hab_core;
use hab_core::package::{self, Identifiable, FromArchive, PackageArchive};
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;

use message::Persistable;

pub use message::depotsrv::*;

impl Serialize for PackageIdent {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut strukt = try!(serializer.serialize_struct("package_ident", 4));
        try!(strukt.serialize_field("origin", self.get_origin()));
        try!(strukt.serialize_field("name", self.get_name()));
        if !self.get_version().is_empty() {
            try!(strukt.serialize_field("version", self.get_version()));
        }
        if !self.get_release().is_empty() {
            try!(strukt.serialize_field("release", self.get_release()));
        }
        strukt.end()
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
        if ver.is_empty() { None } else { Some(ver) }
    }

    fn release(&self) -> Option<&str> {
        let rel = self.get_ident().get_release();
        if rel.is_empty() { None } else { Some(rel) }
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
        if ver.is_empty() { None } else { Some(ver) }
    }

    fn release(&self) -> Option<&str> {
        let rel = self.get_release();
        if rel.is_empty() { None } else { Some(rel) }
    }
}

impl Serialize for OriginKeyIdent {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut strukt = try!(serializer.serialize_struct("origin_key", 3));
        try!(strukt.serialize_field("origin", self.get_origin()));
        try!(strukt.serialize_field("revision", self.get_revision()));
        try!(strukt.serialize_field("location", self.get_location()));
        strukt.end()
    }
}

impl Serialize for Package {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut strukt = try!(serializer.serialize_struct("package", 7));
        try!(strukt.serialize_field("ident", self.get_ident()));
        try!(strukt.serialize_field("checksum", self.get_checksum()));
        try!(strukt.serialize_field("manifest", self.get_manifest()));
        try!(strukt.serialize_field("deps", self.get_deps()));
        try!(strukt.serialize_field("tdeps", self.get_tdeps()));
        try!(strukt.serialize_field("exposes", self.get_exposes()));
        try!(strukt.serialize_field("config", self.get_config()));
        strukt.end()
    }
}
