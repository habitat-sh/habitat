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
use rdeps::rdeps;

pub struct Spider {
    packages_path: PathBuf,
    package_max: usize,
    package_map: HashMap<String, (usize, NodeIndex)>,
    package_names: Vec<String>,
    graph: Graph<usize, usize>,
}

impl Spider {
    pub fn new(path: &str) -> Self {
        Spider {
            packages_path: PathBuf::from(path),
            package_max: 0,
            package_map: HashMap::new(),
            package_names: Vec::new(),
            graph: Graph::<usize, usize>::new(),
        }
    }

    fn generate_id(&mut self, name: &str) -> (usize, NodeIndex) {
        let id = if self.package_map.contains_key(name) {
            let val = *self.package_map.get(name).unwrap();
            val
        } else {
            self.package_names.push(String::from(name.clone()));
            assert_eq!(self.package_names[self.package_max], name);
            let node_index = self.graph.add_node(self.package_max);
            self.package_map.insert(String::from(name), (self.package_max, node_index));
            self.package_max = self.package_max + 1;
            (self.package_max - 1, node_index)
        };
        id
    }

    pub fn crawl(&mut self) {
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
                            let (pkg_id, pkg_node) = self.generate_id(&name);

                            assert_eq!(pkg_id, pkg_node.index());
                            debug!("{} ({})", name, pkg_id);

                            let deps = o.get_deps();
                            for dep in deps {
                                let depname = format!("{}", dep);
                                debug!("|_ {}", depname);
                                let (dep_id, dep_node) = self.generate_id(&depname);
                                self.graph.extend_with_edges(&[(dep_node, pkg_node)]);
                            }
                        }
                        Err(e) => error!("Error parsing package from archive: {:?}", e),
                    }
                }
                Err(e) => {
                    error!("Error reading, archive={:?} error={:?}", &archive, &e);
                }
            }
            debug!("");
        }

        let mut end_time = PreciseTime::now();
        generation_duration = start_time.to(end_time);

        // println!("Graph:\n{:?}",
        //          Dot::with_config(&self.graph, &[Config::EdgeNoLabel]));
        // println!("Is cyclic: {}\n", is_cyclic_directed(&self.graph));

        debug!("Reverse dependencies:");

        for (pkg_name, pkg_id) in &self.package_map {
            let (id, node) = *pkg_id;
            debug!("{}", pkg_name);

            start_time = PreciseTime::now();
            match rdeps(&self.graph, node) {
                Ok(v) => {
                    end_time = PreciseTime::now();
                    for n in v {
                        debug!("|_ {}", self.package_names[n]);
                    }
                }
                Err(e) => panic!("Error: {:?}", e),
            }

            let rdeps_duration = start_time.to(end_time);
            total_rdeps_duration = total_rdeps_duration + rdeps_duration;
            debug!("{} sec", rdeps_duration);
            debug!("");
        }

        println!("Statistics:\n");
        println!("Total packages processed: {}", self.package_max - 1);
        println!("Time to process packages: {} sec", generation_duration);
        println!("Avg. time for rdeps search: {} sec",
                 total_rdeps_duration / (self.package_max - 1) as i32);
    }
}
