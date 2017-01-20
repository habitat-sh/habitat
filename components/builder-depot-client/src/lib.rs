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
extern crate rustc_serialize;
extern crate serde;
extern crate serde_json;
extern crate tee;
extern crate url;

pub mod error;

pub use error::{Error, Result};

use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

use broadcast::BroadcastWriter;
use hab_core::package::{Identifiable, PackageArchive};
use hab_http::ApiClient;
use hyper::client::{Body, IntoUrl, Response, RequestBuilder};
use hyper::status::StatusCode;
use hyper::header::{Authorization, Bearer};
use hyper::Url;
use protocol::{depotsrv, net};
use rustc_serialize::{json, Decodable};
use tee::TeeReader;

header! { (XFileName, "X-Filename") => [String] }
header! { (ETag, "ETag") => [String] }

include!(concat!(env!("OUT_DIR"), "/serde_types.rs"));

#[derive(RustcDecodable)]
pub struct PackageResults<T>
    where T: Decodable
{
    pub range_start: isize,
    pub range_end: isize,
    pub total_count: isize,
    pub package_list: Vec<T>,
}

fn package_results_from_json<T: Decodable>(encoded: &str) -> PackageResults<T> {
    let results: PackageResults<T> = json::decode(&encoded).unwrap();
    results
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
        let revisions: Vec<depotsrv::OriginKeyIdent> = json::decode(&encoded).unwrap();
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
        let mut res =
            try!(self.add_authz(self.inner.get(&format!("origins/{}/secret_keys/latest", origin)),
                           token)
                .send());
        if res.status != StatusCode::Ok {
            return Err(err_from_response(res));
        }
        let mut encoded = String::new();
        try!(res.read_to_string(&mut encoded));
        let key = serde_json::from_str(&encoded).unwrap();
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
        let package: depotsrv::Package = json::decode(&encoded).unwrap();
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
        let customize = |url: &mut Url| {
            url.query_pairs_mut().append_pair("checksum", &checksum);
        };
        debug!("Reading from {}", &pa.path.display());

        let result = if let Some(mut progress) = progress {
            progress.size(file_size);
            let mut reader = TeeReader::new(file, progress);
            self.add_authz(self.inner.post_with_custom_url(&path, customize), token)
                .body(Body::SizedBody(&mut reader, file_size))
                .send()
        } else {
            self.add_authz(self.inner.post_with_custom_url(&path, customize), token)
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
        let customize = |url: &mut Url| {
            url.query_pairs_mut().append_pair("checksum", &checksum);
        };
        debug!("Reading from {}", &pa.path.display());

        let result = self.add_authz(self.inner.post_with_custom_url(&path, customize), token)
            .body(Body::SizedBody(&mut file, file_size))
            .send();
        match result {
            Ok(Response { status: StatusCode::Created, .. }) => Ok(()),
            Ok(response) => Err(err_from_response(response)),
            Err(e) => Err(Error::from(e)),
        }
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
                    package_results_from_json(&encoded);

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
        let tmp_file_path = dst_path.join(format!("{}.tmp", file_name));
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
            match json::decode::<net::NetError>(&buff) {
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
