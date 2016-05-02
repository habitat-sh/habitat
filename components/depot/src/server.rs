// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

use std::fs::{self, File};
use std::io::{Read, Write, BufWriter};
use std::path::PathBuf;

use dbcache;
use depot_core::{ETag, XFileName};
use depot_core::data_object::{self, DataObject};
use iron::prelude::*;
use iron::{status, headers, AfterMiddleware};
use iron::request::Body;
use router::{Params, Router};
use rustc_serialize::json;
use urlencoded::UrlEncodedQuery;

use super::Depot;
use config::Config;
use error::{Error, Result};
use hcore::package::{self, PackageArchive};

fn write_file(filename: &PathBuf, body: &mut Body) -> Result<bool> {
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
                    return Err(Error::WriteSyncFailed);
                }
                written = written + (bytes_written as i64);
            }
        };
    }
    info!("File added to Depot at {}", filename.to_string_lossy());
    try!(fs::rename(&tempfile, &filename));
    Ok(true)
}

fn upload_key(depot: &Depot, req: &mut Request) -> IronResult<Response> {
    debug!("Upload Key {:?}", req);
    let rext = req.extensions.get::<Router>().unwrap();
    let key = rext.find("key").unwrap();
    let file = depot.key_path(&key);

    try!(write_file(&file, &mut req.body));

    let short_name = file.file_name().unwrap().to_string_lossy();
    let mut response = Response::with((status::Created, format!("/key/{}", &short_name)));

    let mut base_url = req.url.clone();
    base_url.path = vec![String::from("key"), String::from(key)];
    response.headers.set(headers::Location(format!("{}", base_url)));
    Ok(response)
}

fn upload_package(depot: &Depot, req: &mut Request) -> IronResult<Response> {
    debug!("Upload {:?}", req);
    let checksum_from_param = match extract_query_value("checksum", req) {
        Some(checksum) => checksum,
        None => return Ok(Response::with(status::BadRequest)),
    };
    let params = req.extensions.get::<Router>().unwrap();
    let ident: package::PackageIdent = extract_ident(params);

    if !ident.fully_qualified() {
        return Ok(Response::with(status::BadRequest));
    }

    match depot.datastore.packages.get(&ident) {
        Ok(_) |
        Err(Error::DataStore(dbcache::Error::EntityNotFound)) => {
            if let Some(_) = depot.archive(&ident) {
                return Ok(Response::with((status::Conflict)));
            }
        }
        Err(e) => {
            error!("upload_package:1, err={:?}", e);
            return Ok(Response::with(status::InternalServerError));
        }
    }

    let filename = depot.archive_path(&ident);
    try!(write_file(&filename, &mut req.body));
    let mut archive = PackageArchive::new(filename);
    debug!("Package Archive: {:#?}", archive);
    let checksum_from_artifact = match archive.checksum() {
        Ok(cksum) => cksum,
        Err(e) => {
            info!("Could not compute a checksum for {:#?}: {:#?}", archive, e);
            return Ok(Response::with(status::UnprocessableEntity));
        }
    };
    if checksum_from_param != checksum_from_artifact {
        info!("Checksums did not match: from_param={:?}, from_artifact={:?}",
              checksum_from_param,
              checksum_from_artifact);
        return Ok(Response::with(status::UnprocessableEntity));
    }
    let object = match data_object::Package::from_archive(&mut archive) {
        Ok(object) => object,
        Err(e) => {
            info!("Error building package from archive: {:#?}", e);
            return Ok(Response::with(status::UnprocessableEntity));
        }
    };
    if ident.satisfies(&object.ident) {
        depot.datastore.packages.write(&object).unwrap();
        let mut response = Response::with((status::Created,
                                           format!("/pkgs/{}/download", object.ident)));
        let mut base_url = req.url.clone();
        base_url.path = vec![String::from("pkgs"),
                             object.ident.to_string(),
                             String::from("download")];
        response.headers.set(headers::Location(format!("{}", base_url)));
        Ok(response)
    } else {
        info!("Ident mismatch, expected={:?}, got={:?}",
              ident,
              &object.ident);
        Ok(Response::with(status::UnprocessableEntity))
    }
}

fn download_key(depot: &Depot, req: &mut Request) -> IronResult<Response> {
    debug!("Download {:?}", req);
    let rext = req.extensions.get::<Router>().unwrap();

    let key = match rext.find("key") {
        Some(key) => key,
        None => return Ok(Response::with(status::BadRequest)),
    };

    let short_filename = format!("{}.asc", key);
    let filename = depot.keys_path().join(&short_filename);

    let mut response = Response::with((status::Ok, filename));
    response.headers.set(XFileName(short_filename.clone()));

    Ok(response)
}

fn download_package(depot: &Depot, req: &mut Request) -> IronResult<Response> {
    debug!("Download {:?}", req);
    let params = req.extensions.get::<Router>().unwrap();
    let ident: data_object::PackageIdent = extract_data_ident(params);

    match depot.datastore.packages.get(&ident) {
        Ok(ident) => {
            if let Some(archive) = depot.archive(&ident) {
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
                panic!("Inconsistent package metadata! Exit and run `hab-depot repair` to fix data integrity.");
            }
        }
        Err(Error::DataStore(dbcache::Error::EntityNotFound)) => {
            Ok(Response::with((status::NotFound)))
        }
        Err(e) => {
            error!("download_package:1, err={:?}", e);
            Ok(Response::with(status::InternalServerError))
        }
    }
}

fn list_packages(depot: &Depot, req: &mut Request) -> IronResult<Response> {
    let params = req.extensions.get::<Router>().unwrap();
    let ident: String = if params.find("pkg").is_none() {
        match params.find("origin") {
            Some(origin) => origin.to_string(),
            None => return Ok(Response::with(status::BadRequest)),
        }
    } else {
        extract_data_ident(params).ident().to_owned()
    };

    if let Some(view) = params.find("view") {
        match depot.datastore.views.view_pkg_idx.all(view, &ident) {
            Ok(packages) => {
                let body = json::encode(&packages).unwrap();
                Ok(Response::with((status::Ok, body)))
            }
            Err(Error::DataStore(dbcache::Error::EntityNotFound)) => {
                Ok(Response::with((status::NotFound)))
            }
            Err(e) => {
                error!("list_packages:1, err={:?}", e);
                Ok(Response::with(status::InternalServerError))
            }
        }
    } else {
        match depot.datastore.packages.index.all(&ident) {
            Ok(packages) => {
                let body = json::encode(&packages).unwrap();
                Ok(Response::with((status::Ok, body)))
            }
            Err(Error::DataStore(dbcache::Error::EntityNotFound)) => {
                Ok(Response::with((status::NotFound)))
            }
            Err(e) => {
                error!("list_packages:2, err={:?}", e);
                Ok(Response::with(status::InternalServerError))
            }
        }
    }
}

fn list_views(depot: &Depot, _req: &mut Request) -> IronResult<Response> {
    let views = try!(depot.datastore.views.all());
    let body = json::encode(&views).unwrap();
    Ok(Response::with((status::Ok, body)))
}

fn show_package(depot: &Depot, req: &mut Request) -> IronResult<Response> {
    let params = req.extensions.get::<Router>().unwrap();
    let mut ident: data_object::PackageIdent = extract_data_ident(params);

    if let Some(view) = params.find("view") {
        if !ident.fully_qualified() {
            match depot.datastore.views.view_pkg_idx.latest(view, &ident.to_string()) {
                Ok(ident) => {
                    match depot.datastore.packages.get(&ident) {
                        Ok(pkg) => render_package(&pkg),
                        Err(Error::DataStore(dbcache::Error::EntityNotFound)) => {
                            Ok(Response::with(status::NotFound))
                        }
                        Err(e) => {
                            error!("show_package:1, err={:?}", e);
                            Ok(Response::with(status::InternalServerError))
                        }
                    }
                }
                Err(Error::DataStore(dbcache::Error::EntityNotFound)) => {
                    Ok(Response::with(status::NotFound))
                }
                Err(e) => {
                    error!("show_package:2, err={:?}", e);
                    Ok(Response::with(status::InternalServerError))
                }
            }
        } else {
            match depot.datastore.views.view_pkg_idx.is_member(view, &ident) {
                Ok(true) => {
                    match depot.datastore.packages.get(&ident) {
                        Ok(pkg) => render_package(&pkg),
                        Err(Error::DataStore(dbcache::Error::EntityNotFound)) => {
                            Ok(Response::with(status::NotFound))
                        }
                        Err(e) => {
                            error!("show_package:3, err={:?}", e);
                            Ok(Response::with(status::InternalServerError))
                        }
                    }
                }
                Ok(false) => Ok(Response::with(status::NotFound)),
                Err(e) => {
                    error!("show_package:4, err={:?}", e);
                    Ok(Response::with(status::InternalServerError))
                }
            }
        }
    } else {
        if !ident.fully_qualified() {
            match depot.datastore.packages.index.latest(&ident) {
                Ok(id) => ident = id.into(),
                Err(Error::DataStore(dbcache::Error::EntityNotFound)) => {
                    return Ok(Response::with(status::NotFound));
                }
                Err(e) => {
                    error!("show_package:5, err={:?}", e);
                    return Ok(Response::with(status::InternalServerError));
                }
            }
        }

        match depot.datastore.packages.get(&ident) {
            Ok(pkg) => render_package(&pkg),
            Err(Error::DataStore(dbcache::Error::EntityNotFound)) => {
                Ok(Response::with(status::NotFound))
            }
            Err(e) => {
                error!("show_package:6, err={:?}", e);
                Ok(Response::with(status::InternalServerError))
            }
        }
    }
}

fn render_package(pkg: &data_object::Package) -> IronResult<Response> {
    let body = json::encode(pkg).unwrap();
    let mut response = Response::with((status::Ok, body));
    response.headers.set(ETag(pkg.checksum.clone()));
    Ok(response)
}

fn promote_package(depot: &Depot, req: &mut Request) -> IronResult<Response> {
    let params = req.extensions.get::<Router>().unwrap();
    let view = params.find("view").unwrap();

    match depot.datastore.views.is_member(view) {
        Ok(true) => {
            let ident: package::PackageIdent = extract_ident(params);
            match depot.datastore.packages.get(&ident) {
                Ok(package) => {
                    depot.datastore.views.associate(view, &package).unwrap();
                    Ok(Response::with(status::Ok))
                }
                Err(Error::DataStore(dbcache::Error::EntityNotFound)) => {
                    Ok(Response::with(status::NotFound))
                }
                Err(e) => {
                    error!("promote:2, err={:?}", e);
                    return Ok(Response::with(status::InternalServerError));
                }
            }
        }
        Ok(false) => Ok(Response::with(status::NotFound)),
        Err(e) => {
            error!("promote:1, err={:?}", e);
            return Ok(Response::with(status::InternalServerError));
        }
    }
}

fn extract_ident(params: &Params) -> package::PackageIdent {
    package::PackageIdent::new(params.find("origin").unwrap(),
                               params.find("pkg").unwrap(),
                               params.find("version"),
                               params.find("release"))
}

fn extract_data_ident(params: &Params) -> data_object::PackageIdent {
    let ident: package::PackageIdent = extract_ident(params);
    data_object::PackageIdent::new(ident)
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

struct Cors;

impl AfterMiddleware for Cors {
    fn after(&self, _req: &mut Request, mut res: Response) -> IronResult<Response> {
        res.headers.set(headers::AccessControlAllowOrigin::Any);
        Ok(res)
    }
}

pub fn router(config: Config) -> Result<Chain> {
    let depot = try!(Depot::new(config));
    let depot1 = depot.clone();
    let depot2 = depot.clone();
    let depot3 = depot.clone();
    let depot4 = depot.clone();
    let depot5 = depot.clone();
    let depot6 = depot.clone();
    let depot7 = depot.clone();
    let depot8 = depot.clone();
    let depot9 = depot.clone();
    let depot10 = depot.clone();
    let depot11 = depot.clone();
    let depot12 = depot.clone();
    let depot13 = depot.clone();
    let depot14 = depot.clone();
    let depot15 = depot.clone();
    let depot16 = depot.clone();
    let depot17 = depot.clone();
    let depot18 = depot.clone();

    let router = router!(
        get "/views" => move |r: &mut Request| list_views(&depot1, r),
        get "/views/:view/pkgs/:origin" => move |r: &mut Request| list_packages(&depot18, r),
        get "/views/:view/pkgs/:origin/:pkg" => move |r: &mut Request| list_packages(&depot2, r),
        get "/views/:view/pkgs/:origin/:pkg/latest" => move |r: &mut Request| show_package(&depot3, r),
        get "/views/:view/pkgs/:origin/:pkg/:version" => move |r: &mut Request| list_packages(&depot4, r),
        get "/views/:view/pkgs/:origin/:pkg/:version/latest" => move |r: &mut Request| show_package(&depot5, r),
        get "/views/:view/pkgs/:origin/:pkg/:version/:release" => move |r: &mut Request| show_package(&depot6, r),

        post "/views/:view/pkgs/:origin/:pkg/:version/:release/promote" => move |r: &mut Request| promote_package(&depot7, r),

        get "/pkgs/:origin" => move |r: &mut Request| list_packages(&depot8, r),
        get "/pkgs/:origin/:pkg" => move |r: &mut Request| list_packages(&depot9, r),
        get "/pkgs/:origin/:pkg/latest" => move |r: &mut Request| show_package(&depot10, r),
        get "/pkgs/:origin/:pkg/:version" => move |r: &mut Request| list_packages(&depot11, r),
        get "/pkgs/:origin/:pkg/:version/latest" => move |r: &mut Request| show_package(&depot12, r),
        get "/pkgs/:origin/:pkg/:version/:release" => move |r: &mut Request| show_package(&depot13, r),

        get "/pkgs/:origin/:pkg/:version/:release/download" => move |r: &mut Request| download_package(&depot14, r),
        post "/pkgs/:origin/:pkg/:version/:release" => move |r: &mut Request| upload_package(&depot15, r),

        post "/keys/:key" => move |r: &mut Request| upload_key(&depot16, r),
        get "/keys/:key" => move |r: &mut Request| download_key(&depot17, r)
    );
    let mut chain = Chain::new(router);
    chain.link_after(Cors);
    Ok(chain)
}

pub fn run(config: Config) -> Result<()> {
    let listen_addr = config.listen_addr.clone();
    let chain = try!(router(config));
    Iron::new(chain).http(listen_addr).unwrap();
    Ok(())
}

impl From<Error> for IronError {
    fn from(err: Error) -> IronError {
        IronError {
            error: Box::new(err),
            response: Response::with((status::InternalServerError, "Internal Habitat error")),
        }
    }
}
