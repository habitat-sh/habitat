// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

use std::fs::{self, File};
use std::io;
use std::io::Write;
use std::env;
use gpgme;
use gpgme::ops;
use fs::GPG_CACHE;
use error::{BldrResult, BldrError, ErrorKind};
use util::perm;

static LOGKEY: &'static str = "KEY";
static BLDR_GPG_CACHE_ENV_VAR: &'static str = "BLDR_GPG_CACHE";

static DEFAULT_KEY_TYPE: &'static str = "RSA";
const DEFAULT_KEY_LENGTH: u16 = 2048;

static DEFAULT_SUBKEY_TYPE: &'static str = "RSA";
const DEFAULT_SUBKEY_LENGTH: u16 = 2048;

static CACHE_DIR_PERMS: &'static str = "0700";

#[derive(Clone, Debug)]
pub struct KeygenParams<'a> {
    pub keyname: &'a str,
    pub email: &'a str,
    pub comment: &'a str,
    pub expire_days: u16,
    pub passphrase: Option<&'a String>,
    pub key_type: String,
    pub key_length: u16,
    pub subkey_type: String,
    pub subkey_length: u16,
}

impl<'a> KeygenParams<'a> {
    /// Create a `KeygenParams` with several values defaulted
    pub fn new(keyname: &'a str, email: &'a str, comment: &'a str) -> KeygenParams<'a> {
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
    pub fn to_gpg_params_string(&self) -> String {
        let ptext = match self.passphrase.as_ref() {
            Some(p) => format!("Passphrase: {}", p),
            None => "".to_string(),
        };

// NOTE: you can't pass Passphrase: with an empty string
// so don't pass the string at all if a value doesn't exist
        format!("
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
                name_email = self.email,
                expire_date = self.expire_days,
                passphrase_text = ptext)
    }
}

/// Use either the hardcoded bldr GPG path
/// or use the value from BLDR_GPG_CACHE_ENV_VAR
fn gpg_cache_dir() -> String {
    let gpgcache = match env::var(BLDR_GPG_CACHE_ENV_VAR) {
        Ok(val) => String::from(val),
        Err(_) => String::from(GPG_CACHE),
    };
    debug!("GPG cache = {}", gpgcache);
    gpgcache
}

/// Initialize the gpgme context w/ OpenPGP protocol and
/// GPG cache directory (which may come from an env var)
fn init_ctx() -> BldrResult<gpgme::Context> {
    let mut ctx = try!(gpgme::create_context());
    try!(ctx.set_engine_info(gpgme::PROTOCOL_OPENPGP, None, Some(gpg_cache_dir())));
    Ok(ctx)
}

/// Create the GPG cache directory if it doesn't exist
/// Set the permissions on the directory so the user has r/w/x only
fn ensure_gpg_dir() -> BldrResult<()> {
    try!(fs::create_dir_all(gpg_cache_dir()));
    try!(perm::set_permissions(&gpg_cache_dir(), CACHE_DIR_PERMS));
    Ok(())
}

pub fn decrypt<'a>(file: &str) -> BldrResult<gpgme::Data<'a>> {
    let mut ctx = try!(init_ctx());
    try!(ensure_gpg_dir());
    let mut signature = {
        let f = match File::open(&file) {
            Ok(f) => f,
            Err(e) => return Err(BldrError::from(e)),
        };
        match gpgme::Data::from_seekable_reader(f) {
            Ok(data) => data,
            Err(wrapped_error) => return Err(BldrError::from(wrapped_error.error())),
        }
    };
    let mut out = try!(gpgme::Data::new());
    try!(ctx.decrypt(&mut signature, &mut out));
    Ok(out)
}

/// encrypt and sign a file
/// Encrypt uses a service public key and a user public key as recipients,
/// message signing uses a **user** *private* key.
pub fn encrypt_and_sign(userkey: &str,
                        password: &str,
                        servicekey: &str,
                        infile: &str,
                        outfile: &str)
                        -> BldrResult<()> {
    let mut ctx = try!(init_ctx());

    let mut infiledata = {
        let f = match File::open(&infile) {
            Ok(f) => f,
            Err(e) => return Err(BldrError::from(e)),
        };
        match gpgme::Data::from_seekable_reader(f) {
            Ok(data) => data,
            Err(wrapped_error) => return Err(BldrError::from(wrapped_error.error())),

        }
    };

    debug!("Loading keys");
    let ukey = try!(find_key(userkey));
    // let ukey2 = try!(find_key(userkey));
    let skey = try!(find_key(servicekey));

    if let None = ukey {
        return Err(bldr_error!(ErrorKind::InvalidKeyParameter(String::from("User key not found"))));
    }
    //
    // if let None = ukey2 {
    // return Err(bldr_error!(ErrorKind::InvalidKeyParameter(String::from("User key not found"))));
    // }
    //
    if let None = skey {
        return Err(bldr_error!(ErrorKind::InvalidKeyParameter(String::from("Service key not \
                                                                            found"))));
    }

    let ukey = ukey.unwrap();
    // let ukey2 = ukey2.unwrap();
    let skey = skey.unwrap();

    let recipients = vec![skey, ukey];
    debug!("Keys loaded");

    // generate text output, including BEGIN GPG MESSAGE / ENG GPG MESSAGE
    ctx.set_armor(true);

    // ensure we don't pick up any stragglers
    ctx.clear_signers();

    // user signs this key for a service
    try!(ctx.add_signer(&recipients[1]));

    // move the password to the heap for the closure below
    let p = password.to_string();

    // NOTE: with_passphase_cb will probably change it's parameter types
    // in a future version of rust-gpgme
    let mut guard = ctx.with_passphrase_cb(move |_: Option<&str>,
                                                 _: Option<&str>,
                                                 _,
                                                 out: &mut io::Write| {
        debug!("Using password '{}'", p.clone());
        try!(out.write_all(p.as_bytes()));
        Ok(())
    });

    // store the encrypted output here
    let mut output = try!(gpgme::Data::new());

    // NOTE: ENCRYPT_ALWAYS_TRUST is being used below,
    // This assumes that any key in your GPG keystore is trusted.
    // see also: https://www.gnupg.org/gph/en/manual/x334.html
    try!(guard.encrypt_and_sign(&recipients,
                                ops::ENCRYPT_ALWAYS_TRUST,
                                &mut infiledata,
                                &mut output));
    try!(write_output(outfile, output));
    Ok(())
}

/// Decrypt uses a service private key and a user public key to verify an encrypted message.
///	A service OR a user should be able to decrypt bldr-encrypted a message.
pub fn decrypt_and_verify(infile: &str, outfile: &str) -> BldrResult<()> {
    let mut ctx = try!(init_ctx());
    let mut infiledata = {
        let f = match File::open(&infile) {
            Ok(f) => f,
            Err(e) => return Err(BldrError::from(e)),
        };
        match gpgme::Data::from_seekable_reader(f) {
            Ok(data) => data,
            Err(wrapped_error) => return Err(BldrError::from(wrapped_error.error())),
        }
    };
    let mut output = try!(gpgme::Data::new());

    match ctx.decrypt_and_verify(&mut infiledata, &mut output) {
        Ok((decrypt_result, verify_result)) => {
            debug!("Wrong key usage? {}", decrypt_result.wrong_key_usage());
            debug!("unsupported_algorithm? {:?}",
                   decrypt_result.unsupported_algorithm());
            debug!("recipients? {:?}", decrypt_result.recipients());
            debug!("filename? {:?}", decrypt_result.filename());
            debug!("signatures? {:?}", verify_result.signatures());
            debug!("filename? {:?}", verify_result.filename());
        }
        Err(e) => return Err(BldrError::from(e)),
    };

    try!(write_output(outfile, output));
    Ok(())
}

/// Keep me, I'm useful for troubleshooting key import errors.
fn debug_import_result(result: ops::ImportResult) {
    for i in result.imports() {
        debug!("Fingerprint: {}", i.fingerprint().unwrap_or("none"));
        debug!("Error: {:?}", i.result().err());
        let status = i.status();
        if status.contains(ops::IMPORT_NEW) {
            debug!("NEW");
        }
        if status.contains(ops::IMPORT_SECRET) {
            debug!("SECRET");
        }
        if status.contains(ops::IMPORT_SIG) {
            debug!("SIG");
        }
        if status.contains(ops::IMPORT_SUBKEY) {
            debug!("SUBKEY");
        }
        if status.contains(ops::IMPORT_UID) {
            debug!("UID");
        }
    }

    debug!("Considered: {}", result.considered());
    debug!("Imported: {}", result.imported());
    debug!("Imported RSA: {}", result.imported_rsa());
    debug!("Not imported: {}", result.not_imported());
    debug!("Unchanged: {}", result.unchanged());

    debug!("No user id: {}", result.no_user_id());

    debug!("New user ids: {}", result.new_user_ids());
    debug!("New subkeys: {}", result.new_subkeys());
    debug!("New signatures: {}", result.new_signatures());
    debug!("New revocations: {}", result.new_revocations());

    debug!("Secret read: {}", result.secret_read());
    debug!("Secret imported: {}", result.secret_imported());
    debug!("Secret unchanged: {}", result.secret_unchanged());
}

/// Import a public key into the GPG cache
pub fn import(keyfile: &str) -> BldrResult<()> {
    try!(ensure_gpg_dir());
    let mut ctx = try!(init_ctx());
    let mut data = try!(gpgme::Data::load(&keyfile));
    ctx.set_protocol(gpgme::PROTOCOL_OPENPGP).unwrap();
    match ctx.import(&mut data) {
        Ok(result) => {
            debug_import_result(result);
            Ok(())
        }
        Err(e) => Err(BldrError::from(e)),
    }
}

/// Export a public key from the GPG cache.
pub fn export(key: &str, outfile: &str) -> BldrResult<()> {
    let mut ctx = try!(init_ctx());
    let key_search = try!(find_key(&key));
    if let None = key_search {
        return Err(bldr_error!(ErrorKind::InvalidKeyParameter(String::from("Key not found"))));
    }
    let k = key_search.unwrap();
    let keys = vec![k];
    let mode = ops::ExportMode::empty();
    let mut output = gpgme::Data::new().unwrap();
    ctx.set_armor(true);
    if let Err(e) = ctx.export_keys(&keys, mode, Some(&mut output)) {
        return Err(BldrError::from(e));
    };

    try!(write_output(outfile, output));
    Ok(())
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
            Err(wrapped_error) => return Err(BldrError::from(wrapped_error.error())),
        }
    };

    let mut plain = try!(gpgme::Data::new());
    match ctx.verify(&mut signature, None, Some(&mut plain)) {
        Ok(_) => Ok(plain),
        Err(e) => Err(BldrError::from(e)),
    }
}

/// Generate a key with the given params. It must not already exist
/// in the GPG cache.
/// `RUST_LOG=bldr=debug` is your friend for this one, as the `ctx.generate_key`
/// function takes an ugly xml-ish string.
pub fn generate(params: &KeygenParams) -> BldrResult<String> {
    try!(ensure_gpg_dir());

    let k = try!(find_key(&params.keyname));
    if k.is_some() {
        return Err(bldr_error!(ErrorKind::InvalidKeyParameter(String::from("Key already exists"))));
    }

    let mut ctx = try!(init_ctx());
    let s = params.to_gpg_params_string();
    debug!("{}", s);
    // GnuPG does not support public and secret, they should be NULL.
    // https://www.gnupg.org/documentation/manuals/gpgme/Generating-Keys.html#Generating-Keys
    match ctx.generate_key(s, None, None) {
        Ok(key_gen_result) => {
            match key_gen_result.fingerprint() {
                Some(fp) => return Ok(fp.to_string()),
                // Not sure why this would happen, but here it is!
                None => Ok("No fingerprint".to_string()),
            }
        }
        Err(e) => Err(BldrError::from(e)),
    }
}

/// Return the **first** key that contains the `keyname` **first** in it's list of users
pub fn find_key(keyname: &str) -> BldrResult<Option<gpgme::keys::Key>> {
    try!(ensure_gpg_dir());
    let mut ctx = try!(init_ctx());
    let mode = ops::KeyListMode::empty();
    ctx.set_key_list_mode(mode).unwrap();
    let mut keys = try!(ctx.keys());
    // irritatingly nested
    for key in keys.by_ref().filter_map(Result::ok) {
        match key.user_ids().enumerate().next() {
            Some((_, user)) => {
                if let Some(n) = user.name() {
                    if n == keyname {
                        return Ok(Some(key.clone()));
                    }
                }
            }
            None => {}
        }
    }
    Ok(None)
}

/// query all keys in the gpg cache and return a vec
/// with a copy of each
pub fn list() -> BldrResult<Vec<gpgme::keys::Key>> {
    try!(ensure_gpg_dir());
    let mut ctx = try!(init_ctx());

    // https://www.gnupg.org/documentation/manuals/gpgme/Key-Listing-Mode.html#Key-Listing-Mode
    let mut mode = ops::KeyListMode::empty();
    // this is the default mode, but specify it anyways to be explicit
    mode.insert(ops::KEY_LIST_MODE_LOCAL);
    ctx.set_key_list_mode(mode).unwrap();

    let mut allkeys = Vec::new();
    // get ALL keys
    let mut keys = ctx.keys().unwrap();
    for key in keys.by_ref().filter_map(Result::ok) {
        allkeys.push(key.clone());
    }
    Ok(allkeys)
}

/// write the output from a gpgme::Data object to a file
fn write_output(outfile: &str, output: gpgme::Data) -> BldrResult<()> {
    match output.into_string() {
        Ok(o) => {
            let mut f = try!(File::create(outfile));
            try!(f.write_all(o.as_bytes()));
        }
        Err(e) => {
            match e {
                Some(utf8_error) => return Err(BldrError::from(utf8_error)),
                None => panic!("File output error: {:?}", e),
            }
        }
    }
    Ok(())
}
