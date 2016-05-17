// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::path::Path;

use rustc_serialize::base64::{STANDARD, ToBase64, FromBase64};
use sodiumoxide::crypto::sign;

use error::{Error, Result};
use super::{HART_FORMAT_VERSION, SIG_HASH_TYPE, SigKeyPair};
use super::keys::parse_name_with_rev;

/// Generate and sign a package
pub fn sign<P1: ?Sized, P2: ?Sized>(src: &P1, dst: &P2, pair: &SigKeyPair) -> Result<()>
    where P1: AsRef<Path>,
          P2: AsRef<Path>
{
    let hash = try!(super::hash::hash_file(&src));
    debug!("File hash for {} = {}", src.as_ref().display(), &hash);

    let signature = sign::sign(&hash.as_bytes(), try!(pair.secret()));
    let output_file = try!(File::create(dst));
    let mut writer = BufWriter::new(&output_file);
    let () = try!(write!(writer,
                         "{}\n{}\n{}\n{}\n\n",
                         HART_FORMAT_VERSION,
                         pair.name_with_rev(),
                         SIG_HASH_TYPE,
                         signature.to_base64(STANDARD)));
    let mut file = try!(File::open(src));
    try!(io::copy(&mut file, &mut writer));
    Ok(())
}

/// return a BufReader to the .tar bytestream, skipping the signed header
pub fn get_archive_reader<P: AsRef<Path>>(src: &P) -> Result<BufReader<File>> {
    let f = try!(File::open(src));
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

/// verify the crypto signature of a .hart file
pub fn verify<P1: ?Sized, P2: ?Sized>(src: &P1, cache_key_path: &P2) -> Result<(String, String)>
    where P1: AsRef<Path>,
          P2: AsRef<Path>
{
    let f = try!(File::open(src));
    let mut reader = BufReader::new(f);

    let _ = {
        let mut buffer = String::new();
        match reader.read_line(&mut buffer) {
            Ok(0) => {
                return Err(Error::CryptoError("Corrupt payload, can't read format version"
                                                  .to_string()))
            }
            Ok(_) => {
                if buffer.trim() != HART_FORMAT_VERSION {
                    let msg = format!("Unsupported format version: {}", &buffer.trim());
                    return Err(Error::CryptoError(msg));
                }
            }
            Err(e) => return Err(Error::from(e)),

        };
        buffer.trim().to_string()
    };
    let pair = {
        let mut buffer = String::new();
        if try!(reader.read_line(&mut buffer)) <= 0 {
            return Err(Error::CryptoError("Corrupt payload, can't read origin key name"
                                              .to_string()));
        }
        try!(SigKeyPair::get_pair_for(buffer.trim(), cache_key_path))
    };
    let _ = {
        let mut buffer = String::new();
        match reader.read_line(&mut buffer) {
            Ok(0) => {
                return Err(Error::CryptoError("Corrupt payload, can't read hash type".to_string()))
            }
            Ok(_) => {
                if buffer.trim() != SIG_HASH_TYPE {
                    let msg = format!("Unsupported signature type: {}", &buffer.trim());
                    return Err(Error::CryptoError(msg));
                }
            }
            Err(e) => return Err(Error::from(e)),
        };
    };
    let signature = {
        let mut buffer = String::new();
        match reader.read_line(&mut buffer) {
            Ok(0) => {
                return Err(Error::CryptoError("Corrupt payload, can't read signature".to_string()))
            }
            Ok(_) => {
                match buffer.trim().as_bytes().from_base64() {
                    Ok(sig) => sig,
                    Err(e) => {
                        return Err(Error::CryptoError(format!("Can't decode signature: {}", e)));
                    }
                }
            }
            Err(e) => return Err(Error::from(e)),
        }
    };
    let _ = {
        let mut buffer = String::new();
        if try!(reader.read_line(&mut buffer)) <= 0 {
            return Err(Error::CryptoError("Corrupt payload, can't find end of header".to_string()));
        }
    };
    let expected_hash = match sign::verify(signature.as_slice(), try!(pair.public())) {
        Ok(signed_data) => {
            match String::from_utf8(signed_data) {
                Ok(hash) => hash,
                Err(_) => {
                    return Err(Error::CryptoError("Error parsing artifact signature".to_string()))
                }
            }
        }
        Err(_) => return Err(Error::CryptoError("Verification failed".to_string())),
    };
    debug!("VERIFIED, checking signed hash against mine");

    let computed_hash = try!(super::hash::hash_reader(&mut reader));

    debug!("Expected hash {}", expected_hash);
    debug!("My hash {}", computed_hash);
    if computed_hash == expected_hash {
        Ok((pair.name_with_rev(), expected_hash))
    } else {
        let msg = format!("Habitat artifact is invalid, \
                          hashes don't match (expected: {}, computed: {})",
                          expected_hash,
                          computed_hash);
        Err(Error::CryptoError(msg))
    }
}

pub fn artifact_signer<P: AsRef<Path>>(src: &P) -> Result<String> {
    let f = try!(File::open(src));
    let mut reader = BufReader::new(f);

    let _ = {
        let mut buffer = String::new();
        match reader.read_line(&mut buffer) {
            Ok(0) => {
                return Err(Error::CryptoError("Corrupt payload, can't read format version"
                                                  .to_string()))
            }
            Ok(_) => {
                if buffer.trim() != HART_FORMAT_VERSION {
                    let msg = format!("Unsupported format version: {}", &buffer.trim());
                    return Err(Error::CryptoError(msg));
                }
            }
            Err(e) => return Err(Error::from(e)),

        };
        buffer.trim().to_string()
    };
    let name_with_rev = {
        let mut buffer = String::new();
        if try!(reader.read_line(&mut buffer)) <= 0 {
            return Err(Error::CryptoError("Corrupt payload, can't read origin key name"
                                              .to_string()));
        }
        try!(parse_name_with_rev(buffer.trim()));
        buffer.trim().to_string()
    };
    Ok(name_with_rev)
}

#[cfg(test)]
mod test {
    use std::fs::{self, File};
    use std::io::{BufRead, BufReader, Read, Write};

    use tempdir::TempDir;

    use super::*;
    use super::super::SigKeyPair;
    use super::super::test_support::*;

    #[test]
    fn sign_and_verify() {
        let cache = TempDir::new("key_cache").unwrap();
        let pair = SigKeyPair::generate_pair_for_origin("unicorn", cache.path()).unwrap();
        let dst = cache.path().join("signed.dat");

        sign(&fixture("signme.dat"), &dst, &pair).unwrap();
        verify(&dst, cache.path()).unwrap();
        assert!(true);
    }

    #[test]
    #[should_panic(expected = "Secret key is required but not present for")]
    fn sign_missing_private_key() {
        let cache = TempDir::new("key_cache").unwrap();
        let pair = SigKeyPair::generate_pair_for_origin("unicorn", cache.path()).unwrap();
        let dst = cache.path().join("signed.dat");

        // Delete the secret key
        fs::remove_file(SigKeyPair::get_secret_key_path(&pair.name_with_rev(), cache.path())
                            .unwrap())
            .unwrap();
        // Now reload the key pair which will be missing the secret key
        let pair = SigKeyPair::get_latest_pair_for("unicorn", cache.path()).unwrap();

        sign(&fixture("signme.dat"), &dst, &pair).unwrap();
    }

    #[test]
    #[should_panic(expected = "Public key is required but not present for")]
    fn verify_missing_public_key() {
        let cache = TempDir::new("key_cache").unwrap();
        let pair = SigKeyPair::generate_pair_for_origin("unicorn", cache.path()).unwrap();
        let dst = cache.path().join("signed.dat");
        sign(&fixture("signme.dat"), &dst, &pair).unwrap();

        // Delete the public key
        fs::remove_file(SigKeyPair::get_public_key_path(&pair.name_with_rev(), cache.path())
                            .unwrap())
            .unwrap();
        // Now reload the key pair which will be missing the public key
        let _ = SigKeyPair::get_latest_pair_for("unicorn", cache.path()).unwrap();

        verify(&dst, cache.path()).unwrap();
    }

    #[test]
    #[should_panic(expected = "Corrupt payload, can\\'t read format version")]
    fn verify_empty_format_version() {
        let cache = TempDir::new("key_cache").unwrap();
        let dst = cache.path().join("signed.dat");
        let mut f = File::create(&dst).unwrap();
        f.write_all("".as_bytes()).unwrap();

        verify(&dst, cache.path()).unwrap();
    }

    #[test]
    #[should_panic(expected = "Unsupported format version: SOME-VERSION")]
    fn verify_invalid_format_version() {
        let cache = TempDir::new("key_cache").unwrap();
        let dst = cache.path().join("signed.dat");
        let mut f = File::create(&dst).unwrap();
        f.write_all("SOME-VERSION\nuhoh".as_bytes()).unwrap();

        verify(&dst, cache.path()).unwrap();
    }

    #[test]
    #[should_panic(expected = "parse_name_with_rev:1 Cannot parse")]
    fn verify_empty_key_name() {
        let cache = TempDir::new("key_cache").unwrap();
        let dst = cache.path().join("signed.dat");
        let mut f = File::create(&dst).unwrap();
        f.write_all("HART-1\n\nuhoh".as_bytes()).unwrap();

        verify(&dst, cache.path()).unwrap();
    }

    #[test]
    #[should_panic(expected = "parse_name_with_rev:1 Cannot parse")]
    fn verify_invalid_key_name() {
        let cache = TempDir::new("key_cache").unwrap();
        let dst = cache.path().join("signed.dat");
        let mut f = File::create(&dst).unwrap();
        f.write_all("HART-1\nnope-nope\nuhoh".as_bytes()).unwrap();

        verify(&dst, cache.path()).unwrap();
    }

    #[test]
    #[should_panic(expected = "Corrupt payload, can\\'t read hash type")]
    fn verify_empty_hash_type() {
        let cache = TempDir::new("key_cache").unwrap();
        let pair = SigKeyPair::generate_pair_for_origin("unicorn", cache.path()).unwrap();
        let dst = cache.path().join("signed.dat");
        let mut f = File::create(&dst).unwrap();
        f.write_all(format!("HART-1\n{}\n", pair.name_with_rev()).as_bytes()).unwrap();

        verify(&dst, cache.path()).unwrap();
    }

    #[test]
    #[should_panic(expected = "Unsupported signature type: BESTEST")]
    fn verify_invalid_hash_type() {
        let cache = TempDir::new("key_cache").unwrap();
        let pair = SigKeyPair::generate_pair_for_origin("unicorn", cache.path()).unwrap();
        let dst = cache.path().join("signed.dat");
        let mut f = File::create(&dst).unwrap();
        f.write_all(format!("HART-1\n{}\nBESTEST\nuhoh", pair.name_with_rev()).as_bytes()).unwrap();

        verify(&dst, cache.path()).unwrap();
    }

    #[test]
    #[should_panic(expected = "Corrupt payload, can\\'t read signature")]
    fn verify_empty_signature() {
        let cache = TempDir::new("key_cache").unwrap();
        let pair = SigKeyPair::generate_pair_for_origin("unicorn", cache.path()).unwrap();
        let dst = cache.path().join("signed.dat");
        let mut f = File::create(&dst).unwrap();
        f.write_all(format!("HART-1\n{}\nBLAKE2b\n", pair.name_with_rev()).as_bytes()).unwrap();

        verify(&dst, cache.path()).unwrap();
    }

    #[test]
    #[should_panic(expected = "Can\\'t decode signature")]
    fn verify_invalid_signature_decode() {
        let cache = TempDir::new("key_cache").unwrap();
        let pair = SigKeyPair::generate_pair_for_origin("unicorn", cache.path()).unwrap();
        let dst = cache.path().join("signed.dat");
        let mut f = File::create(&dst).unwrap();
        f.write_all(format!("HART-1\n{}\nBLAKE2b\nnot:base64:signature",
                            pair.name_with_rev())
                        .as_bytes())
         .unwrap();

        verify(&dst, cache.path()).unwrap();
    }

    #[test]
    #[should_panic(expected = "Corrupt payload, can\\'t find end of header")]
    fn verify_missing_end_of_header() {
        let cache = TempDir::new("key_cache").unwrap();
        let pair = SigKeyPair::generate_pair_for_origin("unicorn", cache.path()).unwrap();
        let dst = cache.path().join("signed.dat");
        let mut f = File::create(&dst).unwrap();
        f.write_all(format!("HART-1\n{}\nBLAKE2b\nbase64\n", pair.name_with_rev()).as_bytes())
         .unwrap();

        verify(&dst, cache.path()).unwrap();
    }

    #[test]
    #[should_panic(expected = "Habitat artifact is invalid")]
    fn verify_corrupted_archive() {
        let cache = TempDir::new("key_cache").unwrap();
        let pair = SigKeyPair::generate_pair_for_origin("unicorn", cache.path()).unwrap();
        let dst = cache.path().join("signed.dat");
        let dst_corrupted = cache.path().join("corrupted.dat");

        sign(&fixture("signme.dat"), &dst, &pair).unwrap();
        let mut corrupted = File::create(&dst_corrupted).unwrap();
        let f = File::open(&dst).unwrap();
        let f = BufReader::new(f);
        let mut lines = f.lines();
        corrupted.write(lines.next().unwrap().unwrap().as_bytes()).unwrap(); // version
        corrupted.write("\n".as_bytes()).unwrap();
        corrupted.write(lines.next().unwrap().unwrap().as_bytes()).unwrap(); // key
        corrupted.write("\n".as_bytes()).unwrap();
        corrupted.write(lines.next().unwrap().unwrap().as_bytes()).unwrap(); // hash type
        corrupted.write("\n".as_bytes()).unwrap();
        corrupted.write(lines.next().unwrap().unwrap().as_bytes()).unwrap(); // signature
        corrupted.write("\n\n".as_bytes()).unwrap();
        corrupted.write_all("payload-wont-match-signature".as_bytes()).unwrap(); // archive

        verify(&dst_corrupted, cache.path()).unwrap();
    }

    #[test]
    fn get_archive_reader_working() {
        let cache = TempDir::new("key_cache").unwrap();
        let pair = SigKeyPair::generate_pair_for_origin("unicorn", cache.path()).unwrap();
        let src = cache.path().join("src.in");
        let dst = cache.path().join("src.signed");
        let mut f = File::create(&src).unwrap();
        f.write_all("harty goodness".as_bytes()).unwrap();
        sign(&src, &dst, &pair).unwrap();

        let mut buffer = String::new();
        let mut reader = get_archive_reader(&dst).unwrap();
        reader.read_to_string(&mut buffer).unwrap();
        assert_eq!(buffer.as_bytes(), "harty goodness".as_bytes());
    }
}
