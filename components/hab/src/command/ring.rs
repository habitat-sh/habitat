// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

pub mod key {
    pub mod export {
        use std::io;
        use std::fs::File;
        use std::path::Path;

        use hcore::crypto::SymKey;

        use error::Result;

        pub fn start(ring: &str, cache: &Path) -> Result<()> {
            let latest = try!(SymKey::get_latest_pair_for(ring, cache));
            let path = try!(SymKey::get_secret_key_path(&latest.name_with_rev(), cache));
            let mut file = try!(File::open(&path));
            debug!("Streaming file contents of {} to standard out",
                   &path.display());
            try!(io::copy(&mut file, &mut io::stdout()));
            Ok(())
        }
    }

    pub mod generate {
        use std::path::Path;

        use hcore::crypto::SymKey;

        use error::Result;

        pub fn start(ring: &str, cache: &Path) -> Result<()> {
            let pair = try!(SymKey::generate_pair_for_ring(ring, cache));
            println!("Successfully generated ring key {}", pair.name_with_rev());
            Ok(())
        }
    }

    pub mod import {
        use std::path::Path;

        use hcore::crypto::SymKey;

        use error::Result;

        pub fn start(content: &str, cache: &Path) -> Result<()> {
            let pair = try!(SymKey::write_file_from_str(content, cache));
            println!("Imported key {}", pair.name_with_rev());
            Ok(())
        }
    }
}
