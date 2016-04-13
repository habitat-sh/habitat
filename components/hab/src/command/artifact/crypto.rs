// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

use error::{Error, Result};
use hcore::crypto;

pub fn generate_origin_key(origin: &str) -> Result<()> {
    let keyname = try!(crypto::generate_origin_sig_key(origin));
    println!("Successfully generated origin key {}", keyname);
    Ok(())
}

pub fn generate_user_key(origin: &str, user: &str) -> Result<()> {
    let keyname = try!(crypto::generate_user_box_key(origin, user));
    println!("Successfully generated user key {}", keyname);
    Ok(())
}

pub fn generate_service_key(origin: &str, service: &str, group: &str) -> Result<()> {
    let keyname = try!(crypto::generate_service_box_key(origin, service, group));
    println!("Successfully generated service key {}", keyname);
    Ok(())
}

pub fn hash(infile: &str) -> Result<()> {
    let h = try!(crypto::hash_file(&infile));
    println!("{}", h);
    Ok(())
}

pub fn sign(origin: &str, infile: &str, outfile: &str) -> Result<()> {
    let key_pairs = try!(crypto::read_sig_origin_keys(origin));
    if key_pairs.len() < 1 {
        let msg = format!("Error: no origin keys found with the name: {}", &origin);
        return Err(Error::CryptoCLI(msg));
    }
    // we're safe to unwrap here
    let signing_key = key_pairs.first().unwrap();
    debug!("Using key {}", signing_key.name);

    let sk = match signing_key.secret.as_ref() {
        Some(sk) => sk,
        None => {
            let msg = format!("Error: secret origin key not available: {}", &origin);
            return Err(Error::CryptoCLI(msg));
        }
    };
    try!(crypto::artifact_sign(infile, outfile, &signing_key.rev, &sk));
    Ok(())
}

pub fn verify(infile: &str) -> Result<()> {
    try!(crypto::artifact_verify(infile));
    println!("Habitat artifact is valid");
    Ok(())
}

