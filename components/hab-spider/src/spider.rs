use std::path::PathBuf;
use std::collections::HashMap;
use walkdir::WalkDir;
use hab_core::package::{FromArchive, PackageArchive};
use protocol::depotsrv;
use petgraph::Graph;
use petgraph::graph::NodeIndex;
// use petgraph::dot::{Dot, Config};
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

    pub fn crawl(&mut self) -> (usize, usize) {
        assert!(self.package_max == 0);

        let mut directories = vec![];

        for entry in WalkDir::new(&self.packages_path).follow_links(false) {
            let entry = entry.unwrap();
            if entry.metadata().unwrap().is_dir() {
                directories.push(entry);
                continue;
            }

            let mut archive = PackageArchive::new(PathBuf::from(entry.path()));

            match archive.ident() {
                Ok(_) => {
                    // TODO: Remove this check
                    // if ident.origin != "core" {
                    //     continue;
                    // }
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
                                let (_, dep_node) = self.generate_id(&depname);
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

        // println!("Graph:\n{:?}",
        //          Dot::with_config(&self.graph, &[Config::EdgeNoLabel]));
        // println!("Is cyclic: {}\n", is_cyclic_directed(&self.graph));

        (self.graph.node_count(), self.graph.edge_count())
    }

    pub fn rdeps(&self, name: &str) -> Vec<String> {
        let mut v = Vec::new();

        match self.package_map.get(name) {
            Some(&(_, pkg_node)) => {
                match rdeps(&self.graph, pkg_node) {
                    Ok(deps) => {
                        for n in deps {
                            v.push(self.package_names[n].clone());
                        }
                    }
                    Err(e) => panic!("Error: {:?}", e),
                }
            }
            None => (),
        }

        v
    }

    pub fn rdeps_dump(&self) {
        debug!("Reverse dependencies:");

        for (pkg_name, pkg_id) in &self.package_map {
            let (_, node) = *pkg_id;
            debug!("{}", pkg_name);

            match rdeps(&self.graph, node) {
                Ok(v) => {
                    for n in v {
                        debug!("|_ {}", self.package_names[n]);
                    }
                }
                Err(e) => panic!("Error: {:?}", e),
            }
        }
    }
}
