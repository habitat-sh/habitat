#[macro_use]
extern crate log;

use crate::error::Result;
use clap::{App,
           Arg};
use env_logger;
use habitat_butterfly::rumor::{dat_file,
                               Departure,
                               Election,
                               ElectionUpdate,
                               Service,
                               ServiceConfig,
                               ServiceFile};
use log::error;
use std::{path::Path,
          process};

pub mod error;

fn main() {
    env_logger::init();
    let matches =
        App::new("Habitat RST Reader").about("Introspection for the butterfly RST file")
                                      .arg(Arg::with_name("FILE").required(true)
                                                                 .index(1)
                                                                 .help("Path to the RST file"))
                                      .arg(Arg::with_name("STATS").short("s")
                                                                  .long("stats")
                                                                  .conflicts_with("FOLLOW")
                                                                  .help("Display statistics \
                                                                         about the contents of \
                                                                         the file"))
                                      .get_matches();

    let file = matches.value_of("FILE").unwrap();
    let stats = matches.is_present("STATS");
    let dat_file = dat_file::DatFile::read(Path::new(file)).unwrap_or_else(|e| {
                                                               error!("Could not read dat file \
                                                                       {}: {}",
                                                                      file, e);
                                                               process::exit(1);
                                                           });

    let result = if stats {
        output_stats(dat_file)
    } else {
        output_rumors(dat_file)
    };

    if result.is_err() {
        error!("Error processing dat file: {:?}", result);
        process::exit(1);
    }
}

fn output_rumors(mut dat_file: dat_file::DatFile) -> Result<()> {
    let mut version = [0; 1];

    dat_file.read_header(&mut version)?;

    for member in dat_file.read_members()? {
        println!("{}", member);
    }

    for service in dat_file.read_rumors::<Service>()? {
        println!("{}", service);
    }

    for service_config in dat_file.read_rumors::<ServiceConfig>()? {
        println!("{}", service_config);
    }

    for service_file in dat_file.read_rumors::<ServiceFile>()? {
        println!("{}", service_file);
    }

    for election in dat_file.read_rumors::<Election>()? {
        println!("{}", election);
    }

    for update_election in dat_file.read_rumors::<ElectionUpdate>()? {
        println!("{}", update_election);
    }

    if version[0] >= 2 {
        for departure in dat_file.read_rumors::<Departure>()? {
            println!("{}", departure);
        }
    }

    Ok(())
}

fn output_stats(mut dat_file: dat_file::DatFile) -> Result<()> {
    let mut membership = 0;
    let mut services = 0;
    let mut service_configs = 0;
    let mut service_files = 0;
    let mut elections = 0;
    let mut update_elections = 0;
    let mut departures = 0;

    let mut version = [0; 1];

    dat_file.read_header(&mut version)?;
    membership += dat_file.read_members()?.len();

    services += dat_file.read_rumors::<Service>()?.len();
    service_configs += dat_file.read_rumors::<ServiceConfig>()?.len();
    service_files += dat_file.read_rumors::<ServiceFile>()?.len();
    elections += dat_file.read_rumors::<Election>()?.len();
    update_elections += dat_file.read_rumors::<ElectionUpdate>()?.len();

    if version[0] >= 2 {
        departures += dat_file.read_rumors::<Departure>()?.len();
    }

    println!("Summary:");
    println!();
    println!("Membership: {}", membership);
    println!("Services: {}", services);
    println!("Service Configs: {}", service_configs);
    println!("Service Files: {}", service_files);
    println!("Elections: {}", elections);
    println!("Update Elections: {}", update_elections);
    println!("Departures: {}", departures);

    Ok(())
}
