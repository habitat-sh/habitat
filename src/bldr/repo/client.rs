//
// Copyright:: Copyright (c) 2015 Chef Software, Inc.
// License:: Apache License, Version 2.0
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::io::{Read, Write, BufWriter};
use std::fs::{self, File};
use std::path::Path;

use hyper;
use hyper::client::{Client, Body};
use hyper::status::StatusCode;
use rustc_serialize::json;

use super::XFileName;
use error::{BldrResult, BldrError};
use pkg::Package;

/// Download a public key from a remote repository to the given filepath.
///
/// # Failures
///
/// * Key cannot be found
/// * Remote repository is not available
/// * File cannot be created and written to
pub fn fetch_key(repo: &str, key: &str, path: &str) -> BldrResult<String> {
    let url = format!("{}/keys/{}", repo, key);
    download(key, &url, path)
}

/// Download a sepcific package identified by it's derivation, name, version, and release, to the
/// given filepath.
///
/// # Failures
///
/// * Package cannot be found
/// * Remote repository is not available
/// * File cannot be created and written to
pub fn fetch_package_exact(repo: &str, package: &Package, path: &str) -> BldrResult<String> {
    let url = format!("{}/pkgs/{}/{}/{}/{}/download",
                      repo,
                      package.derivation,
                      package.name,
                      package.version,
                      package.release);
    match download(&package.name, &url, path) {
        Ok(path) => Ok(path),
        Err(BldrError::HTTP(StatusCode::NotFound)) =>
            Err(BldrError::RemotePackageNotFound(package.derivation.clone(),
                                                 package.name.clone(),
                                                 Some(package.version.clone()),
                                                 Some(package.release.clone()))),
        Err(e) => Err(e),
    }
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
/// * Remote repository is not available
/// * File cannot be created and written to
pub fn fetch_package(repo: &str,
                     derivation: &str,
                     package: &str,
                     version: &Option<String>,
                     release: &Option<String>,
                     path: &str)
                     -> BldrResult<String> {
    let url = if release.is_some() && version.is_some() {
        format!("{}/pkgs/{}/{}/{}/{}/download",
                repo,
                derivation,
                package,
                release.as_ref().unwrap(),
                version.as_ref().unwrap())
    } else if release.is_some() {
        format!("{}/pkgs/{}/{}/{}/download",
                repo,
                derivation,
                package,
                release.as_ref().unwrap())
    } else {
        format!("{}/pkgs/{}/{}/download", repo, derivation, package)
    };
    match download(package, &url, path) {
        Ok(path) => Ok(path),
        Err(BldrError::HTTP(StatusCode::NotFound)) =>
            Err(BldrError::RemotePackageNotFound(derivation.to_string(),
                                                 package.to_string(),
                                                 version.clone(),
                                                 release.clone())),
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
/// * Remote repository is not available
pub fn show_package_latest(repo: &str,
                           derivation: &str,
                           name: &str,
                           version: Option<&str>)
                           -> BldrResult<Package> {
    let url = url_show_package(repo, derivation, name, version);
    let client = Client::new();
    let request = client.get(&url);
    let mut res = try!(request.send());

    if res.status != hyper::status::StatusCode::Ok {
        let ver = if version.is_some() {
            Some(version.unwrap().to_string())
        } else {
            None
        };
        return Err(BldrError::RemotePackageNotFound(derivation.to_string(),
                                                    name.to_string(),
                                                    ver,
                                                    None));
    }

    let mut encoded = String::new();
    try!(res.read_to_string(&mut encoded));
    let package: Package = json::decode(&encoded).unwrap();
    Ok(package)
}

/// Upload a public key to a remote repository.
///
/// # Failures
///
/// * Remote repository is not available
/// * File cannot be read
pub fn put_key(repo: &str, path: &Path) -> BldrResult<()> {
    let mut file = try!(File::open(path));
    let file_name = try!(path.file_name().ok_or(BldrError::NoFilePart));
    let url = format!("{}/keys/{}", repo, file_name.to_string_lossy());
    upload(&url, &mut file)
}

/// Upload a package to a remote repository.
///
/// # Failures
///
/// * Remote repository is not available
/// * File cannot be read
pub fn put_package(repo: &str, package: &Package) -> BldrResult<()> {
    let mut file = try!(File::open(package.cache_file()));
    let url = format!("{}/pkgs/{}/{}/{}/{}",
                      repo,
                      package.derivation,
                      package.name,
                      package.version,
                      package.release);
    upload(&url, &mut file)
}

fn url_show_package(repo: &str, derivation: &str, name: &str, version: Option<&str>) -> String {
    if version.is_some() {
        format!("{}/pkgs/{}/{}/{}",
                repo,
                derivation,
                name,
                version.as_ref().unwrap())
    } else {
        format!("{}/pkgs/{}/{}", repo, derivation, name)
    }
}

fn download(status: &str, url: &str, path: &str) -> BldrResult<String> {
    debug!("Making request to url {}", url);
    let client = Client::new();
    let mut res = try!(client.get(url).send());
    debug!("Response: {:?}", res);

    if res.status != hyper::status::StatusCode::Ok {
        return Err(BldrError::HTTP(res.status));
    }

    let file_name = match res.headers.get::<XFileName>() {
        Some(filename) => format!("{}", filename),
        None => return Err(BldrError::NoXFilename),
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
                    return Err(BldrError::WriteSyncFailed);
                }
                written = written + (bytes_written as i64);
                progress(status, written, &length, false);
            }
        };
    }
    try!(fs::rename(&tempfile, &finalfile));
    Ok(finalfile)
}

fn upload(url: &str, file: &mut File) -> BldrResult<()> {
    debug!("Uploading to {}", url);
    let client = Client::new();
    let metadata = try!(file.metadata());
    let res = try!(client.post(url).body(Body::SizedBody(file, metadata.len())).send());
    debug!("Response {:?}", res);
    Ok(())
}

fn progress(status: &str, written: i64, length: &str, finished: bool) {
    let progress = format!("   {}: {}/{}", status, written, length);
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
