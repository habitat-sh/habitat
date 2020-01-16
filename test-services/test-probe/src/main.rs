#[macro_use]
extern crate serde_derive;
use actix_web::{web,
                App as ActixApp,
                HttpServer};
use clap::{App,
           Arg};

mod config;
mod context;
mod error;

fn app<'a, 'b>() -> App<'a, 'b> {
    App::new("test-probe").arg(Arg::with_name("config").short("c")
                                                       .long("config")
                                                       .value_name("CONFIG")
                                                       .help("Sets a custom config file")
                                                       .takes_value(true))
}

fn main() -> std::io::Result<()> {
    let matches = app().get_matches();
    let config = config::from_matches(&matches).expect("Could not create application");
    let bind_addr = format!("{}:{}", config.host, config.port);
    let server = HttpServer::new(move || {
                     ActixApp::new().data(config.clone())
                                    .route("context", web::get().to(context::show))
                                    .route("context/{path:.*}", web::get().to(context::show))
                 }).workers(1)
                   .bind(&bind_addr)?;

    println!("Starting test-probe on {}", bind_addr);
    server.run()
}
