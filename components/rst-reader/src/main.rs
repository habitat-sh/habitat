#[macro_use]
extern crate log;

use crate::error::Result;
use clap::{App,
           Arg};
use env_logger;
use habitat_butterfly::{member::Membership,
                        protocol::Message,
                        rumor::{dat_file,
                                Departure,
                                Election,
                                ElectionUpdate,
                                Service,
                                ServiceConfig,
                                ServiceFile}};
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
    let dat_file = dat_file::DatFile::new(path.to_path_buf());

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
    let mut reader = dat_file.reader_for_file()?;

    dat_file.read_header(&mut version, &mut reader)?;
    dat_file.read_members(&mut reader, |r| {
                match Membership::from_bytes(&r) {
                    Ok(m) => println!("{}", m),
                    Err(err) => warn!("Error reading membership rumor from dat file, {}", err),
                }

                Ok(())
            })?;

    dat_file.read_services(&mut reader, |r| {
                let rumor = Service::from_bytes(&r)?;
                println!("{}", rumor);
                Ok(())
            })?;

    dat_file.read_service_configs(&mut reader, |r| {
                let rumor = ServiceConfig::from_bytes(&r)?;
                println!("{}", rumor);
                Ok(())
            })?;

    dat_file.read_service_files(&mut reader, |r| {
                let rumor = ServiceFile::from_bytes(&r)?;
                println!("{}", rumor);
                Ok(())
            })?;

    dat_file.read_elections(&mut reader, |r| {
                let rumor = Election::from_bytes(&r)?;
                println!("{}", rumor);
                Ok(())
            })?;

    dat_file.read_update_elections(&mut reader, |r| {
                let rumor = ElectionUpdate::from_bytes(&r)?;
                println!("{}", rumor);
                Ok(())
            })?;

    if version[0] >= 2 {
        dat_file.read_departures(&mut reader, |r| {
                    let rumor = Departure::from_bytes(&r)?;
                    println!("{}", rumor);
                    Ok(())
                })?;
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
    let mut reader = dat_file.reader_for_file()?;

    dat_file.read_header(&mut version, &mut reader)?;
    dat_file.read_members(&mut reader, |r| {
                match Membership::from_bytes(&r) {
                    Ok(_) => membership += 1,
                    Err(err) => warn!("Error reading membership rumor from dat file, {}", err),
                }

                Ok(())
            })?;

    dat_file.read_services(&mut reader, |r| {
                let _ = Service::from_bytes(&r)?;
                services += 1;
                Ok(())
            })?;

    dat_file.read_service_configs(&mut reader, |r| {
                let _ = ServiceConfig::from_bytes(&r)?;
                service_configs += 1;
                Ok(())
            })?;

    dat_file.read_service_files(&mut reader, |r| {
                let _ = ServiceFile::from_bytes(&r)?;
                service_files += 1;
                Ok(())
            })?;

    dat_file.read_elections(&mut reader, |r| {
                let _ = Election::from_bytes(&r)?;
                elections += 1;
                Ok(())
            })?;

    dat_file.read_update_elections(&mut reader, |r| {
                let _ = ElectionUpdate::from_bytes(&r)?;
                update_elections += 1;
                Ok(())
            })?;

    if version[0] >= 2 {
        dat_file.read_departures(&mut reader, |r| {
                    let _ = Departure::from_bytes(&r)?;
                    departures += 1;
                    Ok(())
                })?;
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
