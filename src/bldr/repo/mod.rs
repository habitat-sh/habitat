// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

pub mod client;
pub mod data_object;
pub mod data_store;

use iron::prelude::*;
use iron::status;
use iron::request::Body;
use iron::headers;
use router::{Params, Router};
use rustc_serialize::json;

use std::net;
use std::sync::Arc;
use std::fs::{self, File};
use std::io::{Read, Write, BufWriter};
use std::path::{Path, PathBuf};

use error::{BldrError, BldrResult, ErrorKind};
use config::Config;
use self::data_store::{Cursor, DataStore, Database, Transaction};
use self::data_object::DataObject;
use package::{self, Package, PackageArchive};

static LOGKEY: &'static str = "RE";

header! { (XFileName, "X-Filename") => [String] }

pub struct Repo {
    pub path: String,
    pub datastore: DataStore,
}

impl Repo {
    pub fn new(path: String) -> BldrResult<Arc<Repo>> {
        let dbpath = Path::new(&path).join("datastore");
        let datastore = try!(DataStore::open(dbpath.as_path()));
        Ok(Arc::new(Repo {
            path: path,
            datastore: datastore,
        }))
    }

    // Return a PackageArchive representing the given package. None is returned if the repository
    // doesn't have an archive for the given package.
    fn archive(&self, ident: &package::PackageIdent) -> Option<PackageArchive> {
        let file = self.archive_path(&ident);
        match fs::metadata(&file) {
            Ok(_) => Some(PackageArchive::new(file)),
            Err(_) => None,
        }
    }

    // Return a formatted string representing the filename of an archive for the given package
    // identifier pieces.
    fn archive_path(&self, ident: &package::PackageIdent) -> PathBuf {
        self.packages_path()
            .join(&ident.origin)
            .join(&ident.name)
            .join(ident.version.as_ref().unwrap())
            .join(ident.release.as_ref().unwrap())
            .join(format!("{}-{}-{}-{}.bldr", &ident.origin, &ident.name, ident.version.as_ref().unwrap(), ident.release.as_ref().unwrap()))
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

impl<'a> Into<package::PackageIdent> for &'a Params {
    fn into(self) -> package::PackageIdent {
        package::PackageIdent::new(self.find("origin").unwrap(), self.find("pkg").unwrap(), self.find("version"), self.find("release"))
    }
}

impl<'a> Into<data_object::PackageIdent> for &'a Params {
    fn into(self) -> data_object::PackageIdent {
        let origin = self.find("origin").unwrap();
        let name = self.find("pkg");
        let version = self.find("version");
        let release = self.find("release");
        if release.is_some() && version.is_some() && name.is_some() {
            data_object::PackageIdent::new(format!("{}/{}/{}/{}", origin, name.unwrap(), version.unwrap(), release.unwrap()))
        } else if version.is_some() && name.is_some() {
            data_object::PackageIdent::new(format!("{}/{}/{}", origin, name.unwrap(), version.unwrap()))
        } else if name.is_some() {
            data_object::PackageIdent::new(format!("{}/{}", origin, name.unwrap()))
        } else {
            data_object::PackageIdent::new(origin.to_string())
        }
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
    let ident: package::PackageIdent = {
        let params = req.extensions.get::<Router>().unwrap();
        params.into()
    };

    if !ident.fully_qualified() {
        return Ok(Response::with((status::BadRequest)));
    }

    let txn = try!(repo.datastore.packages.txn_rw());
    if let Ok(_) = txn.get(&ident.to_string()) {
        if let Some(_) = repo.archive(&ident) {
            return Ok(Response::with((status::Conflict)));
        } else {
            // This should never happen. Writing the package to disk and recording it's existence
            // in the metadata is a transactional operation and one cannot exist without the other.
            //
            // JW TODO: write the depot repair tool and wire it into the `bldr-depot repair` command
            panic!("Inconsistent package metadata! Exit and run `bldr-depot repair` to fix data integrity.");
        }
    }

    let filename = repo.archive_path(&ident);
    try!(write_file(&filename, &mut req.body));
    let archive = PackageArchive::new(filename);
    let object = match data_object::Package::from_archive(&archive) {
        Ok(object) => object,
        Err(_) => return Ok(Response::with(status::UnprocessableEntity)),
    };
    if ident.satisfies(&object.ident.clone().into()) {
        // JW TODO: handle failure here?
        try!(repo.datastore.packages.write(&txn, &object));
        try!(txn.commit());

        let mut response = Response::with((status::Created, format!("/pkgs/{}/download", object.ident)));
        let mut base_url = req.url.clone();
        let parts: Vec<&str> = object.ident.parts();
        base_url.path = vec![String::from("pkgs"),
                             parts[0].to_string(),
                             parts[1].to_string(),
                             parts[2].to_string(),
                             parts[3].to_string(),
                             String::from("download")];
        response.headers.set(headers::Location(format!("{}", base_url)));
        Ok(response)
    } else {
        Ok(Response::with(status::UnprocessableEntity))
    }
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
    let params = req.extensions.get::<Router>().unwrap();
    // JW TODO: check for repo param
    let ident: data_object::PackageIdent = params.into();

    let result = if ident.parts().len() == 4 {
        let txn = try!(repo.datastore.packages.txn_ro());
        match txn.get(&ident.to_string()) {
            Ok(package) => {
                let value: package::PackageIdent = package.ident.into();
                Ok(value)
            },
            Err(e) => Err(e)
        }
    } else {
        // JW TODO: fix scoping of cursor/transactions and refactor this
        let r = {
            let idx = try!(repo.datastore.packages.index.txn_ro());
            let mut cursor = try!(idx.cursor_ro());
            if let Some(e) = cursor.set_key(&ident.to_string()).err() {
                Err(e)
            } else {
                cursor.last_dup()
            }
        };
        match r {
            Ok(v) => {
                let txn = try!(repo.datastore.packages.txn_ro());
                let value: package::PackageIdent = try!(txn.get(&v.ident())).ident.into();
                Ok(value)
            },
            Err(e) => Err(BldrError::from(e))
        }
    };

    match result {
        Ok(ident) => {
            if let Some(archive) = repo.archive(&ident) {
                match fs::metadata(&archive.path) {
                    Ok(_) => {
                        let mut response = Response::with((status::Ok, archive.path.clone()));
                        response.headers.set(XFileName(archive.file_name()));
                        Ok(response)
                    }
                    Err(_) => Ok(Response::with(status::NotFound)),
                }
            } else {
                // This should never happen. Writing the package to disk and recording it's existence
                // in the metadata is a transactional operation and one cannot exist without the other.
                //
                // JW TODO: write the depot repair tool and wire it into the `bldr-depot repair` command
                panic!("Inconsistent package metadata! Exit and run `bldr-depot repair` to fix data integrity.");
            }
        },
        Err(BldrError { err: ErrorKind::MdbError(data_store::MdbError::NotFound), ..}) => Ok(Response::with((status::NotFound))),
        Err(_) => unreachable!("unknown error"),
    }
}

fn list_packages(repo: &Repo, req: &mut Request) -> IronResult<Response> {
    let params = req.extensions.get::<Router>().unwrap();
    let ident: data_object::PackageIdent = params.into();
    let mut packages: Vec<package::PackageIdent> = vec![];

    let txn = try!(repo.datastore.packages.index.txn_ro());
    let mut cursor = try!(txn.cursor_ro());
    let result = match cursor.set_key(ident.ident()) {
        Ok((_, value)) => {
            packages.push(value.into());
            loop {
                match cursor.next_dup() {
                    Ok((_, value)) => packages.push(value.into()),
                    Err(_) => break
                }
            }
            Ok(())
        },
        Err(e) => Err(BldrError::from(e))
    };

    match result {
        Ok(()) => {
            let body = json::encode(&packages).unwrap();
            Ok(Response::with((status::Ok, body)))
        },
        Err(BldrError { err: ErrorKind::MdbError(data_store::MdbError::NotFound), ..}) => Ok(Response::with((status::NotFound))),
        Err(_) => unreachable!("unknown error"),
    }
}

fn show_package(repo: &Repo, req: &mut Request) -> IronResult<Response> {
    let params = req.extensions.get::<Router>().unwrap();
    let ident: package::PackageIdent = params.into();

    let result = if ident.fully_qualified() {
        let txn = try!(repo.datastore.packages.txn_ro());
        txn.get(&ident.to_string())
    } else {
        let r = {
            let idx = try!(repo.datastore.packages.index.txn_ro());
            let mut cursor = try!(idx.cursor_ro());
            if let Some(e) = cursor.set_key(&ident.to_string()).err() {
                Err(e)
            } else {
                cursor.last_dup()
            }
        };
        match r {
            Ok(v) => {
                let txn = try!(repo.datastore.packages.txn_ro());
                txn.get(&v.ident())
            },
            Err(e) => Err(e)
        }
    };
    match result {
        Ok(data) => {
            // JW TODO: re-enable proper json encoding when I have a plan for proper decoding
            // let body = json::encode(&data.to_json()).unwrap();
            let body = json::encode(&data).unwrap();
            Ok(Response::with((status::Ok, body)))
        },
        Err(BldrError { err: ErrorKind::MdbError(data_store::MdbError::NotFound), ..}) => Ok(Response::with((status::NotFound))),
        Err(e) => unreachable!("unknown error: {:?}", e),
    }
}

pub fn repair(config: &Config) -> BldrResult<()> {
    let repo = try!(Repo::new(String::from(config.path())));
    repo.datastore.clear()
}

pub fn run(config: &Config) -> BldrResult<()> {
    let repo = try!(Repo::new(String::from(config.path())));
    // let repo2 = repo.clone();
    let repo3 = repo.clone();
    let repo4 = repo.clone();
    let repo5 = repo.clone();
    let repo6 = repo.clone();
    let repo7 = repo.clone();
    let repo8 = repo.clone();
    let repo9 = repo.clone();
    let repo10 = repo.clone();
    let repo11 = repo.clone();
    let repo12 = repo.clone();
    let repo13 = repo.clone();
    let repo14 = repo.clone();
    let repo15 = repo.clone();
    let repo16 = repo.clone();
    let repo17 = repo.clone();
    let router = router!(
        // JW TODO: update list/show/download function to cover scoping rules of these routes repos
        get "/pkgs/:origin/:pkg/:version" => move |r: &mut Request| list_packages(&repo3, r),
        get "/pkgs/:origin/:pkg" => move |r: &mut Request| list_packages(&repo4, r),
        get "/pkgs/:origin/:pkg/latest" => move |r: &mut Request| show_package(&repo5, r),
        get "/pkgs/:origin/:pkg/:version/:release/download" => move |r: &mut Request| download_package(&repo6, r),
        get "/pkgs/:origin/:pkg/:version/download" => move |r: &mut Request| download_package(&repo7, r),
        get "/pkgs/:origin/:pkg/download" => move |r: &mut Request| download_package(&repo8, r),

        post "/pkgs/:origin/:pkg/:version/:release" => move |r: &mut Request| upload_package(&repo9, r),
        get "/pkgs/:origin/:pkg/:version/latest" => move |r: &mut Request| show_package(&repo10, r),
        get "/pkgs/:origin/:pkg/:version/:release" => move |r: &mut Request| show_package(&repo11, r),
        get "/pkgs/:origin/:pkg/latest" => move |r: &mut Request| show_package(&repo12, r),
        get "/pkgs/:origin/:pkg/:version" => move |r: &mut Request| list_packages(&repo13, r),
        get "/pkgs/:origin/:pkg" => move |r: &mut Request| list_packages(&repo14, r),
        get "/pkgs/:origin" => move |r: &mut Request| list_packages(&repo15, r),

        post "/keys/:key" => move |r: &mut Request| upload_key(&repo16, r),
        get "/keys/:key" => move |r: &mut Request| download_key(&repo17, r)
    );
    Iron::new(router).http(config.repo_addr()).unwrap();
    Ok(())
}
