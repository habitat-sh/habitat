// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

pub mod export {
    use std::io;
    use std::fs::File;

    use hcore::{self, crypto};

    use error::{Error, Result};

    pub fn start(ring: &str) -> Result<()> {
        let ctx = crypto::Context::default();
        let mut candidates = try!(ctx.read_sym_keys(&ring));
        let latest = match candidates.len() {
            1 => candidates.remove(0),
            _ => {
                let msg = format!("Cannot find a suitable key for ring: {}", ring);
                return Err(Error::HabitatCore(hcore::Error::CryptoError(msg)));
            }
        };

        let path = try!(ctx.get_sym_secret_key_path(&latest.name_with_rev));
        let mut file = try!(File::open(&path));
        debug!("Streaming file contents of {} to standard out",
               &path.display());
        try!(io::copy(&mut file, &mut io::stdout()));
        Ok(())
    }
}

pub mod import {
    use hcore::crypto;

    use error::Result;

    pub fn start(content: &str) -> Result<()> {
        let ctx = crypto::Context::default();
        let keyname = try!(ctx.write_sym_key_from_str(content));
        println!("Imported key {}", keyname);
        Ok(())
    }
}

pub mod generate {
    use hcore::crypto;

    use error::Result;

    pub fn start(ring: &str) -> Result<()> {
        let crypto_ctx = crypto::Context::default();
        let keyname = try!(crypto_ctx.generate_ring_sym_key(ring));
        println!("Successfully generated ring key {}", keyname);
        Ok(())
    }
}
