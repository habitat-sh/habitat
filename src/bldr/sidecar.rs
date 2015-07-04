use iron::prelude::*;
use iron::status;
use router::Router;
use std::sync::Arc;
use std::thread;

use error::{BldrError, BldrResult};

use pkg::{self, Package, Signal};
use health_check;

struct Sidecar {
    pub package: Package,
}

impl Sidecar {
    fn new(pkg: &str) -> BldrResult<Arc<Sidecar>> {
        let package = try!(pkg::latest(pkg));
        Ok(Arc::new(Sidecar{package: package}))
    }
}

fn config(sidecar: &Sidecar, _req: &mut Request) -> IronResult<Response> {
    let last_config = try!(sidecar.package.last_config());
    Ok(Response::with((status::Ok, last_config)))
}

fn status(sidecar: &Sidecar, _req: &mut Request) -> IronResult<Response> {
    let output = try!(sidecar.package.signal(Signal::Status));
    Ok(Response::with((status::Ok, output)))
}

fn health(sidecar: &Sidecar, _req: &mut Request) -> IronResult<Response> {
    let result = try!(sidecar.package.health_check());

    match result.status {
        health_check::Status::Ok | health_check::Status::Warning => {
            Ok(Response::with((status::Ok, format!("{}", result))))
        },
        health_check::Status::Critical => {
            Ok(Response::with((status::ServiceUnavailable, format!("{}", result))))
        },
        health_check::Status::Unknown => {
            Ok(Response::with((status::InternalServerError, format!("{}", result))))
        },
    }
}

pub fn run(pkg: &str) -> BldrResult<()> {
    let pkg_name = String::from(pkg);
    thread::spawn(move || -> BldrResult<()> {
        // The sidecar is in an Arc. The clones are
        // creating instances to share, and when they all go away, we'll
        // reap the instance. Turns out they won't really ever go away,
        // but you do what you need to :)
        let sidecar = try!(Sidecar::new(&pkg_name));
        let sidecar2 = sidecar.clone();
        let sidecar3 = sidecar.clone();

        let mut router = Router::new();

        router.get("/config", move |r: &mut Request| config(&sidecar, r));
        router.get("/status", move |r: &mut Request| status(&sidecar2, r));
        router.get("/health", move |r: &mut Request| health(&sidecar3, r));

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
