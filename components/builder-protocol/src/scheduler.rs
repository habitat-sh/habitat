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

use std::result;
use message::Routable;

pub use message::scheduler::*;
use message::originsrv::OriginPackage;
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;

impl From<OriginPackage> for Package {
    fn from(value: OriginPackage) -> Package {
        let mut package = Package::new();

        let name = format!("{}", value.get_ident());

        let deps = value.get_deps().iter().map(|x| format!("{}", x)).collect();

        package.set_ident(name);
        package.set_deps(deps);
        package
    }
}

impl From<Package> for PackageCreate {
    fn from(value: Package) -> PackageCreate {
        let mut package = PackageCreate::new();

        let name = format!("{}", value.get_ident());

        let deps = value.get_deps().iter().map(|x| format!("{}", x)).collect();

        package.set_ident(name);
        package.set_deps(deps);
        package
    }
}

impl Into<Package> for PackagePreCreate {
    fn into(self) -> Package {
        let mut package = Package::new();

        let name = format!("{}", self.get_ident());

        let deps = self.get_deps().iter().map(|x| format!("{}", x)).collect();

        package.set_ident(name);
        package.set_deps(deps);
        package
    }
}

impl Routable for GroupCreate {
    type H = String;

    fn route_key(&self) -> Option<Self::H> {
        Some(format!("{}/{}", self.get_origin(), self.get_package()))
    }
}

impl Routable for GroupGet {
    type H = String;

    fn route_key(&self) -> Option<Self::H> {
        Some(self.get_group_id().to_string())
    }
}

impl Routable for PackageCreate {
    type H = String;

    fn route_key(&self) -> Option<Self::H> {
        Some(self.get_ident().to_string())
    }
}

impl Routable for PackagePreCreate {
    type H = String;

    fn route_key(&self) -> Option<Self::H> {
        Some(self.get_ident().to_string())
    }
}

impl Routable for PackageStatsGet {
    type H = String;

    fn route_key(&self) -> Option<Self::H> {
        Some(self.get_origin().to_string())
    }
}

impl Routable for JobStatus {
    type H = String;

    fn route_key(&self) -> Option<Self::H> {
        Some(self.get_job().get_id().to_string())
    }
}

impl Serialize for GroupState {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self as u64 {
            0 => serializer.serialize_str("Pending"),
            1 => serializer.serialize_str("Dispatching"),
            2 => serializer.serialize_str("Complete"),
            3 => serializer.serialize_str("Failed"),
            _ => panic!("Unexpected enum value"),
        }
    }
}

impl Serialize for ProjectState {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self as u64 {
            0 => serializer.serialize_str("NotStarted"),
            1 => serializer.serialize_str("InProgress"),
            2 => serializer.serialize_str("Success"),
            3 => serializer.serialize_str("Failure"),
            4 => serializer.serialize_str("Skipped"),
            _ => panic!("Unexpected enum value"),
        }
    }
}

impl Serialize for Project {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut strukt = try!(serializer.serialize_struct("project", 4));
        try!(strukt.serialize_field("name", &self.get_name()));
        try!(strukt.serialize_field("ident", &self.get_ident()));
        try!(strukt.serialize_field("state", &self.get_state()));
        try!(strukt.serialize_field("job_id", &self.get_job_id()));
        strukt.end()
    }
}

impl Serialize for Group {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut strukt = try!(serializer.serialize_struct("group", 3));
        try!(strukt.serialize_field("id", &self.get_id()));
        try!(strukt.serialize_field("state", &self.get_state()));
        try!(strukt.serialize_field("projects", &self.get_projects()));
        try!(strukt.serialize_field("created_at", &self.get_created_at()));
        strukt.end()
    }
}

impl Serialize for PackageStats {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut strukt = try!(serializer.serialize_struct("packagestats", 2));
        try!(strukt.serialize_field("plans", &self.get_plans()));
        try!(strukt.serialize_field("builds", &self.get_builds()));
        strukt.end()
    }
}
