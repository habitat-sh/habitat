// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

//! Habitat core encryption and cryptography.
//!
//! This module uses [libsodium](https://github.com/jedisct1/libsodium) and its Rust counterpart
//! [sodiumoxide](https://github.com/dnaq/sodiumoxide) for cryptographic operations.
//!
//! # Concepts and terminology:
//!
//! - All public keys, certificates, and signatures are to be referred to as **public**.
//! - All secret or private keys are to be referred to as **secret**.
//! - All symmetric encryption keys are to be referred to as **secret**.
//! - In general, the word `key` by itself does not indicate something as
//! **public** or **secret**. The exceptions to this rule are as follows:
//!     - if the word key appears in a URL, then we are referring to a public key to
//!       conform to other API's that offer similar public key downloading functionality.
//!     - the word `key` appears as part of a file suffix, where it is then considered as
//!       a **secret key** file.
//! - Referring to keys (by example):
//!     - A key name: `habitat`
//!     - A key rev: `201603312016`
//!     - A key name with rev: `habitat-201603312016`
//!     - A key file: `habitat-201603312016.pub`
//!     - A key path or fully qualified key path: `/foo/bar/habitat-201603312016.pub`
//! - An **Origin** refers to build-time operations, including signing and verifification of a
//! Habitat artifact.
//! - An **Organization** or **Org** refers to run-time operations such as deploying a package
//! signed in a different origin into your own organization. This is abbreviated as "org" in
//! user-facing command line parameters and internal variable names.
//! - To distinguish between **Org** and **Origin**, the following might help: Habitat packages
//! come from an **Origin** and run in an **Organization**.
//! - A **Ring** is the full set of Supervisors that communicate with each other.
//! - A **Signing key**, also known as a **sig** key, is used to sign and verify Habitat artifacts.
//! The file contains a `sig.key` file suffix. Note that sig keys are not compatible with box keys.
//! - A **Box key** is used for encryption and decryption of arbitrary data. The file contains a
//! `.box.key` file suffix. Note that box keys are not compatible with sig keys.
//! - A **Sym key** is used for symmetric encryption, meaning that a shared secret is used to
//! encrypt a message into a ciphertext and that same secret is used later to decryt the ciphertext
//! into the original message.
//! - A **Ring key** is a **sym** key used when sending messages between the Supervisors to prevent
//! a third party from intercepting the traffic.
//! - **Key revisions** - There can exist several keys for any given user, service, ring, or origin
//! via different revision numbers. Revision numbers appear following the key name and are in the
//! format `{year}{month}{day}{hour24}{minute}{second}`. For all user-facing cryptographic
//! operations (such as sign, verify, encrypt, decrypt, etc.), the latest key is tried first, and
//! upon failure, the keys will be tred in reverse chronological order until success or there are
//! no more keys.
//!
//! ***TODO: key revisions are generated as part of a filename, but only the most recent key is
//! used during crypto operations.***
//!
//! # Key file naming
//!
//! ## Origin key
//!
//! ```text
//! <origin_name>-<revision>.pub
//! <origin_name>-<revision>.sig.key
//! ```
//!
//! Example origin key file names ("sig" keys):
//!
//! ```text
//! habitat-201603312016.pub
//! habitat-201603312016.sig.key
//! your_company-201604021516.pub
//! your_company-201604021516.sig.key
//! ```
//!
//! ## User key
//!
//! ```text
//! <user_name>-<revision>.pub
//! <user_name>-<revision>.box.key
//! ```
//!
//! Example user keys ("box" keys)
//!
//! ```text
//! dave-201603312016.pub
//! some_user-201603312016.pub
//! ```
//!
//! ## Service key
//!
//! ```text
//! <service_name>.<group>@<organization>-<revision>.pub
//! <service_name>.<group>@<organization>-<revision>.box.key
//! ```
//!
//! Example Service keys:
//!
//! ```text
//! redis.default@habitat-201603312016.pub
//! ```
//!
//! ## Ring key
//!
//! ```text
//! <ring_name>-<revision>.sym.key
//! ```
//!
//! Example Ring keys:
//!
//! ```text
//! staging-201603312016.sym.key
//! ```
//!
//! # File fomats
//!
//! ## Habitat artifacts
//!
//! A signed Habitat artifact (a file with the extention `.hart`) has 5 plaintext lines followed by
//! a binary blob of data, which is an unsigned, compressed tarfile. The lines are as follows:
//!
//! 1. The artifact format version
//! 1. The name with revision of the origin key which was used to sign the artifact
//! 1. The hashing algorithm used, which at present is only `BLAKE2b`, but may expand in the future
//! 1. A Base64 *signed* value of the binary blob's Base64 file hash
//! 1. The last line is left empty, meaning that 2 newline characters (`\n`) separate the header
//!    from the payload
//!
//! The remainder of the file is a compressed tarball of the contents to be extracted on disk. At
//! present, the tarball is compressed using `xz` but is considered an implementation detail. Also
//! note unlike the format of keys, the compressed tarball is **not** Base64 encoded--it is the
//! compressed tarball itself.
//!
//! Note that the BLAKE2b hash functions use a digest length of 32 bytes (256 bits!). More details
//! about the hashing strategy can be found in the [libsodium hashing
//! documentation](https://download.libsodium.org/doc/hashing/generic_hashing.html).
//!
//! Signing uses a secret origin key, while verifying uses the public origin key. Thus, it it safe
//! to distribute public origin keys.
//!
//! Example header:
//!
//! ```text
//! HART-1
//! habitat-20160405144945
//! BLAKE2b
//! signed BLAKE2b signature
//!
//! <binary-blob>
//! ```
//!
//! Due to the simple, line-driven structure of the header it's possible to examine the contents of
//! a Habitat artifact using standard Unix tooling:
//!
//! ```text
//! $ head -4 /path/to/acme-glibc-2.22-20160310192356-x86_64-linux.hart
//! HART-1
//! habitat-20160405144945
//! BLAKE2b
//! abc123...
//! ```
//!
//! Note that the `abc123` would be a Base64 string in a real file.
//!
//! It's also possible to extract a plain compressed tarball from a signed Habitat artifact using
//! the `tail(1)` Unix command:
//!
//! ```text
//! tail -n +6 /tmp/somefile.hart > somefile.tar.xz
//! ```
//!
//! The above command starts streaming the file to standard out at line 6, skipping the first 5
//! plaintext lines.
//!
//! If the Habitat artifact needs to be extracted on disk without verifying its integrity or
//! authenticity, this can be accomplised with:
//!
//! ```text
//! tail -n +6 /tmp/somefile.hart | xzcat | tar x -C /
//! ```
//!
//! **Caution!** Working with Habitat artifacts in this manner this is not normally recommended and
//! is **not** a supported workflow for working with Habitat artifacts--they are signed for very
//! important reasons.
//!
//! ## Encrypted payloads
//!
//! The first 4 lines of an encrypted payload are as follows:
//!
//! 1. The encrypted format version
//! 1. The key name, including revision of the source user
//! 1. The key name, including revision of the recipient service
//! 1. A nonce, in Bas64 format.
//! 1. The encrypted message in Bas64 format.
//!
//! ```text
//! BOX-1
//! signing key name
//! recipient key name
//! nonce_base64
//!
//! <ciphertext_base64>
//! ```
//!
//! ## Ring keys
//!
//! There are 3 lines, that is 3 parts that are separtated by a newline character `\n`. They are as
//! follows:
//!
//! 1. Encrypted format version
//! 1. The ring key name, including revision
//! 1. The key itself, which is Bas64-encoded
//!
//! ```text
//! SYM-1
//! staging-20160405144945
//!
//! <symkey_base64>
//! ```

use std::ptr;
use std::collections::HashSet;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io;
use std::io::{BufReader, BufWriter};
use std::mem;
use std::path::{Path, PathBuf};

use libsodium_sys;
use rustc_serialize::base64::{STANDARD, ToBase64, FromBase64};
use rustc_serialize::hex::ToHex;
use sodiumoxide::init as nacl_init;
use sodiumoxide::crypto::sign;
use sodiumoxide::crypto::box_;
use sodiumoxide::crypto::secretbox;
use sodiumoxide::crypto::secretbox::Key as SymSecretKey;
use sodiumoxide::crypto::sign::ed25519::SecretKey as SigSecretKey;
use sodiumoxide::crypto::sign::ed25519::PublicKey as SigPublicKey;
use sodiumoxide::crypto::box_::curve25519xsalsa20poly1305::PublicKey as BoxPublicKey;
use sodiumoxide::crypto::box_::curve25519xsalsa20poly1305::SecretKey as BoxSecretKey;
use sodiumoxide::crypto::box_::curve25519xsalsa20poly1305::{Nonce, gen_nonce};
use sodiumoxide::randombytes::randombytes;
use time;

use env as henv;
use error::{Error, Result};
use fs::CACHE_KEY_PATH;
use util::perm;

/// The suffix on the end of a public sig/box file
static PUBLIC_KEY_SUFFIX: &'static str = "pub";

/// The suffix on the end of a public sig file
static SECRET_SIG_KEY_SUFFIX: &'static str = "sig.key";

/// The suffix on the end of a secret box file
static SECRET_BOX_KEY_SUFFIX: &'static str = "box.key";

/// The suffix on the end of a secret symmetric key file
static SECRET_SYM_KEY_SUFFIX: &'static str = "sym.key";

/// The hashing function we're using during sign/verify
/// See also: https://download.libsodium.org/doc/hashing/generic_hashing.html
static SIG_HASH_TYPE: &'static str = "BLAKE2b";

/// This environment variable allows you to override the fs::CACHE_KEY_PATH
/// at runtime. This is useful for testing.
static CACHE_KEY_PATH_ENV_VAR: &'static str = "HAB_CACHE_KEY_PATH";

/// Create secret key files with these permissions
static PUBLIC_KEY_PERMISSIONS: &'static str = "0400";
static SECRET_KEY_PERMISSIONS: &'static str = "0400";

static HART_FORMAT_VERSION: &'static str = "HART-1";
static BOX_FORMAT_VERSION: &'static str = "BOX-1";

static PUBLIC_SIG_KEY_VERSION: &'static str = "SIG-PUB-1";
static SECRET_SIG_KEY_VERSION: &'static str = "SIG-SEC-1";
static PUBLIC_BOX_KEY_VERSION: &'static str = "BOX-PUB-1";
static SECRET_BOX_KEY_VERSION: &'static str = "BOX-SEC-1";
static SECRET_SYM_KEY_VERSION: &'static str = "SYM-SEC-1";

const BUF_SIZE: usize = 1024;

/// A pair key related key (public and secret) which have a name and revision.
///
/// Depending on the type of keypair, the public key may be empty or not apply, or one or both of
/// the keys may not be present due to the loading context. For example, the act of verifying a
/// signed message or artifact only requires the public key to be present, whereas the act of
/// signing will require the secret key to be present.
#[derive(Clone)]
pub struct KeyPair<P, S> {
    /// The name of the key, ex: "habitat"
    pub name: String,
    /// The name with revision of the key, ex: "habitat-201604051449"
    pub name_with_rev: String,
    /// The sodiumoxide public key
    pub public: Option<P>,
    /// The sodiumocide private key
    pub secret: Option<S>,
}

impl<P, S> KeyPair<P, S> {
    /// make it easy for your friends and family to make new key pairs
    pub fn new(name: String, name_with_rev: String, p: Option<P>, s: Option<S>) -> KeyPair<P, S> {
        KeyPair {
            name: name,
            name_with_rev: name_with_rev,
            public: p,
            secret: s,
        }
    }
}

pub type SigKeyPair = KeyPair<SigPublicKey, SigSecretKey>;
pub type BoxKeyPair = KeyPair<BoxPublicKey, BoxSecretKey>;
pub type SymKey = KeyPair<(), SymSecretKey>;

#[derive(PartialEq, Eq)]
pub enum KeyType {
    Sig,
    Box,
    Sym,
}

/// If an env var is set, then return it's value.
/// If it's not, return the default
fn env_var_or_default(env_var: &str, default: &str) -> String {
    let value = match henv::var(env_var) {
        Ok(val) => String::from(val),
        Err(_) => String::from(default),
    };
    value
}

/// Return the canonical location for nacl keys
/// This value can be overridden via CACHE_KEY_PATH_ENV_VAR,
/// which is useful for testing
pub fn nacl_key_dir() -> String {
    env_var_or_default(CACHE_KEY_PATH_ENV_VAR, CACHE_KEY_PATH)
}

/// takes a Path to a key, and returns the origin and revision in a tuple
/// ex: /src/foo/core-xyz-20160423193745.pub yields ("core", "20160423193745")
/// TODO DP: this should be in a crypto::utils package
pub fn parse_origin_key_filename<P: AsRef<Path>>(keyfile: P) -> Result<(String, String)> {
    let stem = match keyfile.as_ref().file_stem().and_then(|s| s.to_str()) {
        Some(s) => s,
        None => return Err(Error::CryptoError("Can't parse key filename".to_string()))
    };

    parse_origin_key_name(stem)
}

/// takes a string in the form origin-revision and returns a
/// tuple in the form (origin, revision)
pub fn parse_origin_key_name(origin_rev: &str) -> Result<(String, String)> {
    let mut chunks: Vec<&str> = origin_rev.split("-").collect();
    if chunks.len() < 2 {
        return Err(Error::CryptoError("Invalid origin key name".to_string()))
    }
    let rev = match chunks.pop() {
        Some(r) => r,
        None => return Err(Error::CryptoError("Invalid origin key revision".to_string()))
    };
    let origin = chunks.join("-").trim().to_owned();
    Ok((origin, rev.trim().to_owned()))
}

#[test]
fn test_parse_origin_key_name() {
    assert!(parse_origin_key_name("foo").is_ok() == false);
    match parse_origin_key_name("foo-20160423193745\n") {
        Ok((origin, rev)) => {
            assert!(origin == "foo");
            assert!(rev == "20160423193745");
        }
        Err(_) => panic!("Fail!")
    };


    match parse_origin_key_name("foo-bar-baz-20160423193745") {
        Ok((origin, rev)) => {
            assert!(origin == "foo-bar-baz");
            assert!(rev == "20160423193745");
        }
        Err(_) => panic!("Fail!")
    };
}

#[test]
fn test_parse_origin_key_filename() {
    if parse_origin_key_filename(Path::new("/tmp/foo.pub")).is_ok() {
        panic!("Shouldn't match")
    };


    match parse_origin_key_filename(Path::new("/tmp/core-20160423193745.pub")) {
        Ok((origin, rev)) => {
            assert!(origin == "core");
            assert!(rev == "20160423193745");
        }
        Err(_) => panic!("Bad filename")
    };

    match parse_origin_key_filename(Path::new("/tmp/multi-dash-origin-20160423193745.pub")) {
        Ok((origin, rev)) => {
            assert!(origin == "multi-dash-origin");
            assert!(rev == "20160423193745");
        }
        Err(_) => panic!("Bad filename")
    };
}


/// A Context makes crypto operations available centered on a given
/// key cache directory.
#[derive(Debug)]
pub struct Context {
    pub key_cache: String,
}

impl Default for Context {
    fn default() -> Context {
        nacl_init();
        Context { key_cache: nacl_key_dir() }
    }
}

impl Context {
    pub fn new(cache: &str) -> Context {
        nacl_init();
        Context { key_cache: cache.to_string() }
    }

    /// Calculate the BLAKE2b hash of a file, return as a hex string
    /// digest size = 32 BYTES
    /// NOTE: the hashing is keyless
    pub fn hash_file<P: AsRef<Path>>(&self, filename: &P) -> Result<String> {
        let file = try!(File::open(filename.as_ref()));
        let mut reader = BufReader::new(file);
        self.hash_reader(&mut reader)
    }

    pub fn hash_reader(&self, reader: &mut BufReader<File>) -> Result<String> {
        let mut out = [0u8; libsodium_sys::crypto_generichash_BYTES];
        let mut st = vec![0u8; (unsafe { libsodium_sys::crypto_generichash_statebytes() })];
        let pst = unsafe {
            mem::transmute::<*mut u8, *mut libsodium_sys::crypto_generichash_state>(st.as_mut_ptr())
        };

        unsafe {
            libsodium_sys::crypto_generichash_init(pst, ptr::null_mut(), 0, out.len());
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
        Ok(out.to_hex())
    }

    /// Generate and sign a package
    pub fn artifact_sign(&self,
                         infilename: &str,
                         outfilename: &str,
                         key_with_rev: &str,
                         sk: &SigSecretKey)
                         -> Result<()> {

        let hash = try!(self.hash_file(&infilename));
        debug!("File hash = {}", hash);

        let signature = sign::sign(&hash.as_bytes(), &sk);
        let output_file = try!(File::create(outfilename));
        let mut writer = BufWriter::new(&output_file);
        let () = try!(write!(writer,
                             "{}\n{}\n{}\n{}\n\n",
                             HART_FORMAT_VERSION,
                             key_with_rev,
                             SIG_HASH_TYPE,
                             signature.to_base64(STANDARD)));
        let mut file = try!(File::open(infilename));
        try!(io::copy(&mut file, &mut writer));
        Ok(())
    }

    /// return a BufReader to the .tar bytestream, skipping the signed header
    pub fn get_artifact_reader(&self, infilename: &str) -> Result<BufReader<File>> {
        let f = try!(File::open(infilename));
        let mut your_format_version = String::new();
        let mut your_key_name = String::new();
        let mut your_hash_type = String::new();
        let mut your_signature_raw = String::new();
        let mut empty_line = String::new();

        let mut reader = BufReader::new(f);
        if try!(reader.read_line(&mut your_format_version)) <= 0 {
            return Err(Error::CryptoError("Can't read format version".to_string()));
        }
        if try!(reader.read_line(&mut your_key_name)) <= 0 {
            return Err(Error::CryptoError("Can't read keyname".to_string()));
        }
        if try!(reader.read_line(&mut your_hash_type)) <= 0 {
            return Err(Error::CryptoError("Can't read hash type".to_string()));
        }
        if try!(reader.read_line(&mut your_signature_raw)) <= 0 {
            return Err(Error::CryptoError("Can't read signature".to_string()));
        }
        if try!(reader.read_line(&mut empty_line)) <= 0 {
            return Err(Error::CryptoError("Can't end of header".to_string()));
        }
        Ok(reader)
    }

    /// opens up a .hart file and returns the name of the (key, revision)
    /// of the signer
    pub fn get_artifact_signer<P: AsRef<Path>>(&self, infilename: P) -> Result<(String, String)> {
        let f = try!(File::open(infilename));
        let mut your_format_version = String::new();
        let mut your_key_name = String::new();
        let mut reader = BufReader::new(f);
        if try!(reader.read_line(&mut your_format_version)) <= 0 {
            return Err(Error::CryptoError("Can't read format version".to_string()));
        }
        if try!(reader.read_line(&mut your_key_name)) <= 0 {
            return Err(Error::CryptoError("Can't read keyname".to_string()));
        }
        let origin_rev_pair = try!(parse_origin_key_name(&your_key_name));
        Ok(origin_rev_pair)
    }

    /// verify the crypto signature of a .hart file
    pub fn artifact_verify(&self, infilename: &str) -> Result<()> {

        let f = try!(File::open(infilename));
        let mut your_format_version = String::new();
        let mut your_key_name = String::new();
        let mut your_hash_type = String::new();
        let mut your_signature_raw = String::new();
        let mut empty_line = String::new();

        let mut reader = BufReader::new(f);
        if try!(reader.read_line(&mut your_format_version)) <= 0 {
            return Err(Error::CryptoError("Corrupt payload, can't read format version"
                                              .to_string()));
        }
        if try!(reader.read_line(&mut your_key_name)) <= 0 {
            return Err(Error::CryptoError("Corrupt payload, can't read origin key name"
                                              .to_string()));
        }
        if try!(reader.read_line(&mut your_hash_type)) <= 0 {
            return Err(Error::CryptoError("Corrupt payload, can't read hash type".to_string()));
        }
        if try!(reader.read_line(&mut your_signature_raw)) <= 0 {
            return Err(Error::CryptoError("Corrupt payload, can't read signature".to_string()));
        }
        if try!(reader.read_line(&mut empty_line)) <= 0 {
            return Err(Error::CryptoError("Corrupt payload, can't end of header".to_string()));
        }

        // all input lines WILL have a newline at the end
        let your_format_version = your_format_version.trim();
        let your_key_name = your_key_name.trim();
        let your_hash_type = your_hash_type.trim();
        let your_signature_raw = your_signature_raw.trim();

        debug!("Your format version = [{}]", your_format_version);
        debug!("Your key name = [{}]", your_key_name);
        debug!("Your hash type = [{}]", your_hash_type);
        debug!("Your signature = [{}]", your_signature_raw);

        if your_format_version.trim() != HART_FORMAT_VERSION {
            let msg = format!("Unsupported format version: {}. Supported format versions: [{}]",
                              &your_format_version,
                              HART_FORMAT_VERSION);
            return Err(Error::CryptoError(msg));
        }

        let your_sig_pk = match self.get_sig_public_key(&your_key_name) {
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

        debug!("VERIFIED, checking signed hash against mine");

        let your_hash = match String::from_utf8(signed_data) {
            Ok(your_hash) => your_hash,
            Err(_) => {
                return Err(Error::CryptoError("Error parsing artifact signature".to_string()))
            }
        };

        let my_hash = try!(self.hash_reader(&mut reader));

        debug!("My hash {}", my_hash);
        debug!("Your hash {}", your_hash);
        if my_hash == your_hash {
            Ok(())
        } else {
            Err(Error::CryptoError("Habitat package is invalid".to_string()))
        }
    }

    /// A user can encrypt data with a service as the recipient.
    /// Key names and nonce are embedded in the payload.
    pub fn encrypt(&self,
                   data: &[u8],
                   service_key_name: &str,
                   service_pk: &BoxPublicKey,
                   user_key_name: &str,
                   user_sk: &BoxSecretKey)
                   -> Result<Vec<u8>> {
        let nonce = gen_nonce();
        let ciphertext = box_::seal(data, &nonce, service_pk, &user_sk);

        debug!("User key [{}]", user_key_name);
        debug!("Service key [{}]", service_key_name);
        debug!("Nonce [{}]", nonce[..].to_base64(STANDARD));
        let out = format!("{}\n{}\n{}\n{}\n{}",
                          BOX_FORMAT_VERSION,
                          user_key_name,
                          service_key_name,
                          nonce[..].to_base64(STANDARD),
                          &ciphertext.to_base64(STANDARD));
        Ok(out.into_bytes())
    }

    /// Decrypt data from a user that was received at a service
    /// Key names are embedded in the message payload which must
    /// be present while decrypting.
    pub fn decrypt(&self, payload: &Vec<u8>) -> Result<Vec<u8>> {
        debug!("Decrypt key path = {}", &self.key_cache);
        let mut p = payload.as_slice();
        let mut file_version = String::new();
        let mut user_key_name = String::new();
        let mut service_key_name = String::new();
        let mut raw_nonce = String::new();
        let mut raw_data = Vec::new();

        if try!(p.read_line(&mut file_version)) <= 0 {
            return Err(Error::CryptoError("Corrupt payload, can't read file version".to_string()));
        }
        if try!(p.read_line(&mut user_key_name)) <= 0 {
            return Err(Error::CryptoError("Corrupt payload, can't read user key name".to_string()));
        }
        if try!(p.read_line(&mut service_key_name)) <= 0 {
            return Err(Error::CryptoError("Corrupt payload, can't read service key name"
                                              .to_string()));
        }
        if try!(p.read_line(&mut raw_nonce)) <= 0 {
            return Err(Error::CryptoError("Corrupt payload, can't read nonce".to_string()));
        }
        if try!(p.read_to_end(&mut raw_data)) <= 0 {
            return Err(Error::CryptoError("Corrupt payload, can't read ciphertext".to_string()));
        }
        // all these values will have a newline at the end
        let file_version = file_version.trim();
        let user_key_name = user_key_name.trim();
        let service_key_name = service_key_name.trim();
        let raw_nonce = raw_nonce.trim();

        // only check after file_version has been trimmed
        if file_version != BOX_FORMAT_VERSION {
            return Err(Error::CryptoError("Invalid encrypted payload version ".to_string()));
        }

        debug!("Version [{}]", file_version);
        debug!("User key [{}]", user_key_name);
        debug!("Service key [{}]", service_key_name);
        debug!("Raw nonce [{}]", raw_nonce);

        let nonce_decoded = match raw_nonce.as_bytes().from_base64() {
            Ok(b64) => b64,
            Err(e) => return Err(Error::CryptoError(format!("Can't decode nonce: {}", e))),
        };

        let nonce = match Nonce::from_slice(&nonce_decoded) {
            Some(n) => n,
            None => return Err(Error::CryptoError("Invalid nonce length".to_string())),
        };

        let data_decoded = match raw_data.from_base64() {
            Ok(b64) => b64,
            Err(e) => return Err(Error::CryptoError(format!("Can't decode ciphertext: {}", e))),
        };

        // service secret key
        // user public key
        let user_pk = match self.get_box_public_key(&user_key_name) {
            Ok(pk) => pk,
            Err(_) => {
                let msg = format!("Cannot find user key {}", &user_key_name);
                return Err(Error::CryptoError(msg));
            }
        };

        let service_sk = match self.get_box_secret_key(&service_key_name) {
            Ok(sk) => sk,
            Err(_) => {
                let msg = format!("Cannot find service secret key {}", &service_key_name);
                return Err(Error::CryptoError(msg));
            }
        };
        let result = box_::open(&data_decoded, &nonce, &user_pk, &service_sk);
        match result {
            Ok(v) => Ok(v),
            // the Err result from open returns (), so return a "Can't decrypt"
            // message instead
            Err(_) => return Err(Error::CryptoError("Can't decrypt payload".to_string())),
        }
    }

    /// Encrypts a byte slice of data using a given `SymKey`.
    ///
    /// The return is a `Result` of a tuple of `Vec<u8>` structs, the first being the random nonce
    /// value and the second being the ciphertext.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// extern crate habitat_core;
    /// extern crate tempdir;
    ///
    /// use habitat_core::crypto::Context;
    /// use tempdir::TempDir;
    ///
    /// fn main() {
    ///     let key_cache = TempDir::new("key_cache").unwrap();
    ///     let ctx = Context { key_cache: key_cache.into_path().to_string_lossy().into_owned() };
    ///     let name = ctx.generate_ring_sym_key("beyonce").unwrap();
    ///     let keys = ctx.read_sym_keys(&name).unwrap();
    ///     let sym_key = keys.first().unwrap();
    ///
    ///     let (nonce, ciphertext) = ctx.sym_encrypt(&sym_key, "Guess who?".as_bytes()).unwrap();
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// * If the secret key component of the `SymKey` is not present
    pub fn sym_encrypt(&self, sym_key: &SymKey, data: &[u8]) -> Result<(Vec<u8>, Vec<u8>)> {
        let key = match sym_key.secret.as_ref() {
            Some(s) => s,
            None => return Err(Error::CryptoError("Secret key not present to encrypt".to_string())),
        };
        let nonce = secretbox::gen_nonce();
        Ok((nonce.as_ref().to_vec(), secretbox::seal(data, &nonce, &key)))
    }

    /// Decrypts a byte slice of ciphertext using a given nonce value and a `SymKey`.
    ///
    /// The return is a `Result` of a byte vector containing the original, unencrypted data.
    ///
    /// # Examples
    ///
    /// Basic usage
    ///
    /// ```
    /// extern crate habitat_core;
    /// extern crate tempdir;
    ///
    /// use habitat_core::crypto::Context;
    /// use tempdir::TempDir;
    ///
    /// fn main() {
    ///     let key_cache = TempDir::new("key_cache").unwrap();
    ///     let ctx = Context { key_cache: key_cache.into_path().to_string_lossy().into_owned() };
    ///     let name = ctx.generate_ring_sym_key("beyonce").unwrap();
    ///     let keys = ctx.read_sym_keys(&name).unwrap();
    ///     let sym_key = keys.first().unwrap();
    ///
    ///     let (nonce, ciphertext) = ctx.sym_encrypt(&sym_key, "Guess who?".as_bytes()).unwrap();
    ///     let message = ctx.sym_decrypt(&sym_key, &nonce, &ciphertext).unwrap();
    ///
    ///     // The original message is decrypted
    ///     assert_eq!(message, "Guess who?".to_string().into_bytes());
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// * If the secret key component of the `SymKey` is not present
    /// * If the size of the provided nonce data is not the required size
    /// * If the ciphertext was not decryptable given the nonce and symmetric key
    pub fn sym_decrypt(&self,
                       sym_key: &SymKey,
                       nonce: &[u8],
                       ciphertext: &[u8])
                       -> Result<Vec<u8>> {
        let key = match sym_key.secret.as_ref() {
            Some(s) => s,
            None => return Err(Error::CryptoError("Secret key not present to decrypt".to_string())),
        };
        let nonce = match secretbox::Nonce::from_slice(&nonce) {
            Some(n) => n,
            None => {
                return Err(Error::CryptoError("The length of the bytes isn't equal to \
                                                  the length of a nonce"
                                                  .to_string()))
            }
        };
        match secretbox::open(ciphertext, &nonce, &key) {
            Ok(msg) => Ok(msg),
            Err(_) => {
                Err(Error::CryptoError("Secret key and nonce could not decrypt ciphertext"
                                           .to_string()))
            }
        }
    }

    // *******************************************
    // Key generation functions
    // *******************************************
    /// given a box byte vec, read the keys (with rev) of the user and service.
    pub fn get_box_user_and_service_keys(&self, payload: &Vec<u8>) -> Result<(String, String)> {
        let mut p = payload.as_slice();
        let mut file_version = String::new();
        let mut user_key_name = String::new();
        let mut service_key_name = String::new();

        if try!(p.read_line(&mut file_version)) <= 0 {
            return Err(Error::CryptoError("Corrupt payload, can't read file version".to_string()));
        }

        if try!(p.read_line(&mut user_key_name)) <= 0 {
            return Err(Error::CryptoError("Corrupt payload, can't read user key name".to_string()));
        }
        if try!(p.read_line(&mut service_key_name)) <= 0 {
            return Err(Error::CryptoError("Corrupt payload, can't read service key name"
                                              .to_string()));
        }
        // all these values will have a newline at the end
        let file_version = file_version.trim();
        let user_key_name = user_key_name.trim();
        let service_key_name = service_key_name.trim();

        // only check after file_version has been trimmed
        if file_version != BOX_FORMAT_VERSION {
            return Err(Error::CryptoError("Invalid encrypted payload version ".to_string()));
        }

        Ok((user_key_name.to_string(), service_key_name.to_string()))
    }

    pub fn generate_origin_sig_key(&self, origin: &str) -> Result<String> {
        let revision = self.mk_revision_string();
        let keyname = self.mk_origin_sig_key_name(origin, &revision);
        debug!("new origin sig key name = {}", &keyname);
        try!(self.generate_sig_keypair_files(&keyname));
        Ok(keyname)
    }

    /// generate a service box key, return the name of the key we generated
    pub fn generate_service_box_key(&self, org: &str, service_group: &str) -> Result<String> {
        let revision = self.mk_revision_string();
        let keyname = self.mk_service_box_key_name(org, &revision, service_group);
        debug!("new user sig key name = {}", &keyname);
        try!(self.generate_box_keypair_files(&keyname));
        Ok(keyname)
    }

    /// generate a user box key, return the name of the key we generated
    pub fn generate_user_box_key(&self, user: &str) -> Result<String> {
        let revision = self.mk_revision_string();
        let keyname = self.mk_user_box_key_name(&revision, &user);
        debug!("new user sig key name = {}", &keyname);
        try!(self.generate_box_keypair_files(&keyname));
        Ok(keyname)
    }

    /// generate a ring key, return the name of the key we generated
    pub fn generate_ring_sym_key(&self, ring: &str) -> Result<String> {
        let revision = self.mk_revision_string();
        let keyname = self.mk_ring_sym_key_name(&revision, &ring);
        debug!("new ring key name = {}", &keyname);
        let _ = try!(self.generate_sym_key_file(&keyname));
        Ok(keyname)
    }

    /// generates a revision string in the form:
    /// `{year}{month}{day}{hour24}{minute}{second}`
    /// Timestamps are in UTC time.
    fn mk_revision_string(&self) -> String {
        let now = time::now_utc();
        // https://github.com/rust-lang-deprecated/time/blob/master/src/display.rs
        // http://man7.org/linux/man-pages/man3/strftime.3.html
        match now.strftime("%Y%m%d%H%M%S") {
            Ok(result) => format!("{}", result),
            Err(_) => panic!("can't parse system time"),
        }
    }

    fn mk_key_filename(&self, dir: &str, keyname: &str, suffix: &str) -> String {
        format!("{}/{}.{}", dir, keyname, suffix)
    }

    fn mk_origin_sig_key_name(&self, origin: &str, revision: &str) -> String {
        format!("{}-{}", origin, revision)
    }

    fn mk_service_box_key_name(&self, org: &str, revision: &str, service_group: &str) -> String {
        format!("{}@{}-{}", service_group, org, revision)
    }

    fn mk_user_box_key_name(&self, revision: &str, user: &str) -> String {
        format!("{}-{}", user, revision)
    }

    fn mk_ring_sym_key_name(&self, revision: &str, ring: &str) -> String {
        format!("{}-{}", ring, revision)
    }

    fn generate_box_keypair_files(&self, keyname: &str) -> Result<(BoxPublicKey, BoxSecretKey)> {
        let (pk, sk) = box_::gen_keypair();

        let public_keyfile = self.mk_key_filename(&self.key_cache, keyname, PUBLIC_KEY_SUFFIX);
        let secret_keyfile = self.mk_key_filename(&self.key_cache, keyname, SECRET_BOX_KEY_SUFFIX);
        debug!("public box keyfile = {}", &public_keyfile);
        debug!("secret box keyfile = {}", &secret_keyfile);
        try!(self.write_keypair_files(KeyType::Box,
                                      &keyname,
                                      Some(&public_keyfile),
                                      Some(&pk[..].to_base64(STANDARD).into_bytes()),
                                      &secret_keyfile,
                                      &sk[..].to_base64(STANDARD).into_bytes()));
        Ok((pk, sk))
    }

    fn generate_sig_keypair_files(&self, keyname: &str) -> Result<(SigPublicKey, SigSecretKey)> {
        let (pk, sk) = sign::gen_keypair();

        let public_keyfile = self.mk_key_filename(&self.key_cache, keyname, PUBLIC_KEY_SUFFIX);
        let secret_keyfile = self.mk_key_filename(&self.key_cache, keyname, SECRET_SIG_KEY_SUFFIX);
        debug!("public sig keyfile = {}", &public_keyfile);
        debug!("secret sig keyfile = {}", &secret_keyfile);

        try!(self.write_keypair_files(KeyType::Sig,
                                      &keyname,
                                      Some(&public_keyfile),
                                      Some(&pk[..].to_base64(STANDARD).into_bytes()),
                                      &secret_keyfile,
                                      &sk[..].to_base64(STANDARD).into_bytes()));
        Ok((pk, sk))
    }

    fn generate_sym_key_file(&self, keyname: &str) -> Result<SymSecretKey> {
        let sk = secretbox::gen_key();

        let secret_keyfile = self.mk_key_filename(&self.key_cache, keyname, SECRET_SYM_KEY_SUFFIX);
        debug!("secret ring keyfile = {}", &secret_keyfile);

        try!(self.write_keypair_files(KeyType::Sym,
                                      &keyname,
                                      None,
                                      None,
                                      &secret_keyfile,
                                      &sk[..].to_base64(STANDARD).into_bytes()));
        Ok(sk)
    }

    fn write_keypair_files<P: AsRef<Path>>(&self,
                                           key_type: KeyType,
                                           keyname: &str,
                                           public_keyfile: Option<P>,
                                           public_content: Option<&Vec<u8>>,
                                           secret_keyfile: P,
                                           secret_content: &Vec<u8>)
                                           -> Result<()> {
        if let Some(public_keyfile) = public_keyfile {
            let public_version = match key_type {
                KeyType::Sig => PUBLIC_SIG_KEY_VERSION,
                KeyType::Box => PUBLIC_BOX_KEY_VERSION,
                KeyType::Sym => unreachable!("Sym keys do not have a public key"),
            };

            let public_content = match public_content {
                Some(c) => c,
                None => return Err(Error::CryptoError(format!("Invalid calling of this function"))),
            };

            if let Some(pk_dir) = public_keyfile.as_ref().parent() {
                try!(fs::create_dir_all(pk_dir));
            } else {
                return Err(Error::BadKeyPath(public_keyfile.as_ref()
                                                           .to_string_lossy()
                                                           .into_owned()));
            }
            if public_keyfile.as_ref().exists() && public_keyfile.as_ref().is_file() {
                return Err(Error::CryptoError(format!("Public keyfile already exists {}",
                                                      public_keyfile.as_ref().display())));
            }
            let public_file = try!(File::create(public_keyfile.as_ref()));
            let mut public_writer = BufWriter::new(&public_file);
            try!(write!(public_writer, "{}\n{}\n\n", public_version, keyname));
            try!(public_writer.write_all(public_content));
            try!(perm::set_permissions(public_keyfile, PUBLIC_KEY_PERMISSIONS));
        }

        let secret_version = match key_type {
            KeyType::Sig => SECRET_SIG_KEY_VERSION,
            KeyType::Box => SECRET_BOX_KEY_VERSION,
            KeyType::Sym => SECRET_SYM_KEY_VERSION,
        };
        if let Some(sk_dir) = secret_keyfile.as_ref().parent() {
            try!(fs::create_dir_all(sk_dir));
        } else {
            return Err(Error::BadKeyPath(secret_keyfile.as_ref().to_string_lossy().into_owned()));
        }
        if secret_keyfile.as_ref().exists() && secret_keyfile.as_ref().is_file() {
            return Err(Error::CryptoError(format!("Secret keyfile already exists {}",
                                                  secret_keyfile.as_ref().display())));
        }
        let secret_file = try!(File::create(secret_keyfile.as_ref()));
        let mut secret_writer = BufWriter::new(&secret_file);
        try!(write!(secret_writer, "{}\n{}\n\n", secret_version, keyname));
        try!(secret_writer.write_all(secret_content));
        try!(perm::set_permissions(secret_keyfile, SECRET_KEY_PERMISSIONS));

        Ok(())
    }

    // *******************************************
    // Key reading functions
    // *******************************************

    /// Return a Vec of origin keys with a given name.
    /// The newest key is listed first in the Vec
    /// Origin keys are always "sig" keys. They are used for signing/verifying
    /// packages, not for encryption.
    pub fn read_sig_origin_keys(&self, origin_keyname: &str) -> Result<Vec<SigKeyPair>> {
        let revisions = try!(self.get_key_revisions(origin_keyname));
        let mut key_pairs = Vec::new();
        for rev in &revisions {
            debug!("Attempting to read key rev {} for {}", rev, origin_keyname);
            let pk = match self.get_sig_public_key(rev) {
                Ok(k) => Some(k),
                Err(e) => {
                    // Not an error, just continue
                    debug!("Can't find public key for rev {}: {}", rev, e);
                    None
                }
            };
            let sk = match self.get_sig_secret_key(rev) {
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

    pub fn read_box_keys(&self, keyname: &str) -> Result<Vec<BoxKeyPair>> {
        let revisions = try!(self.get_key_revisions(keyname));
        let mut key_pairs = Vec::new();
        for rev in &revisions {
            debug!("Attempting to read key rev {} for {}", rev, keyname);
            let pk = match self.get_box_public_key(rev) {
                Ok(k) => Some(k),
                Err(e) => {
                    // Not an error, just continue
                    debug!("Can't find public key for rev {}: {}", rev, e);
                    None
                }
            };
            let sk = match self.get_box_secret_key(rev) {
                Ok(k) => Some(k),
                Err(e) => {
                    // Not an error, just continue
                    debug!("Can't find secret key for rev {}: {}", rev, e);
                    None
                }
            };
            let kp = BoxKeyPair::new(keyname.to_string(), rev.clone(), pk, sk);
            key_pairs.push(kp);
        }
        Ok(key_pairs)
    }

    pub fn read_sym_keys(&self, keyname: &str) -> Result<Vec<SymKey>> {
        let revisions = try!(self.get_key_revisions(keyname));
        let mut keys = Vec::new();
        for rev in &revisions {
            debug!("Attempting to read key rev {} for {}", rev, keyname);
            let sk = match self.get_sym_secret_key(rev) {
                Ok(k) => Some(k),
                Err(e) => {
                    // Not an error, just continue
                    debug!("Can't find secret key for rev {}: {}", rev, e);
                    None
                }
            };
            let k = SymKey::new(keyname.to_string(), rev.clone(), None, sk);
            keys.push(k);
        }
        Ok(keys)
    }

    pub fn get_sig_secret_key(&self, key_with_rev: &str) -> Result<SigSecretKey> {
        let bytes = try!(self.get_sig_secret_key_bytes(key_with_rev));
        match SigSecretKey::from_slice(&bytes) {
            Some(sk) => Ok(sk),
            None => {
                return Err(Error::CryptoError(format!("Can't read sig secret key for {}",
                                                      key_with_rev)))
            }
        }
    }

    pub fn get_sig_public_key(&self, key_with_rev: &str) -> Result<SigPublicKey> {
        let bytes = try!(self.get_sig_public_key_bytes(key_with_rev));
        match SigPublicKey::from_slice(&bytes) {
            Some(sk) => Ok(sk),
            None => {
                return Err(Error::CryptoError(format!("Can't read sig public key for {}",
                                                      key_with_rev)))
            }
        }
    }

    pub fn get_box_secret_key(&self, key_with_rev: &str) -> Result<BoxSecretKey> {
        let bytes = try!(self.get_box_secret_key_bytes(key_with_rev));
        match BoxSecretKey::from_slice(&bytes) {
            Some(sk) => Ok(sk),
            None => {
                return Err(Error::CryptoError(format!("Can't read box secret key for {}",
                                                      key_with_rev)))
            }
        }
    }

    pub fn get_box_public_key(&self, key_with_rev: &str) -> Result<BoxPublicKey> {
        let bytes = try!(self.get_box_public_key_bytes(key_with_rev));
        match BoxPublicKey::from_slice(&bytes) {
            Some(sk) => Ok(sk),
            None => {
                return Err(Error::CryptoError(format!("Can't read box public key for {}",
                                                      key_with_rev)))
            }
        }
    }

    pub fn get_sym_secret_key(&self, key_with_rev: &str) -> Result<SymSecretKey> {
        let bytes = try!(self.get_sym_secret_key_bytes(key_with_rev));
        match SymSecretKey::from_slice(&bytes) {
            Some(sk) => Ok(sk),
            None => {
                return Err(Error::CryptoError(format!("Can't read sym secret key for {}",
                                                      key_with_rev)))
            }
        }
    }

    fn get_box_public_key_bytes(&self, key_with_rev: &str) -> Result<Vec<u8>> {
        let public_keyfile = self.mk_key_filename(&self.key_cache, key_with_rev, PUBLIC_KEY_SUFFIX);
        self.read_key_bytes(&public_keyfile)
    }

    fn get_box_secret_key_bytes(&self, key_with_rev: &str) -> Result<Vec<u8>> {
        let secret_keyfile = self.mk_key_filename(&self.key_cache,
                                                  key_with_rev,
                                                  SECRET_BOX_KEY_SUFFIX);
        self.read_key_bytes(&secret_keyfile)
    }

    fn get_sig_public_key_bytes(&self, key_with_rev: &str) -> Result<Vec<u8>> {
        let public_keyfile = self.mk_key_filename(&self.key_cache, key_with_rev, PUBLIC_KEY_SUFFIX);
        self.read_key_bytes(&public_keyfile)
    }

    fn get_sig_secret_key_bytes(&self, key_with_rev: &str) -> Result<Vec<u8>> {
        let secret_keyfile = self.mk_key_filename(&self.key_cache,
                                                  key_with_rev,
                                                  SECRET_SIG_KEY_SUFFIX);
        self.read_key_bytes(&secret_keyfile)
    }

    fn get_sym_secret_key_bytes(&self, key_with_rev: &str) -> Result<Vec<u8>> {
        let secret_keyfile = self.mk_key_filename(&self.key_cache,
                                                  key_with_rev,
                                                  SECRET_SYM_KEY_SUFFIX);
        self.read_key_bytes(&secret_keyfile)
    }

    /// Returns the full path to the secret sym key given a key name with revision.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// extern crate habitat_core;
    /// extern crate tempdir;
    ///
    /// use habitat_core::crypto::Context;
    /// use std::fs::File;
    /// use tempdir::TempDir;
    ///
    /// fn main() {
    ///     let key_cache = TempDir::new("key_cache").unwrap();
    ///     let ctx = Context { key_cache: key_cache.path().to_string_lossy().into_owned() };
    ///     let keyfile = key_cache.path().join("beyonce-20160504220722.sym.key");
    ///     let _ = File::create(&keyfile).unwrap();
    ///
    ///     let path = ctx.get_sym_secret_key_path("beyonce-20160504220722").unwrap();
    ///     assert_eq!(path, keyfile);
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// * If no file exists at the the computed file path
    pub fn get_sym_secret_key_path(&self, key_with_rev: &str) -> Result<PathBuf> {
        let secret_keyfile = self.mk_key_filename(&self.key_cache,
                                                  key_with_rev,
                                                  SECRET_SYM_KEY_SUFFIX);
        let path = PathBuf::from(secret_keyfile);
        if !path.is_file() {
            return Err(Error::CryptoError(format!("No sym secret key found at {}",
                                                  path.display())));
        }
        Ok(path)
    }

    /// Read a file into a Vec<u8>
    fn read_key_bytes(&self, keyfile: &str) -> Result<Vec<u8>> {
        let mut f = try!(File::open(keyfile));
        let mut s = String::new();
        if try!(f.read_to_string(&mut s)) <= 0 {
            return Err(Error::CryptoError("Can't read key bytes".to_string()));
        }
        let start_index = match s.find("\n\n") {
            Some(i) => i + 1,
            None => {
                return Err(Error::CryptoError(format!("Malformed key contents for: {}", keyfile)))
            }
        };

        match s[start_index..].as_bytes().from_base64() {
            Ok(keybytes) => Ok(keybytes),
            Err(e) => {
                return Err(Error::CryptoError(format!("Can't read raw key from {}: {}",
                                                      keyfile,
                                                      e)))
            }
        }
    }

    /// If a key "belongs" to a filename revision, then add the full stem of the
    /// file (without path, without .suffix)
    fn check_filename(&self,
                      keyname: &str,
                      filename: String,
                      candidates: &mut HashSet<String>)
                      -> () {
        if filename.ends_with(PUBLIC_KEY_SUFFIX) {
            if filename.starts_with(keyname) {
                // push filename without extension
                // -1 for the '.' before 'pub'
                let (stem, _) = filename.split_at(filename.chars().count() -
                                                  PUBLIC_KEY_SUFFIX.chars().count() -
                                                  1);
                candidates.insert(stem.to_string());
            }
            // SECRET_SIG_KEY_SUFFIX and SECRET_BOX_KEY_SUFFIX are the same at the
            // moment, but don't assume that they'll always be that way.
        } else if filename.ends_with(SECRET_SIG_KEY_SUFFIX) {
            if filename.starts_with(keyname) {
                // -1 for the '.' before the suffix
                let (stem, _) = filename.split_at(filename.chars().count() -
                                                  SECRET_SIG_KEY_SUFFIX.chars().count() -
                                                  1);
                candidates.insert(stem.to_string());
            }
        } else if filename.ends_with(SECRET_BOX_KEY_SUFFIX) {
            // -1 for the '.' before the suffix
            if filename.starts_with(keyname) {
                let (stem, _) = filename.split_at(filename.chars().count() -
                                                  SECRET_BOX_KEY_SUFFIX.chars().count() -
                                                  1);
                candidates.insert(stem.to_string());
            }
        } else if filename.ends_with(SECRET_SYM_KEY_SUFFIX) {
            // -1 for the '.' before the suffix
            if filename.starts_with(keyname) {
                let (stem, _) = filename.split_at(filename.chars().count() -
                                                  SECRET_SYM_KEY_SUFFIX.chars().count() -
                                                  1);
                candidates.insert(stem.to_string());
            }
        }
    }

    /// Take a key name (ex "habitat"), and find all revisions of that
    /// keyname in the nacl_key_dir().
    pub fn get_key_revisions(&self, keyname: &str) -> Result<Vec<String>> {
        // look for .pub keys
        // accumulator for files that match
        // let mut candidates = Vec::new();
        let mut candidates = HashSet::new();
        let paths = match fs::read_dir(&self.key_cache) {
            Ok(p) => p,
            Err(e) => {
                return Err(Error::CryptoError(format!("Error reading key directory {}: {}",
                                                      &self.key_cache,
                                                      e)))
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

            debug!("checking file: {}", &filename);
            self.check_filename(keyname, filename, &mut candidates);
        }

        // traverse the candidates set and sort the entries
        let mut candidate_vec = Vec::new();
        for c in &candidates {
            candidate_vec.push(c.clone());
        }
        candidate_vec.sort();
        // newest key first
        candidate_vec.reverse();
        Ok(candidate_vec)
    }

    /// Writes a sym key to the key cache from the contents of a string slice.
    ///
    /// The return is a `Result` of a `String` containing the key's name with revision.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// extern crate habitat_core;
    /// extern crate tempdir;
    ///
    /// use habitat_core::crypto::Context;
    /// use tempdir::TempDir;
    ///
    /// fn main() {
    ///     let key_cache = TempDir::new("key_cache").unwrap();
    ///     let ctx = Context { key_cache: key_cache.path().to_string_lossy().into_owned() };
    ///     let content = "SYM-SEC-1
    /// beyonce-20160504220722
    ///
    /// RCFaO84j41GmrzWddxMdsXpGdn3iuIy7Mw3xYrjPLsE=";
    ///
    ///     let keyname = ctx.write_sym_key_from_str(content).unwrap();
    ///     assert_eq!(keyname, "beyonce-20160504220722");
    ///     assert!(key_cache.path().join("beyonce-20160504220722.sym.key").is_file());
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// * If there is a key version mismatch
    /// * If the key version is missing
    /// * If the key name with revision is missing
    /// * If the key value (the Bas64 payload) is missing
    /// * If the key file cannot be written to disk
    /// * If an existing key is already installed, but the new content is different from the
    /// existing
    pub fn write_sym_key_from_str(&self, content: &str) -> Result<String> {
        let mut lines = content.lines();
        let _ = match lines.next() {
            Some(val) => {
                if val != SECRET_SYM_KEY_VERSION {
                    return Err(Error::CryptoError(format!("Unsupported key version: {}", val)));
                }
                ()
            }
            None => {
                let msg = format!("write_sym_key_from_str:1 Malformed sym key string:\n({})",
                                  content);
                return Err(Error::CryptoError(msg));
            }
        };
        let keyname = match lines.next() {
            Some(val) => val,
            None => {
                let msg = format!("write_sym_key_from_str:2 Malformed sym key string:\n({})",
                                  content);
                return Err(Error::CryptoError(msg));
            }
        };
        let sk = match lines.nth(1) {
            Some(val) => val,
            None => {
                let msg = format!("write_sym_key_from_str:3 Malformed sym key string:\n({})",
                                  content);
                return Err(Error::CryptoError(msg));
            }
        };
        let secret_keyfile = self.mk_key_filename(&self.key_cache, &keyname, SECRET_SYM_KEY_SUFFIX);
        let tmpfile = {
            let mut t = secret_keyfile.clone();
            t.push('.');
            t.push_str(&randombytes(6).as_slice().to_hex());
            t
        };

        debug!("Writing temp key file {}", &tmpfile);
        try!(self.write_keypair_files(KeyType::Sym,
                                      &keyname,
                                      None,
                                      None,
                                      &tmpfile,
                                      &sk.as_bytes().to_vec()));

        if Path::new(&secret_keyfile).is_file() {
            let existing_hash = try!(self.hash_file(&secret_keyfile));
            let new_hash = try!(self.hash_file(&tmpfile));
            if existing_hash != new_hash {
                let msg = format!("Existing key file {} found but new version hash is different, \
                                  failing to write new file over existing. ({} = {}, {} = {})",
                                  secret_keyfile,
                                  secret_keyfile,
                                  existing_hash,
                                  tmpfile,
                                  new_hash);
                return Err(Error::CryptoError(msg));
            } else {
                // Otherwise, hashes match and we can skip writing over the exisiting file
                debug!("New content hash matches existing file {} hash, removing temp key file {}.",
                       secret_keyfile,
                       tmpfile);
                try!(fs::remove_file(tmpfile));
            }
        } else {
            debug!("Moving {} to {}", tmpfile, secret_keyfile);
            try!(fs::rename(tmpfile, secret_keyfile));
        }

        Ok(keyname.to_string())
    }
}

#[cfg(test)]
mod test {
    use std::env;
    use std::fs::File;
    use std::io::Read;
    use std::path::PathBuf;

    use tempdir::TempDir;

    use crypto::Context;

    pub static VALID_KEY: &'static str = "ring-key-valid-20160504220722.sym.key";
    pub static VALID_NAME: &'static str = "ring-key-valid-20160504220722";

    pub fn random_ctx() -> (TempDir, Context) {
        let tempdir = TempDir::new("key_cache").unwrap();
        let ctx = Context { key_cache: tempdir.path().to_string_lossy().into_owned() };
        (tempdir, ctx)
    }

    pub fn fixture(name: &str) -> PathBuf {
        let file = env::current_exe()
                       .unwrap()
                       .parent()
                       .unwrap()
                       .parent()
                       .unwrap()
                       .parent()
                       .unwrap()
                       .join("tests")
                       .join("fixtures")
                       .join(name);
        if !file.is_file() {
            panic!("No fixture {} exists!", file.display());
        }
        file
    }

    pub fn fixture_as_string(name: &str) -> String {
        let mut file = File::open(fixture(name)).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        content
    }

    mod get_sym_secret_key_path {
        use super::*;

        use std::fs;

        #[test]
        fn returns_a_path() {
            let (cache, ctx) = random_ctx();
            fs::copy(fixture(&format!("keys/{}", VALID_KEY)),
                     cache.path().join(VALID_KEY))
                .unwrap();

            let result = ctx.get_sym_secret_key_path(VALID_NAME).unwrap();
            assert_eq!(result, cache.path().join(VALID_KEY));
        }

        #[test]
        #[should_panic(expected = "No sym secret key found at")]
        fn errors_when_key_doesnt_exist() {
            let (_, ctx) = random_ctx();

            ctx.get_sym_secret_key_path("nope-nope").unwrap();
        }
    }

    mod write_sym_key_from_str {
        use super::*;

        use std::fs::{self, File};
        use std::io::Read;
        use std::path::Path;

        #[test]
        fn writes_new_key_file() {
            let (cache, ctx) = random_ctx();
            let content = fixture_as_string(&format!("keys/{}", VALID_KEY));
            let new_key_file = Path::new(cache.path()).join(VALID_KEY);

            assert_eq!(new_key_file.is_file(), false);
            let keyname = ctx.write_sym_key_from_str(&content).unwrap();
            assert_eq!(keyname, VALID_NAME);
            assert!(new_key_file.is_file());

            let new_content = {
                let mut new_content_file = File::open(new_key_file).unwrap();
                let mut new_content = String::new();
                new_content_file.read_to_string(&mut new_content).unwrap();
                new_content
            };

            assert_eq!(new_content, content);
        }

        #[test]
        fn doesnt_error_when_key_exists_and_is_identical() {
            let (cache, ctx) = random_ctx();
            let content = fixture_as_string(&format!("keys/{}", VALID_KEY));
            let new_key_file = Path::new(cache.path()).join(VALID_KEY);

            // install the key into the cache
            fs::copy(fixture(&format!("keys/{}", VALID_KEY)), &new_key_file).unwrap();

            let keyname = ctx.write_sym_key_from_str(&content).unwrap();
            assert_eq!(keyname, VALID_NAME);
            assert!(new_key_file.is_file());
        }

        #[test]
        #[should_panic(expected = "Unsupported key version")]
        fn error_when_version_is_supported() {
            let (_, ctx) = random_ctx();
            let content = fixture_as_string("keys/ring-key-invalid-version-20160504221247.sym.key");

            ctx.write_sym_key_from_str(&content).unwrap();
        }

        #[test]
        #[should_panic(expected = "write_sym_key_from_str:1 Malformed sym key string")]
        fn error_when_missing_version() {
            let (_, ctx) = random_ctx();

            ctx.write_sym_key_from_str("").unwrap();
        }

        #[test]
        #[should_panic(expected = "write_sym_key_from_str:2 Malformed sym key string")]
        fn error_when_missing_name() {
            let (_, ctx) = random_ctx();

            ctx.write_sym_key_from_str("SYM-SEC-1\n").unwrap();
        }

        #[test]
        #[should_panic(expected = "write_sym_key_from_str:3 Malformed sym key string")]
        fn error_when_missing_key() {
            let (_, ctx) = random_ctx();

            ctx.write_sym_key_from_str("SYM-SEC-1\nim-in-trouble-123\n").unwrap();
        }

        #[test]
        #[should_panic(expected = "Existing key file")]
        fn error_when_key_exists_and_hashes_differ() {
            let (cache, ctx) = random_ctx();
            let key = fixture("keys/ring-key-valid-20160504220722.sym.key");
            fs::copy(key,
                     cache.path().join("ring-key-valid-20160504220722.sym.key"))
                .unwrap();

            ctx.write_sym_key_from_str("SYM-SEC-1\nring-key-valid-20160504220722\n\nsomething")
               .unwrap();
        }
    }
}
