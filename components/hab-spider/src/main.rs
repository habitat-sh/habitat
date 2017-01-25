
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate time;
extern crate petgraph;
extern crate walkdir;
extern crate habitat_core as hab_core;
extern crate habitat_builder_protocol as protocol;
extern crate clap;

pub mod rdeps;
pub mod spider;

use clap::{Arg, App};
use spider::Spider;
use std::io;
use time::PreciseTime;

fn main() {
    env_logger::init().unwrap();

    let matches = App::new("hab-spider")
        .version("0.1.0")
        .about("Habitat package graph builder")
        .arg(Arg::with_name("PATH")
            .help("The path to the packages root")
            .required(true)
            .index(1))
        .get_matches();

    let path = matches.value_of("PATH").unwrap();

    let mut spider = Spider::new(&path);
    let start_time = PreciseTime::now();
    let (ncount, ecount) = spider.crawl();
    let end_time = PreciseTime::now();

    println!("OK: {} nodes, {} edges ({} sec)",
             ncount,
             ecount,
             start_time.to(end_time));

    println!("\nRdeps for core/cacerts/2016.04.20/20160612081125:");
    println!("{:?}",
             spider.rdeps("core/cacerts/2016.04.20/20160612081125"));

    println!("\nHit ENTER to exit");
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
}
