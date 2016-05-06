// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

pub mod key {
    pub mod download {
        use std::path::Path;

        use depot_client;

        use error::Result;

        pub fn start(depot: &str,
                     origin: &str,
                     revision: Option<&str>,
                     cache: &Path)
                     -> Result<()> {
            match revision {
                Some(rev) => {
                    try!(depot_client::get_origin_key(depot,
                                                      origin,
                                                      rev,
                                                      cache.to_string_lossy().as_ref()));
                }
                None => {
                    try!(depot_client::get_origin_keys(depot,
                                                       origin,
                                                       cache.to_string_lossy().as_ref()));
                }
            };
            println!("Successfully downloaded origin key(s)");
            Ok(())
        }
    }

    pub mod generate {
        use std::path::Path;

        use hcore::crypto::SigKeyPair;

        use error::Result;

        pub fn start(origin: &str, cache: &Path) -> Result<()> {
            let pair = try!(SigKeyPair::generate_pair_for_origin(origin, cache));
            println!("Successfully generated origin key {}", pair.name_with_rev());
            Ok(())
        }
    }

    pub mod upload {
        use std::fs::File;
        use std::io::{BufRead, BufReader};
        use std::path::Path;

        use depot_client;
        use hcore;
        use hcore::crypto::keys::parse_name_with_rev;

        use error::{Error, Result};

        pub fn start(depot: &str, keyfile: &Path) -> Result<()> {
            let name_with_rev = {
                let f = try!(File::open(&keyfile));
                let f = BufReader::new(f);
                let mut lines = f.lines();
                let _ = match lines.next() {
                    Some(val) => {
                        let val = try!(val);
                        if &val != "SIG-PUB-1" {
                            let msg = format!("Unsupported version: {}", &val);
                            return Err(Error::HabitatCore(hcore::Error::CryptoError(msg)));
                        }
                        ()
                    }
                    None => {
                        let msg = "Corrupt key file, can't read file version".to_string();
                        return Err(Error::HabitatCore(hcore::Error::CryptoError(msg)));
                    }
                };
                match lines.next() {
                    Some(val) => try!(val),
                    None => {
                        let msg = "Corrupt key file, can't read name with rev".to_string();
                        return Err(Error::HabitatCore(hcore::Error::CryptoError(msg)));
                    }
                }
            };
            let (name, rev) = try!(parse_name_with_rev(&name_with_rev));
            try!(depot_client::post_origin_key(depot, &name, &rev, keyfile));
            println!("Successfully uploaded origin key {}", &name_with_rev);
            Ok(())
        }
    }
}
