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

use error::{BldrError, BldrResult};
use config::Config;

use pkg::Package;

header! { (XFileName, "X-Filename") => [String] }

struct Repo {
    pub path: String,
}

impl Repo {
    fn new(path: &str) -> BldrResult<Arc<Repo>> {
        Ok(Arc::new(Repo { path: String::from(path) }))
    }
}

pub struct ListenAddr(pub net::Ipv4Addr);
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

fn write_file(path: PathBuf, filename: PathBuf, body: &mut Body) -> BldrResult<bool> {
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
                    return Err(BldrError::WriteSyncFailed);
                }
                written = written + (bytes_written as i64);
            }
        };
    }
    println!("   repo: file added to repository at {}",
             filename.to_string_lossy());
    try!(fs::rename(&tempfile, &filename));
    Ok(true)
}

fn upload_key(repo: &Repo, req: &mut Request) -> IronResult<Response> {
    println!("Upload Key {:?}", req);
    let rext = req.extensions.get::<Router>().unwrap();

    let key = rext.find("key").unwrap();

    let path = Path::new(&repo.path).join("keys");
    let short_name = format!("{}.asc", key);
    let filename = path.join(&short_name);

    try!(write_file(path, filename, &mut req.body));

    let mut response = Response::with((status::Created, format!("/key/{}", &short_name)));

    let mut base_url = req.url.clone();
    base_url.path = vec![String::from("key"), String::from(key)];
    response.headers.set(headers::Location(format!("{}", base_url)));
    Ok(response)
}

fn upload_package(repo: &Repo, req: &mut Request) -> IronResult<Response> {
    println!("Upload {:?}", req);
    let rext = req.extensions.get::<Router>().unwrap();

    let deriv = rext.find("deriv").unwrap();
    let pkg = rext.find("pkg").unwrap();
    let version = rext.find("version").unwrap();
    let release = rext.find("release").unwrap();

    let path = Path::new(&repo.path)
                   .join(format!("pkgs/{}/{}/{}/{}", deriv, pkg, version, release));
    let filename = path.join(format!("{}-{}-{}-{}.bldr", deriv, pkg, version, release));

    try!(write_file(path, filename, &mut req.body));

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
    println!("Download {:?}", req);
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
    println!("Download {:?}", req);
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

    let (version, release) = if param_ver.is_some() && param_rel.is_some() {
        (param_ver.unwrap().to_string(),
         param_rel.unwrap().to_string())
    } else {
        match Package::latest(deriv, pkg, None, Some(&format!("{}/pkgs", &repo.path))) {
            Ok(package) => (package.version, package.release),
            Err(BldrError::Io(_)) => return Ok(Response::with(status::NotFound)),
            Err(_) => return Ok(Response::with(status::InternalServerError)),
        }
    };

    let short_filename = format!("{}-{}-{}-{}.bldr", deriv, pkg, version, release);
    let path = Path::new(&repo.path)
                   .join(format!("pkgs/{}/{}/{}/{}", deriv, pkg, version, release));
    let file = path.join(&short_filename);

    match fs::metadata(&file) {
        Ok(_) => {
            let mut response = Response::with((status::Ok, file));
            response.headers.set(XFileName(short_filename.clone()));
            Ok(response)
        }
        Err(_) => {
            Ok(Response::with(status::NotFound))
        }
    }
}

fn show_package_latest(repo: &Repo, req: &mut Request) -> IronResult<Response> {
    let rext = req.extensions.get::<Router>().unwrap();
    let package = rext.find("pkg").unwrap();
    let deriv = rext.find("deriv").unwrap();
    let version = rext.find("version");
    match Package::latest(&deriv,
                          &package,
                          version,
                          Some(&format!("{}/pkgs", &repo.path))) {
        Ok(package) => {
            let body = json::encode(&package).unwrap();
            Ok(Response::with((status::Ok, body)))
        }
        Err(_) => {
            Ok(Response::with((status::NotFound)))
        }
    }
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
    let router = router!(
        post "/pkgs/:deriv/:pkg/:version/:release" => move |r: &mut Request| upload_package(&repo, r),
        get "/pkgs/:deriv/:pkg/:version/:release/download" => move |r: &mut Request| download_package(&repo2, r),
        get "/pkgs/:deriv/:pkg/:version/download" => move |r: &mut Request| download_package(&repo3, r),
        get "/pkgs/:deriv/:pkg/download" => move |r: &mut Request| download_package(&repo4, r),
        get "/pkgs/:deriv/:pkg/:version" => move |r: &mut Request| show_package_latest(&repo5, r),
        get "/pkgs/:deriv/:pkg" => move |r: &mut Request| show_package_latest(&repo6, r),

        post "/keys/:key" => move |r: &mut Request| upload_key(&repo7, r),
        get "/keys/:key" => move |r: &mut Request| download_key(&repo8, r)
    );
    Iron::new(router).http(config.repo_addr()).unwrap();
    Ok(())
}
