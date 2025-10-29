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
//! - In general, the word `key` by itself does not indicate something as **public** or **secret**.
//!   The exceptions to this rule are as follows:
//!     - if the word key appears in a URL, then we are referring to a public key to conform to
//!       other APIs that offer similar public key downloading functionality.
//!     - the word `key` appears as part of a file suffix, where it is then considered as a **secret
//!       key** file.
//! - Referring to keys (by example):
//!     - A key name: `habitat`
//!     - A key rev: `201603312016`
//!     - A key name with rev: `habitat-201603312016`
//!     - A key file: `habitat-201603312016.pub`
//!     - A key path or fully qualified key path: `/foo/bar/habitat-201603312016.pub`
//! - An **Origin** refers to build-time operations, including signing and verification of a Habitat
//!   artifact.
//! - An **Organization** or **Org** refers to run-time operations such as deploying a package
//!   signed in a different origin into your own organization. This is abbreviated as "org" in
//!   user-facing command line parameters and internal variable names.
//! - To distinguish between **Org** and **Origin**, the following might help: Habitat packages come
//!   from an **Origin** and run in an **Organization**.
//! - A **Ring** is the full set of Supervisors that communicate with each other.
//! - A **Signing key**, also known as a **sig** key, is used to sign and verify Habitat artifacts.
//!   The file contains a `sig.key` file suffix. Note that sig keys are not compatible with box
//!   keys.
//! - A **Box key** is used for encryption and decryption of arbitrary data. The file contains a
//!   `.box.key` file suffix. Note that box keys are not compatible with sig keys.
//! - A **Sym key** is used for symmetric encryption, meaning that a shared secret is used to
//!   encrypt a message into a ciphertext and that same secret is used later to decrypt the
//!   ciphertext into the original message.
//! - A **Ring key** is a **sym** key used when sending messages between the Supervisors to prevent
//!   a third party from intercepting the traffic.
//! - **Key revisions** - There can exist several keys for any given user, service, ring, or origin
//!   via different revision numbers. Revision numbers appear following the key name and are in the
//!   format `{year}{month}{day}{hour24}{minute}{second}`. For all user-facing cryptographic
//!   operations (such as sign, verify, encrypt, decrypt, etc.), the latest key is tried first, and
//!   upon failure, the keys will be tried in reverse chronological order until success or there are
//!   no more keys.
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
//! # File formats
//!
//! ## Habitat artifacts
//!
//! A signed Habitat artifact (a file with the extension `.hart`) has 5 plaintext lines followed by
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
//! authenticity, this can be accomplished with:
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
//! There are 3 lines, that is 3 parts that are separated by a newline character `\n`. They are as
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

use std::ffi::c_void;

use crate::error::{Error,
                   Result};

/// The suffix on the end of a public sig/box file
pub const PUBLIC_KEY_SUFFIX: &str = "pub";
/// The suffix on the end of a public sig file
pub const SECRET_SIG_KEY_SUFFIX: &str = "sig.key";
/// The suffix on the end of a secret box file
pub const SECRET_BOX_KEY_SUFFIX: &str = "box.key";
/// The suffix on the end of a secret symmetric key file
pub const SECRET_SYM_KEY_SUFFIX: &str = "sym.key";
/// The hashing function we're using during sign/verify
/// See also: https://download.libsodium.org/doc/hashing/generic_hashing.html
pub const SIG_HASH_TYPE: &str = "BLAKE2b";
/// This environment variable allows you to override the fs::CACHE_KEY_PATH
/// at runtime. This is useful for testing.
pub const CACHE_KEY_PATH_ENV_VAR: &str = "HAB_CACHE_KEY_PATH";
pub const HART_FORMAT_VERSION: &str = "HART-1";
pub const BOX_FORMAT_VERSION: &str = "BOX-1";
pub const ANONYMOUS_BOX_FORMAT_VERSION: &str = "ANONYMOUS-BOX-1";

pub const PUBLIC_SIG_KEY_VERSION: &str = "SIG-PUB-1";
pub const SECRET_SIG_KEY_VERSION: &str = "SIG-SEC-1";
pub const PUBLIC_BOX_KEY_VERSION: &str = "BOX-PUB-1";
pub const SECRET_BOX_KEY_VERSION: &str = "BOX-SEC-1";
pub const SECRET_SYM_KEY_VERSION: &str = "SYM-SEC-1";

pub mod artifact;
#[cfg(windows)]
pub mod dpapi;
mod hash;
pub mod keys;

pub use hash::Blake2bHash;

pub fn init() -> Result<()> {
    if (unsafe { libsodium_sys::sodium_init() } > 0) {
        return Err(Error::SodiumInitFailed);
    }
    Ok(())
}

/// A comparison function that takes a consistent amount of time to compare
/// values of a given number of bytes so as to be resistant to timing attacks.
/// This function should be used whenever comparing a secret value to one
/// supplied by a user.
pub fn secure_eq<T, U>(t: T, u: U) -> bool
    where T: AsRef<[u8]>,
          U: AsRef<[u8]>
{
    let t_ref = t.as_ref();
    let u_ref = u.as_ref();
    unsafe {
        libsodium_sys::sodium_memcmp(t_ref.as_ptr() as *const c_void,
                                     u_ref.as_ptr() as *const c_void,
                                     t_ref.len())
        == 0
    }
}

#[cfg(test)]
pub mod test_support {
    use crate::{crypto::keys::{Key,
                               KeyCache},
                error as herror};

    use std::{fs::File,
              io::Read,
              path::PathBuf,
              thread,
              time::{Duration,
                     Instant}};
    use tempfile::{Builder,
                   TempDir};

    pub fn fixture(name: &str) -> PathBuf {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests")
                                                            .join("fixtures")
                                                            .join(name);
        if !path.is_file() {
            panic!("Fixture '{}' not found at: {:?}", name, path);
        }
        path
    }

    pub fn fixture_as_string(name: &str) -> String {
        let mut file = File::open(fixture(name)).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        content
    }

    pub fn wait_until_ok<F, T>(some_fn: F) -> Option<T>
        where F: Fn() -> Result<T, herror::Error>
    {
        let wait_duration = Duration::from_secs(30);
        let current_time = Instant::now();
        while current_time.elapsed() < wait_duration {
            if let Ok(s) = some_fn() {
                return Some(s);
            }
        }
        None
    }

    /// Returns the `TempDir` that backs the cache to prevent it from
    /// getting `Drop`ped too early; feel free to ignore it.
    pub fn new_cache() -> (KeyCache, TempDir) {
        let dir = Builder::new().prefix("key_cache").tempdir().unwrap();
        let cache = KeyCache::new(dir.path());
        // Not strictly required, of course, since we know we just
        // created the directory.
        cache.setup().unwrap();
        (cache, dir)
    }

    pub fn wait_1_sec() { thread::sleep(Duration::from_secs(1)); }

    /// Helper function to return a specific kind of key read from a
    /// file in our fixtures directory.
    pub fn fixture_key<K, E>(path_in_fixtures: &str) -> K
        where K: Key + std::str::FromStr<Err = E>,
              E: std::fmt::Debug
    {
        fixture_as_string(path_in_fixtures).parse::<K>().unwrap()
    }
}
