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

#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate chrono;
extern crate habitat_builder_protocol as protocol;
extern crate habitat_core as hab_core;
extern crate habitat_http_client as hab_http;
extern crate broadcast;
#[macro_use]
extern crate hyper;
extern crate hyper_openssl;
#[macro_use]
extern crate log;
extern crate pbr;
extern crate protobuf;
extern crate rand;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[allow(unused_imports)]
#[macro_use]
extern crate serde_json;
extern crate tee;
extern crate url;

pub mod error;
pub use error::{Error, Result};

use std::fmt;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::string::ToString;

use broadcast::BroadcastWriter;
use chrono::DateTime;
use hab_core::package::{Identifiable, PackageArchive};
use hab_http::ApiClient;
use hab_http::util::decoded_response;
use hyper::client::{Body, IntoUrl, Response, RequestBuilder};
use hyper::status::StatusCode;
use hyper::header::{Accept, Authorization, Bearer, ContentType};
use hyper::Url;
use protobuf::core::ProtobufEnum;
use protocol::{originsrv, net};
use rand::{Rng, thread_rng};
use tee::TeeReader;
use url::percent_encoding::{percent_encode, PATH_SEGMENT_ENCODE_SET};

header! { (XFileName, "X-Filename") => [String] }
header! { (ETag, "ETag") => [String] }

const DEFAULT_API_PATH: &'static str = "/v1";

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

#[derive(Clone, Deserialize)]
pub struct OriginKeyIdent {
    pub origin: String,
    pub revision: String,
    pub location: String,
}

#[derive(Clone, Default, Deserialize)]
pub struct Project {
    pub name: String,
    pub ident: String,
    pub state: String,
    pub job_id: String,
}

#[derive(Clone, Deserialize)]
pub struct OriginSecret {
    pub id: String,
    pub origin_id: String,
    pub name: String,
    pub value: String,
}

impl fmt::Display for Project {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = format!("{:50} {}", self.ident, self.state);

        if let Ok(j) = self.job_id.parse::<i64>() {
            if j > 0 {
                let job_ids = format!(" (Job ID {})", self.job_id);
                s = s + &job_ids;
            }
        }

        write!(f, "{}", s)
    }
}

#[derive(Default, Deserialize)]
pub struct SchedulerResponse {
    pub id: String,
    pub state: String,
    pub projects: Vec<Project>,
    pub created_at: String,
    pub project_name: String,
}

impl fmt::Display for SchedulerResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = Vec::new();
        output.push(format!(
            "Status for Job Group {} ({}): {}",
            self.id,
            self.project_name,
            self.state
        ));

        if let Ok(c) = DateTime::parse_from_rfc3339(&self.created_at) {
            output.push(format!("Created at: {}", c.to_string()));
        }

        if self.projects.len() > 0 {
            output.push("".to_string());
            output.push(format!("Reverse dependencies:"));
            let mut projects = self.projects.clone();
            projects.sort_by(|a, b| a.ident.cmp(&b.ident));

            for project in projects.iter() {
                output.push(project.to_string())
            }
        }

        write!(f, "{}", output.join("\n"))
    }
}

impl Into<originsrv::OriginKeyIdent> for OriginKeyIdent {
    fn into(self) -> originsrv::OriginKeyIdent {
        let mut out = originsrv::OriginKeyIdent::new();
        out.set_origin(self.origin);
        out.set_revision(self.revision);
        out.set_location(self.location);
        out
    }
}

impl Into<originsrv::OriginSecret> for OriginSecret {
    fn into(self) -> originsrv::OriginSecret {
        let mut out = originsrv::OriginSecret::new();
        out.set_id(self.id.parse::<u64>().unwrap());
        out.set_origin_id(self.origin_id.parse::<u64>().unwrap());
        out.set_name(self.name);
        out.set_value(self.value);
        out
    }
}

/// Custom conversion logic to allow `serde` to successfully
/// round-trip `u64` datatypes through JSON serialization.
///
/// To use it, add `#[serde(with = "json_u64")]` to any `u64`-typed struct
/// fields.
mod json_u64 {
    use serde::{self, Deserialize, Serializer, Deserializer};

    pub fn serialize<S>(num: &u64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", num);
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<u64, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse::<u64>().map_err(serde::de::Error::custom)
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct OriginPrivateSigningKey {
    #[serde(with = "json_u64")]
    pub id: u64,
    #[serde(with = "json_u64")]
    pub origin_id: u64,
    pub name: String,
    pub revision: String,
    pub body: Vec<u8>,
    #[serde(with = "json_u64")]
    pub owner_id: u64,
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

impl Into<originsrv::OriginPackage> for Package {
    fn into(self) -> originsrv::OriginPackage {
        let mut out = originsrv::OriginPackage::new();
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

impl Into<originsrv::OriginPackageIdent> for PackageIdent {
    fn into(self) -> originsrv::OriginPackageIdent {
        let mut out = originsrv::OriginPackageIdent::new();
        out.set_origin(self.origin);
        out.set_name(self.name);
        out.set_version(self.version);
        out.set_release(self.release);
        out
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PackageResults<T> {
    pub range_start: isize,
    pub range_end: isize,
    pub total_count: isize,
    pub data: Vec<T>,
}

#[derive(Clone, Deserialize)]
pub struct OriginChannelIdent {
    pub name: String,
}

impl Into<originsrv::OriginChannelIdent> for OriginChannelIdent {
    fn into(self) -> originsrv::OriginChannelIdent {
        let mut out = originsrv::OriginChannelIdent::new();
        out.set_name(self.name);
        out
    }
}

pub trait DisplayProgress: Write {
    fn size(&mut self, size: u64);
    fn finish(&mut self);
}

pub struct Client(ApiClient);

impl Client {
    pub fn new<U>(
        endpoint: U,
        product: &str,
        version: &str,
        fs_root_path: Option<&Path>,
    ) -> Result<Self>
    where
        U: IntoUrl,
    {
        let mut endpoint = endpoint.into_url()?;
        if !endpoint.cannot_be_a_base() && endpoint.path() == "/" {
            endpoint.set_path(DEFAULT_API_PATH);
        }
        Ok(Client(
            ApiClient::new(endpoint, product, version, fs_root_path)?,
        ))
    }

    /// Retrieves the status of every group job in an origin
    ///
    /// # Failures
    ///
    /// * Remote Builder is not available
    pub fn get_origin_schedule(&self, origin: &str) -> Result<String> {
        debug!(
            "Retrieving schedule for all job groups in the {} origin",
            origin
        );

        let path = format!("depot/pkgs/schedule/{}/status", origin);
        let res = self.0.get(&path).send()?;

        if res.status != StatusCode::Ok {
            return Err(err_from_response(res));
        }

        let sr: Vec<SchedulerResponse> = decoded_response(res)?;
        let mut resp = Vec::new();

        for s in sr.iter() {
            resp.push(s.to_string());
            resp.push("".to_string());
            resp.push("-------------------------------------".to_string());
            resp.push("".to_string());
        }

        Ok(resp.join("\n"))
    }

    /// Retrieves the status of a group job
    ///
    /// # Failures
    ///
    /// * Remote Builder is not available
    pub fn get_schedule(&self, group_id: i64) -> Result<SchedulerResponse> {
        debug!("Retrieving schedule for job group {}", group_id);

        let path = format!("depot/pkgs/schedule/{}", group_id);
        let res = self.0.get(&path).send()?;

        if res.status != StatusCode::Ok {
            return Err(err_from_response(res));
        }

        let sr: SchedulerResponse = decoded_response(res)?;
        Ok(sr)
    }

    /// Schedules a job for a package ident
    ///
    /// # Failures
    ///
    /// * Key cannot be found
    /// * Remote Builder is not available
    pub fn schedule_job<I>(&self, ident: &I, package_only: bool, token: &str) -> Result<(String)>
    where
        I: Identifiable,
    {
        // TODO (SA): This API needs to be extended to support a target param.
        let path = format!("depot/pkgs/schedule/{}/{}", ident.origin(), ident.name());
        let result = if package_only {
            let custom =
                |url: &mut Url| { url.query_pairs_mut().append_pair("package_only", "true"); };
            self.add_authz(self.0.post_with_custom_url(&path, custom), token)
                .send()
        } else {
            self.add_authz(self.0.post(&path), token).send()
        };
        match result {
            Ok(response) => {
                if response.status == StatusCode::Ok {
                    let sr: SchedulerResponse = decoded_response(response)?;
                    Ok(sr.id)
                } else {
                    Err(err_from_response(response))
                }
            }
            Err(e) => Err(Error::from(e)),
        }
    }

    /// Download a public encryption key from a remote Builder to the given filepath.
    ///
    /// # Failures
    ///
    /// * Key cannot be found
    /// * Remote Builder is not available
    /// * File cannot be created and written to
    pub fn fetch_origin_public_encryption_key<D, P: ?Sized>(
        &self,
        origin: &str,
        token: &str,
        dst_path: &P,
        progress: Option<D>,
    ) -> Result<PathBuf>
    where
        P: AsRef<Path>,
        D: DisplayProgress + Sized,
    {
        self.download(
            &format!("depot/origins/{}/encryption_key", origin),
            dst_path.as_ref(),
            Some(token),
            progress,
        )
    }

    /// Create secret for an origin
    ///
    /// # Failures
    ///
    /// * Remote Builder is not available
    pub fn create_origin_secret(
        &self,
        origin: &str,
        token: &str,
        key: &str,
        secret: &str,
    ) -> Result<()> {
        let path = format!("depot/origins/{}/secret", origin);
        let body = json!({
            "name": key,
            "value": secret
        });

        let sbody = serde_json::to_string(&body)?;
        let res = self.add_authz(self.0.post(&path), token)
            .body(&sbody)
            .header(Accept::json())
            .header(ContentType::json())
            .send()?;

        if res.status != StatusCode::Created {
            return Err(err_from_response(res));
        }

        Ok(())
    }

    /// Delete a secret for an origin
    ///
    /// # Failures
    ///
    /// * Remote Builder is not available
    pub fn delete_origin_secret(&self, origin: &str, token: &str, key: &str) -> Result<()> {
        let path = format!("depot/origins/{}/secret/{}", origin, key);

        let res = self.add_authz(self.0.delete(&path), token).send()?;

        if res.status != StatusCode::Ok {
            return Err(err_from_response(res));
        }

        Ok(())
    }

    /// List all secrets keys for an origin
    ///
    /// # Failures
    ///
    /// * Remote Builder is not available
    pub fn list_origin_secrets(&self, origin: &str, token: &str) -> Result<Vec<String>> {
        let path = format!("depot/origins/{}/secret", origin);

        let mut res = self.add_authz(self.0.get(&path), token).send()?;

        if res.status != StatusCode::Ok {
            return Err(err_from_response(res));
        }

        let mut encoded = String::new();
        res.read_to_string(&mut encoded)?;
        debug!("Response body: {:?}", encoded);
        let secret_keys: Vec<String> = serde_json::from_str::<Vec<OriginSecret>>(&encoded)?
            .into_iter()
            .map(|m| m.into())
            .map(|s: originsrv::OriginSecret| s.get_name().to_string())
            .collect();
        Ok(secret_keys)
    }

    /// Download a public key from a remote Builder to the given filepath.
    ///
    /// # Failures
    ///
    /// * Key cannot be found
    /// * Remote Builder is not available
    /// * File cannot be created and written to
    pub fn fetch_origin_key<D, P: ?Sized>(
        &self,
        origin: &str,
        revision: &str,
        dst_path: &P,
        progress: Option<D>,
    ) -> Result<PathBuf>
    where
        P: AsRef<Path>,
        D: DisplayProgress + Sized,
    {
        self.download(
            &format!("depot/origins/{}/keys/{}", origin, revision),
            dst_path.as_ref(),
            None,
            progress,
        )
    }

    /// Download a secret key from a remote Builder to the given filepath.
    ///
    /// # Failures
    ///
    /// * Key cannot be found
    /// * Remote Builder is not available
    /// * File cannot be created and written to
    pub fn fetch_secret_origin_key<D, P: ?Sized>(
        &self,
        origin: &str,
        token: &str,
        dst_path: &P,
        progress: Option<D>,
    ) -> Result<PathBuf>
    where
        P: AsRef<Path>,
        D: DisplayProgress + Sized,
    {
        self.download(
            &format!("depot/origins/{}/secret_keys/latest", origin),
            dst_path.as_ref(),
            Some(token),
            progress,
        )
    }

    pub fn show_origin_keys(&self, origin: &str) -> Result<Vec<originsrv::OriginKeyIdent>> {
        let mut res = self.0.get(&origin_keys_path(origin)).send()?;
        debug!("Response: {:?}", res);

        if res.status != StatusCode::Ok {
            return Err(err_from_response(res));
        };

        let mut encoded = String::new();
        res.read_to_string(&mut encoded)?;
        debug!("Response body: {:?}", encoded);
        let revisions: Vec<originsrv::OriginKeyIdent> =
            serde_json::from_str::<Vec<OriginKeyIdent>>(&encoded)?
                .into_iter()
                .map(|m| m.into())
                .collect();
        Ok(revisions)
    }

    /// Return a list of channels for a given package
    ///
    /// # Failures
    ///
    /// * Remote Builder is not available
    /// * Package does not exist
    pub fn package_channels<I>(&self, ident: &I, token: Option<&str>) -> Result<Vec<String>>
    where
        I: Identifiable,
    {
        if !ident.fully_qualified() {
            return Err(Error::IdentNotFullyQualified);
        }

        let path = package_channels_path(ident);
        debug!("Retrieving channels for {}", ident);

        let mut res = self.maybe_add_authz(self.0.get(&path), token).send()?;

        if res.status != StatusCode::Ok {
            return Err(err_from_response(res));
        }

        let mut encoded = String::new();
        res.read_to_string(&mut encoded)?;
        debug!("Response body: {:?}", encoded);
        let channels: Vec<String> = serde_json::from_str::<Vec<String>>(&encoded)?
            .into_iter()
            .map(|m| m.into())
            .collect();
        Ok(channels)
    }

    /// Upload a public origin key to a remote Builder.
    ///
    /// # Failures
    ///
    /// * Remote Builder is not available
    /// * File cannot be read
    ///
    /// # Panics
    ///
    /// * Authorization token was not set on client
    pub fn put_origin_key<D>(
        &self,
        origin: &str,
        revision: &str,
        src_path: &Path,
        token: &str,
        progress: Option<D>,
    ) -> Result<()>
    where
        D: DisplayProgress + Sized,
    {
        let path = format!("depot/origins/{}/keys/{}", &origin, &revision);
        let mut file = File::open(src_path)?;
        let file_size = file.metadata()?.len();

        let result = if let Some(mut progress) = progress {
            progress.size(file_size);
            let mut reader = TeeReader::new(file, progress);
            self.add_authz(self.0.post(&path), token)
                .body(Body::SizedBody(&mut reader, file_size))
                .send()
        } else {
            self.add_authz(self.0.post(&path), token)
                .body(Body::SizedBody(&mut file, file_size))
                .send()
        };
        match result {
            Ok(Response { status: StatusCode::Created, .. }) => Ok(()),
            Ok(response) => Err(err_from_response(response)),
            Err(e) => Err(Error::from(e)),
        }
    }

    /// Download a secret key from a remote Builder to the given filepath.
    ///
    /// # Failures
    ///
    /// * Remote Builder is not available
    /// * File cannot be read
    ///
    /// # Panics
    ///
    /// * Authorization token was not set on client
    pub fn fetch_origin_secret_key<P>(
        &self,
        origin: &str,
        token: &str,
        dst_path: P,
    ) -> Result<PathBuf>
    where
        P: AsRef<Path>,
    {
        self.x_download(&origin_secret_keys_latest(origin), dst_path.as_ref(), token)
    }

    /// Upload a secret origin key to a remote Builder.
    ///
    /// # Failures
    ///
    /// * Remote Builder is not available
    /// * File cannot be read
    ///
    /// # Panics
    ///
    /// * Authorization token was not set on client
    pub fn put_origin_secret_key<D>(
        &self,
        origin: &str,
        revision: &str,
        src_path: &Path,
        token: &str,
        progress: Option<D>,
    ) -> Result<()>
    where
        D: DisplayProgress + Sized,
    {
        let path = format!("depot/origins/{}/secret_keys/{}", &origin, &revision);
        let mut file = File::open(src_path)?;
        let file_size = file.metadata()?.len();

        let result = if let Some(mut progress) = progress {
            progress.size(file_size);
            let mut reader = TeeReader::new(file, progress);
            self.add_authz(self.0.post(&path), token)
                .body(Body::SizedBody(&mut reader, file_size))
                .send()
        } else {
            self.add_authz(self.0.post(&path), token)
                .body(Body::SizedBody(&mut file, file_size))
                .send()
        };
        match result {
            Ok(Response { status: StatusCode::Created, .. }) => Ok(()),
            Ok(response) => Err(err_from_response(response)),
            Err(e) => Err(Error::from(e)),
        }
    }

    /// Download the latest release of a package.
    ///
    /// By the time this function is called, the ident must be fully qualified. The download URL in
    /// the depot requires a fully qualified ident to work. If you want the latest version of
    /// a package, e.g. core/redis, you can display package details for that via a different URL,
    /// e.g. /pkgs/core/redis/latest but that only _shows_ you the details - it doesn't download
    /// the package.
    ///
    /// # Failures
    ///
    /// * Package cannot be found
    /// * Remote Builder is not available
    /// * File cannot be created and written to
    pub fn fetch_package<D, I, P>(
        &self,
        ident: &I,
        token: Option<&str>,
        dst_path: &P,
        progress: Option<D>,
    ) -> Result<PackageArchive>
    where
        P: AsRef<Path> + ?Sized,
        I: Identifiable,
        D: DisplayProgress + Sized,
    {
        // Given that the download URL requires a fully qualified package, the channel is
        // irrelevant, per https://github.com/habitat-sh/habitat/issues/2722. This function is fine
        // as is.
        match self.download(&package_download(ident), dst_path.as_ref(), token, progress) {
            Ok(file) => Ok(PackageArchive::new(PathBuf::from(file))),
            Err(e) => Err(e),
        }
    }

    /// Returns a package struct for the latest package.
    ///
    /// An optional version can be specified which will scope the release returned to the latest
    /// release of that package.
    ///
    /// # Failures
    ///
    /// * Package cannot be found
    /// * Remote Builder is not available
    pub fn show_package<I>(
        &self,
        package: &I,
        channel: Option<&str>,
        token: Option<&str>,
    ) -> Result<originsrv::OriginPackage>
    where
        I: Identifiable,
    {
        // TODO: When channels are fully rolled out, we may want to make
        //       the channel specifier mandatory instead of being an Option
        let mut url = if let Some(channel) = channel {
            channel_package_path(channel, package)
        } else {
            package_path(package)
        };

        if !package.fully_qualified() {
            url.push_str("/latest");
        }

        let mut res = self.maybe_add_authz(self.0.get(&url), token).send()?;
        if res.status != StatusCode::Ok {
            return Err(err_from_response(res));
        }

        let mut encoded = String::new();
        res.read_to_string(&mut encoded)?;
        debug!("Body: {:?}", encoded);
        let package: originsrv::OriginPackage = serde_json::from_str::<Package>(&encoded)?.into();
        Ok(package)
    }

    /// Upload a package to a remote Builder.
    ///
    /// # Failures
    ///
    /// * Remote Builder is not available
    /// * File cannot be read
    ///
    /// # Panics
    ///
    /// * Authorization token was not set on client
    pub fn put_package<D>(
        &self,
        pa: &mut PackageArchive,
        token: &str,
        progress: Option<D>,
    ) -> Result<()>
    where
        D: DisplayProgress + Sized,
    {
        let checksum = pa.checksum()?;
        let ident = pa.ident()?;
        let mut file = File::open(&pa.path)?;
        let file_size = file.metadata()?.len();
        let path = package_path(&ident);
        let custom = |url: &mut Url| { url.query_pairs_mut().append_pair("checksum", &checksum); };
        debug!("Reading from {}", &pa.path.display());

        let result = if let Some(mut progress) = progress {
            progress.size(file_size);
            let mut reader = TeeReader::new(file, progress);
            self.add_authz(self.0.post_with_custom_url(&path, custom), token)
                .body(Body::SizedBody(&mut reader, file_size))
                .send()
        } else {
            self.add_authz(self.0.post_with_custom_url(&path, custom), token)
                .body(Body::SizedBody(&mut file, file_size))
                .send()
        };
        match result {
            Ok(Response { status: StatusCode::Created, .. }) => Ok(()),
            Ok(response) => Err(err_from_response(response)),
            Err(e) => Err(Error::from(e)),
        }
    }

    pub fn x_put_package(&self, pa: &mut PackageArchive, token: &str) -> Result<()> {
        let checksum = pa.checksum()?;
        let ident = pa.ident()?;
        let mut file = File::open(&pa.path)?;
        let file_size = file.metadata()?.len();
        let path = package_path(&ident);
        let custom = |url: &mut Url| {
            url.query_pairs_mut()
                .append_pair("checksum", &checksum)
                .append_pair("builder", "");
        };
        debug!("Reading from {}", &pa.path.display());

        let result = self.add_authz(self.0.post_with_custom_url(&path, custom), token)
            .body(Body::SizedBody(&mut file, file_size))
            .send();
        match result {
            Ok(Response { status: StatusCode::Created, .. }) => Ok(()),
            Ok(response) => Err(err_from_response(response)),
            Err(e) => Err(Error::from(e)),
        }
    }

    /// Promote a package to a given channel
    ///
    /// # Failures
    ///
    /// * Remote Builder is not available
    ///
    /// # Panics
    ///
    /// * If package does not exist in Builder
    /// * Authorization token was not set on client
    pub fn promote_package<I>(&self, ident: &I, channel: &str, token: &str) -> Result<()>
    where
        I: Identifiable,
    {
        if !ident.fully_qualified() {
            return Err(Error::IdentNotFullyQualified);
        }
        let path = channel_package_promote(channel, ident);
        debug!("Promoting package {}", ident);

        let res = self.add_authz(self.0.put(&path), token).send()?;

        if res.status != StatusCode::Ok {
            return Err(err_from_response(res));
        };

        Ok(())
    }

    /// Demote a package from a given channel
    ///
    /// # Failures
    ///
    /// * Remote Builder is not available
    ///
    /// # Panics
    ///
    /// * If package does not exist in Builder
    /// * Authorization token was not set on client
    pub fn demote_package<I>(&self, ident: &I, channel: &str, token: &str) -> Result<()>
    where
        I: Identifiable,
    {
        if !ident.fully_qualified() {
            return Err(Error::IdentNotFullyQualified);
        }
        let path = channel_package_demote(channel, ident);
        debug!("Demoting package {}", ident);

        let res = self.add_authz(self.0.put(&path), token).send()?;

        if res.status != StatusCode::Ok {
            return Err(err_from_response(res));
        };

        Ok(())
    }

    /// Create a custom channel
    ///
    /// # Failures
    ///
    /// * Remote Builder is not available
    pub fn create_channel(&self, origin: &str, channel: &str, token: &str) -> Result<()> {
        let path = format!("depot/channels/{}/{}", origin, channel);
        debug!("Creating channel, path: {:?}", path);

        let res = self.add_authz(self.0.post(&path), token).send()?;

        if res.status != StatusCode::Created {
            return Err(err_from_response(res));
        };

        Ok(())
    }

    /// Delete a custom channel
    ///
    /// # Failures
    ///
    /// * Remote Builder is not available
    pub fn delete_channel(&self, origin: &str, channel: &str, token: &str) -> Result<()> {
        let path = format!("depot/channels/{}/{}", origin, channel);
        debug!("Deleting channel, path: {:?}", path);

        let res = self.add_authz(self.0.delete(&path), token).send()?;

        if res.status != StatusCode::Ok {
            return Err(err_from_response(res));
        };

        Ok(())
    }

    /// Return a list of channels for a given origin
    ///
    /// # Failures
    /// * Remote Builder is not available
    /// * Authorization token was not set on client
    pub fn list_channels(
        &self,
        origin: &str,
        include_sandbox_channels: bool,
    ) -> Result<Vec<String>> {
        let path = format!("depot/channels/{}", origin);
        let mut res;

        if include_sandbox_channels {
            res = self.0
                .get_with_custom_url(&path, |url| url.set_query(Some("sandbox=true")))
                .send()?;
        } else {
            res = self.0.get(&path).send()?;
        }

        match res.status {
            StatusCode::Ok |
            StatusCode::PartialContent => {
                let mut encoded = String::new();
                res.read_to_string(&mut encoded)?;
                let results: Vec<OriginChannelIdent> = serde_json::from_str(&encoded)?;
                let channels = results.into_iter().map(|o| o.name).collect();
                Ok(channels)
            }
            _ => Err(err_from_response(res)),
        }
    }

    /// Returns a vector of PackageIdent structs
    ///
    /// # Failures
    ///
    /// * Remote depot unavailable
    pub fn search_package(
        &self,
        search_term: &str,
        token: Option<&str>,
    ) -> Result<(Vec<hab_core::package::PackageIdent>, bool)> {
        let mut res = self.maybe_add_authz(self.0.get(&package_search(search_term)), token)
            .send()?;
        match res.status {
            StatusCode::Ok |
            StatusCode::PartialContent => {
                let mut encoded = String::new();
                res.read_to_string(&mut encoded)?;
                let package_results: PackageResults<hab_core::package::PackageIdent> =
                    serde_json::from_str(&encoded)?;
                let packages: Vec<hab_core::package::PackageIdent> = package_results.data;
                Ok((packages, res.status == StatusCode::PartialContent))
            }
            _ => Err(err_from_response(res)),
        }
    }

    fn maybe_add_authz<'a>(
        &'a self,
        rb: RequestBuilder<'a>,
        token: Option<&str>,
    ) -> RequestBuilder {
        if token.is_some() {
            rb.header(Authorization(Bearer { token: token.unwrap().to_string() }))
        } else {
            rb
        }
    }

    fn add_authz<'a>(&'a self, rb: RequestBuilder<'a>, token: &str) -> RequestBuilder {
        rb.header(Authorization(Bearer { token: token.to_string() }))
    }

    fn download<D>(
        &self,
        path: &str,
        dst_path: &Path,
        token: Option<&str>,
        progress: Option<D>,
    ) -> Result<PathBuf>
    where
        D: DisplayProgress + Sized,
    {
        let mut res = self.maybe_add_authz(self.0.get(path), token).send()?;

        debug!("Response: {:?}", res);

        if res.status != hyper::status::StatusCode::Ok {
            return Err(err_from_response(res));
        }
        fs::create_dir_all(&dst_path)?;

        let file_name = match res.headers.get::<XFileName>() {
            Some(filename) => format!("{}", filename),
            None => return Err(Error::NoXFilename),
        };
        let tmp_file_path = dst_path.join(format!(
            "{}.tmp-{}",
            file_name,
            thread_rng().gen_ascii_chars().take(8).collect::<String>()
        ));
        let dst_file_path = dst_path.join(file_name);
        debug!("Writing to {}", &tmp_file_path.display());
        let mut f = File::create(&tmp_file_path)?;
        match progress {
            Some(mut progress) => {
                let size: u64 = res.headers.get::<hyper::header::ContentLength>().map_or(
                    0,
                    |v| **v,
                );
                progress.size(size);
                let mut writer = BroadcastWriter::new(&mut f, progress);
                io::copy(&mut res, &mut writer)?
            }
            None => io::copy(&mut res, &mut f)?,
        };
        debug!(
            "Moving {} to {}",
            &tmp_file_path.display(),
            &dst_file_path.display()
        );
        fs::rename(&tmp_file_path, &dst_file_path)?;
        Ok(dst_file_path)
    }

    // TODO: Ideally we would have a single download function that can support
    // both progress and non-progress versions, however the Rust compiler cannot
    // infer the type for a None for a Display + Sized trait, and makes this task
    // much more difficult than it should be. Fix later.
    fn x_download(&self, path: &str, dst_path: &Path, token: &str) -> Result<PathBuf> {
        let mut res = self.add_authz(self.0.get(path), token).send()?;
        debug!("Response: {:?}", res);

        if res.status != hyper::status::StatusCode::Ok {
            return Err(err_from_response(res));
        }
        fs::create_dir_all(&dst_path)?;

        let file_name = match res.headers.get::<XFileName>() {
            Some(filename) => format!("{}", filename),
            None => return Err(Error::NoXFilename),
        };
        let tmp_file_path = dst_path.join(format!(
            "{}.tmp-{}",
            file_name,
            thread_rng().gen_ascii_chars().take(8).collect::<String>()
        ));
        let dst_file_path = dst_path.join(file_name);
        debug!("Writing to {}", &tmp_file_path.display());
        let mut f = File::create(&tmp_file_path)?;
        io::copy(&mut res, &mut f)?;

        debug!(
            "Moving {} to {}",
            &tmp_file_path.display(),
            &dst_file_path.display()
        );
        fs::rename(&tmp_file_path, &dst_file_path)?;
        Ok(dst_file_path)
    }
}

fn err_from_response(mut response: hyper::client::Response) -> Error {
    if response.status == StatusCode::Unauthorized {
        return Error::APIError(
            response.status,
            "Your GitHub token requires both user:email and read:org permissions.".to_string(),
        );
    }

    let mut buff = String::new();
    match response.read_to_string(&mut buff) {
        Ok(_) => {
            match serde_json::from_str::<NetError>(&buff) {
                Ok(err) => Error::APIError(response.status, err.to_string()),
                Err(_) => Error::APIError(response.status, buff),
            }
        }
        Err(_) => {
            buff.truncate(0);
            Error::APIError(response.status, buff)
        }
    }
}

fn origin_keys_path(origin: &str) -> String {
    format!("depot/origins/{}/keys", origin)
}

fn origin_secret_keys_latest(origin: &str) -> String {
    format!("depot/origins/{}/secret_keys/latest", origin)
}

fn package_download<I>(package: &I) -> String
where
    I: Identifiable,
{
    format!("{}/download", package_path(package))
}

fn package_path<I>(package: &I) -> String
where
    I: Identifiable,
{
    format!("depot/pkgs/{}", package)
}

fn package_search(term: &str) -> String {
    let encoded_term = percent_encode(term.as_bytes(), PATH_SEGMENT_ENCODE_SET);
    format!("depot/pkgs/search/{}", encoded_term)
}

fn channel_package_path<I>(channel: &str, package: &I) -> String
where
    I: Identifiable,
{
    let mut path = format!(
        "depot/channels/{}/{}/pkgs/{}",
        package.origin(),
        channel,
        package.name()
    );
    if let Some(version) = package.version() {
        path.push_str("/");
        path.push_str(version);
        if let Some(release) = package.release() {
            path.push_str("/");
            path.push_str(release);
        }
    }
    path
}

fn package_channels_path<I>(package: &I) -> String
where
    I: Identifiable,
{
    format!(
        "depot/pkgs/{}/{}/{}/{}/channels",
        package.origin(),
        package.name(),
        package.version().unwrap(),
        package.release().unwrap()
    )
}

fn channel_package_promote<I>(channel: &str, package: &I) -> String
where
    I: Identifiable,
{
    format!(
        "depot/channels/{}/{}/pkgs/{}/{}/{}/promote",
        package.origin(),
        channel,
        package.name(),
        package.version().unwrap(),
        package.release().unwrap()
    )
}

fn channel_package_demote<I>(channel: &str, package: &I) -> String
where
    I: Identifiable,
{
    format!(
        "depot/channels/{}/{}/pkgs/{}/{}/{}/demote",
        package.origin(),
        channel,
        package.name(),
        package.version().unwrap(),
        package.release().unwrap()
    )
}

#[cfg(test)]
mod tests {
    use serde_json;
    use super::*;

    #[test]
    fn json_round_trip_u64_fields() {
        let pre = OriginPrivateSigningKey {
            id: 705705315793903646,
            origin_id: 705705305031319582,
            name: "core".to_string(),
            revision: "20160810182414".to_string(),
            body: vec![1, 2, 3],
            owner_id: 0,
        };

        let as_json = serde_json::to_value(&pre).unwrap();
        let expected = json!({
            "id": "705705315793903646",
            "origin_id": "705705305031319582",
            "name": "core",
            "revision": "20160810182414",
            "body": [
                1,
                2,
                3
            ],
            "owner_id": "0"
        });
        assert_eq!(as_json, expected);

        let post: OriginPrivateSigningKey = serde_json::from_value(as_json).unwrap();
        assert_eq!(pre.id, post.id);
        assert_eq!(pre.origin_id, post.origin_id);
        assert_eq!(pre.owner_id, post.owner_id);
    }
}
