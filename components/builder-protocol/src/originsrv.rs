// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

use std::cmp::{Eq, Ordering, PartialOrd};
use std::fmt;
use std::result;
use std::str::FromStr;

use hab_core;
use hab_core::package::{self, Identifiable, FromArchive, PackageArchive};
use hab_core::package::ident::version_sort;

use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;

pub use message::originsrv::*;
use message::Routable;
use sharding::InstaId;

pub trait Pageable {
    fn get_range(&self) -> [u64; 2];

    fn limit(&self) -> i64 {
        (self.get_range()[1] - self.get_range()[0] + 1) as i64
    }
}

impl Routable for AccountInvitationListRequest {
    type H = u64;

    fn route_key(&self) -> Option<Self::H> {
        Some(self.get_account_id())
    }
}

impl Routable for AccountInvitationListResponse {
    type H = u64;

    fn route_key(&self) -> Option<Self::H> {
        Some(self.get_account_id())
    }
}

impl Serialize for AccountInvitationListResponse {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut strukt = serializer.serialize_struct(
            "account_invitation_list_response",
            2,
        )?;
        strukt.serialize_field(
            "account_id",
            &self.get_account_id().to_string(),
        )?;
        strukt.serialize_field(
            "invitations",
            self.get_invitations(),
        )?;
        strukt.end()
    }
}

impl Routable for CheckOriginAccessRequest {
    type H = String;

    fn route_key(&self) -> Option<Self::H> {
        Some(self.get_origin_name().to_string())
    }
}

impl Serialize for Origin {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut strukt = serializer.serialize_struct("origin", 4)?;
        strukt.serialize_field("id", &self.get_id().to_string())?;
        strukt.serialize_field("name", self.get_name())?;
        strukt.serialize_field(
            "owner_id",
            &self.get_owner_id().to_string(),
        )?;
        strukt.serialize_field(
            "private_key_name",
            self.get_private_key_name(),
        )?;
        strukt.end()
    }
}

impl Serialize for OriginChannel {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut strukt = serializer.serialize_struct("origin", 4)?;
        strukt.serialize_field("id", &self.get_id())?;
        strukt.serialize_field("name", self.get_name())?;
        strukt.serialize_field("owner_id", &self.get_owner_id())?;
        strukt.end()
    }
}

impl Routable for OriginChannelCreate {
    type H = InstaId;

    fn route_key(&self) -> Option<Self::H> {
        Some(InstaId(self.get_origin_id()))
    }
}

impl Routable for OriginChannelGet {
    type H = String;

    fn route_key(&self) -> Option<Self::H> {
        Some(self.get_origin_name().to_string())
    }
}

impl Routable for OriginPackageChannelListRequest {
    type H = String;

    fn route_key(&self) -> Option<Self::H> {
        Some(String::from(self.get_ident().get_origin()))
    }
}

impl Serialize for OriginChannelIdent {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut strukt = serializer.serialize_struct("origin_key", 3)?;
        strukt.serialize_field("name", self.get_name())?;
        strukt.end()
    }
}

impl Routable for OriginChannelListRequest {
    type H = InstaId;

    fn route_key(&self) -> Option<Self::H> {
        Some(InstaId(self.get_origin_id()))
    }
}

impl Routable for OriginChannelListResponse {
    type H = InstaId;

    fn route_key(&self) -> Option<Self::H> {
        Some(InstaId(self.get_origin_id()))
    }
}

impl Serialize for OriginChannelListResponse {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut strukt = serializer.serialize_struct(
            "origin_channel_list_response",
            2,
        )?;
        strukt.serialize_field("channels", self.get_channels())?;
        strukt.end()
    }
}

impl Routable for OriginChannelPackageGet {
    type H = String;

    fn route_key(&self) -> Option<Self::H> {
        Some(self.get_ident().get_origin().to_string())
    }
}

impl Routable for OriginChannelPackageLatestGet {
    type H = String;

    fn route_key(&self) -> Option<Self::H> {
        Some(self.get_ident().get_origin().to_string())
    }
}

impl Pageable for OriginChannelPackageListRequest {
    fn get_range(&self) -> [u64; 2] {
        [self.get_start(), self.get_stop()]
    }
}

impl Routable for OriginChannelPackageListRequest {
    type H = String;

    fn route_key(&self) -> Option<Self::H> {
        Some(self.get_ident().get_origin().to_string())
    }
}

impl Routable for OriginChannelDelete {
    type H = InstaId;

    fn route_key(&self) -> Option<Self::H> {
        Some(InstaId(self.get_origin_id()))
    }
}


impl Routable for OriginCreate {
    type H = String;

    fn route_key(&self) -> Option<Self::H> {
        Some(self.get_name().to_string())
    }
}

impl Routable for OriginGet {
    type H = String;

    fn route_key(&self) -> Option<Self::H> {
        // JW TODO: This won't accurately find the origin without it. We can switch to using the ID
        // of the origin or perform a reverse lookup by storing the name->ID map on a particular
        // originsrv server.
        Some(self.get_name().to_string())
    }
}

impl Serialize for OriginInvitation {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut strukt = serializer.serialize_struct("origin_invitation", 6)?;
        strukt.serialize_field("id", &self.get_id().to_string())?;
        strukt.serialize_field(
            "account_id",
            &self.get_account_id().to_string(),
        )?;
        strukt.serialize_field(
            "account_name",
            self.get_account_name(),
        )?;
        strukt.serialize_field(
            "origin_id",
            &self.get_origin_id().to_string(),
        )?;
        strukt.serialize_field(
            "origin_name",
            self.get_origin_name(),
        )?;
        strukt.serialize_field(
            "owner_id",
            &self.get_owner_id().to_string(),
        )?;
        strukt.end()
    }
}

impl Routable for OriginInvitationAcceptRequest {
    type H = InstaId;

    fn route_key(&self) -> Option<Self::H> {
        Some(InstaId(self.get_invite_id()))
    }
}

impl Routable for OriginInvitationCreate {
    type H = InstaId;

    fn route_key(&self) -> Option<Self::H> {
        Some(InstaId(self.get_origin_id()))
    }
}

impl Routable for OriginInvitationListRequest {
    type H = InstaId;

    fn route_key(&self) -> Option<Self::H> {
        Some(InstaId(self.get_origin_id()))
    }
}

impl Routable for OriginInvitationListResponse {
    type H = InstaId;

    fn route_key(&self) -> Option<Self::H> {
        Some(InstaId(self.get_origin_id()))
    }
}

impl Serialize for OriginInvitationListResponse {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut strukt = serializer.serialize_struct(
            "origin_invitation_list_response",
            2,
        )?;
        strukt.serialize_field(
            "origin_id",
            &self.get_origin_id().to_string(),
        )?;
        strukt.serialize_field(
            "invitations",
            self.get_invitations(),
        )?;
        strukt.end()
    }
}

impl Serialize for OriginKeyIdent {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut strukt = serializer.serialize_struct("origin_key", 3)?;
        strukt.serialize_field("origin", self.get_origin())?;
        strukt.serialize_field("revision", self.get_revision())?;
        strukt.serialize_field("location", self.get_location())?;
        strukt.end()
    }
}

impl Routable for OriginMemberListRequest {
    type H = InstaId;

    fn route_key(&self) -> Option<Self::H> {
        Some(InstaId(self.get_origin_id()))
    }
}

impl Serialize for OriginMemberListResponse {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut strukt = serializer.serialize_struct(
            "origin_member_list_response",
            2,
        )?;
        strukt.serialize_field(
            "origin_id",
            &self.get_origin_id().to_string(),
        )?;
        strukt.serialize_field("members", self.get_members())?;
        strukt.end()
    }
}

impl Routable for OriginMemberRemove {
    type H = InstaId;

    fn route_key(&self) -> Option<Self::H> {
        Some(InstaId(self.get_origin_id()))
    }
}

impl fmt::Display for OriginPackage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.get_ident().fmt(f)
    }
}

impl FromArchive for OriginPackage {
    type Error = hab_core::Error;

    fn from_archive(archive: &mut PackageArchive) -> hab_core::Result<Self> {
        let ident = match archive.ident() {
            Ok(value) => OriginPackageIdent::from(value),
            Err(e) => return Err(hab_core::Error::from(e)),
        };
        let manifest = archive.manifest()?;
        let deps = archive.deps()?.into_iter().map(|d| d.into()).collect();
        let tdeps = archive.tdeps()?.into_iter().map(|d| d.into()).collect();
        let exposes = archive.exposes()?.into_iter().map(|d| d as u32).collect();
        let config = archive.config()?;
        let checksum = archive.checksum()?;
        let target = archive.target()?;

        let mut package = OriginPackage::new();
        package.set_ident(ident);
        package.set_manifest(manifest);
        package.set_target(target.to_string());
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

impl Into<package::PackageIdent> for OriginPackage {
    fn into(self) -> package::PackageIdent {
        self.get_ident().clone().into()
    }
}

impl Serialize for OriginPackage {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut strukt = serializer.serialize_struct("origin_package", 8)?;
        strukt.serialize_field("ident", self.get_ident())?;
        strukt.serialize_field("checksum", self.get_checksum())?;
        strukt.serialize_field("manifest", self.get_manifest())?;
        strukt.serialize_field("target", self.get_target())?;
        strukt.serialize_field("deps", self.get_deps())?;
        strukt.serialize_field("tdeps", self.get_tdeps())?;
        strukt.serialize_field("exposes", self.get_exposes())?;
        strukt.serialize_field("config", self.get_config())?;
        strukt.end()
    }
}

impl FromArchive for OriginPackageCreate {
    type Error = hab_core::Error;

    fn from_archive(archive: &mut PackageArchive) -> hab_core::Result<Self> {
        let ident = match archive.ident() {
            Ok(value) => OriginPackageIdent::from(value),
            Err(e) => return Err(hab_core::Error::from(e)),
        };
        let manifest = archive.manifest()?;
        let deps = archive.deps()?.into_iter().map(|d| d.into()).collect();
        let tdeps = archive.tdeps()?.into_iter().map(|d| d.into()).collect();
        let exposes = archive.exposes()?.into_iter().map(|d| d as u32).collect();
        let config = archive.config()?;
        let checksum = archive.checksum()?;
        let target = archive.target()?;

        let mut package = OriginPackageCreate::new();
        package.set_ident(ident);
        package.set_manifest(manifest);
        package.set_target(target.to_string());
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

impl Routable for OriginPackageCreate {
    type H = InstaId;

    fn route_key(&self) -> Option<Self::H> {
        Some(InstaId(self.get_origin_id()))
    }
}

impl Routable for OriginPackageGet {
    type H = String;

    fn route_key(&self) -> Option<Self::H> {
        Some(String::from(self.get_ident().get_origin()))
    }
}

impl fmt::Display for OriginPackageIdent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if !self.get_version().is_empty() && !self.get_release().is_empty() {
            write!(
                f,
                "{}/{}/{}/{}",
                self.get_origin(),
                self.get_name(),
                self.get_version(),
                self.get_release()
            )
        } else if !self.get_version().is_empty() {
            write!(
                f,
                "{}/{}/{}",
                self.get_origin(),
                self.get_name(),
                self.get_version()
            )
        } else {
            write!(f, "{}/{}", self.get_origin(), self.get_name())
        }
    }
}

impl From<hab_core::package::PackageIdent> for OriginPackageIdent {
    fn from(value: hab_core::package::PackageIdent) -> OriginPackageIdent {
        let mut ident = OriginPackageIdent::new();
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

impl FromStr for OriginPackageIdent {
    type Err = hab_core::Error;

    fn from_str(value: &str) -> result::Result<Self, Self::Err> {
        let mut parts = value.split("/");
        let mut ident = OriginPackageIdent::new();
        if let Some(part) = parts.next() {
            if part.len() > 0 {
                ident.set_origin(part.to_string());
            }
        }
        if let Some(part) = parts.next() {
            if part.len() > 0 {
                ident.set_name(part.to_string());
            }
        }
        if let Some(part) = parts.next() {
            if part.len() > 0 {
                ident.set_version(part.to_string());
            }
        }
        if let Some(part) = parts.next() {
            if part.len() > 0 {
                ident.set_release(part.to_string());
            }
        }
        Ok(ident)
    }
}

impl Identifiable for OriginPackageIdent {
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

impl Into<package::PackageIdent> for OriginPackageIdent {
    fn into(self) -> package::PackageIdent {
        package::PackageIdent::new(
            self.get_origin(),
            self.get_name(),
            Some(self.get_version()),
            Some(self.get_release()),
        )
    }
}

impl PartialOrd for OriginPackageIdent {
    fn partial_cmp(&self, other: &OriginPackageIdent) -> Option<Ordering> {
        if self.get_name() != other.get_name() {
            return None;
        }
        if self.get_version() == "" && other.get_version() == "" {
            return None;
        }
        if self.get_version() == "" && other.get_version() != "" {
            return Some(Ordering::Less);
        }
        if self.get_version() != "" && other.get_version() == "" {
            return Some(Ordering::Greater);
        }
        if self.get_release() == "" && other.get_release() == "" {
            return None;
        }
        if self.get_release() == "" && other.get_release() != "" {
            return Some(Ordering::Less);
        }
        if self.get_release() != "" && other.get_release() == "" {
            return Some(Ordering::Greater);
        }
        match version_sort(self.get_version(), other.get_version()) {
            ord @ Ok(Ordering::Greater) |
            ord @ Ok(Ordering::Less) => ord.ok(),
            Ok(Ordering::Equal) => Some(self.get_release().cmp(&other.get_release())),
            Err(_) => {
                match self.get_version().cmp(other.get_version()) {
                    ord @ Ordering::Greater |
                    ord @ Ordering::Less => Some(ord),
                    Ordering::Equal => Some(self.get_release().cmp(&other.get_release())),
                }
            }
        }
    }
}

impl Ord for OriginPackageIdent {
    fn cmp(&self, other: &OriginPackageIdent) -> Ordering {
        // We purposely want core to show up first, before all other origins, so we rig the sorting
        if self.get_origin() == "core" && other.get_origin() != "core" {
            return Ordering::Less;
        } else if other.get_origin() == "core" && self.get_origin() != "core" {
            return Ordering::Greater;
        } else {
            if self.get_origin() != other.get_origin() {
                return self.get_origin().cmp(&other.get_origin());
            }
        }

        if self.get_name() != other.get_name() {
            return self.get_name().cmp(&other.get_name());
        }

        match version_sort(self.get_version(), other.get_version()) {
            ord @ Ok(Ordering::Greater) |
            ord @ Ok(Ordering::Less) => ord.unwrap(),
            Ok(Ordering::Equal) => self.get_release().cmp(&other.get_release()),
            Err(_) => Ordering::Less,
        }
    }
}

impl Eq for OriginPackageIdent {}

impl Serialize for OriginPackageIdent {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut strukt = serializer.serialize_struct("origin_package_ident", 4)?;
        strukt.serialize_field("origin", self.get_origin())?;
        strukt.serialize_field("name", self.get_name())?;
        if !self.get_version().is_empty() {
            strukt.serialize_field("version", self.get_version())?;
        }
        if !self.get_release().is_empty() {
            strukt.serialize_field("release", self.get_release())?;
        }
        strukt.end()
    }
}

impl Serialize for OriginPackageVersion {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut strukt = serializer.serialize_struct("origin_package_version", 4)?;
        strukt.serialize_field("origin", self.get_origin())?;
        strukt.serialize_field("name", self.get_name())?;
        strukt.serialize_field("version", self.get_version())?;
        strukt.serialize_field(
            "release_count",
            &self.get_release_count().to_string(),
        )?;
        strukt.serialize_field("latest", &self.get_latest())?;
        strukt.end()
    }
}

impl Routable for OriginPackageLatestGet {
    type H = String;

    fn route_key(&self) -> Option<Self::H> {
        Some(String::from(self.get_ident().get_origin()))
    }
}

impl Pageable for OriginPackageListRequest {
    fn get_range(&self) -> [u64; 2] {
        [self.get_start(), self.get_stop()]
    }
}

impl Routable for OriginPackageListRequest {
    type H = String;

    fn route_key(&self) -> Option<Self::H> {
        Some(String::from(self.get_ident().get_origin()))
    }
}

impl Routable for OriginPackageVersionListRequest {
    type H = String;

    fn route_key(&self) -> Option<Self::H> {
        Some(String::from(self.get_origin()))
    }
}

impl Routable for OriginPackageGroupPromote {
    type H = String;

    fn route_key(&self) -> Option<Self::H> {
        Some(self.get_origin().to_string())
    }
}

impl Routable for OriginPackagePromote {
    type H = String;

    fn route_key(&self) -> Option<Self::H> {
        Some(String::from(self.get_ident().get_origin()))
    }
}

impl Routable for OriginPackageDemote {
    type H = String;

    fn route_key(&self) -> Option<Self::H> {
        Some(String::from(self.get_ident().get_origin()))
    }
}

impl Pageable for OriginPackageSearchRequest {
    fn get_range(&self) -> [u64; 2] {
        [self.get_start(), self.get_stop()]
    }
}

impl Routable for OriginPackageSearchRequest {
    type H = String;

    fn route_key(&self) -> Option<Self::H> {
        Some(String::from(self.get_origin()))
    }
}

impl Pageable for OriginPackageUniqueListRequest {
    fn get_range(&self) -> [u64; 2] {
        [self.get_start(), self.get_stop()]
    }
}

impl Routable for OriginPackageUniqueListRequest {
    type H = String;

    fn route_key(&self) -> Option<Self::H> {
        Some(String::from(self.get_origin()))
    }
}

impl Serialize for OriginProject {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("project", 2)?;
        state.serialize_field("id", &self.get_id().to_string())?;
        state.serialize_field(
            "origin_id",
            &self.get_origin_id().to_string(),
        )?;
        state.serialize_field("origin_name", self.get_origin_name())?;
        state.serialize_field(
            "package_name",
            self.get_package_name(),
        )?;
        state.serialize_field("name", self.get_name())?;
        state.serialize_field("plan_path", self.get_plan_path())?;
        state.serialize_field(
            "owner_id",
            &self.get_owner_id().to_string(),
        )?;
        state.serialize_field("vcs_type", self.get_vcs_type())?;
        state.serialize_field("vcs_data", self.get_vcs_data())?;
        state.end()
    }
}

impl Routable for OriginProjectCreate {
    type H = InstaId;

    fn route_key(&self) -> Option<Self::H> {
        Some(InstaId(self.get_project().get_origin_id()))
    }
}

impl Routable for OriginProjectDelete {
    type H = String;

    fn route_key(&self) -> Option<Self::H> {
        let name = self.get_name();
        let origin_name = match name.split('/').nth(0) {
            Some(origin_name) => origin_name,
            None => {
                println!(
                    "Cannot route origin project get; malformed project name - routing on \
                        screwedup to not kill the service"
                );
                "screwedup"
            }
        };
        Some(String::from(origin_name))
    }
}

impl Routable for OriginProjectGet {
    type H = String;

    fn route_key(&self) -> Option<Self::H> {
        let name = self.get_name();
        let origin_name = match name.split('/').nth(0) {
            Some(origin_name) => origin_name,
            None => {
                println!(
                    "Cannot route origin project get; malformed project name - routing on \
                        screwedup to not kill the service"
                );
                "screwedup"
            }
        };
        Some(String::from(origin_name))
    }
}

impl Routable for OriginProjectUpdate {
    type H = InstaId;

    fn route_key(&self) -> Option<Self::H> {
        Some(InstaId(self.get_project().get_origin_id()))
    }
}

impl Serialize for OriginPublicKey {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut strukt = serializer.serialize_struct("origin_public_key", 6)?;
        strukt.serialize_field("id", &self.get_id().to_string())?;
        strukt.serialize_field(
            "origin_id",
            &self.get_origin_id().to_string(),
        )?;
        strukt.serialize_field("name", self.get_name())?;
        strukt.serialize_field("revision", self.get_revision())?;
        strukt.serialize_field("body", self.get_body())?;
        strukt.serialize_field(
            "owner_id",
            &self.get_owner_id().to_string(),
        )?;
        strukt.end()
    }
}

impl Routable for OriginPublicKeyCreate {
    type H = InstaId;

    fn route_key(&self) -> Option<Self::H> {
        Some(InstaId(self.get_origin_id()))
    }
}

impl Routable for OriginPublicKeyGet {
    type H = String;

    fn route_key(&self) -> Option<Self::H> {
        Some(self.get_origin().to_string())
    }
}

impl Routable for OriginPublicKeyLatestGet {
    type H = String;

    fn route_key(&self) -> Option<Self::H> {
        Some(self.get_origin().to_string())
    }
}

impl Routable for OriginPublicKeyListRequest {
    type H = InstaId;

    fn route_key(&self) -> Option<Self::H> {
        Some(InstaId(self.get_origin_id()))
    }
}

impl Routable for OriginPublicKeyListResponse {
    type H = InstaId;

    fn route_key(&self) -> Option<Self::H> {
        Some(InstaId(self.get_origin_id()))
    }
}

impl Serialize for OriginPublicKeyListResponse {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut strukt = serializer.serialize_struct(
            "origin_public_key_list_response",
            2,
        )?;
        strukt.serialize_field(
            "origin_id",
            &self.get_origin_id().to_string(),
        )?;
        strukt.serialize_field("keys", self.get_keys())?;
        strukt.end()
    }
}

impl Serialize for OriginSecretKey {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut strukt = serializer.serialize_struct("origin_secret_key", 6)?;
        strukt.serialize_field("id", &self.get_id().to_string())?;
        strukt.serialize_field(
            "origin_id",
            &self.get_origin_id().to_string(),
        )?;
        strukt.serialize_field("name", self.get_name())?;
        strukt.serialize_field("revision", self.get_revision())?;
        strukt.serialize_field("body", self.get_body())?;
        strukt.serialize_field(
            "owner_id",
            &self.get_owner_id().to_string(),
        )?;
        strukt.end()
    }
}

impl Routable for OriginSecretKeyCreate {
    type H = InstaId;

    fn route_key(&self) -> Option<Self::H> {
        Some(InstaId(self.get_origin_id()))
    }
}

impl Routable for OriginSecretKeyGet {
    type H = String;

    fn route_key(&self) -> Option<Self::H> {
        Some(String::from(self.get_origin()))
    }
}
