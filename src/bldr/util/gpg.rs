// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

use std::fs::{self, File};
use gpgme;
use gpgme::ops;

use fs::GPG_CACHE;
use error::{BldrResult, BldrError, ErrorKind};
use util::perm;
use ansi_term::Colour::Blue;

static LOGKEY: &'static str = "KEY";


static DEFAULT_KEY_TYPE: &'static str = "RSA";
const DEFAULT_KEY_LENGTH: u16 = 2048;

static DEFAULT_SUBKEY_TYPE: &'static str = "RSA";
const DEFAULT_SUBKEY_LENGTH: u16 = 2048;

#[derive(Clone)]
pub struct KeygenParams {
    pub keyname: String,
    pub email: String,
    pub comment: String,
    pub expire_days: u16,
    pub passphrase: Option<String>,
    pub key_type: String,
    pub key_length: u16,
    pub subkey_type: String,
    pub subkey_length: u16,
}

impl KeygenParams {
    /// Create a default `KeygenParams`
    pub fn new(keyname: String, email: String, comment: String) -> KeygenParams {
        KeygenParams {
            keyname: keyname,
            email: email,
            comment: comment,
            expire_days: 0,
            passphrase: None,
            key_type: DEFAULT_KEY_TYPE.to_string(),
            key_length: DEFAULT_KEY_LENGTH,
            subkey_type: DEFAULT_SUBKEY_TYPE.to_string(),
            subkey_length: DEFAULT_SUBKEY_LENGTH,
        }
    }

    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn to_gpg_params_string(self) -> String {
        let ptext = match self.passphrase {
            Some(p) => format!("Passphrase: {}", p),
            None => "".to_string(),
        };

    // NOTE: you can't pass Passphrase: with an empty string
    // so don't pass the string at all if a value doesn't exist
        let s = format!("
                <GnupgKeyParms format=\"internal\">
                    Key-Type: {key_type}
                    Key-Length: {key_length}
                    Subkey-Type: {subkey_type}
                    Subkey-Length: {subkey_length}
                    Name-Real: {name_real}
                    Name-Comment: {name_comment}
                    Name-Email: {name_email}
                    Expire-Date: {expire_date}
                    {passphrase_text}
                </GnupgKeyParms>\n",
                        key_type = self.key_type,
                        key_length = self.key_length,
                        subkey_type = self.subkey_type,
                        subkey_length = self.subkey_length,
                        name_real = self.keyname,
                        name_comment = self.comment,
                        name_email = self.email.clone(),
                        expire_date = self.expire_days,
                        passphrase_text = ptext);
        s
    }
}

fn init_ctx() -> BldrResult<gpgme::Context> {
    let mut ctx = try!(gpgme::create_context());
    try!(ctx.set_engine_info(gpgme::PROTOCOL_OPENPGP, None, Some(String::from(GPG_CACHE))));
    Ok(ctx)
}

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

pub fn generate(params: KeygenParams) -> BldrResult<()> {
    try!(fs::create_dir_all(GPG_CACHE));
    try!(perm::set_permissions(GPG_CACHE, "0700"));

    let exists = try!(key_exists(&params.keyname));
    if exists == true {
        return Err(bldr_error!(ErrorKind::InvalidKeyParameter(String::from("Key already exists"))));
    }

    let mut ctx = try!(init_ctx());
    let s = params.to_gpg_params_string();
    debug!("{}", s);
    // GnuPG does not support public and secret, they should be NULL.
    // https://www.gnupg.org/documentation/manuals/gpgme/Generating-Keys.html#Generating-Keys
    match ctx.generate_key(s.clone(), None, None) {
        Ok(key_gen_result) => {
            outputln!("Fingerprint: {}", Blue.bold().paint(key_gen_result.fingerprint().unwrap()));
            Ok(())
        }
        Err(e) => {
            outputln!("Error generating key {}", e);
            Err(BldrError::from(e))
        }
    }
}

pub fn key_exists(keyname: &str) -> BldrResult<bool> {
    try!(fs::create_dir_all(GPG_CACHE));
    try!(perm::set_permissions(GPG_CACHE, "0700"));

    let mut ctx = try!(init_ctx());
    let mode = ops::KeyListMode::empty();
    ctx.set_key_list_mode(mode).unwrap();
    let searchexp = vec![keyname];
    // pull this out into it's own value,
    // otherwise ctx doesn't live long enough
    let keymatch = ctx.find_keys(searchexp);
    match keymatch {
        Ok(keys) => {
            if keys.count() > 0 {
                Ok(true)
            } else {
                Ok(false)
            }
        }
        Err(e) => {
            Err(BldrError::from(e))
        }
    }
}

pub fn list() -> BldrResult<Vec<gpgme::keys::Key>> {
    try!(fs::create_dir_all(GPG_CACHE));
    try!(perm::set_permissions(GPG_CACHE, "0700"));
    let mut ctx = try!(init_ctx());

    // https://www.gnupg.org/documentation/manuals/gpgme/Key-Listing-Mode.html#Key-Listing-Mode
    let mode = ops::KeyListMode::empty();

    ctx.set_key_list_mode(mode).unwrap();

    let mut allkeys = Vec::new();
    // get ALL keys
    let mut keys = ctx.keys().unwrap();

    for key in keys.by_ref().filter_map(Result::ok) {
        allkeys.push(key.clone());
    }
    Ok(allkeys)
}
