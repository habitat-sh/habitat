use std::{fs::File,
          io::{self,
               prelude::*,
               BufReader,
               BufWriter},
          path::Path};

use base64;
use sodiumoxide::crypto::sign;

use super::{hash,
            keys::parse_name_with_rev,
            SigKeyPair,
            HART_FORMAT_VERSION,
            SIG_HASH_TYPE};
use crate::error::{Error,
                   Result};

/// Generate and sign a package
pub fn sign<P1: ?Sized, P2: ?Sized>(src: &P1, dst: &P2, pair: &SigKeyPair) -> Result<()>
    where P1: AsRef<Path>,
          P2: AsRef<Path>
{
    let hash = hash::hash_file(&src)?;
    debug!("File hash for {} = {}", src.as_ref().display(), &hash);

    let signature = sign::sign(&hash.as_bytes(), pair.secret()?);
    let output_file = File::create(dst)?;
    let mut writer = BufWriter::new(&output_file);
    write!(writer,
           "{}\n{}\n{}\n{}\n\n",
           HART_FORMAT_VERSION,
           pair.name_with_rev(),
           SIG_HASH_TYPE,
           base64::encode(&signature))?;
    let mut file = File::open(src)?;
    io::copy(&mut file, &mut writer)?;
    Ok(())
}

/// return a BufReader to the .tar bytestream, skipping the signed header
pub fn get_archive_reader<P: AsRef<Path>>(src: P) -> Result<BufReader<File>> {
    let f = File::open(src)?;
    let mut your_format_version = String::new();
    let mut your_key_name = String::new();
    let mut your_hash_type = String::new();
    let mut your_signature_raw = String::new();
    let mut empty_line = String::new();

    let mut reader = BufReader::new(f);
    if reader.read_line(&mut your_format_version)? == 0 {
        return Err(Error::CryptoError("Can't read format version".to_string()));
    }
    if reader.read_line(&mut your_key_name)? == 0 {
        return Err(Error::CryptoError("Can't read keyname".to_string()));
    }
    if reader.read_line(&mut your_hash_type)? == 0 {
        return Err(Error::CryptoError("Can't read hash type".to_string()));
    }
    if reader.read_line(&mut your_signature_raw)? == 0 {
        return Err(Error::CryptoError("Can't read signature".to_string()));
    }
    if reader.read_line(&mut empty_line)? == 0 {
        return Err(Error::CryptoError("Can't end of header".to_string()));
    }
    Ok(reader)
}

pub struct ArtifactHeader {
    pub format_version: String,
    pub key_name:       String,
    pub hash_type:      String,
    pub signature_raw:  String,
}

impl ArtifactHeader {
    pub fn new(format_version: String,
               key_name: String,
               hash_type: String,
               signature_raw: String)
               -> ArtifactHeader {
        ArtifactHeader { format_version,
                         key_name,
                         hash_type,
                         signature_raw }
    }
}

/// Read only the header of the artifact, fails if any of the components
/// are invalid/missing. Each component of the header has it's whitespace
/// stripped before returning in an `ArtifactHeader` struct
pub fn get_artifact_header<P: ?Sized>(src: &P) -> Result<ArtifactHeader>
    where P: AsRef<Path>
{
    let f = File::open(src)?;
    let mut your_format_version = String::new();
    let mut your_key_name = String::new();
    let mut your_hash_type = String::new();
    let mut your_signature_raw = String::new();
    let mut empty_line = String::new();

    let mut reader = BufReader::new(f);
    if reader.read_line(&mut your_format_version)? == 0 {
        return Err(Error::CryptoError("Can't read format version".to_string()));
    }
    if reader.read_line(&mut your_key_name)? == 0 {
        return Err(Error::CryptoError("Can't read keyname".to_string()));
    }
    if reader.read_line(&mut your_hash_type)? == 0 {
        return Err(Error::CryptoError("Can't read hash type".to_string()));
    }
    if reader.read_line(&mut your_signature_raw)? == 0 {
        return Err(Error::CryptoError("Can't read signature".to_string()));
    }
    if reader.read_line(&mut empty_line)? == 0 {
        return Err(Error::CryptoError("Can't end of header".to_string()));
    }
    let your_format_version = your_format_version.trim().to_string();
    let your_key_name = your_key_name.trim().to_string();
    let your_hash_type = your_hash_type.trim().to_string();
    let your_signature_raw = your_signature_raw.trim().to_string();

    Ok(ArtifactHeader::new(your_format_version,
                           your_key_name,
                           your_hash_type,
                           your_signature_raw))
}

/// verify the crypto signature of a .hart file
pub fn verify<P1: ?Sized, P2: ?Sized>(src: &P1, cache_key_path: &P2) -> Result<(String, String)>
    where P1: AsRef<Path>,
          P2: AsRef<Path>
{
    let f = File::open(src)?;
    let mut reader = BufReader::new(f);

    let _ = {
        let mut buffer = String::new();
        match reader.read_line(&mut buffer) {
            Ok(0) => {
                return Err(Error::CryptoError("Corrupt payload, can't read format \
                                               version"
                                                       .to_string()));
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
        if reader.read_line(&mut buffer)? == 0 {
            return Err(Error::CryptoError("Corrupt payload, can't read origin \
                                           key name"
                                                    .to_string()));
        }
        SigKeyPair::get_pair_for(buffer.trim(), cache_key_path)?
    };
    {
        let mut buffer = String::new();
        match reader.read_line(&mut buffer) {
            Ok(0) => {
                return Err(Error::CryptoError(
                    "Corrupt payload, can't read hash type".to_string(),
                ));
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
                return Err(Error::CryptoError(
                    "Corrupt payload, can't read signature".to_string(),
                ));
            }
            Ok(_) => {
                base64::decode(buffer.trim()).map_err(|e| {
                                                 Error::CryptoError(format!("Can't decode \
                                                                             signature: {}",
                                                                            e))
                                             })?
            }
            Err(e) => return Err(Error::from(e)),
        }
    };
    {
        let mut buffer = String::new();
        if reader.read_line(&mut buffer)? == 0 {
            return Err(Error::CryptoError("Corrupt payload, can't find end of \
                                           header"
                                                  .to_string()));
        }
    };
    let expected_hash = match sign::verify(signature.as_slice(), pair.public()?) {
        Ok(signed_data) => String::from_utf8(signed_data).map_err(|_| {
                               Error::CryptoError("Error parsing artifact signature".to_string())
                           })?,
        Err(_) => return Err(Error::CryptoError("Verification failed".to_string())),
    };
    let computed_hash = hash::hash_reader(&mut reader)?;
    if computed_hash == expected_hash {
        Ok((pair.name_with_rev(), expected_hash))
    } else {
        let msg = format!("Habitat artifact is invalid, hashes don't match (expected: {}, \
                           computed: {})",
                          expected_hash, computed_hash);
        Err(Error::CryptoError(msg))
    }
}

pub fn artifact_signer<P: AsRef<Path>>(src: &P) -> Result<String> {
    let f = File::open(src)?;
    let mut reader = BufReader::new(f);

    let _ = {
        let mut buffer = String::new();
        match reader.read_line(&mut buffer) {
            Ok(0) => {
                return Err(Error::CryptoError("Corrupt payload, can't read format \
                                               version"
                                                       .to_string()));
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
        if reader.read_line(&mut buffer)? == 0 {
            return Err(Error::CryptoError("Corrupt payload, can't read origin \
                                           key name"
                                                    .to_string()));
        }
        parse_name_with_rev(buffer.trim())?;
        buffer.trim().to_string()
    };
    Ok(name_with_rev)
}

#[cfg(test)]
mod test {
    use std::{fs::{self,
                   File},
              io::{BufRead,
                   BufReader,
                   Read,
                   Write}};

    use tempfile::Builder;

    use super::{super::{keys::parse_name_with_rev,
                        test_support::*,
                        SigKeyPair,
                        HART_FORMAT_VERSION,
                        SIG_HASH_TYPE},
                *};

    #[test]
    fn sign_and_verify() {
        let cache = Builder::new().prefix("key_cache").tempdir().unwrap();
        let pair = SigKeyPair::generate_pair_for_origin("unicorn");
        pair.to_pair_files(cache.path()).unwrap();
        let dst = cache.path().join("signed.dat");

        sign(&fixture("signme.dat"), &dst, &pair).unwrap();
        verify(&dst, cache.path()).unwrap();
    }

    #[test]
    #[should_panic(expected = "Secret key is required but not present for")]
    fn sign_missing_private_key() {
        let cache = Builder::new().prefix("key_cache").tempdir().unwrap();
        let pair = SigKeyPair::generate_pair_for_origin("unicorn");
        pair.to_pair_files(cache.path()).unwrap();
        let dst = cache.path().join("signed.dat");

        // Delete the secret key
        fs::remove_file(
            SigKeyPair::get_secret_key_path(&pair.name_with_rev(), cache.path()).unwrap(),
        )
        .unwrap();
        // Now reload the key pair which will be missing the secret key
        let pair = SigKeyPair::get_latest_pair_for("unicorn", cache.path(), None).unwrap();

        sign(&fixture("signme.dat"), &dst, &pair).unwrap();
    }

    #[test]
    #[should_panic(expected = "Public key is required but not present for")]
    fn verify_missing_public_key() {
        let cache = Builder::new().prefix("key_cache").tempdir().unwrap();
        let pair = SigKeyPair::generate_pair_for_origin("unicorn");
        pair.to_pair_files(cache.path()).unwrap();
        let dst = cache.path().join("signed.dat");
        sign(&fixture("signme.dat"), &dst, &pair).unwrap();

        // Delete the public key
        fs::remove_file(
            SigKeyPair::get_public_key_path(&pair.name_with_rev(), cache.path()).unwrap(),
        )
        .unwrap();
        // Now reload the key pair which will be missing the public key
        let _ = SigKeyPair::get_latest_pair_for("unicorn", cache.path(), None).unwrap();

        verify(&dst, cache.path()).unwrap();
    }

    #[test]
    #[should_panic(expected = "Corrupt payload, can\\'t read format version")]
    fn verify_empty_format_version() {
        let cache = Builder::new().prefix("key_cache").tempdir().unwrap();
        let dst = cache.path().join("signed.dat");
        let mut f = File::create(&dst).unwrap();
        f.write_all(b"").unwrap();

        verify(&dst, cache.path()).unwrap();
    }

    #[test]
    #[should_panic(expected = "Unsupported format version: SOME-VERSION")]
    fn verify_invalid_format_version() {
        let cache = Builder::new().prefix("key_cache").tempdir().unwrap();
        let dst = cache.path().join("signed.dat");
        let mut f = File::create(&dst).unwrap();
        f.write_all(b"SOME-VERSION\nuhoh").unwrap();

        verify(&dst, cache.path()).unwrap();
    }

    #[test]
    #[should_panic(expected = "parse_name_with_rev:1 Cannot parse")]
    fn verify_empty_key_name() {
        let cache = Builder::new().prefix("key_cache").tempdir().unwrap();
        let dst = cache.path().join("signed.dat");
        let mut f = File::create(&dst).unwrap();
        f.write_all(b"HART-1\n\nuhoh").unwrap();

        verify(&dst, cache.path()).unwrap();
    }

    #[test]
    #[should_panic(expected = "parse_name_with_rev:1 Cannot parse")]
    fn verify_invalid_key_name() {
        let cache = Builder::new().prefix("key_cache").tempdir().unwrap();
        let dst = cache.path().join("signed.dat");
        let mut f = File::create(&dst).unwrap();
        f.write_all(b"HART-1\nnope-nope\nuhoh").unwrap();

        verify(&dst, cache.path()).unwrap();
    }

    #[test]
    #[should_panic(expected = "Corrupt payload, can\\'t read hash type")]
    fn verify_empty_hash_type() {
        let cache = Builder::new().prefix("key_cache").tempdir().unwrap();
        let pair = SigKeyPair::generate_pair_for_origin("unicorn");
        pair.to_pair_files(cache.path()).unwrap();
        let dst = cache.path().join("signed.dat");
        let mut f = File::create(&dst).unwrap();
        f.write_all(format!("HART-1\n{}\n", pair.name_with_rev()).as_bytes())
         .unwrap();

        verify(&dst, cache.path()).unwrap();
    }

    #[test]
    #[should_panic(expected = "Unsupported signature type: BESTEST")]
    fn verify_invalid_hash_type() {
        let cache = Builder::new().prefix("key_cache").tempdir().unwrap();
        let pair = SigKeyPair::generate_pair_for_origin("unicorn");
        pair.to_pair_files(cache.path()).unwrap();
        let dst = cache.path().join("signed.dat");
        let mut f = File::create(&dst).unwrap();
        f.write_all(format!("HART-1\n{}\nBESTEST\nuhoh", pair.name_with_rev()).as_bytes())
         .unwrap();

        verify(&dst, cache.path()).unwrap();
    }

    #[test]
    #[should_panic(expected = "Corrupt payload, can\\'t read signature")]
    fn verify_empty_signature() {
        let cache = Builder::new().prefix("key_cache").tempdir().unwrap();
        let pair = SigKeyPair::generate_pair_for_origin("unicorn");
        pair.to_pair_files(cache.path()).unwrap();
        let dst = cache.path().join("signed.dat");
        let mut f = File::create(&dst).unwrap();
        f.write_all(format!("HART-1\n{}\nBLAKE2b\n", pair.name_with_rev()).as_bytes())
         .unwrap();

        verify(&dst, cache.path()).unwrap();
    }

    #[test]
    #[should_panic(expected = "Can\\'t decode signature")]
    fn verify_invalid_signature_decode() {
        let cache = Builder::new().prefix("key_cache").tempdir().unwrap();
        let pair = SigKeyPair::generate_pair_for_origin("unicorn");
        pair.to_pair_files(cache.path()).unwrap();
        let dst = cache.path().join("signed.dat");
        let mut f = File::create(&dst).unwrap();
        f.write_all(format!("HART-1\n{}\nBLAKE2b\nnot:base64:signature",
                            pair.name_with_rev()).as_bytes())
         .unwrap();

        verify(&dst, cache.path()).unwrap();
    }

    #[test]
    #[should_panic(expected = "Corrupt payload, can\\'t find end of header")]
    fn verify_missing_end_of_header() {
        let cache = Builder::new().prefix("key_cache").tempdir().unwrap();
        let pair = SigKeyPair::generate_pair_for_origin("unicorn");
        pair.to_pair_files(cache.path()).unwrap();
        let dst = cache.path().join("signed.dat");
        let mut f = File::create(&dst).unwrap();
        f.write_all(
            format!("HART-1\n{}\nBLAKE2b\nU3VycHJpc2Uh\n", pair.name_with_rev()).as_bytes(),
        )
        .unwrap();

        verify(&dst, cache.path()).unwrap();
    }

    #[test]
    #[should_panic(expected = "Habitat artifact is invalid")]
    fn verify_corrupted_archive() {
        let cache = Builder::new().prefix("key_cache").tempdir().unwrap();
        let pair = SigKeyPair::generate_pair_for_origin("unicorn");
        pair.to_pair_files(cache.path()).unwrap();
        let dst = cache.path().join("signed.dat");
        let dst_corrupted = cache.path().join("corrupted.dat");

        sign(&fixture("signme.dat"), &dst, &pair).unwrap();
        let mut corrupted = File::create(&dst_corrupted).unwrap();
        let f = File::open(&dst).unwrap();
        let f = BufReader::new(f);
        let mut lines = f.lines();
        corrupted.write_all(lines.next().unwrap().unwrap().as_bytes())
                 .unwrap(); // version
        corrupted.write_all(b"\n").unwrap();
        corrupted.write_all(lines.next().unwrap().unwrap().as_bytes())
                 .unwrap(); // key
        corrupted.write_all(b"\n").unwrap();
        corrupted.write_all(lines.next().unwrap().unwrap().as_bytes())
                 .unwrap(); // hash type
        corrupted.write_all(b"\n").unwrap();
        corrupted.write_all(lines.next().unwrap().unwrap().as_bytes())
                 .unwrap(); // signature
        corrupted.write_all(b"\n\n").unwrap();
        corrupted.write_all(b"payload-wont-match-signature")
                 .unwrap(); // archive

        verify(&dst_corrupted, cache.path()).unwrap();
    }

    #[test]
    fn get_archive_reader_working() {
        let cache = Builder::new().prefix("key_cache").tempdir().unwrap();
        let pair = SigKeyPair::generate_pair_for_origin("unicorn");
        pair.to_pair_files(cache.path()).unwrap();
        let src = cache.path().join("src.in");
        let dst = cache.path().join("src.signed");
        let mut f = File::create(&src).unwrap();
        f.write_all(b"hearty goodness").unwrap();
        sign(&src, &dst, &pair).unwrap();

        let mut buffer = String::new();
        let mut reader = get_archive_reader(&dst).unwrap();
        reader.read_to_string(&mut buffer).unwrap();
        assert_eq!(buffer.as_bytes(), b"hearty goodness");
    }

    #[test]
    fn verify_get_artifact_header() {
        let cache = Builder::new().prefix("key_cache").tempdir().unwrap();
        let pair = SigKeyPair::generate_pair_for_origin("unicorn");
        pair.to_pair_files(cache.path()).unwrap();
        let src = cache.path().join("src.in");
        let dst = cache.path().join("src.signed");
        let mut f = File::create(&src).unwrap();
        f.write_all(b"hearty goodness").unwrap();
        sign(&src, &dst, &pair).unwrap();

        let hart_header = get_artifact_header(&dst).unwrap();
        assert_eq!(HART_FORMAT_VERSION, hart_header.format_version);
        let (key_name, _rev) = parse_name_with_rev(&hart_header.key_name).unwrap();
        assert_eq!("unicorn", key_name);
        assert_eq!(SIG_HASH_TYPE, hart_header.hash_type);
        assert!(!hart_header.signature_raw.is_empty());
    }
}
