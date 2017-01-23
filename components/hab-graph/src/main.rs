
#[macro_use]
extern crate log;
extern crate petgraph;
extern crate walkdir;

pub mod graph;

use petgraph::Graph;
use petgraph::dot::{Dot, Config};
use walkdir::WalkDir;

extern crate habitat_core as hab_core;

use hab_core::package::{FromArchive, PackageArchive};

extern crate habitat_builder_protocol as protocol;

use protocol::depotsrv;

use std::path::PathBuf;

fn main() {
    println!("Hello, graph!\n");

    let mut deps = Graph::<usize, usize>::new();
    let a = deps.add_node(10);
    let b = deps.add_node(11);
    deps.extend_with_edges(&[(a, b), (b, a)]);

    let path = PathBuf::from("/Users/salam/Workspace/habitat/components/hab-graph/pkgs");
    build_graph_from_fs(path);
}

fn build_graph_from_fs(packages_path: PathBuf) {
    let mut directories = vec![];
    for entry in WalkDir::new(&packages_path).follow_links(false) {
        let entry = entry.unwrap();
        if entry.metadata().unwrap().is_dir() {
            // println!("{}", entry.path().display());
            directories.push(entry);
            continue;
        }

        let mut archive = PackageArchive::new(PathBuf::from(entry.path()));

        match archive.ident() {
            Ok(ident) => {
                match depotsrv::Package::from_archive(&mut archive) {
                    Ok(o) => {
                        println!("{} :", o.get_ident().get_name());
                        let deps = o.get_deps();
                        for dep in deps {
                            println!("    {}", dep.get_name());
                        }
                    }
                    Err(e) => println!("Error parsing package from archive: {:?}", e),
                }
            }
            Err(e) => {
                println!("Error reading, archive={:?} error={:?}", &archive, &e);
            }
        }
    }
}
