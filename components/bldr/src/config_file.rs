// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

use std::collections::HashMap;
use std::fmt;
use std::fs::{self, File};
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use core::fs::SERVICE_HOME;
use openssl::crypto::hash as openssl_hash;
use rustc_serialize::hex::ToHex;
use time::{SteadyTime, Duration};

use error::{BldrResult, ErrorKind};

static LOGKEY: &'static str = "CF";
const IDEMPOTENCY_INTERVAL_MINUTES: i64 = 5;

/// Temporary home for a candiate `ServiceGroup` tuple
#[derive(Clone, Debug, Eq, Hash, PartialEq, RustcDecodable, RustcEncodable)]
pub struct ServiceGroup {
    service: String,
    group: String,
}

impl ServiceGroup {
    pub fn new(service: String, group: String) -> ServiceGroup {
        ServiceGroup {
            service: service,
            group: group,
        }
    }

    pub fn from(sg_str: &str) -> BldrResult<ServiceGroup> {
        let parts: Vec<&str> = sg_str.split(".").collect();
        if parts.len() != 2 {
            return Err(bldr_error!(ErrorKind::InvalidServiceGroupString(sg_str.to_string())));
        }

        let sg = ServiceGroup {
            service: parts[0].to_string(),
            group: parts[1].to_string(),
        };
        Ok(sg)
    }
}

impl fmt::Display for ServiceGroup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}", self.service, self.group)
    }
}

/// The config file struct.
#[derive(Clone, Debug, Eq, RustcDecodable, RustcEncodable)]
pub struct ConfigFile {
    pub service_group: ServiceGroup,
    pub file_name: String,
    body: Vec<u8>,
    checksum: String,
    version_number: u64,
    written: bool,
}

impl ConfigFile {
    pub fn from_file<P: AsRef<Path>>(service_group: ServiceGroup,
                                     file_path: P,
                                     version_number: u64)
                                     -> BldrResult<ConfigFile> {
        let path = file_path.as_ref();
        for part in path.components() {
            let pstr = format!("{}", part.as_os_str().to_string_lossy().into_owned());
            if &pstr == ".." {
                return Err(bldr_error!(ErrorKind::ConfigFileRelativePath(path.to_string_lossy()
                                                                             .into_owned())));
            }
        }
        let mut f = try!(File::open(&path));
        let mut body = Vec::new();
        try!(f.read_to_end(&mut body));

        let file_name = try!(path.file_name().ok_or(bldr_error!(ErrorKind::FileNameError)));
        let checksum = openssl_hash::hash(openssl_hash::Type::SHA256, &body);

        let cf = ConfigFile {
            service_group: service_group,
            file_name: file_name.to_string_lossy().to_string(),
            body: body,
            checksum: checksum.as_slice().to_hex(),
            version_number: version_number,
            written: false,
        };
        Ok(cf)
    }

    #[allow(dead_code)]
    fn from_body(service_group: ServiceGroup,
                 file_name: String,
                 body: Vec<u8>,
                 version_number: u64)
                 -> BldrResult<ConfigFile> {
        let checksum = openssl_hash::hash(openssl_hash::Type::SHA256, &body);

        let cf = ConfigFile {
            service_group: service_group,
            file_name: file_name,
            body: body,
            checksum: checksum.as_slice().to_hex(),
            version_number: version_number,
            written: false,
        };
        Ok(cf)
    }

    /// Updates this struct against another `ConfigFile`. If true is returned, we have changed the config file
    /// and the rumour should stay hot. If false is returned, nothing has changed and the rumour
    /// can start to go cold. The algorithm is as follows:
    ///
    /// * The the `other` has a higher version number, use its data as our data
    /// * If the version numbers are identical but the data differs, loudly warn and return false
    ///   to cool the rumour as it is irreconcilable
    /// * Otherwise our version number is higher and we return false as no updates should occur
    pub fn update_via(&mut self, other: ConfigFile) -> bool {
        if self.version_number < other.version_number {
            *self = other;
            true
        } else if self.version_number == other.version_number && *self != other {
            // We have a big problem: this means that the 2 config files are *not* the same but
            // they have the same `version_number`. This is probably irreconcilable at present.
            outputln!("This config file has the same version number ({}) as \
                  the other  ConfigFile but our data is different, meaning \
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
        if &self.file_name == "gossip.toml" {
            PathBuf::from(format!("{}/{}/{}",
                                  SERVICE_HOME,
                                  self.service_group.service,
                                  self.file_name))
        } else {
            PathBuf::from(format!("{}/{}/files/{}",
                                  SERVICE_HOME,
                                  self.service_group.service,
                                  self.file_name))
        }
    }

    pub fn checksum_file(&self) -> BldrResult<String> {
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

    pub fn write(&self) -> BldrResult<bool> {
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
            outputln!("Writing new config file from gossip: {}",
                      filename.to_string_lossy());
            let new_filename = format!("{}.write", filename.to_string_lossy());
            {
                let mut new_file = try!(File::create(&new_filename));
                try!(new_file.write_all(&self.body));
            }
            try!(fs::rename(new_filename, self.on_disk_path()));
            Ok(true)
        }
    }
}

impl fmt::Display for ConfigFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "ConfigFile {} {} (F: {}, C: {})",
               self.service_group,
               self.version_number,
               self.file_name,
               self.checksum)
    }
}

impl PartialEq for ConfigFile {
    fn eq(&self, other: &ConfigFile) -> bool {
        self.service_group == other.service_group && self.file_name == other.file_name &&
        self.checksum == other.checksum && self.version_number == other.version_number
    }
}

#[derive(Debug)]
pub struct ConfigFileList {
    my_service_group: ServiceGroup,
    // (ServiceGroup, Filename)
    config_files: HashMap<(ServiceGroup, String), ConfigFile>,
    next_idempotency_check: SteadyTime,
}

impl ConfigFileList {
    pub fn new(service_group: ServiceGroup) -> ConfigFileList {
        ConfigFileList {
            my_service_group: service_group,
            config_files: HashMap::new(),
            next_idempotency_check: SteadyTime::now() +
                                    Duration::minutes(IDEMPOTENCY_INTERVAL_MINUTES),
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
        self.config_files
            .iter()
            .any(|(&(ref sg, _), ref cf)| sg == &self.my_service_group && cf.written == false)
    }

    /// Write the files out to disk. We currently are a bit badly factored here - we have both the
    /// gossip data and "regular" files you might want to distribute, and we treat them the same.
    /// That's fine for now, and maybe for a long time, but it's a bit ugly.
    ///
    /// One result of that is that this funciton returns a tuple of boolean values. The first is
    /// `needs_file_updated`, which means a file has changed, and we run the file_updated hook. The
    /// second is `needs_reconfigure`, which means the `gossip.toml` has changed, and we need to
    /// reconfigure the daemon and the `service_config`.
    pub fn write(&mut self) -> BldrResult<(bool, bool)> {
        let mut needs_file_updated = false;
        let mut needs_reconfigure = false;
        for (&(ref sg, _), ref cf) in self.config_files.iter() {
            if sg != &self.my_service_group {
                continue;
            }
            let written = try!(cf.write());
            if needs_file_updated == false && written == true {
                needs_file_updated = true;
            }
            if cf.file_name == "gossip.toml" {
                needs_reconfigure = true;
            }
        }
        Ok((needs_file_updated, needs_reconfigure))
    }

    pub fn process(&mut self, remote_cf: ConfigFile) -> bool {
        if let Some(mut current_cf) = self.get_mut(&remote_cf.service_group, &remote_cf.file_name) {
            return current_cf.update_via(remote_cf);
        }
        self.config_files.insert((remote_cf.service_group.clone(), remote_cf.file_name.clone()),
                                 remote_cf);
        true
    }

    fn get_mut(&mut self,
               service_group: &ServiceGroup,
               file_name: &str)
               -> Option<&mut ConfigFile> {
        self.config_files.get_mut(&(service_group.clone(), file_name.to_string()))
    }
}

#[cfg(test)]
mod test {
    use std::env;
    use std::path::PathBuf;

    use config_file::{ConfigFile, ServiceGroup};

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
        let cf = ConfigFile::from_file(ServiceGroup::from("petty.gunslingers").unwrap(),
                                       fixture("chef-public.gpg").as_path(),
                                       2)
                     .unwrap();
        assert_eq!(cf.service_group,
                   ServiceGroup::from("petty.gunslingers").unwrap());
        assert_eq!(cf.file_name, "chef-public.gpg");
        assert_eq!(cf.checksum,
                   "437ee1b702f1d14b9e2b322810b510bb25d43a260098b7820b85f3b0c09c45fa");
        assert_eq!(cf.version_number, 2);
    }

    #[test]
    fn new_from_body() {
        let cf = ConfigFile::from_body(ServiceGroup::from("chromeo.footwork").unwrap(),
                                       "tracks.txt".to_string(),
                                       "Rage\n".as_bytes().to_vec(),
                                       45)
                     .unwrap();
        assert_eq!(cf.service_group,
                   ServiceGroup::from("chromeo.footwork").unwrap());
        assert_eq!(cf.file_name, "tracks.txt");
        assert_eq!(cf.body, "Rage\n".as_bytes().to_vec());
        assert_eq!(cf.checksum,
                   "8347123270c1b97dd06de84921b3eb7babd41cb4fd8b2f78a4651903f8904bb1");
        assert_eq!(cf.version_number, 45);
    }

    #[test]
    fn update_via_when_other_version_is_higher() {
        let mut me = ConfigFile::from_body(ServiceGroup::from("foofighters.arlandria").unwrap(),
                                           "wasted_light.csv".to_string(),
                                           "rope\n".as_bytes().to_vec(),
                                           20)
                         .unwrap();

        let other = ConfigFile::from_body(ServiceGroup::from("foofighters.arlandria").unwrap(),
                                          "wasted_light.csv".to_string(),
                                          "rope\n".as_bytes().to_vec(),
                                          99)
                        .unwrap();
        assert_eq!(me == other, false);
        assert_eq!(me.update_via(other.clone()), true);
        assert_eq!(me == other, true);
    }

    #[test]
    fn update_via_when_other_is_older_and_not_equal() {
        let mut me = ConfigFile::from_body(ServiceGroup::from("heart.barracuda").unwrap(),
                                           "greatest_hits.db".to_string(),
                                           "woot\n".as_bytes().to_vec(),
                                           99)
                         .unwrap();

        let other_service = ConfigFile::from_body(ServiceGroup::from("oops.barracuda").unwrap(),
                                                  "greatest_hits.db".to_string(),
                                                  "woot\n".as_bytes().to_vec(),
                                                  20)
                                .unwrap();
        assert_eq!(me.update_via(other_service.clone()), false);
        assert_eq!(me == other_service, false);

        let other_group = ConfigFile::from_body(ServiceGroup::from("heart.oops").unwrap(),
                                                "greatest_hits.db".to_string(),
                                                "woot\n".as_bytes().to_vec(),
                                                20)
                              .unwrap();
        assert_eq!(me.update_via(other_group.clone()), false);
        assert_eq!(me == other_group, false);

        let other_file_name = ConfigFile::from_body(ServiceGroup::from("heart.barracuda").unwrap(),
                                                    "oops".to_string(),
                                                    "woot\n".as_bytes().to_vec(),
                                                    20)
                                  .unwrap();
        assert_eq!(me.update_via(other_file_name.clone()), false);
        assert_eq!(me == other_file_name, false);

        let other_body = ConfigFile::from_body(ServiceGroup::from("heart.barracuda").unwrap(),
                                               "greatest_hits.db".to_string(),
                                               "oops".as_bytes().to_vec(),
                                               20)
                             .unwrap();
        assert_eq!(me.update_via(other_body.clone()), false);
        assert_eq!(me == other_body, false);
    }

    #[test]
    fn update_via_when_same_version_but_different_data() {
        let other = ConfigFile::from_body(ServiceGroup::from("soundgarden.badmotorfinger")
                                              .unwrap(),
                                          "rusty.cage".to_string(),
                                          "tracks\n".as_bytes().to_vec(),
                                          42)
                        .unwrap();
        let mut me = ConfigFile::from_body(ServiceGroup::from("heart.barracuda").unwrap(),
                                           "greatest_hits.db".to_string(),
                                           "tracks\n".as_bytes().to_vec(),
                                           42)
                         .unwrap();
        assert_eq!(me.update_via(other.clone()), false);
        assert_eq!(me == other, false);
    }

    #[test]
    fn update_via_when_other_is_equal() {
        let other = ConfigFile::from_body(ServiceGroup::from("heart.barracuda").unwrap(),
                                          "greatest_hits.db".to_string(),
                                          "woot\n".as_bytes().to_vec(),
                                          20)
                        .unwrap();
        let mut me = ConfigFile::from_body(ServiceGroup::from("heart.barracuda").unwrap(),
                                           "greatest_hits.db".to_string(),
                                           "woot\n".as_bytes().to_vec(),
                                           20)
                         .unwrap();
        assert_eq!(me.update_via(other.clone()), false);
        assert_eq!(me == other, true);
    }
}
