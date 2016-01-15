// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

use std::fs::{self, File};

use gpgme;

use fs::GPG_CACHE;
use error::{BldrResult, BldrError, ErrorKind};
use util::perm;

static LOGKEY: &'static str = "GP";

pub fn decrypt<'a>(file: &str) -> BldrResult<gpgme::Data<'a>> {
    let mut ctx = try!(init_ctx());
    let mut signature = {
        let f = match File::open(&file) {
            Ok(f) => f,
            Err(_) => return Err(bldr_error!(ErrorKind::FileNotFound(String::from(file)))),
        };
        match gpgme::Data::from_seekable_reader(f) {
            Ok(data) => data,
            Err(_) => return Err(bldr_error!(ErrorKind::FileNotFound(String::from(file)))),
        }
    };
    let mut out = try!(gpgme::Data::new());
    try!(ctx.decrypt(&mut signature, &mut out));
    Ok(out)
}

pub fn import(keyfile: &str) -> BldrResult<()> {
    try!(fs::create_dir_all(GPG_CACHE));
    try!(perm::set_permissions(GPG_CACHE, "0700"));
    let mut ctx = try!(init_ctx());
    let mut data = try!(gpgme::Data::load(&keyfile));
    try!(data.set_encoding(gpgme::data::ENCODING_URL));
    match ctx.import(&mut data) {
        Ok(_) => {
            outputln!("{} GPG key imported", keyfile);
            Ok(())
        }
        Err(e) => Err(BldrError::from(e)),
    }
}

pub fn verify<'a>(file: &str) -> BldrResult<gpgme::Data<'a>> {
    let mut ctx = try!(init_ctx());

    let mut signature = {
        let f = match File::open(&file) {
            Ok(f) => f,
            Err(_) => return Err(bldr_error!(ErrorKind::FileNotFound(String::from(file)))),
        };
        match gpgme::Data::from_seekable_reader(f) {
            Ok(data) => data,
            Err(_) => return Err(bldr_error!(ErrorKind::FileNotFound(String::from(file)))),
        }
    };

    let mut plain = try!(gpgme::Data::new());
    match ctx.verify(&mut signature, None, Some(&mut plain)) {
        Ok(_) => Ok(plain),
        Err(e) => Err(BldrError::from(e)),
    }
}

fn init_ctx() -> BldrResult<gpgme::Context> {
    let mut ctx = try!(gpgme::create_context());
    try!(ctx.set_engine_info(gpgme::PROTOCOL_OPENPGP, None, Some(String::from(GPG_CACHE))));
    Ok(ctx)
}
