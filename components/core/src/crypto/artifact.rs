use super::{hash,
            SigKeyPair,
            HART_FORMAT_VERSION,
            SIG_HASH_TYPE};
use crate::{crypto::keys::NamedRevision,
            error::{Error,
                    Result}};
use sodiumoxide::crypto::sign;
use std::{fs::File,
          io::{self,
               prelude::*,
               BufReader,
               BufWriter},
          path::Path};

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
pub fn get_archive_reader<P>(src: P) -> Result<BufReader<File>>
    where P: AsRef<Path>
{
    let (_header, reader) = artifact_header_and_archive(src)?;
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
pub fn get_artifact_header<P>(src: P) -> Result<ArtifactHeader>
    where P: AsRef<Path>
{
    let (artifact, _reader) = artifact_header_and_archive(src)?;
    Ok(artifact.into())
}

struct ArtifactHeaderBetter {
    format:    String,
    signer:    NamedRevision,
    hash_type: String,
    signature: Vec<u8>,
}

// TODO (CM): Ideally, ArtifactHeaderBetter would *be*
// ArtifactHeader, but for now, this helps bridge the gap.
impl Into<ArtifactHeader> for ArtifactHeaderBetter {
    fn into(self) -> ArtifactHeader {
        ArtifactHeader::new(self.format,
                            self.signer.to_string(),
                            self.hash_type,
                            base64::encode(self.signature))
    }
}

fn artifact_header_and_archive<P>(path: P) -> Result<(ArtifactHeaderBetter, BufReader<File>)>
    where P: AsRef<Path>
{
    let f = File::open(path)?;
    let mut reader = BufReader::new(f);

    // First line is HART format line.
    let mut line = String::new();
    let format = if reader.read_line(&mut line)? == 0 {
        Err(Error::CryptoError("Corrupt payload, can't read format \
                                version"
                                        .to_string()))
    } else {
        let line = line.trim();
        if line != HART_FORMAT_VERSION {
            Err(Error::CryptoError(format!("Unsupported format version: \
                                            {}",
                                           line)))
        } else {
            Ok(line.to_string())
        }
    }?;

    // Second line is the revision of the signing key used.
    let mut line = String::new();
    let named_revision = if reader.read_line(&mut line)? == 0 {
        Err(Error::CryptoError("Corrupt payload, can't read origin \
                                key name"
                                         .to_string()))
    } else {
        let line = line.trim();
        line.parse::<NamedRevision>()
    }?;

    // Third line is the hash type of the signature.
    let mut line = String::new();
    let hash_type = if reader.read_line(&mut line)? == 0 {
        Err(Error::CryptoError("Corrupt payload, can't read hash type".to_string()))
    } else {
        let line = line.trim();
        if line != SIG_HASH_TYPE {
            Err(Error::CryptoError(format!("Unsupported signature type: \
                                            {}",
                                           line)))
        } else {
            Ok(line.to_string())
        }
    }?;

    // Fourth line is the base64-encoded signature.
    let mut line = String::new();
    let signature = if reader.read_line(&mut line)? == 0 {
        Err(Error::CryptoError("Corrupt payload, can't read signature".to_string()))
    } else {
        let line = line.trim();
        base64::decode(line).map_err(|e| {
                                Error::CryptoError(format!("Can't decode signature: {}", e))
                            })
    }?;

    // Fifth line should be an empty delimiter line.
    let mut line = String::new();
    if reader.read_line(&mut line)? == 0 {
        Err(Error::CryptoError("Corrupt payload, can't find end of \
                                header"
                                       .to_string()))
    } else {
        let line = line.trim();
        if !line.is_empty() {
            Err(Error::CryptoError(format!("Expected empty delimiter \
                                            line in header; got '{}'",
                                           line)))
        } else {
            Ok(())
        }
    }?;

    // The rest of the file will be the compressed tarball of the
    // archive. We'll return the reader as a pointer to that segment
    // of the file for further processing (either signature
    // verification or decompression).
    let header = ArtifactHeaderBetter { format,
                                        signer: named_revision,
                                        hash_type,
                                        signature };

    Ok((header, reader))
}

pub fn verify<P1: ?Sized, P2: ?Sized>(src: &P1, cache_key_path: &P2) -> Result<(String, String)>
    where P1: AsRef<Path>,
          P2: AsRef<Path>
{
    let (header, mut reader) = artifact_header_and_archive(src)?;

    // TODO (CM): We need to be passing the public key into this
    // function, not the cache path.
    let pair = SigKeyPair::get_pair_for(&header.signer.to_string(), cache_key_path.as_ref())?;

    let expected_hash = match sign::verify(header.signature.as_slice(), pair.public()?) {
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

/// Parse a HART file (referred to by filesystem path) to discover the
/// signing key revision that was used to sign it.
pub fn artifact_signer<P>(hart_file_path: P) -> Result<String>
    where P: AsRef<Path>
{
    let (header, _reader) = artifact_header_and_archive(hart_file_path)?;
    // TODO (CM): Eventually, return NamedRevision
    Ok(header.signer.to_string())
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
    #[should_panic(expected = "Cannot parse named revision \\'\\'")]
    fn verify_empty_key_name() {
        let cache = Builder::new().prefix("key_cache").tempdir().unwrap();
        let dst = cache.path().join("signed.dat");
        let mut f = File::create(&dst).unwrap();
        f.write_all(b"HART-1\n\nuhoh").unwrap();

        verify(&dst, cache.path()).unwrap();
    }

    #[test]
    #[should_panic(expected = "Cannot parse named revision \\'nope-nope\\'")]
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

    mod artifact_header {
        use super::*;

        #[test]
        fn get_artifact_header_works() {
            let hart_path = fixture("happyhumans-possums-8.1.4-20160427165340-x86_64-linux.hart");
            let header = get_artifact_header(&hart_path).unwrap();

            assert_eq!(header.format_version, "HART-1");
            assert_eq!(header.key_name, "happyhumans-20160424223347");
            assert_eq!(header.hash_type, "BLAKE2b");
            assert_eq!(header.signature_raw,
                       "U0cp/+npru0ZxhK76zm+PDVSV/707siyrO1r7T6CZZ4ShSLrIxyx8jLSMr5wnLuGrVIV358smQPWOSTOmyfFCjBmMmM1ZjRkZTE0NWM3Zjc4NjAxY2FhZTljN2I4NzY3MDk4NDEzZDA1NzM5ZGU5MTNjMDEyOTIyYjdlZWQ3NjA=");
        }
    }

    mod artifact_signer {
        use super::*;

        #[test]
        fn get_named_revision_from_artifact() {
            let hart_path = fixture("happyhumans-possums-8.1.4-20160427165340-x86_64-linux.hart");
            let signer = artifact_signer(&hart_path).unwrap();
            let expected: NamedRevision = "happyhumans-20160424223347".parse().unwrap();
            assert_eq!(signer, expected.to_string());
        }

        #[test]
        #[should_panic(expected = "Cannot parse named revision")]
        fn fails_on_invalid_hart() {
            // Not really a HART file, but has enough of a header to
            // be parsed by `artifact_signer`. It has an invalid
            // signing key identifier.
            let hart_path = fixture("bogus_and_corrupt.hart");
            artifact_signer(&hart_path).unwrap();
        }
    }
}
