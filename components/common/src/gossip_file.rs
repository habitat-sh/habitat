// Copyright (c) 2016 Chef Software Inc. and/or applicable contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std;
use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use hcore::crypto::{BoxKeyPair, default_cache_key_path};
use hcore::fs;
use hcore::service::ServiceGroup;
use hcore::util::perm::{set_owner_and_group, set_permissions};
use openssl::crypto::hash as openssl_hash;
use rustc_serialize::{Encodable, Encoder};
use rustc_serialize::hex::ToHex;
use time::{SteadyTime, Duration};
use toml;
use error::{Error, Result};

const IDEMPOTENCY_INTERVAL_MINUTES: i64 = 5;

pub const GOSSIP_TOML: &'static str = "gossip.toml";

pub const UPLOADED_FILE_PERMISSIONS: &'static str = "0770";

/// The gossip file struct.
#[derive(Clone, Debug, Eq, RustcDecodable, RustcEncodable)]
pub struct GossipFile {
    pub service_group: ServiceGroup,
    pub file_name: String,
    body: Vec<u8>,
    checksum: String,
    version_number: u64,
    written: bool,
    encrypted: bool,
}

impl GossipFile {
    /// creates a GossipFile with file containing valid TOML
    /// The file name that's passed in isn't retained,
    /// it's changed to `gossip.toml`.
    pub fn from_file<P: AsRef<Path>>(service_group: ServiceGroup,
                                     file_path: P,
                                     version_number: u64)
                                     -> Result<GossipFile> {
        let path = file_path.as_ref();
        for part in path.components() {
            let pstr = format!("{}", part.as_os_str().to_string_lossy().into_owned());
            if &pstr == ".." {
                return Err(Error::GossipFileRelativePath(path.to_string_lossy()
                    .into_owned()));
            }
        }
        let mut f = try!(File::open(&path));
        let mut body = Vec::new();
        try!(f.read_to_end(&mut body));

        let toml_body = try!(String::from_utf8(body.clone()));
        try!(is_data_toml(toml_body));
        let checksum = openssl_hash::hash(openssl_hash::Type::SHA256, &body);

        let cf = GossipFile {
            service_group: service_group,
            file_name: GOSSIP_TOML.to_string(),
            body: body,
            checksum: checksum.as_slice().to_hex(),
            version_number: version_number,
            written: false,
            encrypted: false,
        };
        Ok(cf)
    }

    /// Creates an encrypted GossipFile.  The file does NOT need to
    /// contain TOML. Note, this file is NOT remaned to `gossip.toml`
    /// internally. It will be gossiped around the ring and stored in the
    /// `/hab/svc/<service>/files/file_name` directory, where file_name
    /// is `GossipFile.file_name`.
    pub fn from_file_encrypt<P: AsRef<Path> + ?Sized>(user_pair: &BoxKeyPair,
                                                      service_pair: &BoxKeyPair,
                                                      file_path: &P,
                                                      version_number: u64)
                                                      -> Result<GossipFile> {
        let path = file_path.as_ref();
        if path.file_name().unwrap() == GOSSIP_TOML {
            return Err(Error::CantUploadGossipToml);
        }
        for part in path.components() {
            let pstr = format!("{}", part.as_os_str().to_string_lossy().into_owned());
            if &pstr == ".." {
                return Err(Error::GossipFileRelativePath(path.to_string_lossy()
                    .into_owned()));
            }
        }
        let mut f = try!(File::open(&path));
        let mut body = Vec::new();
        try!(f.read_to_end(&mut body));

        let encrypted_body = try!(user_pair.encrypt(&body, service_pair));

        let file_name = try!(path.file_name().ok_or(Error::FileNameError));
        let checksum = openssl_hash::hash(openssl_hash::Type::SHA256, &body);
        let cf = GossipFile {
            service_group: try!(ServiceGroup::from_str(&service_pair.name)),
            file_name: file_name.to_string_lossy().to_string(),
            body: encrypted_body,
            checksum: checksum.as_slice().to_hex(),
            version_number: version_number,
            written: false,
            encrypted: true,
        };
        Ok(cf)
    }


    /// creates a GossipFile with a Vec<u8> containing valid TOML
    /// The GossipFile.file_name is automatically assigned to `gossip.toml`.
    pub fn from_body(service_group: ServiceGroup,
                     body: Vec<u8>,
                     version_number: u64)
                     -> Result<GossipFile> {
        let checksum = openssl_hash::hash(openssl_hash::Type::SHA256, &body);

        let toml_body = try!(String::from_utf8(body.clone()));
        try!(is_data_toml(toml_body));

        let cf = GossipFile {
            service_group: service_group,
            file_name: GOSSIP_TOML.to_string(),
            body: body,
            checksum: checksum.as_slice().to_hex(),
            version_number: version_number,
            written: false,
            encrypted: false,
        };
        Ok(cf)
    }

    /// Updates this struct against another `GossipFile`. If true is returned, we have changed the gossip file
    /// and the rumour should stay hot. If false is returned, nothing has changed and the rumour
    /// can start to go cold. The algorithm is as follows:
    ///
    /// * The `other` has a higher version number, use its data as our data
    /// * If the version numbers are identical but the data differs, loudly warn and return false
    ///   to cool the rumour as it is irreconcilable
    /// * Otherwise our version number is higher and we return false as no updates should occur
    pub fn update_via(&mut self, other: GossipFile) -> bool {
        if self.version_number < other.version_number {
            *self = other;
            true
        } else if self.version_number == other.version_number && *self != other {
            // We have a big problem: this means that the 2 gossip files are *not* the same but
            // they have the same `version_number`. This is probably irreconcilable at present.
            println!("This gossip file has the same version number ({}) as \
                  the other GossipFile but our data is different, meaning \
                  that we can't pick a winner. We will trust our data and \
                  hope a higher version is published later. \
                  (My data: {}, other data: {})",
                     self.version_number,
                     self,
                     &other);
            false
        } else {
            false
        }
    }

    pub fn on_disk_path(&self) -> PathBuf {
        if &self.file_name == GOSSIP_TOML {
            fs::svc_path(&self.service_group.service).join(&self.file_name)
        } else {
            fs::svc_files_path(&self.service_group.service).join(&self.file_name)
        }
    }

    pub fn checksum_file(&self) -> Result<String> {
        let mut file = try!(File::open(self.on_disk_path()));
        let mut buf = [0u8; 1024];
        let mut h = openssl_hash::Hasher::new(openssl_hash::Type::SHA256);
        loop {
            let bytes_read = try!(file.read(&mut buf));
            if bytes_read == 0 {
                break;
            }
            try!(h.write(&buf[0..bytes_read]));
        }
        Ok(h.finish().as_slice().to_hex())
    }

    pub fn write(&mut self, svc_user: &str, svc_group: &str) -> Result<bool> {
        let current_checksum = match self.checksum_file() {
            Ok(checksum) => checksum,
            Err(_) => String::new(),
        };
        if self.checksum == current_checksum {
            debug!("File {} has matching checksum; skipping it",
                   self.on_disk_path().to_string_lossy());
            Ok(false)
        } else {
            let filename = self.on_disk_path();
            println!("Writing new file from gossip: {}",
                     filename.to_string_lossy());
            let new_filename = format!("{}.write", filename.to_string_lossy());
            {
                if self.encrypted {
                    let mut new_file = try!(File::create(&new_filename));
                    // I'm the recipient, because GossipFileList::write()
                    // checks before calling this function.
                    // However, if decrypt() can't find user/service keys,
                    // this write will fail.
                    println!("Attempting to decrypt {}", &self.file_name);
                    let decrypted_bytes = try!(BoxKeyPair::decrypt(&self.body,
                                                                   &default_cache_key_path(None)));
                    println!("Successfully decrypted {}", &self.file_name);
                    try!(new_file.write_all(&decrypted_bytes));
                } else {
                    let mut new_file = try!(File::create(&new_filename));
                    try!(new_file.write_all(&self.body));
                }
            }
            try!(std::fs::rename(&new_filename, self.on_disk_path()));
            try!(set_owner_and_group(&self.on_disk_path(), svc_user, svc_group));
            try!(set_permissions(&self.on_disk_path(), UPLOADED_FILE_PERMISSIONS));
            self.written = true;
            Ok(true)
        }
    }
}

impl fmt::Display for GossipFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "GossipFile {} {} (F: {}, C: {})",
               self.service_group,
               self.version_number,
               self.file_name,
               self.checksum)
    }
}

impl PartialEq for GossipFile {
    fn eq(&self, other: &GossipFile) -> bool {
        self.service_group == other.service_group && self.file_name == other.file_name &&
        self.checksum == other.checksum && self.version_number == other.version_number
    }
}


/// try to parse the string as Toml
fn is_data_toml(body: String) -> Result<()> {
    let mut parser = toml::Parser::new(&body);
    if let None = parser.parse() {
        return Err(Error::InvalidTomlError(format_errors(&parser)));
    };
    Ok(())
}


/// generate a string from toml parser errors
fn format_errors(parser: &toml::Parser) -> String {
    let mut msg = String::new();
    for err in &parser.errors {
        let (loline, locol) = parser.to_linecol(err.lo);
        let (hiline, hicol) = parser.to_linecol(err.hi);
        msg.push_str(&format!("\t{}:{}-{}:{} error: {}\n",
                              loline,
                              locol,
                              hiline,
                              hicol,
                              err.desc));
    }
    msg
}


#[derive(Clone, Debug)]
pub struct FileWriteRetry {
    file_name: String,
    total_retries: u32,
    next_retry: SteadyTime,
    last_failure_reason: String,
}

impl FileWriteRetry {
    pub fn new(file_name: String,
               total_retries: u32,
               next_retry: SteadyTime,
               last_failure_reason: String)
               -> FileWriteRetry {
        FileWriteRetry {
            file_name: file_name,
            total_retries: total_retries,
            next_retry: next_retry,
            last_failure_reason: last_failure_reason,
        }
    }
}

// supply a custom encoder due to the use of SteadyTime
impl Encodable for FileWriteRetry {
    fn encode<S: Encoder>(&self, s: &mut S) -> std::result::Result<(), S::Error> {
        try!(s.emit_struct("FileWriteRetry", 3, |s| {
            try!(s.emit_struct_field("file_name", 0, |s| self.file_name.encode(s)));
            try!(s.emit_struct_field("last_failure_reason",
                                     1,
                                     |s| self.last_failure_reason.encode(s)));
            try!(s.emit_struct_field("total_retries", 2, |s| self.total_retries.encode(s)));
            Ok(())
        }));
        Ok(())
    }
}

#[derive(Debug)]
pub struct GossipFileList {
    my_service_group: ServiceGroup,
    // (ServiceGroup, Filename)
    gossip_files: HashMap<(ServiceGroup, String), GossipFile>,
    next_idempotency_check: SteadyTime,
    pub file_write_retries: HashMap<String, FileWriteRetry>,
}

impl GossipFileList {
    pub fn new(service_group: ServiceGroup) -> GossipFileList {
        GossipFileList {
            my_service_group: service_group,
            gossip_files: HashMap::new(),
            next_idempotency_check: SteadyTime::now() +
                                    Duration::minutes(IDEMPOTENCY_INTERVAL_MINUTES),
            file_write_retries: HashMap::new(),
        }
    }

    // We should write out if our metadata tells us to. Also, if we haven't checked the state of
    // the filesystem in the last 5 minutes, we're going to call write regardless, just to make
    // sure nothing has changed out from under us on disk. Viva la Chef!
    pub fn needs_write(&mut self) -> bool {
        if SteadyTime::now() >= self.next_idempotency_check {
            info!("Checking idempotency of configuration files; my interval has expired!");
            self.next_idempotency_check = SteadyTime::now() +
                                          Duration::minutes(IDEMPOTENCY_INTERVAL_MINUTES);
            return true;
        }
        self.gossip_files
            .iter()
            .any(|(&(ref sg, _), ref cf)| sg == &self.my_service_group && cf.written == false)
    }

    /// Write the files out to disk. We currently are a bit badly factored here - we have both the
    /// gossip data and "regular" files you might want to distribute, and we treat them the same.
    /// That's fine for now, and maybe for a long time, but it's a bit ugly.
    ///
    /// One result of that is that this function returns a tuple of boolean values. The first is
    /// `needs_file_updated`, which means a file has changed, and we run the file_updated hook. The
    /// second is `needs_reconfigure`, which means the `gossip.toml` has changed, and we need to
    /// reconfigure the daemon and the `service_config`.
    pub fn write(&mut self, svc_user: &str, svc_group: &str) -> Result<(bool, bool)> {
        let mut needs_file_updated = false;
        let mut needs_reconfigure = false;
        for (&(ref sg, _), ref mut gf) in self.gossip_files.iter_mut() {
            // Don't write this file if it's not my_service_group.
            // Take note, this applies to encrypted payloads as well.
            // If it's not for "me", I won't write it out.
            if sg != &self.my_service_group {
                continue;
            }

            // see if we need to skip this write if it's a retry but
            // it's not quite time yet
            let needs_retry = self.file_write_retries.contains_key(&gf.file_name);
            if needs_retry {
                // we just checked if file_write_retries contains this key,
                // so it's ok to unwrap
                let fwr = self.file_write_retries.get(&gf.file_name).unwrap();
                if SteadyTime::now() < fwr.next_retry {
                    debug!("Waiting to retry write for {}", &gf.file_name);
                    continue;
                }
            }

            // Try to write the GossipFile body to a file. Upon failure, add
            // the file to the file_write_retries HashMap so it will be retried
            // upon the next call to GossipFileList.write()
            let written = match gf.write(svc_user, svc_group) {
                Ok(b) => {
                    if needs_retry {
                        // we don't need to retry and more, clear this flag
                        // and continue
                        let _ = self.file_write_retries.remove(&gf.file_name);
                    }
                    b
                }
                Err(e) => {
                    // the write failed, let's log the error and retry again later
                    println!("Failed to write file {}:{}",
                             gf.on_disk_path().to_string_lossy(),
                             e);

                    if needs_retry {
                        // if needs_retry == true, then we know this file is already
                        // in the hashtable
                        let fwr = self.file_write_retries.get_mut(&gf.file_name).unwrap();
                        let next_retries = fwr.total_retries + 1;
                        let exp_backoff_seconds = 2i64.pow(next_retries);
                        let next = SteadyTime::now() + Duration::seconds(exp_backoff_seconds);
                        println!("Retrying write to {} in {} seconds",
                                 &gf.file_name,
                                 exp_backoff_seconds);
                        fwr.next_retry = next;
                        fwr.total_retries = next_retries;
                        fwr.last_failure_reason = e.to_string();
                    } else {
                        let next = SteadyTime::now() + Duration::seconds(1);
                        println!("Retrying write for {} in 1 second", &gf.file_name);
                        let fwr = FileWriteRetry::new(gf.file_name.clone(), 0, next, e.to_string());
                        self.file_write_retries.insert(gf.file_name.clone(), fwr);
                    };
                    false
                }
            };

            if needs_file_updated == false && written == true {
                needs_file_updated = true;
            }
            if gf.file_name == GOSSIP_TOML {
                needs_reconfigure = true;
            }
        }
        Ok((needs_file_updated, needs_reconfigure))
    }

    pub fn process(&mut self, remote_gf: GossipFile) -> bool {
        if let Some(mut current_gf) = self.get_mut(&remote_gf.service_group, &remote_gf.file_name) {
            return current_gf.update_via(remote_gf);
        }
        self.gossip_files.insert((remote_gf.service_group.clone(), remote_gf.file_name.clone()),
                                 remote_gf);
        true
    }

    fn get_mut(&mut self,
               service_group: &ServiceGroup,
               file_name: &str)
               -> Option<&mut GossipFile> {
        self.gossip_files.get_mut(&(service_group.clone(), file_name.to_string()))
    }
}

#[cfg(test)]
mod test {
    use std::env;
    use std::io::prelude::*;
    use std::fs::File;
    use std::path::PathBuf;
    use std::str::FromStr;

    use rustc_serialize::json;
    use tempdir::TempDir;
    use time::SteadyTime;

    use hcore::crypto::BoxKeyPair;
    use hcore::service::ServiceGroup;
    use gossip_file::{GossipFile, FileWriteRetry, GOSSIP_TOML};

    fn fixture(name: &str) -> PathBuf {
        env::current_exe()
            .unwrap()
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .join("tests")
            .join("fixtures")
            .join(name)
    }

    #[test]
    fn new_from_file() {
        let cf = GossipFile::from_file(ServiceGroup::from_str("petty.gunslingers").unwrap(),
                                       fixture("foo.toml").as_path(),
                                       2)
            .unwrap();
        assert_eq!(cf.service_group,
                   ServiceGroup::from_str("petty.gunslingers").unwrap());
        assert_eq!(cf.file_name, GOSSIP_TOML);
        assert_eq!(cf.checksum,
                   "9af65ddf16684e60cf5859d73d878d9607747632283d0d5d945c8c85ca85d420");
        assert_eq!(cf.version_number, 2);
    }


    #[test]
    #[should_panic]
    fn new_from_file_invalid_toml() {
        GossipFile::from_file(ServiceGroup::from_str("petty.gunslingers").unwrap(),
                              fixture("bad.toml").as_path(),
                              2)
            .unwrap();
    }

    #[test]
    fn new_from_file_encrypt() {
        let cache = TempDir::new("key_cache").unwrap();
        let user_pair = BoxKeyPair::generate_pair_for_user("testuser", cache.path()).unwrap();
        let service_pair =
            BoxKeyPair::generate_pair_for_service("someorg", "petty.gunslingers", cache.path())
                .unwrap();
        let gf = GossipFile::from_file_encrypt(&user_pair,
                                               &service_pair,
                                               fixture("foo.toml").as_path(),
                                               1)
            .unwrap();
        assert_eq!(gf.service_group,
                   ServiceGroup::from_str("petty.gunslingers@someorg").unwrap());
        // keeps it's filename
        assert_eq!(gf.file_name, "foo.toml");
        // unencrypted data checksum
        assert_eq!(gf.checksum,
                   "9af65ddf16684e60cf5859d73d878d9607747632283d0d5d945c8c85ca85d420");
        assert_eq!(gf.version_number, 1);

        let val_bytes = BoxKeyPair::decrypt(&gf.body, cache.path()).unwrap();
        let decrypted = String::from_utf8(val_bytes).unwrap();

        // does the decrypted text match whats in the fixture?
        let mut f = File::open(fixture("foo.toml").as_path()).unwrap();
        let mut s = String::new();
        f.read_to_string(&mut s).unwrap();
        assert!(decrypted == s);
    }

    #[test]
    fn new_from_body() {
        let cf = GossipFile::from_body(ServiceGroup::from_str("chromeo.footwork").unwrap(),
                                       "Rage=1\n".as_bytes().to_vec(),
                                       45)
            .unwrap();
        assert_eq!(cf.service_group,
                   ServiceGroup::from_str("chromeo.footwork").unwrap());
        assert_eq!(cf.file_name, GOSSIP_TOML);
        assert_eq!(cf.body, "Rage=1\n".as_bytes().to_vec());
        assert_eq!(cf.checksum,
                   "c14c4757090e7e734941d2b948484b84e4179404f6bab053b3ca21cbb7b0d6c8");
        assert_eq!(cf.version_number, 45);
    }


    #[test]
    #[should_panic]
    fn new_from_body_invalid_toml() {
        GossipFile::from_body(ServiceGroup::from_str("chromeo.footwork").unwrap(),
                              "Rage\n".as_bytes().to_vec(),
                              45)
            .unwrap();
    }


    #[test]
    fn update_via_when_other_version_is_higher() {
        let mut me = GossipFile::from_body(ServiceGroup::from_str("foofighters.arlandria")
                                               .unwrap(),
                                           "rope=1\n".as_bytes().to_vec(),
                                           20)
            .unwrap();

        let other = GossipFile::from_body(ServiceGroup::from_str("foofighters.arlandria").unwrap(),
                                          "rope=1\n".as_bytes().to_vec(),
                                          99)
            .unwrap();
        assert_eq!(me == other, false);
        assert_eq!(me.update_via(other.clone()), true);
        assert_eq!(me == other, true);
    }

    #[test]
    fn update_via_when_other_is_older_and_not_equal() {
        let mut me = GossipFile::from_body(ServiceGroup::from_str("heart.barracuda").unwrap(),
                                           "woot=1\n".as_bytes().to_vec(),
                                           99)
            .unwrap();

        let other_service = GossipFile::from_body(ServiceGroup::from_str("oops.barracuda")
                                                      .unwrap(),
                                                  "woot=1\n".as_bytes().to_vec(),
                                                  20)
            .unwrap();
        assert_eq!(me.update_via(other_service.clone()), false);
        assert_eq!(me == other_service, false);

        let other_group = GossipFile::from_body(ServiceGroup::from_str("heart.oops").unwrap(),
                                                "woot=1\n".as_bytes().to_vec(),
                                                20)
            .unwrap();
        assert_eq!(me.update_via(other_group.clone()), false);
        assert_eq!(me == other_group, false);

        let other_file_name = GossipFile::from_body(ServiceGroup::from_str("heart.barracuda")
                                                        .unwrap(),
                                                    "woot=1\n".as_bytes().to_vec(),
                                                    20)
            .unwrap();
        assert_eq!(me.update_via(other_file_name.clone()), false);
        assert_eq!(me == other_file_name, false);

        let other_body = GossipFile::from_body(ServiceGroup::from_str("heart.barracuda").unwrap(),
                                               "oops=1".as_bytes().to_vec(),
                                               20)
            .unwrap();
        assert_eq!(me.update_via(other_body.clone()), false);
        assert_eq!(me == other_body, false);
    }

    #[test]
    fn update_via_when_same_version_but_different_data() {
        let other = GossipFile::from_body(ServiceGroup::from_str("soundgarden.badmotorfinger")
                                              .unwrap(),
                                          "tracks=1\n".as_bytes().to_vec(),
                                          42)
            .unwrap();
        let mut me = GossipFile::from_body(ServiceGroup::from_str("heart.barracuda").unwrap(),
                                           "tracks=1\n".as_bytes().to_vec(),
                                           42)
            .unwrap();
        assert_eq!(me.update_via(other.clone()), false);
        assert_eq!(me == other, false);
    }

    #[test]
    fn update_via_when_other_is_equal() {
        let other = GossipFile::from_body(ServiceGroup::from_str("heart.barracuda").unwrap(),
                                          "woot=1\n".as_bytes().to_vec(),
                                          20)
            .unwrap();
        let mut me = GossipFile::from_body(ServiceGroup::from_str("heart.barracuda").unwrap(),
                                           "woot=1\n".as_bytes().to_vec(),
                                           20)
            .unwrap();
        assert_eq!(me.update_via(other.clone()), false);
        assert_eq!(me == other, true);
    }

    #[test]
    fn file_write_retry_encode() {
        let fwr = FileWriteRetry::new("foo".to_string(),
                                      100,
                                      SteadyTime::now(),
                                      "something broke".to_string());
        // just make sure we can encode without failure
        let _ = json::encode(&fwr).unwrap();
    }

}
