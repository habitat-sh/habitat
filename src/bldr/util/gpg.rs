//
// Copyright:: Copyright (c) 2015 Chef Software, Inc.
// License:: Apache License, Version 2.0
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

use std::fs::{self, File};

use gpgme;

use fs::GPG_CACHE;
use error::{BldrResult, BldrError, ErrorKind};
use util::perm;

static LOGKEY: &'static str = "GP";

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

pub fn verify(file: &str) -> BldrResult<()> {
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
        Ok(_) => Ok(()),
        Err(e) => Err(BldrError::from(e)),
    }
}

fn init_ctx() -> BldrResult<gpgme::Context> {
    let mut ctx = try!(gpgme::create_context());
    try!(ctx.set_engine_info(gpgme::PROTOCOL_OPENPGP, None, Some(String::from(GPG_CACHE))));
    Ok(ctx)
}
