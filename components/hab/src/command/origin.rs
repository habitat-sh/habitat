// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

pub mod key {

    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use std::path::Path;

    use error::{Error, Result};
    use hcore;

    // shared between origin::key::upload and origin::key::upload_latest
    fn get_name_with_rev(keyfile: &Path, expected_vsn: &str) -> Result<String> {
        let f = try!(File::open(&keyfile));
        let f = BufReader::new(f);
        let mut lines = f.lines();
        let _ = match lines.next() {
            Some(val) => {
                let val = try!(val);
                if &val != expected_vsn {
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
        let name_with_rev = match lines.next() {
            Some(val) => try!(val),
            None => {
                let msg = "Corrupt key file, can't read name with rev".to_string();
                return Err(Error::HabitatCore(hcore::Error::CryptoError(msg)));
            }
        };
        Ok(name_with_rev)
    }

    pub mod download {
        use std::path::Path;

        use ansi_term::Colour::{Blue, Green, Yellow};
        use depot_client::Client;
        use hcore::crypto::SigKeyPair;

        use common::command::ProgressBar;
        use error::Result;

        pub fn start(depot: &str,
                     origin: &str,
                     revision: Option<&str>,
                     cache: &Path)
                     -> Result<()> {
            let depot_client = try!(Client::new(depot, None));
            match revision {
                Some(revision) => {
                    let nwr = format!("{}-{}", origin, revision);
                    let msg = format!("» Downloading public origin key {}", &nwr);
                    println!("{}", Yellow.bold().paint(msg));
                    try!(download_key(&depot_client, &nwr, origin, revision, cache));
                    println!("{}",
                             Blue.paint(format!("★ Download of {} public origin key completed.",
                                                &nwr)));
                }
                None => {
                    let msg = format!("» Downloading public origin keys for {}", origin);
                    println!("{}", Yellow.bold().paint(msg));
                    for key in try!(depot_client.show_origin_keys(origin)) {
                        let nwr = format!("{}-{}", key.get_origin(), key.get_revision());
                        try!(download_key(&depot_client,
                                          &nwr,
                                          key.get_origin(),
                                          key.get_revision(),
                                          cache));
                    }
                    println!("{}",
                             Blue.paint(format!("★ Download of {} public origin keys completed.",
                                                &origin)));
                }
            };
            Ok(())
        }

        fn download_key(depot_client: &Client,
                        nwr: &str,
                        name: &str,
                        rev: &str,
                        cache: &Path)
                        -> Result<()> {
            match SigKeyPair::get_public_key_path(&nwr, &cache) {
                Ok(_) => {
                    println!("{} {}", Green.paint("→ Using"), &nwr);
                }
                Err(_) => {
                    println!("{} {}", Green.bold().paint("↓ Downloading"), &nwr);
                    let mut progress = ProgressBar::default();
                    try!(depot_client.fetch_origin_key(name, rev, cache, Some(&mut progress)));
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
                PairType::Public => {
                    try!(SigKeyPair::get_public_key_path(&latest.name_with_rev(), cache))
                }
                PairType::Secret => {
                    try!(SigKeyPair::get_secret_key_path(&latest.name_with_rev(), cache))
                }
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
                     Blue.paint(format!("★ Generated origin key pair {}.",
                                        &pair.name_with_rev())));
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
        use std::path::Path;

        use ansi_term::Colour::{Blue, Green, Yellow};
        use common::command::ProgressBar;
        use depot_client::{self, Client};
        use hcore::crypto::keys::parse_name_with_rev;
        use error::{Error, Result};
        use super::get_name_with_rev;

        use hcore::crypto::{PUBLIC_SIG_KEY_VERSION, SECRET_SIG_KEY_VERSION};

        pub fn start(depot: &str,
                     public_keyfile: &Path,
                     secret_keyfile: Option<&Path>)
                     -> Result<()> {
            let depot_client = try!(Client::new(depot, None));
            println!("{}",
                     Yellow.bold()
                         .paint(format!("» Uploading public origin key {}",
                                        public_keyfile.display())));

            let name_with_rev = try!(get_name_with_rev(&public_keyfile, PUBLIC_SIG_KEY_VERSION));
            let (name, rev) = try!(parse_name_with_rev(&name_with_rev));
            println!("{} {}",
                     Green.bold().paint("↑ Uploading"),
                     public_keyfile.display());
            let mut progress = ProgressBar::default();

            match depot_client.put_origin_key(&name, &rev, public_keyfile, Some(&mut progress)) {
                Ok(()) => {
                    println!("{} {}", Green.bold().paint("✓ Uploaded"), &name_with_rev);
                }
                Err(depot_client::Error::HTTP(_)) => {
                    println!("{} {}",
                             Yellow.bold()
                                 .paint("✓ Public key revision already exists in the depot"),
                             &name_with_rev);
                }
                Err(e) => {
                    return Err(Error::DepotClient(e));
                }
            };

            println!("{}",
                     Blue.paint(format!("★ Upload of public origin key {} complete.",
                                        &name_with_rev)));
            if let Some(secret_keyfile) = secret_keyfile {
                let name_with_rev = try!(get_name_with_rev(&secret_keyfile,
                                                           SECRET_SIG_KEY_VERSION));
                let (name, rev) = try!(parse_name_with_rev(&name_with_rev));
                println!("{} {}",
                         Green.bold().paint("↑ Uploading"),
                         secret_keyfile.display());
                let mut progress = ProgressBar::default();
                try!(depot_client.put_origin_secret_key(&name,
                                                        &rev,
                                                        secret_keyfile,
                                                        Some(&mut progress)));
                println!("{} {}", Green.bold().paint("✓ Uploaded"), &name_with_rev);
                println!("{}",
                         Blue.paint(format!("★ Upload of secret origin key {} complete.",
                                            &name_with_rev)));
            }

            Ok(())
        }
    }


    pub mod upload_latest {
        use std::path::Path;

        use ansi_term::Colour::{Blue, Green, Yellow};

        use common::command::ProgressBar;
        use depot_client::{self, Client};
        use error::{Error, Result};
        use hcore::crypto::keys::parse_name_with_rev;
        use hcore::crypto::{PUBLIC_SIG_KEY_VERSION, SECRET_SIG_KEY_VERSION, SigKeyPair};
        use super::get_name_with_rev;

        pub fn start(depot: &str, origin: &str, with_secret: bool, cache: &Path) -> Result<()> {
            let depot_client = try!(Client::new(depot, None));
            let latest = try!(SigKeyPair::get_latest_pair_for(origin, cache));
            let public_keyfile = try!(SigKeyPair::get_public_key_path(&latest.name_with_rev(),
                                                                      cache));

            let name_with_rev = try!(get_name_with_rev(&public_keyfile, PUBLIC_SIG_KEY_VERSION));

            let (name, rev) = try!(parse_name_with_rev(&name_with_rev));


            println!("{} {}",
                     Green.bold().paint("↑ Uploading public key"),
                     public_keyfile.display());
            let mut progress = ProgressBar::default();



            match depot_client.put_origin_key(&name, &rev, &public_keyfile, Some(&mut progress)) {
                Ok(()) => {
                    println!("{} {}", Green.bold().paint("✓ Uploaded"), &name_with_rev);
                }
                Err(depot_client::Error::HTTP(_)) => {
                    println!("{} {}",
                             Yellow.bold()
                                 .paint("✓ Public key revision already exists in the depot"),
                             &name_with_rev);
                }
                Err(e) => {
                    return Err(Error::DepotClient(e));
                }
            };

            println!("{}",
                     Blue.paint(format!("★ Upload of public origin key {} complete.",
                                        &name_with_rev)));

            if with_secret {
                let secret_keyfile = try!(SigKeyPair::get_secret_key_path(&latest.name_with_rev(),
                                                                          cache));

                // we already have this value, but get_name_with_rev will also
                // check the SECRET_SIG_KEY_VERSION
                let name_with_rev = try!(get_name_with_rev(&secret_keyfile,
                                                           SECRET_SIG_KEY_VERSION));
                println!("{} {}",
                         Green.bold().paint("↑ Uploading secret key"),
                         secret_keyfile.display());
                let mut progress = ProgressBar::default();
                try!(depot_client.put_origin_secret_key(&name,
                                                        &rev,
                                                        &secret_keyfile,
                                                        Some(&mut progress)));
                println!("{} {}", Green.bold().paint("✓ Uploaded"), &name_with_rev);
                println!("{}",
                         Blue.paint(format!("★ Upload of secret origin key {} complete.",
                                            &name_with_rev)));
            }


            Ok(())
        }
    }
}
