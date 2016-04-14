// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

extern crate habitat_core as hcore;
extern crate habitat_depot_core as depot_core;
#[macro_use]
extern crate hyper;
#[macro_use]
extern crate log;
extern crate rustc_serialize;

pub mod error;

pub use error::{Error, Result};

use std::fs::{self, File};
use std::io::{Read, Write, BufWriter, Seek, SeekFrom};
use std::path::{Path, PathBuf};

use hcore::package::{PackageArchive, PackageIdent};
use depot_core::{XFileName, data_object};
use hyper::client::{Client, Body};
use hyper::status::StatusCode;
use hyper::Url;
use rustc_serialize::json;

/// Download a public key from a remote Depot to the given filepath.
///
/// # Failures
///
/// * Key cannot be found
/// * Remote Depot is not available
/// * File cannot be created and written to
pub fn fetch_key(depot: &str, key: &str, path: &str) -> Result<String> {
    let url = Url::parse(&format!("{}/keys/{}", depot, key)).unwrap();
    download(key, url, path)
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
    let url = Url::parse(&format!("{}/pkgs/{}/download", depot, package)).unwrap();
    match download(&package.name, url, store) {
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
    let url = url_show_package(depot, ident);
    let client = Client::new();
    let request = client.get(&url);
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

/// Upload a public key to a remote Depot.
///
/// # Failures
///
/// * Remote Depot is not available
/// * File cannot be read
pub fn put_key(depot: &str, path: &Path) -> Result<()> {
    let mut file = try!(File::open(path));
    let file_name = try!(path.file_name().ok_or(Error::NoFilePart));
    let url = Url::parse(&format!("{}/keys/{}", depot, file_name.to_string_lossy())).unwrap();
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
    let url = Url::parse(&format!("{}/pkgs/{}?checksum={}", depot, ident, checksum)).unwrap();
    let mut file = try!(File::open(&pa.path));
    upload(url, &mut file)
}

fn url_show_package(depot: &str, package: &PackageIdent) -> String {
    if package.fully_qualified() {
        format!("{}/pkgs/{}", depot, package)
    } else {
        format!("{}/pkgs/{}/latest", depot, package)
    }
}

fn download(status: &str, url: Url, path: &str) -> Result<String> {
    debug!("Making request to url {}", url);
    let client = Client::new();
    let mut res = try!(client.get(url).send());
    debug!("Response: {:?}", res);

    if res.status != hyper::status::StatusCode::Ok {
        return Err(Error::HTTP(res.status));
    }

    let file_name = match res.headers.get::<XFileName>() {
        Some(filename) => format!("{}", filename),
        None => return Err(Error::NoXFilename),
    };
    let length = res.headers
                    .get::<hyper::header::ContentLength>()
                    .map_or("Unknown".to_string(), |v| format!("{}", v));
    // Here is a moment where you can really like Rust. We create
    // a file, wrap it in a BufWriter - which understands how to
    // safely batch writes into large buffer sizes on the heap,
    // saving us the tax of frequent system calls. We then do
    // what we would do in C - create a buffer of bytes, then
    // read into that buffer, and write out to the other side.
    //
    // Under the hood, Hyper uses the "BufReader" to implement
    // reading the request body - so on both ends, we are getting
    // free buffering on the heap, using our stack buffer just to
    // shuttle back and forth. This is essentially with the "tee"
    // function does in the stdlib, but with error handling that
    // a generic "tee" can't really provide easily.
    //
    // What you can't see is this - the compiler helped with
    // making sure all the edge cases of the pattern were covered,
    // and even though its a trivial case, it was pretty great.
    let tempfile = format!("{}/{}.tmp", path, file_name);
    let finalfile = format!("{}/{}", path, file_name);
    let f = try!(File::create(&tempfile));
    let mut writer = BufWriter::new(&f);
    let mut written: i64 = 0;
    let mut buf = [0u8; 100000]; // Our byte buffer
    loop {
        let len = try!(res.read(&mut buf)); // Raise IO errors
        match len {
            0 => {
                // 0 == EOF, so stop writing and finish progress
                progress(status, written, &length, true);
                break;
            }
            _ => {
                // Write the buffer to the BufWriter on the Heap
                let bytes_written = try!(writer.write(&buf[0..len]));
                if bytes_written == 0 {
                    return Err(Error::WriteSyncFailed);
                }
                written = written + (bytes_written as i64);
                progress(status, written, &length, false);
            }
        };
    }
    try!(fs::rename(&tempfile, &finalfile));
    Ok(finalfile)
}

fn upload(url: Url, file: &mut File) -> Result<()> {
    debug!("Uploading to {}", url);
    try!(file.seek(SeekFrom::Start(0)));
    let client = Client::new();
    let metadata = try!(file.metadata());
    let response = try!(client.post(url).body(Body::SizedBody(file, metadata.len())).send());
    if response.status.is_success() {
        Ok(())
    } else {
        debug!("Response {:?}", response);
        Err(Error::HTTP(response.status))
    }
}

fn progress(status: &str, written: i64, length: &str, finished: bool) {
    let progress = format!("{} {}/{}", status, written, length);
    print!("{}", from_char(progress.len(), '\x08'));
    if finished {
        println!("{}", progress);
    } else {
        print!("{}", progress);
    }
}

fn from_char(length: usize, ch: char) -> String {
    if length == 0 {
        return String::new();
    }

    let mut buf = String::new();
    buf.push(ch);
    let size = buf.len() * length;
    buf.reserve(size);
    for _ in 1..length {
        buf.push(ch)
    }
    buf
}

#[cfg(test)]
mod tests {
    use super::from_char;

    #[test]
    fn from_char_returns_the_correct_string() {
        assert_eq!("xxxx", from_char(4, 'x'));
    }
}
