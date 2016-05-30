// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

extern crate habitat_core as hcore;
extern crate habitat_depot_core as depot_core;
extern crate broadcast;
#[macro_use]
extern crate hyper;
#[macro_use]
extern crate log;
extern crate pbr;
extern crate rustc_serialize;
extern crate tee;
extern crate url;

pub mod error;

pub use error::{Error, Result};

use std::fs::{self, File};
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

use broadcast::BroadcastWriter;
use hcore::package::{PackageArchive, PackageIdent};
use hcore::env::http_proxy_unless_domain_exempted;
use depot_core::{XFileName, data_object};
use hyper::client::{Client, Body};
use hyper::status::StatusCode;
use hyper::Url;
use rustc_serialize::json;
use tee::TeeReader;

pub trait DisplayProgress: Write {
    fn size(&mut self, size: u64);
}

/// Download a public key from a remote Depot to the given filepath.
///
/// # Failures
///
/// * Key cannot be found
/// * Remote Depot is not available
/// * File cannot be created and written to
pub fn fetch_origin_key<P: AsRef<Path> + ?Sized>(depot: &str,
                                                 origin: &str,
                                                 revision: &str,
                                                 dst_path: &P,
                                                 progress: Option<&mut DisplayProgress>)
                                                 -> Result<PathBuf> {
    let url = try!(Url::parse(&format!("{}/origins/{}/keys/{}", depot, origin, revision)));
    download(url, dst_path.as_ref(), progress)
}

pub fn show_origin_keys(depot: &str, origin: &str) -> Result<Vec<data_object::OriginKeyIdent>> {
    let url = try!(Url::parse(&format!("{}/origins/{}/keys", depot, origin)));
    let (client, _) = try!(new_client(&url));
    debug!("GET {} with {:?}", &url, &client);
    let request = client.get(url);
    let mut res = try!(request.send());
    debug!("Response: {:?}", res);

    if res.status != hyper::status::StatusCode::Ok {
        return Err(Error::RemoteOriginKeyNotFound(origin.to_string()));
    };

    let mut encoded = String::new();
    try!(res.read_to_string(&mut encoded));
    debug!("Response body: {:?}", encoded);
    let revisions: Vec<data_object::OriginKeyIdent> = json::decode(&encoded).unwrap();
    Ok(revisions)
}

/// Upload a public origin key to a remote Depot.
///
/// # Failures
///
/// * Remote Depot is not available
/// * File cannot be read
pub fn put_origin_key(depot: &str,
                      origin: &str,
                      revision: &str,
                      src_path: &Path,
                      progress: Option<&mut DisplayProgress>)
                      -> Result<()> {
    let url = try!(Url::parse(&format!("{}/origins/{}/keys/{}", depot, &origin, &revision)));

    let mut file = try!(File::open(src_path));
    debug!("Reading from {}", src_path.display());
    upload(url, &mut file, progress)
}

/// Download the latest release of a package.
///
/// An optional version and release can be specified
/// which, when provided, will increase specificity of the release retrieved. Specifying a version
/// and no release will retrieve the latest release of a given version. Specifying both a version
/// and a release will retrieve that exact package.
///
/// # Failures
///
/// * Package cannot be found
/// * Remote Depot is not available
/// * File cannot be created and written to
pub fn fetch_package<P: AsRef<Path> + ?Sized>(depot: &str,
                                              package: &PackageIdent,
                                              dst_path: &P,
                                              progress: Option<&mut DisplayProgress>)
                                              -> Result<PackageArchive> {
    let url = try!(Url::parse(&format!("{}/pkgs/{}/download", depot, package)));
    match download(url, dst_path.as_ref(), progress) {
        Ok(file) => {
            let path = PathBuf::from(file);
            Ok(PackageArchive::new(path))
        }
        Err(Error::HTTP(StatusCode::NotFound)) => Err(Error::RemotePackageNotFound(package.clone())),
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
pub fn show_package(depot: &str, ident: &PackageIdent) -> Result<data_object::Package> {
    let url = try!(url_show_package(depot, ident));
    let (client, _) = try!(new_client(&url));
    let request = client.get(url);
    let mut res = try!(request.send());

    if res.status != hyper::status::StatusCode::Ok {
        return Err(Error::RemotePackageNotFound(ident.clone()));
    }

    let mut encoded = String::new();
    try!(res.read_to_string(&mut encoded));
    debug!("Body: {:?}", encoded);
    let package: data_object::Package = json::decode(&encoded).unwrap();
    Ok(package)
}

/// Upload a package to a remote Depot.
///
/// # Failures
///
/// * Remote Depot is not available
/// * File cannot be read
pub fn put_package(depot: &str, pa: &mut PackageArchive, progress: Option<&mut DisplayProgress>) -> Result<()> {
    let checksum = try!(pa.checksum());
    let ident = try!(pa.ident());
    let mut url = try!(Url::parse(&format!("{}/pkgs/{}", depot, ident)));
    url.query_pairs_mut().append_pair("checksum", &checksum);

    let mut file = try!(File::open(&pa.path));
    debug!("Reading from {}", &pa.path.display());
    upload(url, &mut file, progress)
}

fn new_client(url: &Url) -> Result<(Client, Option<(String, u16)>)> {
    match try!(http_proxy_unless_domain_exempted(url.host_str().unwrap_or(""))) {
        Some((proxy_host, proxy_port)) => {
            debug!("Using proxy {}:{}...", &proxy_host, &proxy_port);
            let proxy_info = Some((proxy_host.clone(), proxy_port.clone()));
            Ok((Client::with_http_proxy(proxy_host, proxy_port), proxy_info))
        }
        None => Ok((Client::new(), None)),
    }
}

fn url_show_package(depot: &str, package: &PackageIdent) -> Result<Url> {
    if package.fully_qualified() {
        Ok(try!(Url::parse(&format!("{}/pkgs/{}", depot, package))))
    } else {
        Ok(try!(Url::parse(&format!("{}/pkgs/{}/latest", depot, package))))
    }
}

fn download(url: Url, dst_path: &Path, progress: Option<&mut DisplayProgress>) -> Result<PathBuf> {
    let (client, _) = try!(new_client(&url));
    debug!("GET {} with {:?}", &url, &client);
    let mut res = try!(client.get(url).send());
    debug!("Response: {:?}", res);

    if res.status != hyper::status::StatusCode::Ok {
        return Err(Error::HTTP(res.status));
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
        Some(progress) => {
            let size: u64 = res.headers.get::<hyper::header::ContentLength>().map_or(0, |v| **v);
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

fn upload(url: Url, file: &mut File, progress: Option<&mut DisplayProgress>) -> Result<()> {
    try!(file.seek(SeekFrom::Start(0)));
    let metadata = try!(file.metadata());
    let size = metadata.len();

    let (client, _) = try!(new_client(&url));
    debug!("POST {} with {:?}", &url, &client);
    let response = match progress {
        Some(progress) => {
            progress.size(size);
            let mut reader = TeeReader::new(file, progress);
            try!(client.post(url).body(Body::SizedBody(&mut reader, size)).send())
        }
        None => try!(client.post(url).body(Body::SizedBody(file, size)).send()),
    };

    if response.status.is_success() {
        Ok(())
    } else {
        debug!("Response {:?}", response);
        Err(Error::HTTP(response.status))
    }
}
