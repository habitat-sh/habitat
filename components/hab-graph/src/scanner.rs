use std::path::PathBuf;
use std::collections::HashMap;
use time::{PreciseTime, Duration};
use walkdir::WalkDir;
use hab_core::package::{FromArchive, PackageArchive};
use protocol::depotsrv;
use petgraph::Graph;
use petgraph::graph::NodeIndex;
use petgraph::algo::is_cyclic_directed;
use petgraph::dot::{Dot, Config};
use graph::rdeps;

pub struct Scanner {
    packages_path: PathBuf,
    package_max: usize,
    package_map: HashMap<String, (usize, NodeIndex)>,
    package_names: Vec<String>,
    graph: Graph<usize, usize>,
}

impl Scanner {
    pub fn new(path: &str) -> Self {
        Scanner {
            packages_path: PathBuf::from(path),
            package_max: 0,
            package_map: HashMap::new(),
            package_names: Vec::new(),
            graph: Graph::<usize, usize>::new(),
        }
    }

    fn generate_id(&mut self, name: String) -> (usize, NodeIndex) {
        let id = if self.package_map.contains_key(&name) {
            print!("(key exists) ");
            *self.package_map.get(&name).unwrap()
        } else {
            print!("({}) ", self.package_max);
            self.package_names.push(String::from(name.clone()));
            assert_eq!(self.package_names[self.package_max], name);
            let node_index = self.graph.add_node(self.package_max);
            self.package_map.insert(name, (self.package_max, node_index));
            self.package_max = self.package_max + 1;
            (self.package_max - 1, node_index)
        };
        id
    }

    pub fn scan(&mut self) {
        assert!(self.package_max == 0);

        let mut directories = vec![];
        let mut start_time = PreciseTime::now();
        let mut generation_duration: Duration = Duration::zero();
        let mut total_rdeps_duration: Duration = Duration::zero();

        for entry in WalkDir::new(&self.packages_path).follow_links(false) {
            let entry = entry.unwrap();
            if entry.metadata().unwrap().is_dir() {
                // println!("{}", entry.path().display());
                directories.push(entry);
                continue;
            }

            let mut archive = PackageArchive::new(PathBuf::from(entry.path()));

            match archive.ident() {
                Ok(ident) => {
                    // TODO: Remove this check
                    if ident.origin != "core" {
                        continue;
                    }
                    match depotsrv::Package::from_archive(&mut archive) {
                        Ok(o) => {
                            let name = format!("{}", o.get_ident());
                            print!("{}", name);

                            let (pkg_id, pkg_node) = self.generate_id(name);

                            println!("");
                            let deps = o.get_deps();
                            for dep in deps {
                                let depname = format!("{}", dep);
                                print!("|_ {}", depname);
                                let (dep_id, dep_node) = self.generate_id(depname);
                                self.graph.extend_with_edges(&[(dep_node, pkg_node)]);
                                println!("");
                            }
                        }
                        Err(e) => println!("Error parsing package from archive: {:?}", e),
                    }
                }
                Err(e) => {
                    println!("Error reading, archive={:?} error={:?}", &archive, &e);
                }
            }
            println!("");
        }

        println!("\nTotal packages processed: {}", self.package_max - 1);

        let mut end_time = PreciseTime::now();
        generation_duration = start_time.to(end_time);
        println!("Time to process: {} sec", generation_duration);

        // println!("Graph:\n{:?}",
        //          Dot::with_config(&self.graph, &[Config::EdgeNoLabel]));
        println!("Is cyclic: {}\n", is_cyclic_directed(&self.graph));

        println!("Reverse dependencies:\n");

        for (pkg_name, pkg_id) in &self.package_map {
            let (id, node) = *pkg_id;
            println!("{}", pkg_name);

            start_time = PreciseTime::now();
            match rdeps(&self.graph, node) {
                Ok(v) => {
                    end_time = PreciseTime::now();
                    for n in v {
                        println!("|_ {}", self.package_names[n]);
                    }
                }
                Err(e) => panic!("Error: {:?}", e),
            }

            let rdeps_duration = start_time.to(end_time);
            total_rdeps_duration = total_rdeps_duration + rdeps_duration;
            println!("Time to process: {} sec", rdeps_duration);
            println!("");
        }

        println!("\nStatistics:");
        println!("\nTotal packages processed: {}", self.package_max - 1);
        println!("Time to process packages: {} sec", generation_duration);
        println!("Avg. time for rdeps search: {} sec",
                 total_rdeps_duration / (self.package_max - 1) as i32);
    }
}
