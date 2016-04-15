// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

use std::io::prelude::*;
use std::fs;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::env;
use std::mem;
use std::path::Path;

use libsodium_sys;
use sodiumoxide::init as nacl_init;
use sodiumoxide::crypto::sign;
use sodiumoxide::crypto::box_;
use sodiumoxide::crypto::sign::ed25519::SecretKey as SigSecretKey;
use sodiumoxide::crypto::sign::ed25519::PublicKey as SigPublicKey;
use sodiumoxide::crypto::box_::curve25519xsalsa20poly1305::PublicKey as BoxPublicKey;
use sodiumoxide::crypto::box_::curve25519xsalsa20poly1305::SecretKey as BoxSecretKey;
use rustc_serialize::base64::{STANDARD, ToBase64, FromBase64};
use time;

use error::{Error, Result};
use fs::CACHE_KEY_PATH;
use util::perm;

/// Habitat uses [libsodium](https://github.com/jedisct1/libsodium) and it's Rust
/// counterpart [sodiumoxide](https://github.com/dnaq/sodiumoxide) for
/// cryptographic operations.
///
/// ### Concepts and terminology:
/// - All public keys/certificates/signatures will be referred to as **public**.
/// - All secret or private keys will be referred to as **secret**.
/// - The word `key` by itself does not indicate **public** or **secret**. The only
/// exception is if the word key appears as part of a file suffix, where it is then
/// considered the **secret key** file.
/// - **Origin** -  refers to build-time operations, including signing and
/// verifification of an artifact.
/// - **Organization** / **Org** - refers to run-time operations that can happen in Habitat,
/// such as deploying a package signed in a different origin into your own organization.
/// Abbreviated as "org" in CLI params and variable names.
/// - **Org vs Origin** - Habitat packages come from an origin and run in an organization
/// - **Signing keys** - aka **sig** keys. These are used to sign and verify
/// packages. Contains a `sig.key` file suffix. Sig keys are NOT compatible with
/// box keys.
/// - **Box keys** - used for encryption/decryption of arbitrary data. Contains a
/// `.box.key` file suffix. Box keys are NOT compatible with sig keys.
/// - **Key revisions** - Habitat can use several keys for any given user, service,
/// or origin via different revision numbers. Revision numbers appear following the
/// key name and are in the format
/// `{year}{month}{day}{hour24}{minute}{second}`. For all user-facing cryptographic
/// operations (sign/verify/encrypt/decrypt), the latest key is tried first, and
/// upon failure, Habitat will try keys in reverse chronological order until
/// success or there are no more keys. ***TODO: key revisions are generated as part
/// of a filename, but only the most recent key is used during crypto operations.***
///
///
/// Example origin key file names ("sig" keys):
///
/// ```text
/// habitat-201603312016.pub
/// habitat-201603312016.sig.key
/// your_company-201604021516.pub
/// your_company-201604021516.sig.key
/// ```
///
/// Example user keys ("box" keys)
///
/// ```text
/// dave@habitat-201603312016.pub
/// some_user@habitat-201603312016.pub
/// ```
///
/// Example Service keys:
///
/// ```text
/// redis.default@habitat-201603312016.pub
/// ```
///
/// ### Habitat signed artifact format
///
/// A signed `.hab` artifact has 3 plaintext lines followed by a binary blob
/// of data, which is the unsigned tarfile.
///
/// - The first plaintext line is the name of the origin signing key that was used
/// to sign this artifact.
/// - The second plaintext line is the hashing algorithm used, which will be
/// `BLAKE2b` unless our use of crypto is expanded some time in the future.
/// - The third plaintext line is a base64 *signed* value of the binary blob's
/// base64 file hash. Signing uses a secret origin key, while verifying uses the
/// public origin key. Thus, it it safe to distribute public origin keys.
///
/// Example header:
/// ```text
/// habitat-20160405144945
/// BLAKE2b
/// signed BLAKE2b signature
/// <binary-blob>
/// ```
///
/// https://download.libsodium.org/doc/hashing/generic_hashing.html
///
/// It's possible to examine the contents of a `.hab` file from a Linux shell:
///
/// ```text
/// $ head -3 /path/to/chef-glibc-2.22-20160310192356.hab
/// habitat-20160405144945
/// BLAKE2b
/// w4yC7/QADdC+NfH/wgN5u4K94nMieb1TxTVzbSfpMwRQ4k+YwhLs1nDXSIbSC8jHdF/7/LqLWtgPvGDmoKIvBDI0aGpIcGdlNDJhMDBnQ3lsMVVFM0JvRlZGSHhXcnBuWWF0/// SllXTXo1ZDg9
/// # Note that this is an example signature only
/// ```
///
/// It is also possible to extract a plain tarball from a signed `.hab` artifact using the following command:
///
/// ```text
/// tail -n +4 /tmp/somefile.hab > somefile.tar
/// # start at line 4, skipping the first 3 plaintext lines.
/// ```

/// The suffix on the end of a public sig/box file
static PUB_KEY_SUFFIX: &'static str = "pub";

/// The suffix on the end of a public sig file
static SECRET_SIG_KEY_SUFFIX: &'static str = "sig.key";

/// The suffix on the end of a secret box file
static SECRET_BOX_KEY_SUFFIX: &'static str = "box.key";

/// The hashing function we're using during sign/verify
/// See also: https://download.libsodium.org/doc/hashing/generic_hashing.html
static SIG_HASH_TYPE: &'static str = "BLAKE2b";

/// This environment variable allows you to override the fs::CACHE_KEY_PATH
/// at runtime. This is useful for testing.
static CACHE_KEY_PATH_ENV_VAR: &'static str = "HAB_CACHE_KEY_PATH";

/// Create secret key files with these permissions
static PUBLIC_KEY_PERMISSIONS: &'static str = "0400";
static SECRET_KEY_PERMISSIONS: &'static str = "0400";

const BUF_SIZE: usize = 1024;

/// You can ask for both keys at once
pub struct SigKeyPair {
    /// The name of the key, ex: "habitat"
    pub name: String,
    /// The name with revision of the key, ex: "habitat-201604051449"
    pub rev: String,
    /// The sodiumoxide public key
    pub public: Option<SigPublicKey>,
    /// The sodiumocide private key
    pub secret: Option<SigSecretKey>,
}

impl SigKeyPair {
    /// make it easy for your friends and family to make new key pairs
    pub fn new(name: String,
               rev: String,
               p: Option<SigPublicKey>,
               s: Option<SigSecretKey>)
               -> SigKeyPair {
        SigKeyPair {
            name: name,
            rev: rev,
            public: p,
            secret: s,
        }
    }
}

/// If an env var is set, then return it's value.
/// If it's not, return the default
fn env_var_or_default(env_var: &str, default: &str) -> String {
    let value = match env::var(env_var) {
        Ok(val) => String::from(val),
        Err(_) => String::from(default),
    };
    value
}

/// Return the canonical location for nacl keys
/// This value can be overridden via CACHE_KEY_PATH_ENV_VAR,
/// which is useful for testing
fn nacl_key_dir() -> String {
    env_var_or_default(CACHE_KEY_PATH_ENV_VAR, CACHE_KEY_PATH)
}

/// Calculate the BLAKE2b hash of a file
/// NOTE: the key is empty
pub fn hash_file<P: AsRef<Path>>(filename: &P) -> Result<String> {
    let key = [0u8; libsodium_sys::crypto_generichash_KEYBYTES];
    let mut file = try!(File::open(filename.as_ref()));
    let mut out = [0u8; libsodium_sys::crypto_generichash_BYTES];
    let mut st = vec![0u8; (unsafe { libsodium_sys::crypto_generichash_statebytes() })];
    let pst = unsafe {
        mem::transmute::<*mut u8, *mut libsodium_sys::crypto_generichash_state>(st.as_mut_ptr())
    };

    unsafe {
        libsodium_sys::crypto_generichash_init(pst, key.as_ptr(), key.len(), out.len());
    }

    let mut buf = [0u8; BUF_SIZE];
    loop {
        let bytes_read = try!(file.read(&mut buf));
        if bytes_read == 0 {
            break;
        }
        let chunk = &buf[0..bytes_read];
        unsafe {
            libsodium_sys::crypto_generichash_update(pst, chunk.as_ptr(), chunk.len() as u64);
        }
    }
    unsafe {
        libsodium_sys::crypto_generichash_final(pst, out.as_mut_ptr(), out.len());
    }
    Ok(out.to_base64(STANDARD))
}


pub fn hash_reader(reader: &mut BufReader<File>) -> Result<String> {
    let key = [0u8; libsodium_sys::crypto_generichash_KEYBYTES];
    let mut out = [0u8; libsodium_sys::crypto_generichash_BYTES];
    let mut st = vec![0u8; (unsafe { libsodium_sys::crypto_generichash_statebytes() })];
    let pst = unsafe {
        mem::transmute::<*mut u8, *mut libsodium_sys::crypto_generichash_state>(st.as_mut_ptr())
    };

    unsafe {
        libsodium_sys::crypto_generichash_init(pst, key.as_ptr(), key.len(), out.len());
    }

    let mut buf = [0u8; BUF_SIZE];
    loop {
        let bytes_read = try!(reader.read(&mut buf));
        if bytes_read == 0 {
            break;
        }
        let chunk = &buf[0..bytes_read];
        unsafe {
            libsodium_sys::crypto_generichash_update(pst, chunk.as_ptr(), chunk.len() as u64);
        }
    }
    unsafe {
        libsodium_sys::crypto_generichash_final(pst, out.as_mut_ptr(), out.len());
    }
    Ok(out.to_base64(STANDARD))

}

/// Generate and sign a package
pub fn artifact_sign(infilename: &str,
                     outfilename: &str,
                     key_with_rev: &str,
                     sk: &SigSecretKey)
                     -> Result<()> {
    nacl_init();

    let hash = try!(hash_file(&infilename));
    debug!("File hash = {}", hash);

    let signature = sign::sign(&hash.as_bytes(), &sk);
    let output_file = try!(File::create(outfilename));
    let mut writer = BufWriter::new(&output_file);
    let _result = write!(writer,
                         "{}\n{}\n{}\n",
                         key_with_rev,
                         SIG_HASH_TYPE,
                         signature.to_base64(STANDARD));
    let mut file = try!(File::open(infilename));
    let mut buf = [0u8; BUF_SIZE];

    loop {
        let bytes_read = try!(file.read(&mut buf));
        if bytes_read == 0 {
            break;
        }
        let _result = writer.write(&buf[0..bytes_read]);
    }
    println!("Successfully created signed binary artifact {}",
             outfilename);
    Ok(())
}

pub fn get_artifact_reader(infilename: &str) -> Result<BufReader<File>> {
    let f = try!(File::open(infilename));
    let mut your_key_name = String::new();
    let mut your_hash_type = String::new();
    let mut your_signature_raw = String::new();

    let mut reader = BufReader::new(f);
    let _result = reader.read_line(&mut your_key_name);
    let _result = reader.read_line(&mut your_hash_type);
    let _result = reader.read_line(&mut your_signature_raw);
    Ok(reader)
}

pub fn artifact_verify(infilename: &str) -> Result<()> {
    nacl_init();

    let f = try!(File::open(infilename));

    let mut your_key_name = String::new();
    let mut your_hash_type = String::new();
    let mut your_signature_raw = String::new();
    let mut reader = BufReader::new(f);
    let _result = reader.read_line(&mut your_key_name);
    let _result = reader.read_line(&mut your_hash_type);
    let _result = reader.read_line(&mut your_signature_raw);

    // all input lines WILL have a newline at the end
    let your_key_name = your_key_name.trim();
    let your_hash_type = your_hash_type.trim();
    let your_signature_raw = your_signature_raw.trim();

    debug!("Your key name = [{}]", your_key_name);
    debug!("Your hash type = [{}]", your_hash_type);
    debug!("Your signature = [{}]", your_signature_raw);

    let your_sig_pk = match get_sig_public_key(&your_key_name) {
        Ok(pk) => pk,
        Err(_) => {
            let msg = format!("Cannot find origin key {} to verify artifact",
                              &your_key_name);
            return Err(Error::CryptoError(msg));
        }
    };

    if your_hash_type.trim() != SIG_HASH_TYPE {
        return Err(Error::CryptoError("Unsupported signature type detected".to_string()));
    }

    let your_signature = match your_signature_raw.as_bytes().from_base64() {
        Ok(sig) => sig,
        Err(e) => {
            let msg = format!("Error converting signature to base64 {}", e);
            return Err(Error::CryptoError(msg));
        }
    };

    let signed_data = match sign::verify(&your_signature, &your_sig_pk) {
        Ok(signed_data) => signed_data,
        Err(_) => return Err(Error::CryptoError("Verification failed".to_string())),
    };

    debug!("VERIFIED, checking signed hash against mine...");

    let your_hash = match String::from_utf8(signed_data) {
        Ok(your_hash) => your_hash,
        Err(_) => return Err(Error::CryptoError("Error parsing artifact signature".to_string())),
    };

    let my_hash = try!(hash_reader(&mut reader));

    debug!("My hash {}", my_hash);
    debug!("Your hash {}", your_hash);
    if my_hash == your_hash {
        Ok(())
    } else {
        Err(Error::CryptoError("Habitat package is invalid".to_string()))
    }
}

/// *********************************************
/// Key generation functions
/// *******************************************

pub fn generate_origin_sig_key(origin: &str) -> Result<String> {
    let revision = mk_revision_string();
    let keyname = mk_origin_sig_key_name(origin, &revision);
    debug!("new origin sig key name = {}", &keyname);
    try!(generate_sig_keypair_files(&keyname));
    Ok(keyname)
}

/// generate a service box key, return the name of the key we generated
pub fn generate_service_box_key(org: &str, service_group: &str) -> Result<String> {
    let revision = mk_revision_string();
    let keyname = mk_service_box_key_name(org, &revision, service_group);
    debug!("new user sig key name = {}", &keyname);
    try!(generate_box_keypair_files(&keyname));
    Ok(keyname)
}

/// generate a user box key, return the name of the key we generated
pub fn generate_user_box_key(org: &str, user: &str) -> Result<String> {
    let revision = mk_revision_string();
    let keyname = mk_user_box_key_name(org, &revision, &user);
    debug!("new user sig key name = {}", &keyname);
    try!(generate_box_keypair_files(&keyname));
    Ok(keyname)
}

fn mk_key_filename(dir: &str, keyname: &str, suffix: &str) -> String {
    format!("{}/{}.{}", dir, keyname, suffix)
}

/// generates a revision string in the form:
/// `{year}{month}{day}{hour24}{minute}{second}`
/// Timestamps are in UTC time.
fn mk_revision_string() -> String {
    let now = time::now_utc();
    // https://github.com/rust-lang-deprecated/time/blob/master/src/display.rs
    // http://man7.org/linux/man-pages/man3/strftime.3.html
    match now.strftime("%Y%m%d%H%M%S") {
        Ok(result) => format!("{}", result),
        Err(_) => panic!("can't parse system time"),
    }
}

fn mk_origin_sig_key_name(origin: &str, revision: &str) -> String {
    format!("{}-{}", origin, revision)
}

fn mk_service_box_key_name(org: &str, revision: &str, service_group: &str) -> String {
    format!("{}@{}-{}", service_group, org, revision)
}

fn mk_user_box_key_name(org: &str, revision: &str, user: &str) -> String {
    format!("{}@{}-{}", user, org, revision)
}

fn generate_box_keypair_files(keyname: &str) -> Result<(BoxPublicKey, BoxSecretKey)> {
    let (pk, sk) = box_::gen_keypair();

    let public_keyfile = mk_key_filename(&nacl_key_dir(), keyname, PUB_KEY_SUFFIX);
    let secret_keyfile = mk_key_filename(&nacl_key_dir(), keyname, SECRET_BOX_KEY_SUFFIX);
    debug!("public box keyfile = {}", &public_keyfile);
    debug!("secret box keyfile = {}", &secret_keyfile);
    try!(write_keypair_files(&public_keyfile,
                             &pk[..].to_base64(STANDARD).into_bytes(),
                             &secret_keyfile,
                             &sk[..].to_base64(STANDARD).into_bytes()));
    Ok((pk, sk))
}

fn generate_sig_keypair_files(keyname: &str) -> Result<(SigPublicKey, SigSecretKey)> {
    let (pk, sk) = sign::gen_keypair();

    let public_keyfile = mk_key_filename(&nacl_key_dir(), keyname, PUB_KEY_SUFFIX);
    let secret_keyfile = mk_key_filename(&nacl_key_dir(), keyname, SECRET_SIG_KEY_SUFFIX);
    debug!("public sig keyfile = {}", &public_keyfile);
    debug!("secret sig keyfile = {}", &secret_keyfile);

    try!(write_keypair_files(&public_keyfile,
                             &pk[..].to_base64(STANDARD).into_bytes(),
                             &secret_keyfile,
                             &sk[..].to_base64(STANDARD).into_bytes()));
    Ok((pk, sk))
}

fn write_keypair_files<K1: AsRef<Path>, K2: AsRef<Path>>(public_keyfile: K1,
                                                         public_content: &Vec<u8>,
                                                         secret_keyfile: K2,
                                                         secret_content: &Vec<u8>)
                                                         -> Result<()> {
    if let Some(pk_dir) = public_keyfile.as_ref().parent() {
        try!(fs::create_dir_all(pk_dir));
    } else {
        return Err(Error::BadKeyPath(public_keyfile.as_ref().to_string_lossy().into_owned()));
    }

    if let Some(sk_dir) = secret_keyfile.as_ref().parent() {
        try!(fs::create_dir_all(sk_dir));
    } else {
        return Err(Error::BadKeyPath(secret_keyfile.as_ref().to_string_lossy().into_owned()));
    }

    if public_keyfile.as_ref().exists() && public_keyfile.as_ref().is_file() {
        return Err(Error::CryptoError(format!("Public keyfile already exists {}",
                                              public_keyfile.as_ref().display())));
    }

    if secret_keyfile.as_ref().exists() && secret_keyfile.as_ref().is_file() {
        return Err(Error::CryptoError(format!("Secret keyfile already exists {}",
                                              secret_keyfile.as_ref().display())));
    }

    let public_file = try!(File::create(public_keyfile.as_ref()));
    let mut public_writer = BufWriter::new(&public_file);
    let _result = try!(public_writer.write_all(public_content));
    try!(perm::set_permissions(public_keyfile, PUBLIC_KEY_PERMISSIONS));

    let secret_file = try!(File::create(secret_keyfile.as_ref()));
    let mut secret_writer = BufWriter::new(&secret_file);
    let _result = try!(secret_writer.write_all(secret_content));
    try!(perm::set_permissions(secret_keyfile, SECRET_KEY_PERMISSIONS));
    Ok(())
}

/// *********************************************
/// Key reading functions
/// *******************************************

/// Return a Vec of origin keys with a given name.
/// The newest key is listed first in the Vec
/// Origin keys are always "sig" keys. They are used for signing/verifying
/// packages, not for encryption.
pub fn read_sig_origin_keys(origin_keyname: &str) -> Result<Vec<SigKeyPair>> {
    let revisions = try!(get_key_revisions(origin_keyname));
    let mut key_pairs = Vec::new();
    for rev in &revisions {
        debug!("Attempting to read key rev {} for {}", rev, origin_keyname);
        let pk = match get_sig_public_key(rev) {
            Ok(k) => Some(k),
            Err(e) => {
                // Not an error, just continue
                debug!("Can't find public key for rev {}: {}", rev, e);
                None
            }
        };
        let sk = match get_sig_secret_key(rev) {
            Ok(k) => Some(k),
            Err(e) => {
                // Not an error, just continue
                debug!("Can't find secret key for rev {}: {}", rev, e);
                None
            }
        };
        let kp = SigKeyPair::new(origin_keyname.to_string(), rev.clone(), pk, sk);
        key_pairs.push(kp);
    }
    Ok(key_pairs)
}

pub fn get_sig_secret_key(keyname: &str) -> Result<SigSecretKey> {
    let bytes = try!(get_sig_secret_key_bytes(keyname));
    match SigSecretKey::from_slice(&bytes) {
        Some(sk) => Ok(sk),
        None => {
            return Err(Error::CryptoError(format!("Can't read sig secret key for {}", keyname)))
        }
    }
}

pub fn get_sig_public_key(keyname: &str) -> Result<SigPublicKey> {
    let bytes = try!(get_sig_public_key_bytes(keyname));
    match SigPublicKey::from_slice(&bytes) {
        Some(sk) => Ok(sk),
        None => {
            return Err(Error::CryptoError(format!("Can't read sig public key for {}", keyname)))
        }
    }
}

pub fn get_box_secret_key(keyname: &str) -> Result<BoxSecretKey> {
    let bytes = try!(get_box_secret_key_bytes(keyname));
    match BoxSecretKey::from_slice(&bytes) {
        Some(sk) => Ok(sk),
        None => {
            return Err(Error::CryptoError(format!("Can't read box secret key for {}", keyname)))
        }
    }
}

pub fn get_box_public_key(keyname: &str) -> Result<BoxPublicKey> {
    let bytes = try!(get_box_public_key_bytes(keyname));
    match BoxPublicKey::from_slice(&bytes) {
        Some(sk) => Ok(sk),
        None => {
            return Err(Error::CryptoError(format!("Can't read box public key for {}", keyname)))
        }
    }
}

fn get_box_public_key_bytes(keyname: &str) -> Result<Vec<u8>> {
    let public_keyfile = mk_key_filename(&nacl_key_dir(), keyname, PUB_KEY_SUFFIX);
    read_key_bytes(&public_keyfile)
}

fn get_box_secret_key_bytes(keyname: &str) -> Result<Vec<u8>> {
    let secret_keyfile = mk_key_filename(&nacl_key_dir(), keyname, SECRET_BOX_KEY_SUFFIX);
    read_key_bytes(&secret_keyfile)
}

fn get_sig_public_key_bytes(keyname: &str) -> Result<Vec<u8>> {
    let public_keyfile = mk_key_filename(&nacl_key_dir(), keyname, PUB_KEY_SUFFIX);
    read_key_bytes(&public_keyfile)
}

fn get_sig_secret_key_bytes(keyname: &str) -> Result<Vec<u8>> {
    let secret_keyfile = mk_key_filename(&nacl_key_dir(), keyname, SECRET_SIG_KEY_SUFFIX);
    read_key_bytes(&secret_keyfile)
}

/// Read a file into a Vec<u8>
fn read_key_bytes(keyfile: &str) -> Result<Vec<u8>> {
    let mut f = try!(File::open(keyfile));
    let mut s = String::new();
    let _numread = try!(f.read_to_string(&mut s));
    match s.as_bytes().from_base64() {
        Ok(keybytes) => Ok(keybytes),
        Err(e) => {
            return Err(Error::CryptoError(format!("Can't read raw key from {}: {}", keyfile, e)))
        }
    }
}

/// Take a key name (ex "habitat"), and find all revisions of that
/// keyname in the nacl_key_dir().
pub fn get_key_revisions(keyname: &str) -> Result<Vec<String>> {
    // look for .pub keys
    let dir = nacl_key_dir();

    // accumulator for files that match
    let mut candidates = Vec::new();

    let paths = match fs::read_dir(&dir) {
        Ok(p) => p,
        Err(e) => {
            return Err(Error::CryptoError(format!("Error reading key directory {}: {}", dir, e)))
        }
    };
    for path in paths {
        match path {
            Ok(ref p) => p,
            Err(e) => {
                debug!("Error reading path {}", e);
                return Err(Error::CryptoError(format!("Error reading key path {}", e)));
            }
        };

        let p: fs::DirEntry = path.unwrap();

        match p.metadata() {
            Ok(md) => {
                if !md.is_file() {
                    continue;
                }
            }
            Err(e) => {
                debug!("Error checking file metadata {}", e);
                continue;
            }
        };
        let filename = match p.file_name().into_string() {
            Ok(f) => f,
            Err(e) => {
                // filename is still an OsString, so print it as debug output
                debug!("Invalid filename {:?}", e);
                return Err(Error::CryptoError(format!("Invalid filename in key path")));
            }
        };

        if filename.ends_with(PUB_KEY_SUFFIX) {
            if filename.starts_with(keyname) {
                // push filename without extension
                // -1 for the '.' before 'pub'
                let (stem, _) = filename.split_at(filename.chars().count() -
                                                  PUB_KEY_SUFFIX.chars().count() -
                                                  1);
                candidates.push(stem.to_string());
            }
        }
    }
    candidates.sort();
    // newest key first
    candidates.reverse();
    Ok(candidates)
}
