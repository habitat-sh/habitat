use actix_web::{App as ActixApp,
                HttpServer,
                web::{self,
                      Data}};
use clap::{Arg,
           Command};

mod config;
mod context;
mod error;

fn app() -> Command {
    Command::new("test-probe").arg(Arg::new("config").short('c')
                                                     .long("config")
                                                     .value_name("CONFIG")
                                                     .help("Sets a custom config file")
                                                     .num_args(1))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let matches = app().get_matches();
    let config = config::from_matches(&matches).expect("Could not create application");
    let bind_addr = format!("{}:{}", config.host, config.port);
    let server = HttpServer::new(move || {
                     ActixApp::new().app_data(Data::new(config.clone()))
                                    .route("context", web::get().to(context::show))
                                    .route("context/{path:.*}", web::get().to(context::show))
                 }).workers(1)
                   .bind(&bind_addr)?;

    println!("Starting test-probe on {}", bind_addr);
    server.run().await
}
