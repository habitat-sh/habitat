// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

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
