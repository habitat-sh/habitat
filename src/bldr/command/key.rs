// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

use std::fs;
use std::path::Path;
use fs::KEY_CACHE;
use config::Config;
use error::{BldrResult, ErrorKind};
use util::gpg;
use regex::Regex;
use ansi_term::Colour::{Yellow, Red};
use time::strptime;

use repo;

static LOGKEY: &'static str = "KU";
static USER_KEY_COMMENT: &'static str = "bldr user key";
static SERVICE_KEY_COMMENT: &'static str = "bldr service key";

/// Uploads a gpg key to a [repo](../repo).
///
/// If the key starts with a `/`, we treat it as a path to a specific file; otherwise, it's a key
/// to grab from the cache in `/opt/bldr/cache/keys`. Either way, we read the file and upload it to
/// the repository.
///
/// # Failures
///
/// * If the file fails to exist, or if we can't read it
/// * If the http upload fails
/// # Examples
///
/// ```bash
/// $ bldr upload-key chef-public -u http://localhost:9633
/// ```
///
/// Will upload the `chef-public` key from the local key cache to the repo url.
///
/// ```bash
/// $ bldr upload-key /tmp/chef-public -u http://localhost:9633
/// ```
///
/// Will upload the key at `/tmp/chef-public.asc` to the repo url.
///
pub fn upload(config: &Config) -> BldrResult<()> {
    let url = config.url().as_ref().unwrap();
    let path = Path::new(config.key());

    match fs::metadata(path) {
        Ok(_) => {
            outputln!("Uploading {}", config.key());
            try!(repo::client::put_key(url, path));
        }
        Err(_) => {
            if path.components().count() == 1 {
                let file = format!("{}/{}.asc", KEY_CACHE, config.key());
                let cached = Path::new(&file);
                match fs::metadata(&cached) {
                    Ok(_) => {
                        outputln!("Uploading {}.asc", config.key());
                        try!(repo::client::put_key(url, cached));
                    }
                    Err(_) => {
                        return Err(bldr_error!(ErrorKind::KeyNotFound(config.key().to_string())))
                    }
                }
            } else {
                return Err(bldr_error!(ErrorKind::FileNotFound(config.key().to_string())));
            }
        }
    }
    outputln!("Complete");
    Ok(())
}

/// Installs a gpg key from a [repo](../repo) or a local file.
/// If `config.url()` is empty, we assume the value
/// of `config.key()` is a path to the key. Otherwise, we download the
/// key from the repo at `config.url()`, drop it in `/opt/bldr/cache/keys`,
/// and then import it into GPG.
///
/// # Failures
///
/// * If the directory `/opt/bldr/cache/keys` cannot be created
/// * If the we fail to download the key from the repo
/// * If the GPG import process fails
///
/// # Examples
///
/// ```bash
/// $ bldr install-key chef-public -u http://localhost:9633
/// ```
///
/// Will download the `chef-public` gpg key from the specified repo.
///
/// ```bash
/// $ bldr install-key /tmp/chef-public.asc
/// ```
///
/// Will install the key found in `/tmp/chef-public.asc`.
///
pub fn install(config: &Config) -> BldrResult<()> {
    match *config.url() {
        Some(ref url) => {
            if url.is_empty() {
                try!(gpg::import(&config.key()));
            }
            try!(fs::create_dir_all(KEY_CACHE));
            let filename = try!(repo::client::fetch_key(url, &config.key(), KEY_CACHE));
            try!(gpg::import(&filename));
        }
        None => try!(gpg::import(&config.key())),
    }
    Ok(())
}



/// list ALL keys in gpg
pub fn list(_config: &Config) -> BldrResult<()> {
    let keys = try!(gpg::list());
    for key in &keys {
        for (_, user) in key.user_ids().enumerate() {
            // Pull out the uid so Cow is happy
            let uid = user.uid().unwrap_or("");
            println!("{}", Yellow.bold().paint(uid));
        }
        println!("   Key: {}", key.id().unwrap_or("---"));
        println!("   Fingerprint: {}", key.fingerprint().unwrap_or("---"));
        let primary_key = key.primary_key().unwrap();
        let expire = match primary_key.expires() {
            None => "Never".to_string(),
            Some(val) => {
                let datetime = strptime(&val.to_string(), "%s").unwrap();
                format!("{}", datetime.rfc822())
            }
        };
        println!("   Expires: {}", expire);
        if key.is_revoked() {
            println!("\t{}", Red.bold().paint("Revoked"));
        }
        if key.is_expired() {
            println!("\t{}", Red.bold().paint("Expired"));
        }
        if key.is_disabled() {
            println!("\t{}", Red.bold().paint("Disabled"));
        }
        if key.is_invalid() {
            println!("\t{}", Red.bold().paint("Invalid"));
        }
    }
    Ok(())
}

/// ensure parameters are correct before generating
/// gpg "xml-ish" parameter string
fn check_params(params: gpg::KeygenParams) -> BldrResult<()> {
    // must be at least 5 characters
    if params.keyname.len() < 5 {
        return Err(bldr_error!(ErrorKind::InvalidKeyParameter("key name must be at least 5 \
                                                               characters in length"
                                                                  .to_string())));
    }

    // must contain an @ sign between 1 or more characters
    let re = Regex::new(r".+@.+").unwrap();
    if !re.is_match(&params.email) {
        return Err(bldr_error!(ErrorKind::InvalidKeyParameter("key email address must contain \
                                                               an @ symbol"
                                                                  .to_string())));
    }
    return Ok(());
}

/// generate a service key name in the form of
/// `bldr: keyname.group`
fn gen_service_key_name(keyname: &str, group: &str) -> String {
    format!("bldr: {}.{}", keyname, group)
}

/// generate a service key email address in the form of
/// `keyname.group@bldr`
fn gen_service_key_email(keyname: &str, group: &str) -> String {
    format!("{}.{}@bldr", keyname, group)
}

/// generate a user key name in the form of
/// `bldr: keyname`
fn gen_user_key_name(keyname: &str) -> String {
    format!("bldr: {}", keyname)
}

/// generate a user key in gpg
/// A user key requires a password and valid email address
/// If the user key already exists in gpg, it will not be created.
pub fn generate_user_key(config: &Config) -> BldrResult<()> {
    let comment = USER_KEY_COMMENT.to_string();
    let email = config.email().clone().unwrap();
    let kn = gen_user_key_name(config.key());
    let mut params = gpg::KeygenParams::new(kn, email, comment);
    params.passphrase = config.password().clone();
    params.expire_days = config.expire_days().unwrap_or(0);
    try!(check_params(params.clone()));
    try!(gpg::generate(params));
    Ok(())
}

/// generate a service key in gpg
/// A service key does NOT require a password
/// A service key has an email address automatically generated, in
/// the form: `service.group@bldr`
pub fn generate_service_key(config: &Config) -> BldrResult<()> {
    let comment = SERVICE_KEY_COMMENT.to_string();
    let kn = gen_service_key_name(config.key(), config.group());
    let ke = gen_service_key_email(config.key(), config.group());
    let mut params = gpg::KeygenParams::new(kn, ke, comment);
    params.passphrase = None;
    params.expire_days = config.expire_days().unwrap_or(0);
    try!(check_params(params.clone()));
    try!(gpg::generate(params));
    Ok(())
}


#[test]
fn gen_key_check_params_test() {
    fn fail_if_err(keyname: &str, email: &str) {
        let params = gpg::KeygenParams::new(keyname.to_string(), email.to_string(), "".to_string());
        let results = check_params(params);
        match results {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        };
    }

    fn fail_if_ok(keyname: &str, email: &str) {
        let params = gpg::KeygenParams::new(keyname.to_string(), email.to_string(), "".to_string());
        let results = check_params(params);
        match results {
            Ok(_) => assert!(false),
            Err(_) => assert!(true),
        };
    }

    // validate key names
    fail_if_ok("", "foo@bar");
    fail_if_ok("a", "foo@bar");
    fail_if_err("abcde", "foo@bar");

    // validate email addresses
    fail_if_ok("foobar", "");
    fail_if_ok("foobar", "a");
    fail_if_ok("foobar", "foo@");
    fail_if_err("foobar", "foo@bar");
}

#[test]
fn gen_service_key_email_test() {
    assert_eq!("foobar.default@bldr",
               gen_service_key_email("foobar", "default"));
}

#[test]
fn gen_service_key_name_test() {
    assert_eq!("bldr: foobar.default",
               gen_service_key_name("foobar", "default"));
}

#[test]
fn gen_user_key_name_test() {
    assert_eq!("bldr: foobar", gen_user_key_name("foobar"));

}
