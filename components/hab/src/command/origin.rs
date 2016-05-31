// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

pub mod key {
    pub mod download {
        use std::path::Path;

        use ansi_term::Colour::{Blue, Green, Yellow};
        use depot_client;
        use hcore::crypto::SigKeyPair;

        use common::command::ProgressBar;
        use error::Result;

        pub fn start(depot: &str, origin: &str, revision: Option<&str>, cache: &Path) -> Result<()> {
            match revision {
                Some(revision) => {
                    let nwr = format!("{}-{}", origin, revision);
                    let msg = format!("» Downloading public origin key {}", &nwr);
                    println!("{}", Yellow.bold().paint(msg));
                    try!(download_key(depot, &nwr, origin, revision, cache));
                    println!("{}",
                             Blue.paint(format!("★ Download of {} public origin key completed.", &nwr)));
                }
                None => {
                    let msg = format!("» Downloading public origin keys for {}", origin);
                    println!("{}", Yellow.bold().paint(msg));
                    for key in try!(depot_client::show_origin_keys(depot, origin)) {
                        let nwr = format!("{}-{}", key.get_origin(), key.get_revision());
                        try!(download_key(depot, &nwr, key.get_origin(), key.get_revision(), cache));
                    }
                    println!("{}",
                             Blue.paint(format!("★ Download of {} public origin keys completed.", &origin)));
                }
            };
            Ok(())
        }

        fn download_key(depot: &str, nwr: &str, name: &str, rev: &str, cache: &Path) -> Result<()> {
            match SigKeyPair::get_public_key_path(&nwr, &cache) {
                Ok(_) => {
                    println!("{} {}", Green.paint("→ Using"), &nwr);
                }
                Err(_) => {
                    println!("{} {}", Green.bold().paint("↓ Downloading"), &nwr);
                    let mut progress = ProgressBar::default();
                    try!(depot_client::fetch_origin_key(depot, name, rev, cache, Some(&mut progress)));
                    println!("{} {}", Green.bold().paint("☑ Cached"), &nwr);
                }
            }
            Ok(())
        }
    }

    pub mod export {
        use std::io;
        use std::fs::File;
        use std::path::Path;

        use hcore::crypto::SigKeyPair;
        use hcore::crypto::keys::PairType;

        use error::Result;

        pub fn start(origin: &str, pair_type: PairType, cache: &Path) -> Result<()> {
            let latest = try!(SigKeyPair::get_latest_pair_for(origin, cache));
            let path = match pair_type {
                PairType::Public => try!(SigKeyPair::get_public_key_path(&latest.name_with_rev(), cache)),
                PairType::Secret => try!(SigKeyPair::get_secret_key_path(&latest.name_with_rev(), cache)),
            };
            let mut file = try!(File::open(&path));
            debug!("Streaming file contents of {} {} to standard out",
                   &pair_type,
                   &path.display());
            try!(io::copy(&mut file, &mut io::stdout()));
            Ok(())
        }
    }

    pub mod generate {
        use std::path::Path;

        use ansi_term::Colour::{Blue, Yellow};
        use hcore::crypto::SigKeyPair;

        use error::Result;

        pub fn start(origin: &str, cache: &Path) -> Result<()> {
            println!("{}",
                     Yellow.bold().paint(format!("» Generating origin key for {}", &origin)));
            let pair = try!(SigKeyPair::generate_pair_for_origin(origin, cache));
            println!("{}",
                     Blue.paint(format!("★ Generated origin key pair {}.", &pair.name_with_rev())));
            Ok(())
        }
    }

    pub mod import {
        use std::path::Path;

        use ansi_term::Colour::{Blue, Yellow};
        use hcore::crypto::SigKeyPair;

        use error::Result;

        pub fn start(content: &str, cache: &Path) -> Result<()> {
            println!("{}",
                     Yellow.bold().paint(format!("» Importing origin key from standard input")));
            let (pair, pair_type) = try!(SigKeyPair::write_file_from_str(content, cache));
            println!("{}",
                     Blue.paint(format!("★ Imported {} origin key {}.",
                                        &pair_type,
                                        &pair.name_with_rev())));
            Ok(())
        }
    }

    pub mod upload {
        use std::fs::File;
        use std::io::{BufRead, BufReader};
        use std::path::Path;

        use ansi_term::Colour::{Blue, Green, Yellow};
        use common::command::ProgressBar;
        use depot_client;
        use hcore;
        use hcore::crypto::keys::parse_name_with_rev;

        use error::{Error, Result};

        pub fn start(depot: &str, keyfile: &Path) -> Result<()> {
            println!("{}",
                     Yellow.bold()
                         .paint(format!("» Uploading public origin key {}", keyfile.display())));
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
            println!("{} {}",
                     Green.bold().paint("↑ Uploading"),
                     keyfile.display());
            let mut progress = ProgressBar::default();
            try!(depot_client::put_origin_key(depot, &name, &rev, keyfile, Some(&mut progress)));
            println!("{} {}", Green.bold().paint("✓ Uploaded"), &name_with_rev);
            println!("{}",
                     Blue.paint(format!("★ Upload of public origin key {} complete.",
                                        &name_with_rev)));
            Ok(())
        }
    }
}
