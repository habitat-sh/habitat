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

    let path = Path::new(file);
    let dat_file = match dat_file::DatFile::new(path.to_path_buf()) {
        Ok(d) => d,
        Err(e) => {
            error!("Could not read dat file: {:?}", e);
            process::exit(1);
        }
    };

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

    for service in dat_file.read_rumors::<Service>(dat_file.service_len())? {
        println!("{}", service);
    }

    for service_config in dat_file.read_rumors::<ServiceConfig>(dat_file.service_config_len())? {
        println!("{}", service_config);
    }

    for service_file in dat_file.read_rumors::<ServiceFile>(dat_file.service_file_len())? {
        println!("{}", service_file);
    }

    for election in dat_file.read_rumors::<Election>(dat_file.election_len())? {
        println!("{}", election);
    }

    for update_election in dat_file.read_rumors::<ElectionUpdate>(dat_file.update_len())? {
        println!("{}", update_election);
    }

    if version[0] >= 2 {
        for departure in dat_file.read_rumors::<Departure>(dat_file.departure_len())? {
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

    services += dat_file.read_rumors::<Service>(dat_file.service_len())?
                        .len();
    service_configs += dat_file.read_rumors::<ServiceConfig>(dat_file.service_config_len())?
                               .len();
    service_files += dat_file.read_rumors::<ServiceFile>(dat_file.service_file_len())?
                             .len();
    elections += dat_file.read_rumors::<Election>(dat_file.election_len())?
                         .len();
    update_elections += dat_file.read_rumors::<ElectionUpdate>(dat_file.update_len())?
                                .len();

    if version[0] >= 2 {
        departures += dat_file.read_rumors::<Departure>(dat_file.departure_len())?
                              .len();
    }

    println!("Summary:");
    println!("");
    println!("Membership: {}", membership);
    println!("Services: {}", services);
    println!("Service Configs: {}", service_configs);
    println!("Service Files: {}", service_files);
    println!("Elections: {}", elections);
    println!("Update Elections: {}", update_elections);
    println!("Departures: {}", departures);

    Ok(())
}
