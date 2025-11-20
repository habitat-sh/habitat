use crate::{crypto::{Blake2bHash,
                     HART_FORMAT_VERSION,
                     SIG_HASH_TYPE,
                     keys::{Key,
                            KeyCache,
                            NamedRevision,
                            SecretOriginSigningKey}},
            error::{Error,
                    Result}};
use std::{fs::File,
          io::{self,
               BufRead,
               BufReader,
               BufWriter,
               prelude::*},
          path::Path};

pub struct ArtifactHeader {
    format:    String,
    signer:    NamedRevision,
    hash_type: String,
    signature: Vec<u8>,
}

impl ArtifactHeader {
    pub fn format(&self) -> &String { &self.format }

    pub fn signer(&self) -> &NamedRevision { &self.signer }

    pub fn hash_type(&self) -> &String { &self.hash_type }

    /// Provide the signature as a base64-encoded string. This is how
    /// the signature appears in a HART file header, and is the most
    /// convenient form for passing around to external software.
    pub fn encoded_signature(&self) -> String { crate::base64::encode(&self.signature) }
}

/// Generate and sign a package
pub fn sign<P1, P2>(src: &P1, dst: &P2, key: &SecretOriginSigningKey) -> Result<()>
    where P1: ?Sized + AsRef<Path>,
          P2: ?Sized + AsRef<Path>
{
    let signature = key.sign(src)?;
    let output_file = File::create(dst)?;
    let mut writer = BufWriter::new(&output_file);
    write!(writer,
           "{}\n{}\n{}\n{}\n\n",
           HART_FORMAT_VERSION,
           key.named_revision(),
           SIG_HASH_TYPE,
           crate::base64::encode(signature))?;
    let mut file = File::open(src)?;
    io::copy(&mut file, &mut writer)?;
    Ok(())
}

/// return a BufReader to the .tar bytestream, skipping the signed header
pub fn get_archive_reader<P>(src: P) -> Result<impl BufRead>
    where P: AsRef<Path>
{
    let (_header, reader) = artifact_header_and_archive(src)?;
    Ok(reader)
}

/// Read only the header of the artifact, fails if any of the components
/// are invalid/missing. Each component of the header has it's whitespace
/// stripped before returning in an `ArtifactHeader` struct
pub fn get_artifact_header<P>(src: P) -> Result<ArtifactHeader>
    where P: AsRef<Path>
{
    let (header, _reader) = artifact_header_and_archive(src)?;
    Ok(header)
}

fn artifact_header_and_archive<P>(path: P) -> Result<(ArtifactHeader, impl BufRead)>
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
        crate::base64::decode(line).map_err(|e| {
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
    let header = ArtifactHeader { format,
                                  signer: named_revision,
                                  hash_type,
                                  signature };

    Ok((header, reader))
}

/// Returns a tuple of the `NamedRevision` of the key that verified
/// the `.hart` file, along with the Blake2b hash of its contents.
pub fn verify<P>(hart_file_path: P, cache: &KeyCache) -> Result<(NamedRevision, Blake2bHash)>
    where P: AsRef<Path>
{
    let (header, mut reader) = artifact_header_and_archive(hart_file_path)?;
    let key = cache.public_signing_key(&header.signer)?;
    let hash = key.verify(header.signature.as_slice(), &mut reader)?;
    Ok((key.named_revision().clone(), hash))
}

/// Parse a HART file (referred to by filesystem path) to discover the
/// signing key revision that was used to sign it.
pub fn artifact_signer<P>(hart_file_path: P) -> Result<NamedRevision>
    where P: AsRef<Path>
{
    let (header, _reader) = artifact_header_and_archive(hart_file_path)?;
    Ok(header.signer)
}

#[cfg(test)]
mod test {
    use super::{super::{HART_FORMAT_VERSION,
                        SIG_HASH_TYPE,
                        test_support::*},
                *};

    #[test]
    #[should_panic(expected = "Corrupt payload, can't read format version")]
    fn verify_empty_format_version() {
        let (cache, dir) = new_cache();

        let dst = dir.path().join("signed.dat");
        let mut f = File::create(&dst).unwrap();
        f.write_all(b"").unwrap();

        verify(&dst, &cache).unwrap();
    }

    #[test]
    #[should_panic(expected = "Unsupported format version: SOME-VERSION")]
    fn verify_invalid_format_version() {
        let (cache, dir) = new_cache();

        let dst = dir.path().join("signed.dat");
        let mut f = File::create(&dst).unwrap();
        f.write_all(b"SOME-VERSION\nuhoh").unwrap();

        verify(&dst, &cache).unwrap();
    }

    #[test]
    #[should_panic(expected = "Cannot parse named revision ''")]
    fn verify_empty_key_name() {
        let (cache, dir) = new_cache();

        let dst = dir.path().join("signed.dat");
        let mut f = File::create(&dst).unwrap();
        f.write_all(b"HART-1\n\nuhoh").unwrap();

        verify(&dst, &cache).unwrap();
    }

    #[test]
    #[should_panic(expected = "Cannot parse named revision 'nope-nope'")]
    fn verify_invalid_key_name() {
        let (cache, dir) = new_cache();

        let dst = dir.path().join("signed.dat");
        let mut f = File::create(&dst).unwrap();
        f.write_all(b"HART-1\nnope-nope\nuhoh").unwrap();

        verify(&dst, &cache).unwrap();
    }

    #[test]
    #[should_panic(expected = "Corrupt payload, can't read hash type")]
    fn verify_empty_hash_type() {
        let (cache, dir) = new_cache();
        let origin = "unicorn".parse().unwrap();
        let (public, _secret) = cache.new_signing_pair(&origin).unwrap();

        let dst = dir.path().join("signed.dat");
        let mut f = File::create(&dst).unwrap();
        f.write_all(format!("HART-1\n{}\n", public.named_revision()).as_bytes())
         .unwrap();

        verify(&dst, &cache).unwrap();
    }

    #[test]
    #[should_panic(expected = "Unsupported signature type: BESTEST")]
    fn verify_invalid_hash_type() {
        let (cache, dir) = new_cache();
        let origin = "unicorn".parse().unwrap();
        let (public, _secret) = cache.new_signing_pair(&origin).unwrap();

        let dst = dir.path().join("signed.dat");
        let mut f = File::create(&dst).unwrap();
        f.write_all(format!("HART-1\n{}\nBESTEST\nuhoh", public.named_revision()).as_bytes())
         .unwrap();

        verify(&dst, &cache).unwrap();
    }

    #[test]
    #[should_panic(expected = "Corrupt payload, can't read signature")]
    fn verify_empty_signature() {
        let (cache, dir) = new_cache();
        let origin = "unicorn".parse().unwrap();
        let (public, _secret) = cache.new_signing_pair(&origin).unwrap();

        let dst = dir.path().join("signed.dat");
        let mut f = File::create(&dst).unwrap();
        f.write_all(format!("HART-1\n{}\nBLAKE2b\n", public.named_revision()).as_bytes())
         .unwrap();

        verify(&dst, &cache).unwrap();
    }

    #[test]
    #[should_panic(expected = "Can't decode signature")]
    fn verify_invalid_signature_decode() {
        let (cache, dir) = new_cache();
        let origin = "unicorn".parse().unwrap();
        let (public, _secret) = cache.new_signing_pair(&origin).unwrap();

        let dst = dir.path().join("signed.dat");
        let mut f = File::create(&dst).unwrap();
        f.write_all(format!("HART-1\n{}\nBLAKE2b\nnot:base64:signature",
                            public.named_revision()).as_bytes())
         .unwrap();

        verify(&dst, &cache).unwrap();
    }

    #[test]
    #[should_panic(expected = "Corrupt payload, can't find end of header")]
    fn verify_missing_end_of_header() {
        let (cache, dir) = new_cache();
        let origin = "unicorn".parse().unwrap();
        let (public, _secret) = cache.new_signing_pair(&origin).unwrap();

        let dst = dir.path().join("signed.dat");
        let mut f = File::create(&dst).unwrap();
        f.write_all(format!("HART-1\n{}\nBLAKE2b\nU3VycHJpc2Uh\n",
                            public.named_revision()).as_bytes())
         .unwrap();

        verify(&dst, &cache).unwrap();
    }

    #[test]
    #[should_panic(expected = "Habitat artifact is invalid")]
    fn verify_corrupted_archive() {
        let (cache, dir) = new_cache();
        let origin = "unicorn".parse().unwrap();
        let (_public, secret) = cache.new_signing_pair(&origin).unwrap();

        let dst = dir.path().join("signed.dat");
        let dst_corrupted = dir.path().join("corrupted.dat");

        sign(&fixture("signme.dat"), &dst, &secret).unwrap();
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

        verify(&dst_corrupted, &cache).unwrap();
    }

    #[test]
    fn get_archive_reader_working() {
        let (cache, dir) = new_cache();
        let origin = "unicorn".parse().unwrap();
        let (_public, secret) = cache.new_signing_pair(&origin).unwrap();

        let src = dir.path().join("src.in");
        let dst = dir.path().join("src.signed");
        let mut f = File::create(&src).unwrap();
        f.write_all(b"hearty goodness").unwrap();
        sign(&src, &dst, &secret).unwrap();

        let mut buffer = String::new();
        let mut reader = get_archive_reader(&dst).unwrap();
        reader.read_to_string(&mut buffer).unwrap();
        assert_eq!(buffer.as_bytes(), b"hearty goodness");
    }

    #[test]
    fn verify_get_artifact_header() {
        let (cache, dir) = new_cache();
        let origin = "unicorn".parse().unwrap();
        let (_public, secret) = cache.new_signing_pair(&origin).unwrap();

        let src = dir.path().join("src.in");
        let dst = dir.path().join("src.signed");
        let mut f = File::create(&src).unwrap();
        f.write_all(b"hearty goodness").unwrap();
        sign(&src, &dst, &secret).unwrap();

        let hart_header = get_artifact_header(&dst).unwrap();
        assert_eq!(HART_FORMAT_VERSION, hart_header.format());
        assert_eq!("unicorn", hart_header.signer().name());
        assert_eq!(SIG_HASH_TYPE, hart_header.hash_type());
        assert!(!hart_header.encoded_signature().is_empty());
    }

    mod artifact_header {
        use super::*;

        #[test]
        fn get_artifact_header_works() {
            let hart_path = fixture("happyhumans-possums-8.1.4-20160427165340-x86_64-linux.hart");
            let header = get_artifact_header(hart_path).unwrap();

            assert_eq!(header.format(), "HART-1");
            assert_eq!(header.signer().to_string(), "happyhumans-20160424223347");
            assert_eq!(header.hash_type(), "BLAKE2b");
            assert_eq!(header.encoded_signature(),
                       "U0cp/+npru0ZxhK76zm+PDVSV/707siyrO1r7T6CZZ4ShSLrIxyx8jLSMr5wnLuGrVIV358smQPWOSTOmyfFCjBmMmM1ZjRkZTE0NWM3Zjc4NjAxY2FhZTljN2I4NzY3MDk4NDEzZDA1NzM5ZGU5MTNjMDEyOTIyYjdlZWQ3NjA=");
        }
    }

    mod artifact_signer {
        use super::*;

        #[test]
        fn get_named_revision_from_artifact() {
            let hart_path = fixture("happyhumans-possums-8.1.4-20160427165340-x86_64-linux.hart");
            let signer = artifact_signer(hart_path).unwrap();
            let expected: NamedRevision = "happyhumans-20160424223347".parse().unwrap();
            assert_eq!(signer, expected);
        }

        #[test]
        #[should_panic(expected = "Cannot parse named revision")]
        fn fails_on_invalid_hart() {
            // Not really a HART file, but has enough of a header to
            // be parsed by `artifact_signer`. It has an invalid
            // signing key identifier.
            let hart_path = fixture("bogus_and_corrupt.hart");
            artifact_signer(hart_path).unwrap();
        }
    }
}
