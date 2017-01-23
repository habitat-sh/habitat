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

// JW TODO: After updating to Rust 1.15, move the types contained in this module back into `lib.rs`

use std::string::ToString;

use protobuf::core::ProtobufEnum;

#[derive(Clone, Deserialize)]
pub struct OriginKeyIdent {
    pub origin: String,
    pub revision: String,
    pub location: String,
}

impl Into<depotsrv::OriginKeyIdent> for OriginKeyIdent {
    fn into(self) -> depotsrv::OriginKeyIdent {
        let mut out = depotsrv::OriginKeyIdent::new();
        out.set_origin(self.origin);
        out.set_revision(self.revision);
        out.set_location(self.location);
        out
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct OriginSecretKey {
    pub id: String,
    pub origin_id: String,
    pub name: String,
    pub revision: String,
    pub body: String,
    pub owner_id: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PackageResults<T> {
    pub range_start: isize,
    pub range_end: isize,
    pub total_count: isize,
    pub package_list: Vec<T>,
}

#[derive(Clone, Deserialize)]
pub struct Package {
    pub ident: PackageIdent,
    pub checksum: String,
    pub manifest: String,
    pub deps: Vec<PackageIdent>,
    pub tdeps: Vec<PackageIdent>,
    pub exposes: Vec<u32>,
    pub config: String,
}

impl Into<depotsrv::Package> for Package {
    fn into(self) -> depotsrv::Package {
        let mut out = depotsrv::Package::new();
        out.set_ident(self.ident.into());
        out.set_checksum(self.checksum);
        out.set_manifest(self.manifest);
        out.set_deps(self.deps.into_iter().map(|m| m.into()).collect());
        out.set_tdeps(self.tdeps.into_iter().map(|m| m.into()).collect());
        out.set_exposes(self.exposes);
        out.set_config(self.config);
        out
    }
}

#[derive(Clone, Deserialize)]
pub struct PackageIdent {
    pub origin: String,
    pub name: String,
    pub version: String,
    pub release: String,
}

impl Into<depotsrv::PackageIdent> for PackageIdent {
    fn into(self) -> depotsrv::PackageIdent {
        let mut out = depotsrv::PackageIdent::new();
        out.set_origin(self.origin);
        out.set_name(self.name);
        out.set_version(self.version);
        out.set_release(self.release);
        out
    }
}

#[derive(Clone, Deserialize)]
#[serde(rename = "error")]
pub struct NetError {
    pub code: i32,
    pub msg: String,
}

impl ToString for NetError {
    fn to_string(&self) -> String {
        let mut out = net::NetError::new();
        out.set_code(net::ErrCode::from_i32(self.code).unwrap());
        out.set_msg(self.msg.clone());
        out.to_string()
    }
}
