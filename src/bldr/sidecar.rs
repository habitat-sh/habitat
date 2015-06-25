use iron::prelude::*;
use iron::status;
use router::Router;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::sync::{Arc, Mutex};
use std::thread;

use error::{BldrError, BldrResult};

use pkg::{self, Package, Signal};

struct Sidecar {
    pub package: Package,
}

impl Sidecar {
    fn new(pkg: &str) -> BldrResult<Arc<Mutex<Sidecar>>> {
        let package = try!(pkg::latest(pkg));
        Ok(Arc::new(Mutex::new(Sidecar{package: package})))
    }
}

fn config(sidecar: &Sidecar, _req: &mut Request) -> IronResult<Response> {
    let mut exposed_file = File::open(sidecar.package.srvc_join_path("last.toml")).unwrap();
    let mut exposed_string = String::new();
    exposed_file.read_to_string(&mut exposed_string).unwrap();
    Ok(Response::with((status::Ok, exposed_string)))
}

fn status(sidecar: &Sidecar, _req: &mut Request) -> IronResult<Response> {
    let output = try!(sidecar.package.signal(Signal::Status));
    Ok(Response::with((status::Ok, output)))
}

pub fn run(pkg: &str) -> BldrResult<()> {
    let pkg_name = String::from(pkg);
    thread::spawn(move || -> BldrResult<()> {
        let sidecar = try!(Sidecar::new(&pkg_name));
        let sidecar2 = sidecar.clone();

        let mut router = Router::new();

        router.get("/config", move |r: &mut Request| config(&sidecar.lock().unwrap(), r));
        router.get("/status", move |r: &mut Request| status(&sidecar2.lock().unwrap(), r));

        Iron::new(router).http("0.0.0.0:9631").unwrap();
        Ok(())
    });
    Ok(())
}

impl From<BldrError> for IronError {
    fn from(err: BldrError) -> IronError {
        IronError{error: Box::new(err), response: Response::with((status::InternalServerError, ""))}
    }
}
