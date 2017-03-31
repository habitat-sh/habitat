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
use protocol::{depotsrv, net};
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
        out.set_deps(self.deps
                         .into_iter()
                         .map(|m| m.into())
                         .collect());
        out.set_tdeps(self.tdeps
                          .into_iter()
                          .map(|m| m.into())
                          .collect());
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

#[derive(Clone, Serialize, Deserialize)]
pub struct PackageResults<T> {
    pub range_start: isize,
    pub range_end: isize,
    pub total_count: isize,
    pub package_list: Vec<T>,
}

pub trait DisplayProgress: Write {
    fn size(&mut self, size: u64);
    fn finish(&mut self);
}

pub struct Client {
    inner: ApiClient,
}

impl Client {
    pub fn new<U: IntoUrl>(hab_depot_url: U,
                           product: &str,
                           version: &str,
                           fs_root_path: Option<&Path>)
                           -> Result<Self> {
        let url = try!(hab_depot_url.into_url());
        Ok(Client { inner: try!(ApiClient::new(&url, product, version, fs_root_path)) })
    }

    /// Download a public key from a remote Depot to the given filepath.
    ///
    /// # Failures
    ///
    /// * Key cannot be found
    /// * Remote Depot is not available
    /// * File cannot be created and written to
    pub fn fetch_origin_key<D, P: ?Sized>(&self,
                                          origin: &str,
                                          revision: &str,
                                          dst_path: &P,
                                          progress: Option<D>)
                                          -> Result<PathBuf>
        where P: AsRef<Path>,
              D: DisplayProgress + Sized
    {
        self.download(&format!("origins/{}/keys/{}", origin, revision),
                      dst_path.as_ref(),
                      progress)
    }

    pub fn show_origin_keys(&self, origin: &str) -> Result<Vec<depotsrv::OriginKeyIdent>> {
        let mut res = try!(self.inner.get(&format!("origins/{}/keys", origin)).send());
        debug!("Response: {:?}", res);

        if res.status != StatusCode::Ok {
            return Err(err_from_response(res));
        };

        let mut encoded = String::new();
        try!(res.read_to_string(&mut encoded));
        debug!("Response body: {:?}", encoded);
        let revisions: Vec<depotsrv::OriginKeyIdent> =
            try!(serde_json::from_str::<Vec<OriginKeyIdent>>(&encoded))
                .into_iter()
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
    pub fn put_origin_key<D>(&self,
                             origin: &str,
                             revision: &str,
                             src_path: &Path,
                             token: &str,
                             progress: Option<D>)
                             -> Result<()>
        where D: DisplayProgress + Sized
    {
        let path = format!("origins/{}/keys/{}", &origin, &revision);
        let mut file = try!(File::open(src_path));
        let file_size = try!(file.metadata()).len();

        let result = if let Some(mut progress) = progress {
            progress.size(file_size);
            let mut reader = TeeReader::new(file, progress);
            self.add_authz(self.inner.post(&path), token)
                .body(Body::SizedBody(&mut reader, file_size))
                .send()
        } else {
            self.add_authz(self.inner.post(&path), token)
                .body(Body::SizedBody(&mut file, file_size))
                .send()
        };
        match result {
            Ok(Response { status: StatusCode::Created, .. }) => Ok(()),
            Ok(response) => {
                if response.status == StatusCode::Unauthorized {
                    Err(Error::APIError(response.status,
                                        "Your GitHub token requires both user:email and read:org \
                                         permissions."
                                                .to_string()))
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
        let mut res = try!(self.add_authz(self.inner.get(&format!("origins/{}/secret_keys/latest",
                                                                  origin)),
                                          token)
                               .send());
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
    pub fn put_origin_secret_key<D>(&self,
                                    origin: &str,
                                    revision: &str,
                                    src_path: &Path,
                                    token: &str,
                                    progress: Option<D>)
                                    -> Result<()>
        where D: DisplayProgress + Sized
    {
        let path = format!("origins/{}/secret_keys/{}", &origin, &revision);
        let mut file = try!(File::open(src_path));
        let file_size = try!(file.metadata()).len();

        let result = if let Some(mut progress) = progress {
            progress.size(file_size);
            let mut reader = TeeReader::new(file, progress);
            self.add_authz(self.inner.post(&path), token)
                .body(Body::SizedBody(&mut reader, file_size))
                .send()
        } else {
            self.add_authz(self.inner.post(&path), token)
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
    pub fn fetch_package<D, I, P: ?Sized>(&self,
                                          ident: &I,
                                          dst_path: &P,
                                          progress: Option<D>)
                                          -> Result<PackageArchive>
        where P: AsRef<Path>,
              I: Identifiable,
              D: DisplayProgress + Sized
    {
        match self.download(&format!("pkgs/{}/download", ident),
                            dst_path.as_ref(),
                            progress) {
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
    pub fn show_package<I: Identifiable>(&self, ident: &I) -> Result<depotsrv::Package> {
        let mut res = try!(self.inner.get(&self.path_show_package(ident)).send());

        if res.status != StatusCode::Ok {
            return Err(err_from_response(res));
        }

        let mut encoded = String::new();
        try!(res.read_to_string(&mut encoded));
        debug!("Body: {:?}", encoded);
        let package: depotsrv::Package = try!(serde_json::from_str::<Package>(&encoded)).into();
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
    pub fn put_package<D>(&self,
                          pa: &mut PackageArchive,
                          token: &str,
                          progress: Option<D>)
                          -> Result<()>
        where D: DisplayProgress + Sized
    {
        let checksum = try!(pa.checksum());
        let ident = try!(pa.ident());
        let mut file = try!(File::open(&pa.path));
        let file_size = try!(file.metadata()).len();
        let path = format!("pkgs/{}", ident);
        let custom = |url: &mut Url| { url.query_pairs_mut().append_pair("checksum", &checksum); };
        debug!("Reading from {}", &pa.path.display());

        let result = if let Some(mut progress) = progress {
            progress.size(file_size);
            let mut reader = TeeReader::new(file, progress);
            self.add_authz(self.inner.post_with_custom_url(&path, custom), token)
                .body(Body::SizedBody(&mut reader, file_size))
                .send()
        } else {
            self.add_authz(self.inner.post_with_custom_url(&path, custom), token)
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
        let path = format!("pkgs/{}", ident);
        let custom = |url: &mut Url| { url.query_pairs_mut().append_pair("checksum", &checksum); };
        debug!("Reading from {}", &pa.path.display());

        let result = self.add_authz(self.inner.post_with_custom_url(&path, custom), token)
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
    /// * If package archive does not have a version/release
    /// * Authorization token was not set on client
    pub fn promote_package(&self,
                           pa: &mut PackageArchive,
                           channel: &str,
                           token: &str)
                           -> Result<()> {
        let ident = try!(pa.ident());
        let path = format!("channels/{}/{}/pkgs/{}/{}/{}/promote",
                           ident.origin,
                           channel,
                           ident.name,
                           ident.version.unwrap(),
                           ident.release.unwrap());

        debug!("Promoting package, path: {}", path);

        let res = self.add_authz(self.inner.put(&path), token).send()?;

        if res.status != StatusCode::Ok {
            return Err(err_from_response(res));
        };

        Ok(())
    }

    /// Returns a vector of PackageIdent structs
    ///
    /// # Failures
    ///
    /// * Remote depot unavailable
    pub fn search_package(&self,
                          search_term: String)
                          -> Result<(Vec<hab_core::package::PackageIdent>, bool)> {

        let mut res = try!(self.inner.get(&format!("pkgs/search/{}", search_term)).send());
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

    fn path_show_package<I: Identifiable>(&self, package: &I) -> String {
        if package.fully_qualified() {
            format!("pkgs/{}", package)
        } else {
            format!("pkgs/{}/latest", package)
        }
    }

    fn download<D>(&self, path: &str, dst_path: &Path, progress: Option<D>) -> Result<PathBuf>
        where D: DisplayProgress + Sized
    {
        let mut res = try!(self.inner.get(path).send());
        debug!("Response: {:?}", res);

        if res.status != hyper::status::StatusCode::Ok {
            return Err(err_from_response(res));
        }
        try!(fs::create_dir_all(&dst_path));

        let file_name = match res.headers.get::<XFileName>() {
            Some(filename) => format!("{}", filename),
            None => return Err(Error::NoXFilename),
        };
        let tmp_file_path =
            dst_path.join(format!("{}.tmp-{}",
                                  file_name,
                                  thread_rng().gen_ascii_chars().take(8).collect::<String>()));
        let dst_file_path = dst_path.join(file_name);
        debug!("Writing to {}", &tmp_file_path.display());
        let mut f = try!(File::create(&tmp_file_path));
        match progress {
            Some(mut progress) => {
                let size: u64 =
                    res.headers.get::<hyper::header::ContentLength>().map_or(0, |v| **v);
                progress.size(size);
                let mut writer = BroadcastWriter::new(&mut f, progress);
                try!(io::copy(&mut res, &mut writer))
            }
            None => try!(io::copy(&mut res, &mut f)),
        };
        debug!("Moving {} to {}",
               &tmp_file_path.display(),
               &dst_file_path.display());
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
