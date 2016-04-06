// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

use error::{Error, Result};
use hcore::hab_crypto;

pub fn sign(origin_key: &str, infile: &str, outfile: &str) -> Result<()> {
    let key_pairs = try!(hab_crypto::read_sig_origin_keys(origin_key));

    if key_pairs.len() < 1 {
        let msg = format!("Error: no origin keys found with the name: {}", &origin_key);
        return Err(Error::CryptoCLI(msg));
    }
    // we're safe to unwrap here
    let signing_key = key_pairs.first().unwrap();
    debug!("Using key {}", signing_key.name);

    let sk = match signing_key.secret.as_ref() {
        Some(sk) => sk,
        None => {
            let msg = format!("Error: secret origin key not available: {}", &origin_key);
            return Err(Error::CryptoCLI(msg));
        }
    };
    try!(hab_crypto::artifact_sign(infile, outfile, &signing_key.rev, &sk));
    Ok(())
}

pub fn verify(infile: &str, outfile: &str) -> Result<()> {
    try!(hab_crypto::artifact_verify(infile, outfile));
    Ok(())
}


pub fn generate_origin_key(origin_key: &str) -> Result<()> {
    try!(hab_crypto::generate_origin_sig_key(origin_key));
    println!("Successfully generated {} origin key", origin_key);
    Ok(())
}
