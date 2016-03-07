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
use urlencoded::UrlEncodedQuery;

use std::net;
use std::sync::Arc;
use std::fs::{self, File};
use std::io::{Read, Write, BufWriter};
use std::path::{Path, PathBuf};

use error::{BldrError, BldrResult, ErrorKind};
use config::Config;
use self::data_store::{Cursor, DataStore, Database, Transaction};
use self::data_object::DataObject;
use package::{self, PackageArchive};

static LOGKEY: &'static str = "RE";

header! { (XFileName, "X-Filename") => [String] }
header! { (ETag, "ETag") => [String] }

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
            .join(format!("{}-{}-{}-{}.bldr",
                          &ident.origin,
                          &ident.name,
                          ident.version.as_ref().unwrap(),
                          ident.release.as_ref().unwrap()))
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
        package::PackageIdent::new(self.find("origin").unwrap(),
                                   self.find("pkg").unwrap(),
                                   self.find("version"),
                                   self.find("release"))
    }
}

impl<'a> Into<data_object::PackageIdent> for &'a Params {
    fn into(self) -> data_object::PackageIdent {
        let ident: package::PackageIdent = self.into();
        data_object::PackageIdent::new(ident)
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
    let checksum = match extract_query_value("checksum", req) {
        Some(checksum) => checksum,
        None => return Ok(Response::with(status::BadRequest)),
    };
    let params = req.extensions.get::<Router>().unwrap();
    let ident: package::PackageIdent = params.into();

    if !ident.fully_qualified() {
        return Ok(Response::with(status::BadRequest));
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
    debug!("Package Archive: {:#?}", archive);
    let object = match data_object::Package::from_archive(&archive) {
        Ok(object) => object,
        Err(e) => {
            debug!("Error building package from archive: {:#?}", e);
            return Ok(Response::with(status::UnprocessableEntity));
        }
    };
    if object.checksum != checksum {
        debug!("Checksums did not match: expected={:?}, got={:?}", checksum, object.checksum);
        return Ok(Response::with(status::UnprocessableEntity));
    }
    if ident.satisfies(&object.ident) {
        // JW TODO: handle write errors here. Storage full as a 507.
        try!(repo.datastore.packages.write(&txn, &object));
        try!(txn.commit());

        let mut response = Response::with((status::Created,
                                           format!("/pkgs/{}/download", object.ident)));
        let mut base_url = req.url.clone();
        base_url.path = vec![String::from("pkgs"),
                             object.ident.to_string(),
                             String::from("download")];
        response.headers.set(headers::Location(format!("{}", base_url)));
        Ok(response)
    } else {
        debug!("Ident failed to satisfy: {:#?}", ident);
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
    let ident: data_object::PackageIdent = params.into();

    let result = {
        let txn = try!(repo.datastore.packages.txn_ro());
        match txn.get(&ident.to_string()) {
            Ok(package) => {
                let value: package::PackageIdent = package.ident.into();
                Ok(value)
            }
            Err(e) => Err(e),
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
        }
        Err(BldrError { err: ErrorKind::MdbError(data_store::MdbError::NotFound), ..}) =>
            Ok(Response::with((status::NotFound))),
        Err(_) => unreachable!("unknown error"),
    }
}

fn list_packages(repo: &Repo, req: &mut Request) -> IronResult<Response> {
    let params = req.extensions.get::<Router>().unwrap();

    if let Some(view) = params.find("repo") {
        list_packages_scoped_to_repo(repo, view)
    } else {
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
}

fn list_packages_scoped_to_repo(repo: &Repo, view: &str) -> IronResult<Response> {
    let txn = try!(repo.datastore.views.view_pkg_idx.txn_ro());
    let mut cursor = try!(txn.cursor_ro());
    match cursor.set_key(&view.to_string()) {
        Ok((_, pkg)) => {
            let mut packages: Vec<data_object::PackageIdent> = vec![];
            packages.push(pkg);
            loop {
                if let Some((_, pkg)) = cursor.next_dup().ok() {
                    packages.push(pkg);
                } else {
                    break;
                }
            }
            let body = json::encode(&packages).unwrap();
            Ok(Response::with((status::Ok, body)))
        }
        Err(BldrError { err: ErrorKind::MdbError(data_store::MdbError::NotFound), ..}) =>
            Ok(Response::with((status::NotFound))),
        Err(_) => unreachable!("unknown error"),
    }
}

fn list_repos(depot: &Repo, _req: &mut Request) -> IronResult<Response> {
    let txn = try!(depot.datastore.views.txn_ro());
    let mut cursor = try!(txn.cursor_ro());
    let mut repos: Vec<data_object::View> = vec![];
    loop {
        match cursor.next() {
            Ok((_, data)) => repos.push(data),
            Err(_) => break,
        }
    }
    let body = json::encode(&repos).unwrap();
    Ok(Response::with((status::Ok, body)))
}

fn show_package(depot: &Repo, req: &mut Request) -> IronResult<Response> {
    let params = req.extensions.get::<Router>().unwrap();
    let ident: data_object::PackageIdent = params.into();

    if let Some(repo) = params.find("repo") {
        if let Some(pkg) = try!(latest_package_in_repo(&ident, depot, repo)) {
            let txn = try!(depot.datastore.packages.txn_ro());
            let package = try!(txn.get(&pkg.ident()));
            let body = json::encode(&package).unwrap();
            Ok(Response::with((status::Ok, body)))
        } else {
            Ok(Response::with((status::NotFound)))
        }
    } else {
        let result = if ident.fully_qualified() {
            let txn = try!(depot.datastore.packages.txn_ro());
            txn.get(&ident.to_string())
        } else {
            let r = {
                let idx = try!(depot.datastore.packages.index.txn_ro());
                let mut cursor = try!(idx.cursor_ro());
                if let Some(e) = cursor.set_key(&ident.to_string()).err() {
                    Err(e)
                } else {
                    cursor.last_dup()
                }
            };
            match r {
                Ok(v) => {
                    let txn = try!(depot.datastore.packages.txn_ro());
                    txn.get(&v.ident())
                }
                Err(e) => Err(e),
            }
        };
        match result {
            Ok(data) => {
                let body = json::encode(&data).unwrap();
                let mut response = Response::with((status::Ok, body));
                response.headers.set(ETag(data.checksum));
                Ok(response)
            }
            Err(BldrError { err: ErrorKind::MdbError(data_store::MdbError::NotFound), ..}) =>
                Ok(Response::with((status::NotFound))),
            Err(e) => unreachable!("unknown error: {:?}", e),
        }
    }
}

fn latest_package_in_repo<P: AsRef<package::PackageIdent>>(ident: P, depot: &Repo, repo: &str) -> BldrResult<Option<data_object::PackageIdent>> {
    let txn = try!(depot.datastore.views.view_pkg_idx.txn_ro());
    let mut cursor = try!(txn.cursor_ro());
    match cursor.set_key(&repo.to_string()) {
        Ok(_) => {
            let mut pkg = try!(cursor.last_dup());
            loop {
                if ident.as_ref().satisfies(&pkg) {
                    return Ok(Some(pkg));
                } else {
                    match cursor.prev_dup() {
                        Ok((_, next)) => {
                            pkg = next;
                            continue;
                        },
                        Err(BldrError { err: ErrorKind::MdbError(data_store::MdbError::NotFound), ..}) => {
                            return Ok(None);
                        },
                        Err(_) => unreachable!("unknown error"),
                    }
                }
            }
        }
        Err(BldrError { err: ErrorKind::MdbError(data_store::MdbError::NotFound), ..}) => {
            return Ok(None);
        },
        Err(_) => unreachable!("unknown error"),
    }
}

fn promote_package(depot: &Repo, req: &mut Request) -> IronResult<Response> {
    let params = req.extensions.get::<Router>().unwrap();
    let repo = params.find("repo").unwrap();

    let txn = try!(depot.datastore.views.txn_rw());
    match txn.get(&repo.to_string()) {
        Ok(view) => {
            let ident: package::PackageIdent = params.into();
            let nested = try!(txn.new_child_rw(&depot.datastore.packages));
            match nested.get(&ident.to_string()) {
                Ok(package) => {
                    try!(depot.datastore.views.associate(&nested, &view, &package));
                    try!(nested.commit());
                    Ok(Response::with((status::Ok)))
                },
                Err(_) => Ok(Response::with((status::NotFound)))
            }
        },
        Err(_) => Ok(Response::with((status::NotFound)))
    }
}

fn extract_query_value(key: &str, req: &mut Request) -> Option<String> {
    match req.get_ref::<UrlEncodedQuery>() {
        Ok(map) => {
            for (k, v) in map.iter() {
                if key == *k {
                    if v.len() < 1 {
                        return None;
                    }
                    return Some(v[0].clone());
                }
            }
            None
        }
        Err(_) => None,
    }
}

pub fn repair(config: &Config) -> BldrResult<()> {
    let repo = try!(Repo::new(String::from(config.path())));
    repo.datastore.clear()
}

pub fn run(config: &Config) -> BldrResult<()> {
    let repo = try!(Repo::new(String::from(config.path())));
    let repo1 = repo.clone();
    let repo2 = repo.clone();
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
        get "/repos" => move |r: &mut Request| list_repos(&repo1, r),
        get "/repos/:repo/pkgs/:origin/:pkg" => move |r: &mut Request| list_packages(&repo2, r),
        get "/repos/:repo/pkgs/:origin/:pkg/latest" => move |r: &mut Request| show_package(&repo3, r),
        get "/repos/:repo/pkgs/:origin/:pkg/:version" => move |r: &mut Request| list_packages(&repo4, r),
        get "/repos/:repo/pkgs/:origin/:pkg/:version/latest" => move |r: &mut Request| show_package(&repo5, r),
        get "/repos/:repo/pkgs/:origin/:pkg/:version/:release" => move |r: &mut Request| show_package(&repo6, r),

        post "/repos/:repo/pkgs/:origin/:pkg/:version/:release/promote" => move |r: &mut Request| promote_package(&repo7, r),

        get "/pkgs/:origin" => move |r: &mut Request| list_packages(&repo8, r),
        get "/pkgs/:origin/:pkg" => move |r: &mut Request| list_packages(&repo9, r),
        get "/pkgs/:origin/:pkg/latest" => move |r: &mut Request| show_package(&repo10, r),
        get "/pkgs/:origin/:pkg/:version" => move |r: &mut Request| list_packages(&repo11, r),
        get "/pkgs/:origin/:pkg/:version/latest" => move |r: &mut Request| show_package(&repo12, r),
        get "/pkgs/:origin/:pkg/:version/:release" => move |r: &mut Request| show_package(&repo13, r),

        get "/pkgs/:origin/:pkg/:version/:release/download" => move |r: &mut Request| download_package(&repo14, r),
        post "/pkgs/:origin/:pkg/:version/:release" => move |r: &mut Request| upload_package(&repo15, r),

        post "/keys/:key" => move |r: &mut Request| upload_key(&repo16, r),
        get "/keys/:key" => move |r: &mut Request| download_key(&repo17, r)
    );
    Iron::new(router).http(config.repo_addr()).unwrap();
    Ok(())
}
