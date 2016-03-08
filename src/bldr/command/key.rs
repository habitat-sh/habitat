// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

use std::fs;
use std::path::Path;
use std::process::{Command, Stdio, Child};

use ansi_term::Colour::{Yellow, Red};
use time::strptime;
use rpassword::read_password;
use regex::Regex;

use config::Config;
use error::{BldrResult, ErrorKind};
use fs::KEY_CACHE;
use package::{Package, PackageIdent};
use depot;
use util::gpg;

static LOGKEY: &'static str = "KU"; // "key utils"
static USER_KEY_COMMENT: &'static str = "bldr user key";
static SERVICE_KEY_COMMENT: &'static str = "bldr service key";
// "bldr managed keys" all start with a prefix
static BLDR_KEY_PREFIX: &'static str = "bldr_";
static BLDR_EMAIL_SUFFIX: &'static str = "@bldr";

/// Uploads a gpg key to a [depot](../depot).
///
/// If the key starts with a `/`, we treat it as a path to a specific file; otherwise, it's a key
/// to grab from the cache in `/opt/bldr/cache/keys`. Either way, we read the file and upload it to
/// the depot.
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
/// Will upload the `chef-public` key from the local key cache to the depot url.
///
/// ```bash
/// $ bldr upload-key --infile /tmp/chef-public -u http://localhost:9633
/// ```
///
/// Will upload the key at `/tmp/chef-public.asc` to the depot url.
///
pub fn upload(config: &Config) -> BldrResult<()> {
    let url = config.url().as_ref().unwrap();
    let path = Path::new(config.key());

    match fs::metadata(path) {
        Ok(_) => {
            outputln!("Uploading {}", config.key());
            try!(depot::client::put_key(url, path));
        }
        Err(_) => {
            if path.components().count() == 1 {
                let file = format!("{}/{}.asc", KEY_CACHE, config.key());
                let cached = Path::new(&file);
                match fs::metadata(&cached) {
                    Ok(_) => {
                        outputln!("Uploading {}.asc", config.key());
                        try!(depot::client::put_key(url, cached));
                    }
                    Err(_) => {
                        return Err(bldr_error!(ErrorKind::KeyNotFound(config.key().to_string())));
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

/// Imports a gpg key from a [depot](../depot) or a local file.
/// If `config.infile() is not empty, we try to load from a file.
/// Otherwise, we load the key `config.key()` from `config.url()`,
/// drop it in `/opt/bldr/cache/keys`, and then import it into GPG.
///
/// # Failures
///
/// * If the directory `/opt/bldr/cache/keys` cannot be created
/// * If the we fail to download the key from the depot
/// * If the GPG import process fails
///
/// # Examples
///
/// ```bash
/// $ bldr import-key chef-public -u http://localhost:9633
/// ```
///
/// Will download the `chef-public` gpg key from the specified depot.
///
/// ```bash
/// $ bldr import-key --infile /tmp/chef-public.asc
/// ```
///
/// Will install the key found in `/tmp/chef-public.asc`.
///
pub fn import(config: &Config) -> BldrResult<()> {
    match *config.infile() {
        Some(ref infile) => {
            try!(gpg::import(infile));
        }
        None => {
            try!(fs::create_dir_all(KEY_CACHE));
            // docopt requires -u to be set, so we should be safe to unwrap here
            let url = config.url().as_ref().unwrap();
            let filename = try!(depot::client::fetch_key(&url, &config.key(), KEY_CACHE));
            try!(gpg::import(&filename));
        }
    }
    outputln!("Successfully imported key(s)");
    Ok(())
}

/// Exports a public user or service key to a local file.
/// When importing a service key, if the group is not specified
/// it defaults to `default`.
///
/// # Examples
/// ```bash
/// $ bldr export-key --user foo --outfile /tmp/foo.pubkey
/// ```
///
/// ```bash
/// $ bldr export-key --service redis --outfile /tmp/redis.pubkey
/// ```
///
/// ```bash
/// $ bldr export-key --service redis --group dev --outfile /tmp/redis.pubkey
/// ```
///
pub fn export(config: &Config) -> BldrResult<()> {
    // it's either a service key or a user key,
    // docopt will fail if you don't specify one or the other
    let keyname = if config.service_key().is_some() {
        gen_service_key_name(&config.service_key().as_ref().unwrap(), config.group())
    } else {
        gen_user_key_name(&config.user_key().as_ref().unwrap())
    };
    let outfile = config.outfile().as_ref().unwrap();
    try!(gpg::export(&keyname, &outfile));
    outputln!("Successfully exported key {} to {}",
              Yellow.bold().paint(keyname),
              Yellow.bold().paint(outfile as &str));
    Ok(())
}

/// Encrypt and sign a file.
/// Valid user and service keys must be specified.
/// The user and service are both specified as recipients in the encrypted message.
/// The user key is used to sign the encrypted message.
/// If a password is not specified for the user key with the `--password` flag,
/// bldr will display an interactive prompt.
pub fn encrypt_and_sign(config: &Config) -> BldrResult<()> {
    // these values are required per docopt, so they really *shouldn't* be empty
    let userkey = config.user_key().as_ref().unwrap();
    let servicekey = config.service_key().as_ref().unwrap();

    let infile = config.infile().as_ref().unwrap();
    let outfile = config.outfile().as_ref().unwrap();

    outputln!("Attempting to encrypt {}, sending output to {}",
              Yellow.bold().paint(infile as &str),
              Yellow.bold().paint(outfile as &str));

    // prepend BLDR_KEY_PREFIX to the key name, these are the bldr keys
    let full_userkey = gen_user_key_name(&userkey);
    let full_servicekey = gen_service_key_name(&servicekey, config.group());
    let password = match config.password().clone() {
        Some(p) => p,
        None => {
            println!("Please enter the key password for {}:", userkey);
            let password = try!(read_password());
            debug!("The password is: '{}'", password);
            password
        }
    };

    debug!("User key: {}", full_userkey);
    debug!("Service key: {}", full_servicekey);
    debug!("Input file: {}", infile);
    debug!("Output file: {}", outfile);

    try!(gpg::encrypt_and_sign(&full_userkey,
                               &password,
                               &full_servicekey,
                               &infile,
                               &outfile));

    outputln!("Successfully wrote encrypted output to {}",
              Yellow.bold().paint(outfile as &str));
    Ok(())
}

/// Decrypt a file
/// The private key of the service must reside in the current GPG cache.
/// The public key of the user must reside in the current GPG cache to
/// verify the signature.
pub fn decrypt_and_verify(config: &Config) -> BldrResult<()> {
    let infile = config.infile().as_ref().unwrap();
    let outfile = config.outfile().as_ref().unwrap();
    let msg = format!("Attempting to decrypt {}, sending output to {}",
                      infile as &str,
                      outfile as &str);
    outputln!("{}", Yellow.bold().paint(msg));
    try!(gpg::decrypt_and_verify(&infile, &outfile));
    outputln!("Successfully wrote decrypted output to {}",
              Yellow.bold().paint(outfile as &str));
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
fn check_params(params: &gpg::KeygenParams) -> BldrResult<()> {
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
/// `bldr_keyname.group`. If a service key is already in the
/// form BLDR_KEY_PREFIX.+, then just return it
fn gen_service_key_name(keyname: &str, group: &str) -> String {
    let re = String::from(BLDR_KEY_PREFIX) + ".+";
    let regex = Regex::new(&re).unwrap();
    if regex.is_match(keyname) {
        // it's already in normalized form
        keyname.to_string()
    } else {
        format!("{}{}.{}", BLDR_KEY_PREFIX, keyname, group)
    }
}

/// generate a service key email address in the form of
/// `keyname.group@bldr`
fn gen_service_key_email(keyname: &str, group: &str) -> String {
    format!("{}.{}{}", keyname, group, BLDR_EMAIL_SUFFIX)
}

/// generate a user key name in the form of
/// `bldr_keyname`. If a user key is already in the
/// form BLDR_KEY_PREFIX.+, then just return it
fn gen_user_key_name(keyname: &str) -> String {
    let re = String::from(BLDR_KEY_PREFIX) + ".+";
    let regex = Regex::new(&re).unwrap();
    if regex.is_match(keyname) {
        // it's already in normalized form
        keyname.to_string()
    } else {
        format!("{}{}", BLDR_KEY_PREFIX, keyname)
    }
}

/// Used to kill the rngd process in the event of an error
struct DroppableChildProcess {
    child: Child,
}

impl Drop for DroppableChildProcess {
    fn drop(&mut self) {
        debug!("Killing rngd");
        let _ = self.child.kill();
        debug!("Waiting for rngd to die");
        let _ = self.child.wait();
        ()
    }
}

/// run rngd in the background to generate entropy while generating keys.
/// The process is killed when it goes out of scope via `DroppableChildProcess`.
fn run_rngd() -> BldrResult<DroppableChildProcess> {
    debug!("Spawning rngd in the background");
    let res = try!(Package::load(&PackageIdent::new("chef", "rngd", None, None), None));
    let rngdpath = res.join_path("sbin/rngd");
    debug!("RNGD path = {}", rngdpath);
    let child = Command::new(rngdpath)
                    .arg("-f")
                    .arg("-r")
                    .arg("/dev/urandom")
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .spawn()
                    .unwrap_or_else(|e| panic!("failed to execute child: {}", e));
    let droppable = DroppableChildProcess { child: child };
    Ok(droppable)
}

/// generate a user key in gpg
/// A user key requires a password and valid email address
/// If the user key already exists in gpg, it will not be created.
pub fn generate_user_key(config: &Config) -> BldrResult<()> {
    let _child = try!(run_rngd());
    let email = config.email().as_ref().unwrap();
    // clap requires user_key to be specified for generate
    let user_key = config.user_key().as_ref().unwrap();
    let keyname = gen_user_key_name(user_key);

    let password = match config.password().clone() {
        Some(p) => p,
        None => {
            println!("Please enter a password for {}:", user_key);
            let p = try!(read_password());
            debug!("The password is: '{}'", p);
            p
        }
    };

    let fingerprint = {
        let mut params = gpg::KeygenParams::new(&keyname, &email, USER_KEY_COMMENT);
        params.passphrase = Some(&password);
        params.expire_days = config.expire_days().unwrap_or(0);
        try!(check_params(&params));
        try!(gpg::generate(&params))
    };

    outputln!("Successfully generated user key {}",
              Yellow.bold().paint(keyname));

    outputln!("Fingerprint: {}", Yellow.bold().paint(fingerprint));
    Ok(())
}

/// generate a service key in gpg
/// A service key does NOT require a password
/// A service key has an email address automatically generated, in
/// the form: `service.group@bldr`
pub fn generate_service_key(config: &Config) -> BldrResult<()> {
    let _child = try!(run_rngd());
    let comment = SERVICE_KEY_COMMENT;
    // clap requires service_key to be specified
    let service_key = config.service_key().as_ref().unwrap();
    let kn = gen_service_key_name(service_key, config.group());
    let ke = gen_service_key_email(service_key, config.group());
    let fingerprint = {
        let mut params = gpg::KeygenParams::new(&kn, &ke, comment);
        params.passphrase = None;
        params.expire_days = config.expire_days().unwrap_or(0);
        try!(check_params(&params));
        try!(gpg::generate(&params))
    };

    outputln!("Successfully generated service key {}",
              Yellow.bold().paint(kn));
    outputln!("Fingerprint: {}", Yellow.bold().paint(fingerprint));
    Ok(())
}

#[test]
fn gen_key_check_params_test() {
    fn fail_if_err(keyname: &str, email: &str) {
        let params = gpg::KeygenParams::new(keyname, email, "");
        let results = check_params(&params);
        match results {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        };
    }

    fn fail_if_ok(keyname: &str, email: &str) {
        let params = gpg::KeygenParams::new(keyname, email, "");
        let results = check_params(&params);
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
    assert_eq!("bldr_foobar.default",
               gen_service_key_name("foobar", "default"));
}

#[test]
fn gen_service_key_name_test_normalized() {
    assert_eq!("bldr_foobar.default",
               gen_service_key_name("bldr_foobar.default", "default"));
}

#[test]
fn gen_user_key_name_test() {
    assert_eq!("bldr_foobar", gen_user_key_name("foobar"));

}

#[test]
fn gen_user_key_name_test_normalized() {
    assert_eq!("bldr_foobar", gen_user_key_name("bldr_foobar"));

}
