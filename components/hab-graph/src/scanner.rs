use std::path::PathBuf;
use std::collections::HashMap;
use walkdir::WalkDir;
use hab_core::package::{FromArchive, PackageArchive};
use protocol::depotsrv;

pub struct Scanner {
    packages_path: PathBuf,
    package_max: usize,
    package_map: HashMap<String, usize>,
}

impl Scanner {
    pub fn new(path: &str) -> Self {
        Scanner {
            packages_path: PathBuf::from(path),
            package_max: 0,
            package_map: HashMap::new(),
        }
    }

    fn generate_id(&mut self, name: String) -> usize {
        let id = if self.package_map.contains_key(&name) {
            print!("(key exists) ");
            *self.package_map.get(&name).unwrap()
        } else {
            print!("({}) ", self.package_max);
            self.package_map.insert(name, self.package_max);
            self.package_max = self.package_max + 1;
            self.package_max - 1
        };
        id
    }

    pub fn scan(&mut self) {
        assert!(self.package_max == 0);

        let mut directories = vec![];

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

                            let id = self.generate_id(name);

                            println!("");
                            let deps = o.get_deps();
                            for dep in deps {
                                let depname = format!("{}", dep);
                                print!("|_ {}", depname);
                                let depid = self.generate_id(depname);
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
    }
}
