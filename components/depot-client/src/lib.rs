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
extern crate url;

pub mod error;

pub use error::{Error, Result};

use std::fs::{self, File};
use std::io::{self, Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};

use broadcast::BroadcastWriter;
use hcore::package::{PackageArchive, PackageIdent};
use hcore::env::http_proxy_unless_domain_exempted;
use depot_core::{XFileName, data_object};
use hyper::client::{Client, Body};
use hyper::status::StatusCode;
use hyper::Url;
use pbr::{ProgressBar, Units};
use rustc_serialize::json;

/// Download a public key from a remote Depot to the given filepath.
///
/// # Failures
///
/// * Key cannot be found
/// * Remote Depot is not available
/// * File cannot be created and written to
pub fn get_origin_key(depot: &str, origin: &str, revision: &str, path: &str) -> Result<String> {
    let url = try!(Url::parse(&format!("{}/origins/{}/keys/{}", depot, origin, revision)));
    debug!("get_origin_key URL = {}", &url);
    let fname = format!("{}/{}-{}.pub", &path, &origin, &revision);
    debug!("Output filename = {}", &fname);
    download(&fname, url, path)
}

/// Download all public keys for a given origin.
///
/// # Failures
///
/// * Origin cannot be found
/// * Remote Depot is not available
/// * File write errors
pub fn get_origin_keys(depot: &str, origin: &str, path: &str) -> Result<()> {
    let url = try!(Url::parse(&format!("{}/origins/{}/keys", depot, origin)));
    debug!("URL = {}", &url);
    let client = try!(new_client(&url));
    let request = client.get(url);
    let mut res = try!(request.send());
    if res.status != hyper::status::StatusCode::Ok {
        return Err(Error::RemoteOriginKeyNotFound(origin.to_string()));
    };

    let mut encoded = String::new();
    try!(res.read_to_string(&mut encoded));
    let revisions: Vec<data_object::OriginKeyIdent> = json::decode(&encoded).unwrap();
    for rev in &revisions {
        try!(get_origin_key(depot, origin, &rev.revision, path));
    }
    Ok(())
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
pub fn fetch_package(depot: &str, package: &PackageIdent, store: &str) -> Result<PackageArchive> {
    let url = try!(Url::parse(&format!("{}/pkgs/{}/download", depot, package)));
    match download(&package.to_string(), url, store) {
        Ok(file) => {
            let path = PathBuf::from(file);
            Ok(PackageArchive::new(path))
        }
        Err(Error::HTTP(StatusCode::NotFound)) => {
            Err(Error::RemotePackageNotFound(package.clone()))
        }
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
    let client = try!(new_client(&url));
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

/// Upload a public origin key to a remote Depot.
///
/// # Failures
///
/// * Remote Depot is not available
/// * File cannot be read
pub fn post_origin_key(depot: &str, origin: &str, revision: &str, path: &Path) -> Result<()> {
    let mut file = try!(File::open(path));
    let url = try!(Url::parse(&format!("{}/origins/{}/keys/{}", depot, origin, revision)));
    upload(url, &mut file)
}

/// Upload a package to a remote Depot.
///
/// # Failures
///
/// * Remote Depot is not available
/// * File cannot be read
pub fn put_package(depot: &str, pa: &mut PackageArchive) -> Result<()> {
    let checksum = try!(pa.checksum());
    let ident = try!(pa.ident());
    let mut url = try!(Url::parse(&format!("{}/pkgs/{}", depot, ident)));
    url.query_pairs_mut().append_pair("checksum", &checksum);

    let mut file = try!(File::open(&pa.path));
    upload(url, &mut file)
}

fn url_show_package(depot: &str, package: &PackageIdent) -> Result<Url> {
    if package.fully_qualified() {
        Ok(try!(Url::parse(&format!("{}/pkgs/{}", depot, package))))
    } else {
        Ok(try!(Url::parse(&format!("{}/pkgs/{}/latest", depot, package))))
    }
}

fn download(status: &str, url: Url, path: &str) -> Result<String> {
    debug!("Making request to url {}", &url);
    let client = try!(new_client(&url));
    let mut res = try!(client.get(url).send());
    debug!("Response: {:?}", res);
    if res.status != hyper::status::StatusCode::Ok {
        return Err(Error::HTTP(res.status));
    }

    let file_name = match res.headers.get::<XFileName>() {
        Some(filename) => format!("{}", filename),
        None => return Err(Error::NoXFilename),
    };
    let tempfile = format!("{}/{}.tmp", path, file_name);
    let finalfile = format!("{}/{}", path, file_name);
    let size: u64 = res.headers.get::<hyper::header::ContentLength>().map_or(0, |v| **v);
    {
        let mut f = try!(File::create(&tempfile));
        let mut pb = ProgressBar::new(size);
        pb.set_units(Units::Bytes);
        let mut writer = BroadcastWriter::new(&mut f, &mut pb);
        println!("Downloading {}", &status);
        try!(io::copy(&mut res, &mut writer));
    }
    try!(fs::rename(&tempfile, &finalfile));
    Ok(finalfile)
}

fn upload(url: Url, file: &mut File) -> Result<()> {
    debug!("Uploading to {}", url);
    try!(file.seek(SeekFrom::Start(0)));
    let client = try!(new_client(&url));
    let metadata = try!(file.metadata());
    let response = try!(client.post(url).body(Body::SizedBody(file, metadata.len())).send());
    if response.status.is_success() {
        Ok(())
    } else {
        debug!("Response {:?}", response);
        Err(Error::HTTP(response.status))
    }
}

fn new_client(url: &Url) -> Result<Client> {
    match try!(http_proxy_unless_domain_exempted(url.host_str().unwrap_or(""))) {
        Some((proxy_host, proxy_port)) => {
            println!("Using proxy {}:{}...", &proxy_host, &proxy_port);
            Ok(Client::with_http_proxy(proxy_host, proxy_port))
        }
        None => Ok(Client::new()),
    }
}
