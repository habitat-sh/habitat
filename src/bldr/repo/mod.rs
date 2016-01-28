// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

pub mod client;

use iron::prelude::*;
use iron::status;
use iron::request::Body;
use iron::headers;
use router::Router;
use rustc_serialize::json;

use std::net;
use std::sync::Arc;
use std::fs::{self, File};
use std::io::{Read, Write, BufWriter};
use std::path::{Path, PathBuf};

use error::{BldrResult, ErrorKind};
use config::Config;

use package::{Package, PackageArchive};

static LOGKEY: &'static str = "RE";

header! { (XFileName, "X-Filename") => [String] }

struct Repo {
    pub path: String,
}

impl Repo {
    fn new(path: &str) -> BldrResult<Arc<Repo>> {
        Ok(Arc::new(Repo { path: String::from(path) }))
    }

    // Return a PackageArchive representing the given package. None is returned if the repository
    // doesn't have an archive for the given package.
    fn archive(&self,
               derivation: &str,
               name: &str,
               version: &str,
               release: &str)
               -> Option<PackageArchive> {
        let file = self.archive_path(derivation, name, version, release);
        match fs::metadata(&file) {
            Ok(_) => Some(PackageArchive::new(file)),
            Err(_) => None,
        }
    }

    // Return a PackageArchive representing the latest release available for the given package
    // derivation, name, and version (optional).
    //
    // If a version is specified the latest release of that version will be returned, if it is
    // omitted the latest release of the latest version is returned.
    fn archive_latest(&self,
                      derivation: &str,
                      name: &str,
                      version: Option<&str>)
                      -> Option<PackageArchive> {
        match Package::load(derivation,
                            name,
                            version.map(String::from),
                            None,
                            Some(self.packages_path().to_str().unwrap())) {
            Ok(package) => self.archive(&package.derivation,
                                        &package.name,
                                        &package.version,
                                        &package.release),
            Err(_) => None,
        }
    }

    // Return a formatted string representing the filename of an archive for the given package
    // identifier pieces.
    fn archive_path(&self, derivation: &str, name: &str, version: &str, release: &str) -> PathBuf {
        self.packages_path()
            .join(derivation)
            .join(name)
            .join(version)
            .join(release)
            .join(format!("{}-{}-{}-{}.bldr", &derivation, &name, &version, &release))
    }

    fn key_path(&self, name: &str) -> PathBuf {
        self.keys_path().join(format!("{}.asc", name))
    }

    fn keys_path(&self) -> PathBuf {
        Path::new(&self.path).join("keys")
    }

    fn packages_path(&self) -> PathBuf {
        Path::new(&self.path).join("pkgs")
    }
}

#[derive(Debug)]
pub struct ListenAddr(pub net::Ipv4Addr);
#[derive(Debug)]
pub struct ListenPort(pub u16);

impl Default for ListenAddr {
    fn default() -> Self {
        ListenAddr(net::Ipv4Addr::new(0, 0, 0, 0))
    }
}

impl Default for ListenPort {
    fn default() -> Self {
        ListenPort(9632)
    }
}

fn write_file(filename: &PathBuf, body: &mut Body) -> BldrResult<bool> {
    let path = filename.parent().unwrap();
    try!(fs::create_dir_all(path));
    let tempfile = format!("{}.tmp", filename.to_string_lossy());
    let f = try!(File::create(&tempfile));
    let mut writer = BufWriter::new(&f);
    let mut written: i64 = 0;
    let mut buf = [0u8; 100000]; // Our byte buffer
    loop {
        let len = try!(body.read(&mut buf)); // Raise IO errors
        match len {
            0 => {
                // 0 == EOF, so stop writing and finish progress
                break;
            }
            _ => {
                // Write the buffer to the BufWriter on the Heap
                let bytes_written = try!(writer.write(&buf[0..len]));
                if bytes_written == 0 {
                    return Err(bldr_error!(ErrorKind::WriteSyncFailed));
                }
                written = written + (bytes_written as i64);
            }
        };
    }
    outputln!("File added to repository at {}", filename.to_string_lossy());
    try!(fs::rename(&tempfile, &filename));
    Ok(true)
}

fn upload_key(repo: &Repo, req: &mut Request) -> IronResult<Response> {
    outputln!("Upload Key {:?}", req);
    let rext = req.extensions.get::<Router>().unwrap();
    let key = rext.find("key").unwrap();
    let file = repo.key_path(&key);

    try!(write_file(&file, &mut req.body));

    let short_name = file.file_name().unwrap().to_string_lossy();
    let mut response = Response::with((status::Created, format!("/key/{}", &short_name)));

    let mut base_url = req.url.clone();
    base_url.path = vec![String::from("key"), String::from(key)];
    response.headers.set(headers::Location(format!("{}", base_url)));
    Ok(response)
}

fn upload_package(repo: &Repo, req: &mut Request) -> IronResult<Response> {
    outputln!("Upload {:?}", req);
    let rext = req.extensions.get::<Router>().unwrap();

    let deriv = rext.find("deriv").unwrap();
    let pkg = rext.find("pkg").unwrap();
    let version = rext.find("version").unwrap();
    let release = rext.find("release").unwrap();

    let filename = repo.archive_path(deriv, pkg, version, release);
    try!(write_file(&filename, &mut req.body));

    let mut response = Response::with((status::Created,
                                       format!("/pkgs/{}/{}/{}/{}/download",
                                               deriv,
                                               pkg,
                                               version,
                                               release)));
    let mut base_url = req.url.clone();
    base_url.path = vec![String::from("pkgs"),
                         String::from(deriv),
                         String::from(pkg),
                         String::from(version),
                         String::from(release),
                         String::from("download")];
    response.headers.set(headers::Location(format!("{}", base_url)));
    Ok(response)
}

fn download_key(repo: &Repo, req: &mut Request) -> IronResult<Response> {
    outputln!("Download {:?}", req);
    let rext = req.extensions.get::<Router>().unwrap();

    let key = match rext.find("key") {
        Some(key) => key,
        None => return Ok(Response::with(status::BadRequest)),
    };

    let path = Path::new(&repo.path).join("keys");
    let short_filename = format!("{}.asc", key);
    let filename = path.join(&short_filename);

    let mut response = Response::with((status::Ok, filename));
    response.headers.set(XFileName(short_filename.clone()));

    Ok(response)
}

fn download_package(repo: &Repo, req: &mut Request) -> IronResult<Response> {
    outputln!("Download {:?}", req);
    let rext = req.extensions.get::<Router>().unwrap();

    let deriv = match rext.find("deriv") {
        Some(deriv) => deriv,
        None => return Ok(Response::with(status::BadRequest)),
    };
    let pkg = match rext.find("pkg") {
        Some(pkg) => pkg,
        None => return Ok(Response::with(status::BadRequest)),
    };
    let param_ver = rext.find("version");
    let param_rel = rext.find("release");

    let archive = if param_ver.is_some() && param_rel.is_some() {
        match repo.archive(&deriv,
                           &pkg,
                           param_ver.as_ref().unwrap(),
                           param_rel.as_ref().unwrap()) {
            Some(archive) => archive,
            None => return Ok(Response::with(status::NotFound)),
        }
    } else {
        match repo.archive_latest(&deriv, &pkg, param_ver) {
            Some(archive) => archive,
            None => return Ok(Response::with(status::NotFound)),
        }
    };

    match fs::metadata(&archive.path) {
        Ok(_) => {
            let mut response = Response::with((status::Ok, archive.path.clone()));
            response.headers.set(XFileName(archive.file_name()));
            Ok(response)
        }
        Err(_) => {
            Ok(Response::with(status::NotFound))
        }
    }
}

fn show_package(repo: &Repo, req: &mut Request) -> IronResult<Response> {
    let rext = req.extensions.get::<Router>().unwrap();
    let package = rext.find("pkg").unwrap();
    let deriv = rext.find("deriv").unwrap();
    let version = rext.find("version");
    let release = rext.find("release");

    let archive = if version.is_some() && release.is_some() {
        match repo.archive(&deriv,
                           &package,
                           version.as_ref().unwrap(),
                           release.as_ref().unwrap()) {
            Some(archive) => archive,
            None => return Ok(Response::with(status::NotFound)),
        }
    } else {
        match repo.archive_latest(&deriv, &package, version) {
            Some(archive) => archive,
            None => return Ok(Response::with(status::NotFound)),
        }
    };

    let package = try!(archive.package());
    let body = json::encode(&package).unwrap();
    Ok(Response::with((status::Ok, body)))
}

pub fn run(config: &Config) -> BldrResult<()> {
    let repo = try!(Repo::new(config.path()));
    let repo2 = repo.clone();
    let repo3 = repo.clone();
    let repo4 = repo.clone();
    let repo5 = repo.clone();
    let repo6 = repo.clone();
    let repo7 = repo.clone();
    let repo8 = repo.clone();
    let repo9 = repo.clone();
    let router = router!(
        post "/pkgs/:deriv/:pkg/:version/:release" => move |r: &mut Request| upload_package(&repo, r),
        get "/pkgs/:deriv/:pkg/:version/:release/download" => move |r: &mut Request| download_package(&repo2, r),
        get "/pkgs/:deriv/:pkg/:version/download" => move |r: &mut Request| download_package(&repo3, r),
        get "/pkgs/:deriv/:pkg/download" => move |r: &mut Request| download_package(&repo4, r),
        get "/pkgs/:deriv/:pkg/:version/:release" => move |r: &mut Request| show_package(&repo5, r),
        get "/pkgs/:deriv/:pkg/:version" => move |r: &mut Request| show_package(&repo6, r),
        get "/pkgs/:deriv/:pkg" => move |r: &mut Request| show_package(&repo7, r),

        post "/keys/:key" => move |r: &mut Request| upload_key(&repo8, r),
        get "/keys/:key" => move |r: &mut Request| download_key(&repo9, r)
    );
    Iron::new(router).http(config.repo_addr()).unwrap();
    Ok(())
}
