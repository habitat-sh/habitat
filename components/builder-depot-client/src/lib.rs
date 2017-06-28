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

use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::string::ToString;

use broadcast::BroadcastWriter;
use hab_core::package::{Identifiable, PackageArchive};
use hab_http::ApiClient;
use hyper::client::{Body, IntoUrl, Response, RequestBuilder};
use hyper::status::StatusCode;
use hyper::header::{Authorization, Bearer};
use hyper::Url;
use protobuf::core::ProtobufEnum;
use protocol::{originsrv, net};
use rand::{Rng, thread_rng};
use tee::TeeReader;

header! { (XFileName, "X-Filename") => [String] }
header! { (ETag, "ETag") => [String] }

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

impl Into<originsrv::OriginKeyIdent> for OriginKeyIdent {
    fn into(self) -> originsrv::OriginKeyIdent {
        let mut out = originsrv::OriginKeyIdent::new();
        out.set_origin(self.origin);
        out.set_revision(self.revision);
        out.set_location(self.location);
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
pub struct OriginSecretKey {
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
    pub package_list: Vec<T>,
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
        depot_url: U,
        product: &str,
        version: &str,
        fs_root_path: Option<&Path>,
    ) -> Result<Self>
    where
        U: IntoUrl,
    {
        Ok(Client(
            ApiClient::new(depot_url, product, version, fs_root_path)?,
        ))
    }

    /// Download a public key from a remote Depot to the given filepath.
    ///
    /// # Failures
    ///
    /// * Key cannot be found
    /// * Remote Depot is not available
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
            &format!("origins/{}/keys/{}", origin, revision),
            dst_path.as_ref(),
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
        try!(res.read_to_string(&mut encoded));
        debug!("Response body: {:?}", encoded);
        let revisions: Vec<originsrv::OriginKeyIdent> = try!(
            serde_json::from_str::<Vec<OriginKeyIdent>>(&encoded)
        ).into_iter()
            .map(|m| m.into())
            .collect();
        Ok(revisions)
    }

    /// Upload a public origin key to a remote Depot.
    ///
    /// # Failures
    ///
    /// * Remote Depot is not available
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
        let path = format!("origins/{}/keys/{}", &origin, &revision);
        let mut file = try!(File::open(src_path));
        let file_size = try!(file.metadata()).len();

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
            Ok(response) => {
                if response.status == StatusCode::Unauthorized {
                    Err(Error::APIError(
                        response.status,
                        "Your GitHub token requires both user:email and read:org \
                                         permissions."
                            .to_string(),
                    ))
                } else {
                    Err(err_from_response(response))
                }
            }
            Err(e) => Err(Error::from(e)),
        }
    }

    /// Download a secret key from a remote Depot to the given filepath.
    ///
    /// # Failures
    ///
    /// * Remote Depot is not available
    /// * File cannot be read
    ///
    /// # Panics
    ///
    /// * Authorization token was not set on client
    pub fn fetch_origin_secret_key(&self, origin: &str, token: &str) -> Result<OriginSecretKey> {
        let mut res = self.add_authz(self.0.get(&origin_secret_keys_latest(origin)), token)
            .send()?;
        if res.status != StatusCode::Ok {
            return Err(err_from_response(res));
        }
        let mut encoded = String::new();
        try!(res.read_to_string(&mut encoded));
        let key = try!(serde_json::from_str(&encoded));
        Ok(key)
    }

    /// Upload a secret origin key to a remote Depot.
    ///
    /// # Failures
    ///
    /// * Remote Depot is not available
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
        let path = format!("origins/{}/secret_keys/{}", &origin, &revision);
        let mut file = try!(File::open(src_path));
        let file_size = try!(file.metadata()).len();

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
    /// An optional version and release can be specified which, when provided, will increase
    /// specificity of the release retrieved. Specifying a version and no release will retrieve
    /// the latest release of a given version. Specifying both a version and a release will
    /// retrieve that exact package.
    ///
    /// # Failures
    ///
    /// * Package cannot be found
    /// * Remote Depot is not available
    /// * File cannot be created and written to
    pub fn fetch_package<D, I, P>(
        &self,
        ident: &I,
        dst_path: &P,
        progress: Option<D>,
    ) -> Result<PackageArchive>
    where
        P: AsRef<Path> + ?Sized,
        I: Identifiable,
        D: DisplayProgress + Sized,
    {
        // JW TODO: We need to add a channel scoped /download route to the API server. Technically
        // this is wrong because we only want to download packages that are in the channel we
        // specified to the API client
        match self.download(&package_download(ident), dst_path.as_ref(), progress) {
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
    /// * Remote Depot is not available
    pub fn show_package<I>(
        &self,
        package: &I,
        channel: Option<&str>,
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
        let mut res = self.0.get(&url).send()?;
        if res.status != StatusCode::Ok {
            return Err(err_from_response(res));
        }

        let mut encoded = String::new();
        try!(res.read_to_string(&mut encoded));
        debug!("Body: {:?}", encoded);
        let package: originsrv::OriginPackage = try!(serde_json::from_str::<Package>(&encoded))
            .into();
        Ok(package)
    }

    /// Upload a package to a remote Depot.
    ///
    /// # Failures
    ///
    /// * Remote Depot is not available
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
        let checksum = try!(pa.checksum());
        let ident = try!(pa.ident());
        let mut file = try!(File::open(&pa.path));
        let file_size = try!(file.metadata()).len();
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
        let checksum = try!(pa.checksum());
        let ident = try!(pa.ident());
        let mut file = try!(File::open(&pa.path));
        let file_size = try!(file.metadata()).len();
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
    /// * Remote Depot is not available
    ///
    /// # Panics
    /// * If package does not exist in the Depot
    /// * Authorization token was not set on client
    pub fn promote_package<I>(&self, ident: &I, channel: &str, token: &str) -> Result<()>
    where
        I: Identifiable,
    {
        let path = channel_package_promote(channel, ident);
        debug!("Promoting package {}", ident);

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
    /// * Remote Depot is not available
    pub fn create_channel(&self, origin: &str, channel: &str, token: &str) -> Result<()> {
        let path = format!("channels/{}/{}", origin, channel);
        debug!("Creating channel, path: {:?}", path);

        let res = self.add_authz(self.0.post(&path), token).send()?;

        if res.status != StatusCode::Created {
            return Err(err_from_response(res));
        };

        Ok(())
    }

    /// Return a list of channels for a given origin
    ///
    /// # Failures
    /// * Remote Depot is not available
    /// * Authorization token was not set on client
    pub fn list_channels(&self, origin: &str) -> Result<Vec<String>> {
        let path = format!("channels/{}", origin);
        let mut res = self.0.get(&path).send()?;

        match res.status {
            StatusCode::Ok |
            StatusCode::PartialContent => {
                let mut encoded = String::new();
                try!(res.read_to_string(&mut encoded));
                let results: Vec<OriginChannelIdent> = try!(serde_json::from_str(&encoded));
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
    ) -> Result<(Vec<hab_core::package::PackageIdent>, bool)> {
        let mut res = self.0.get(&package_search(search_term)).send()?;
        match res.status {
            StatusCode::Ok |
            StatusCode::PartialContent => {
                let mut encoded = String::new();
                try!(res.read_to_string(&mut encoded));
                let package_results: PackageResults<hab_core::package::PackageIdent> =
                    try!(serde_json::from_str(&encoded));
                let packages: Vec<hab_core::package::PackageIdent> = package_results.package_list;
                Ok((packages, res.status == StatusCode::PartialContent))
            }
            _ => Err(err_from_response(res)),
        }
    }

    fn add_authz<'a>(&'a self, rb: RequestBuilder<'a>, token: &str) -> RequestBuilder {
        rb.header(Authorization(Bearer { token: token.to_string() }))
    }

    fn download<D>(&self, path: &str, dst_path: &Path, progress: Option<D>) -> Result<PathBuf>
    where
        D: DisplayProgress + Sized,
    {
        let mut res = try!(self.0.get(path).send());
        debug!("Response: {:?}", res);

        if res.status != hyper::status::StatusCode::Ok {
            return Err(err_from_response(res));
        }
        try!(fs::create_dir_all(&dst_path));

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
        let mut f = try!(File::create(&tmp_file_path));
        match progress {
            Some(mut progress) => {
                let size: u64 = res.headers.get::<hyper::header::ContentLength>().map_or(
                    0,
                    |v| **v,
                );
                progress.size(size);
                let mut writer = BroadcastWriter::new(&mut f, progress);
                try!(io::copy(&mut res, &mut writer))
            }
            None => try!(io::copy(&mut res, &mut f)),
        };
        debug!(
            "Moving {} to {}",
            &tmp_file_path.display(),
            &dst_file_path.display()
        );
        try!(fs::rename(&tmp_file_path, &dst_file_path));
        Ok(dst_file_path)
    }
}

fn err_from_response(mut response: hyper::client::Response) -> Error {
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
    format!("origins/{}/keys", origin)
}

fn origin_secret_keys_latest(origin: &str) -> String {
    format!("origins/{}/secret_keys/latest", origin)
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
    format!("pkgs/{}", package)
}

fn package_search(term: &str) -> String {
    format!("pkgs/search/{}", term)
}

fn channel_package_path<I>(channel: &str, package: &I) -> String
where
    I: Identifiable,
{
    let mut path = format!(
        "channels/{}/{}/pkgs/{}",
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

fn channel_package_promote<I>(channel: &str, package: &I) -> String
where
    I: Identifiable,
{
    format!(
        "channels/{}/{}/pkgs/{}/{}/{}/promote",
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
        let pre = OriginSecretKey {
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

        let post: OriginSecretKey = serde_json::from_value(as_json).unwrap();
        assert_eq!(pre.id, post.id);
        assert_eq!(pre.origin_id, post.origin_id);
        assert_eq!(pre.owner_id, post.owner_id);
    }
}
