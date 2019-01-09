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

#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

use habitat_core as hab_core;
use habitat_http_client as hab_http;
#[macro_use]
extern crate hyper;

#[macro_use]
extern crate log;

use serde;
#[macro_use]
extern crate serde_derive;
#[allow(unused_imports)]
#[macro_use]
extern crate serde_json;

pub mod error;
pub use crate::error::{Error, Result};

use std::fmt;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::string::ToString;

use crate::hab_core::package::{Identifiable, PackageArchive, PackageIdent, PackageTarget};
use crate::hab_http::util::decoded_response;
use crate::hab_http::ApiClient;
use broadcast::BroadcastWriter;
use chrono::DateTime;
use hyper::client::{Body, IntoUrl, RequestBuilder, Response};
use hyper::header::{Accept, Authorization, Bearer, ContentType};
use hyper::status::StatusCode;
use hyper::Url;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
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

impl fmt::Display for NetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[err: {:?}, msg: {}]", self.code, self.msg)
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = Vec::new();
        output.push(format!(
            "Status for Job Group {} ({}): {}",
            self.id, self.project_name, self.state
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

#[derive(Deserialize)]
pub struct ReverseDependencies {
    pub origin: String,
    pub name: String,
    pub rdeps: Vec<String>,
}

#[derive(Default, Deserialize)]
pub struct JobGroupPromoteResponse {
    pub group_id: String,
    pub not_promoted: Vec<PackageIdent>,
}

/// Custom conversion logic to allow `serde` to successfully
/// round-trip `u64` datatypes through JSON serialization.
///
/// To use it, add `#[serde(with = "json_u64")]` to any `u64`-typed struct
/// fields.
mod json_u64 {
    use serde::{self, Deserialize, Deserializer, Serializer};

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

mod json {
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

    #[derive(Clone, Deserialize)]
    pub struct PackageIdent {
        pub origin: String,
        pub name: String,
        pub version: String,
        pub release: String,
    }

    impl From<PackageIdent> for super::PackageIdent {
        fn from(ident: PackageIdent) -> Self {
            super::PackageIdent {
                origin: ident.origin,
                name: ident.name,
                version: Some(ident.version),
                release: Some(ident.release),
            }
        }
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
        let mut endpoint = endpoint.into_url().map_err(Error::UrlParseError)?;
        if !endpoint.cannot_be_a_base() && endpoint.path() == "/" {
            endpoint.set_path(DEFAULT_API_PATH);
        }
        Ok(Client(
            ApiClient::new(endpoint, product, version, fs_root_path)
                .map_err(Error::HabitatHttpClient)?,
        ))
    }

    /// Retrieves the status of every group job in an origin
    ///
    /// # Failures
    ///
    /// * Remote Builder is not available
    pub fn get_origin_schedule(
        &self,
        origin: &str,
        limit: usize,
    ) -> Result<Vec<SchedulerResponse>> {
        debug!("Retrieving status for job groups in the {} origin", origin);

        let path = format!("depot/pkgs/schedule/{}/status", origin);

        let custom = |url: &mut Url| {
            url.query_pairs_mut()
                .append_pair("limit", &limit.to_string());
        };

        let res = self.0.get_with_custom_url(&path, custom).send()?;

        if res.status != StatusCode::Ok {
            return Err(err_from_response(res));
        }

        let sr: Vec<SchedulerResponse> = decoded_response(res)?;
        Ok(sr)
    }

    /// Retrieves the status of a group job
    ///
    /// # Failures
    ///
    /// * Remote Builder is not available
    pub fn get_schedule(&self, group_id: i64, include_projects: bool) -> Result<SchedulerResponse> {
        debug!("Retrieving schedule for job group {}", group_id);

        let path = format!("depot/pkgs/schedule/{}", group_id);

        let custom = |url: &mut Url| {
            url.query_pairs_mut()
                .append_pair("include_projects", &include_projects.to_string());
        };

        let res = self.0.get_with_custom_url(&path, custom).send()?;

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
    pub fn schedule_job(
        &self,
        ident: &PackageIdent,
        package_only: bool,
        token: &str,
    ) -> Result<(String)> {
        // TODO (SA): This API needs to be extended to support a target param.
        let path = format!("depot/pkgs/schedule/{}/{}", ident.origin(), ident.name());
        let result = if package_only {
            let custom = |url: &mut Url| {
                url.query_pairs_mut().append_pair("package_only", "true");
            };
            self.add_authz(self.0.post_with_custom_url(&path, custom), token)
                .send()
        } else {
            self.add_authz(self.0.post(&path), token).send()
        };
        match result {
            Ok(response) => {
                if response.status == StatusCode::Created || response.status == StatusCode::Ok {
                    let sr: SchedulerResponse = decoded_response(response)?;
                    Ok(sr.id)
                } else {
                    Err(err_from_response(response))
                }
            }
            Err(e) => Err(Error::from(e)),
        }
    }

    /// Fetch the reverse dependencies for a package
    ///
    /// # Failures
    ///
    /// * Remote API Server is not available
    pub fn fetch_rdeps(&self, ident: &PackageIdent) -> Result<Vec<String>> {
        debug!("Fetching the reverse dependencies for {}", ident);

        let url = format!("rdeps/{}", ident);
        let mut res = self.0.get(&url).send().map_err(Error::HyperError)?;
        if res.status != StatusCode::Ok {
            return Err(err_from_response(res));
        }

        let mut encoded = String::new();
        res.read_to_string(&mut encoded).map_err(Error::IO)?;
        debug!("Body: {:?}", encoded);
        let rd: ReverseDependencies = serde_json::from_str(&encoded).map_err(Error::Json)?;
        Ok(rd.rdeps.to_vec())
    }

    /// Promote/Demote a job group to/from a channel
    ///
    /// # Failures
    ///
    /// * Remote API Server is not available
    pub fn job_group_promote_or_demote<T: AsRef<str> + serde::Serialize>(
        &self,
        group_id: u64,
        idents: &[T],
        channel: &str,
        token: &str,
        promote: bool,
    ) -> Result<()> {
        let json_idents = json!(idents);
        let body = json!({ "idents": json_idents });
        let sbody = serde_json::to_string(&body).unwrap();
        let url = format!(
            "jobs/group/{}/{}/{}",
            group_id,
            if promote { "promote" } else { "demote" },
            channel
        );
        let res = self
            .add_authz(self.0.post(&url), token)
            .body(&sbody)
            .header(Accept::json())
            .header(ContentType::json())
            .send()
            .map_err(Error::HyperError)?;

        if res.status != StatusCode::NoContent {
            debug!(
                "Failed to {} group, status: {:?}",
                if promote { "promote" } else { "demote" },
                res.status
            );
            return Err(err_from_response(res));
        }

        Ok(())
    }

    /// Cancel a job group
    ///
    /// # Failures
    ///
    /// * Remote API Server is not available
    pub fn job_group_cancel(&self, group_id: u64, token: &str) -> Result<()> {
        let url = format!("jobs/group/{}/cancel", group_id);
        let res = self
            .add_authz(self.0.post(&url), token)
            .send()
            .map_err(Error::HyperError)?;

        if res.status != StatusCode::NoContent {
            debug!("Failed to cancel group, status: {:?}", res.status);
            return Err(err_from_response(res));
        }

        Ok(())
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
            self.0
                .get(&format!("depot/origins/{}/encryption_key", origin)),
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
        let res = self
            .add_authz(self.0.post(&path), token)
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
        res.read_to_string(&mut encoded)
            .map_err(Error::BadResponseBody)?;
        debug!("Response body: {:?}", encoded);
        let secret_keys: Vec<String> = serde_json::from_str::<Vec<OriginSecret>>(&encoded)?
            .into_iter()
            .map(|s| s.name)
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
            self.0
                .get(&format!("depot/origins/{}/keys/{}", origin, revision)),
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
            self.0
                .get(&format!("depot/origins/{}/secret_keys/latest", origin)),
            dst_path.as_ref(),
            Some(token),
            progress,
        )
    }

    pub fn show_origin_keys(&self, origin: &str) -> Result<Vec<OriginKeyIdent>> {
        let mut res = self.0.get(&origin_keys_path(origin)).send()?;
        debug!("Response: {:?}", res);

        if res.status != StatusCode::Ok {
            return Err(err_from_response(res));
        };

        let mut encoded = String::new();
        res.read_to_string(&mut encoded)
            .map_err(Error::BadResponseBody)?;
        debug!("Response body: {:?}", encoded);
        let revisions: Vec<OriginKeyIdent> = serde_json::from_str::<Vec<OriginKeyIdent>>(&encoded)?;
        Ok(revisions)
    }

    /// Return a list of channels for a given package
    ///
    /// # Failures
    ///
    /// * Remote Builder is not available
    /// * Package does not exist
    pub fn package_channels(
        &self,
        ident: &PackageIdent,
        token: Option<&str>,
    ) -> Result<Vec<String>> {
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
        res.read_to_string(&mut encoded)
            .map_err(Error::BadResponseBody)?;
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
        let mut file =
            File::open(src_path).map_err(|e| Error::KeyReadError(src_path.to_path_buf(), e))?;
        let file_size = file
            .metadata()
            .map_err(|e| Error::KeyReadError(src_path.to_path_buf(), e))?
            .len();

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
            Ok(Response {
                status: StatusCode::Created,
                ..
            }) => Ok(()),
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
        let mut file =
            File::open(src_path).map_err(|e| Error::KeyReadError(src_path.to_path_buf(), e))?;
        let file_size = file
            .metadata()
            .map_err(|e| Error::KeyReadError(src_path.to_path_buf(), e))?
            .len();

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
            Ok(Response {
                status: StatusCode::Created,
                ..
            }) => Ok(()),
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
    pub fn fetch_package<D, P>(
        &self,
        ident: &PackageIdent,
        target: &PackageTarget,
        token: Option<&str>,
        dst_path: &P,
        progress: Option<D>,
    ) -> Result<PackageArchive>
    where
        P: AsRef<Path> + ?Sized,
        D: DisplayProgress + Sized,
    {
        // Ensure ident is fully qualified.
        //
        // TODO fn: this will be removed when we can describe a fully qualified ident by type as a
        // param to this function
        if !ident.fully_qualified() {
            return Err(Error::IdentNotFullyQualified);
        }

        let req_builder = self.0.get_with_custom_url(&package_download(ident), |u| {
            u.set_query(Some(&format!("target={}", target)))
        });

        match self.download(req_builder, dst_path.as_ref(), token, progress) {
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
    pub fn show_package(
        &self,
        package: &PackageIdent,
        target: &PackageTarget,
        channel: &str,
        token: Option<&str>,
    ) -> Result<PackageIdent> {
        let mut url = channel_package_path(channel, package);

        if !package.fully_qualified() {
            url.push_str("/latest");
        }

        let mut res = self
            .maybe_add_authz(
                self.0.get_with_custom_url(&url, |u| {
                    u.set_query(Some(&format!("target={}", target)))
                }),
                token,
            )
            .send()?;

        if res.status != StatusCode::Ok {
            return Err(err_from_response(res));
        }

        let mut encoded = String::new();
        res.read_to_string(&mut encoded)
            .map_err(Error::BadResponseBody)?;
        debug!("Body: {:?}", encoded);
        let package: json::Package = serde_json::from_str::<json::Package>(&encoded)?;
        Ok(package.ident.into())
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
        force_upload: bool,
        progress: Option<D>,
    ) -> Result<()>
    where
        D: DisplayProgress + Sized,
    {
        let checksum = pa.checksum()?;
        let ident = pa.ident()?;
        let file = File::open(&pa.path).map_err(|e| Error::PackageReadError(pa.path.clone(), e))?;
        let file_size = file
            .metadata()
            .map_err(|e| Error::PackageReadError(pa.path.clone(), e))?
            .len();

        let path = package_path(&ident);

        let custom = |url: &mut Url| {
            url.query_pairs_mut()
                .append_pair("checksum", &checksum)
                .append_pair("forced", &force_upload.to_string());
        };
        debug!("Reading from {}", &pa.path.display());

        let mut reader: Box<dyn Read> = if let Some(mut progress) = progress {
            progress.size(file_size);
            Box::new(TeeReader::new(file, progress))
        } else {
            Box::new(file)
        };

        let result = self
            .add_authz(self.0.post_with_custom_url(&path, custom), token)
            .body(Body::SizedBody(&mut reader, file_size))
            .send();

        match result {
            Ok(Response {
                status: StatusCode::Created,
                ..
            }) => Ok(()),
            Ok(response) => Err(err_from_response(response)),
            Err(e) => Err(Error::from(e)),
        }
    }

    pub fn x_put_package(&self, pa: &mut PackageArchive, token: &str) -> Result<()> {
        let checksum = pa.checksum()?;
        let ident = pa.ident()?;
        let mut file =
            File::open(&pa.path).map_err(|e| Error::PackageReadError(pa.path.clone(), e))?;
        let file_size = file
            .metadata()
            .map_err(|e| Error::PackageReadError(pa.path.clone(), e))?
            .len();
        let path = package_path(&ident);
        let custom = |url: &mut Url| {
            url.query_pairs_mut()
                .append_pair("checksum", &checksum)
                .append_pair("builder", "");
        };
        debug!("Reading from {}", &pa.path.display());

        let result = self
            .add_authz(self.0.post_with_custom_url(&path, custom), token)
            .body(Body::SizedBody(&mut file, file_size))
            .send();
        match result {
            Ok(Response {
                status: StatusCode::Created,
                ..
            }) => Ok(()),
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
    pub fn promote_package(&self, ident: &PackageIdent, channel: &str, token: &str) -> Result<()> {
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
    pub fn demote_package(&self, ident: &PackageIdent, channel: &str, token: &str) -> Result<()> {
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
            res = self
                .0
                .get_with_custom_url(&path, |url| url.set_query(Some("sandbox=true")))
                .send()?;
        } else {
            res = self.0.get(&path).send()?;
        }

        match res.status {
            StatusCode::Ok | StatusCode::PartialContent => {
                let mut encoded = String::new();
                res.read_to_string(&mut encoded)
                    .map_err(Error::BadResponseBody)?;
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
    ) -> Result<(Vec<PackageIdent>, bool)> {
        let mut res = self
            .maybe_add_authz(self.0.get(&package_search(search_term)), token)
            .send()?;
        match res.status {
            StatusCode::Ok | StatusCode::PartialContent => {
                let mut encoded = String::new();
                res.read_to_string(&mut encoded)
                    .map_err(Error::BadResponseBody)?;
                let package_results: PackageResults<PackageIdent> = serde_json::from_str(&encoded)?;
                let packages: Vec<PackageIdent> = package_results.data;
                Ok((packages, res.status == StatusCode::PartialContent))
            }
            _ => Err(err_from_response(res)),
        }
    }

    fn maybe_add_authz<'a>(
        &'a self,
        rb: RequestBuilder<'a>,
        token: Option<&str>,
    ) -> RequestBuilder<'_> {
        if token.is_some() {
            rb.header(Authorization(Bearer {
                token: token.unwrap().to_string(),
            }))
        } else {
            rb
        }
    }

    fn add_authz<'a>(&'a self, rb: RequestBuilder<'a>, token: &str) -> RequestBuilder<'_> {
        rb.header(Authorization(Bearer {
            token: token.to_string(),
        }))
    }

    fn download<'a, D>(
        &'a self,
        rb: RequestBuilder<'a>,
        dst_path: &Path,
        token: Option<&str>,
        progress: Option<D>,
    ) -> Result<PathBuf>
    where
        D: DisplayProgress + Sized,
    {
        let mut res = self.maybe_add_authz(rb, token).send()?;

        debug!("Response: {:?}", res);

        if res.status != hyper::status::StatusCode::Ok {
            return Err(err_from_response(res));
        }

        fs::create_dir_all(&dst_path)?;

        let file_name = res
            .headers
            .get::<XFileName>()
            .expect("XFileName missing from response")
            .to_string();
        let tmp_file_path = dst_path.join(format!(
            "{}.tmp-{}",
            file_name,
            thread_rng()
                .sample_iter(&Alphanumeric)
                .take(8)
                .collect::<String>()
        ));
        let dst_file_path = dst_path.join(file_name);
        debug!("Writing to {}", &tmp_file_path.display());
        let mut f = File::create(&tmp_file_path)?;
        match progress {
            Some(mut progress) => {
                let size: u64 = res
                    .headers
                    .get::<hyper::header::ContentLength>()
                    .map_or(0, |v| **v);
                progress.size(size);
                let mut writer = BroadcastWriter::new(&mut f, progress);
                io::copy(&mut res, &mut writer).map_err(Error::BadResponseBody)?
            }
            None => io::copy(&mut res, &mut f).map_err(Error::BadResponseBody)?,
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

        let file_name = res
            .headers
            .get::<XFileName>()
            .expect("XFileName missing from response")
            .to_string();
        let tmp_file_path = dst_path.join(format!(
            "{}.tmp-{}",
            file_name,
            thread_rng()
                .sample_iter(&Alphanumeric)
                .take(8)
                .collect::<String>()
        ));
        let dst_file_path = dst_path.join(file_name);
        debug!("Writing to {}", &tmp_file_path.display());
        let mut f = File::create(&tmp_file_path)?;
        io::copy(&mut res, &mut f).map_err(Error::BadResponseBody)?;
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
            "Please check that you have specified a valid Personal Access Token.".to_string(),
        );
    }

    let mut buff = String::new();
    match response.read_to_string(&mut buff) {
        Ok(_) => match serde_json::from_str::<NetError>(&buff) {
            Ok(err) => Error::APIError(response.status, err.to_string()),
            Err(_) => Error::APIError(response.status, buff),
        },
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

fn package_download(package: &PackageIdent) -> String {
    format!("{}/download", package_path(package))
}

fn package_path(package: &PackageIdent) -> String {
    format!("depot/pkgs/{}", package)
}

fn package_search(term: &str) -> String {
    let encoded_term = percent_encode(term.as_bytes(), PATH_SEGMENT_ENCODE_SET);
    format!("depot/pkgs/search/{}", encoded_term)
}

fn channel_package_path(channel: &str, package: &PackageIdent) -> String {
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

fn package_channels_path(package: &PackageIdent) -> String {
    format!(
        "depot/pkgs/{}/{}/{}/{}/channels",
        package.origin(),
        package.name(),
        package.version().unwrap(),
        package.release().unwrap()
    )
}

fn channel_package_promote(channel: &str, package: &PackageIdent) -> String {
    format!(
        "depot/channels/{}/{}/pkgs/{}/{}/{}/promote",
        package.origin(),
        channel,
        package.name(),
        package.version().unwrap(),
        package.release().unwrap()
    )
}

fn channel_package_demote(channel: &str, package: &PackageIdent) -> String {
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
    use super::*;
    use serde_json;

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
