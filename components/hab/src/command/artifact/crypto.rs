// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

use std::path::{Path};

use depot_client;
use error::{Error, Result};
use hcore::crypto;


pub fn download_origin_keys(depot: &str, origin: &str, revision: Option<&str>) -> Result<()> {
    let outdir = crypto::nacl_key_dir();
    match revision {
        Some(rev) => {
            try!(depot_client::get_origin_key(depot, origin, rev, &outdir));
        }
        None => {
             try!(depot_client::get_origin_keys(depot, origin, &outdir));
        }
    };
    println!("Successfully downloaded origin key(s)");
    Ok(())
}

pub fn upload_origin_key(depot: &str, keyfile: &Path) -> Result<()> {
    let (origin, revision) = try!(crypto::parse_origin_key_filename(keyfile));
    try!(depot_client::post_origin_key(depot, &origin, &revision, keyfile));
    println!("Successfully uploaded origin key");
    Ok(())
}

pub fn generate_origin_key(origin: &str) -> Result<()> {
    let crypto_ctx = crypto::Context::default();
    let keyname = try!(crypto_ctx.generate_origin_sig_key(origin));
    println!("Successfully generated origin key {}", keyname);
    Ok(())
}

pub fn generate_user_key(user: &str) -> Result<()> {
    let crypto_ctx = crypto::Context::default();
    let keyname = try!(crypto_ctx.generate_user_box_key(user));
    println!("Successfully generated user key {}", keyname);
    Ok(())
}

pub fn generate_service_key(org: &str, service_group: &str) -> Result<()> {
    let crypto_ctx = crypto::Context::default();
    let keyname = try!(crypto_ctx.generate_service_box_key(org, service_group));
    println!("Successfully generated service key {}", keyname);
    Ok(())
}

pub fn hash(infile: &str) -> Result<()> {
    let crypto_ctx = crypto::Context::default();
    let h = try!(crypto_ctx.hash_file(&infile));
    println!("{}", h);
    Ok(())
}

pub fn sign(origin: &str, infile: &str, outfile: &str) -> Result<()> {
    let crypto_ctx = crypto::Context::default();
    let key_pairs = try!(crypto_ctx.read_sig_origin_keys(origin));
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
    try!(crypto_ctx.artifact_sign(infile, outfile, &signing_key.name_with_rev, &sk));
    println!("Successfully created signed binary artifact {}", outfile);
    Ok(())
}

pub fn verify(infile: &str) -> Result<()> {
    let crypto_ctx = crypto::Context::default();
    try!(crypto_ctx.artifact_verify(infile));
    println!("Habitat artifact is valid");
    Ok(())
}
