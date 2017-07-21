// Copyright (c) 2016-2017 Chef Software Inc. and/or applicable contributors
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

//! The ServiceFile rumor.
//!
//! Holds the toml configuration injected for a service.

use std::cmp::Ordering;
use std::mem;
use std::ops::{Deref, DerefMut};

use habitat_core::crypto::{BoxKeyPair, default_cache_key_path};
use habitat_core::service::ServiceGroup;
use protobuf::{self, Message};

use error::Result;
use message::swim::{ServiceFile as ProtoServiceFile, Rumor as ProtoRumor,
                    Rumor_Type as ProtoRumor_Type};
use rumor::Rumor;

#[derive(Debug, Clone, Serialize)]
pub struct ServiceFile(ProtoRumor);

impl PartialOrd for ServiceFile {
    fn partial_cmp(&self, other: &ServiceFile) -> Option<Ordering> {
        if self.get_service_group() != other.get_service_group() {
            None
        } else {
            Some(self.get_incarnation().cmp(&other.get_incarnation()))
        }
    }
}

impl PartialEq for ServiceFile {
    fn eq(&self, other: &ServiceFile) -> bool {
        self.get_service_group() == other.get_service_group() &&
            self.get_incarnation() == other.get_incarnation() &&
            self.get_encrypted() == other.get_encrypted() &&
            self.get_filename() == other.get_filename() &&
            self.get_body() == other.get_body()
    }
}

impl From<ProtoRumor> for ServiceFile {
    fn from(pr: ProtoRumor) -> ServiceFile {
        ServiceFile(pr)
    }
}

impl From<ServiceFile> for ProtoRumor {
    fn from(service_file: ServiceFile) -> ProtoRumor {
        service_file.0
    }
}

impl Deref for ServiceFile {
    type Target = ProtoServiceFile;

    fn deref(&self) -> &ProtoServiceFile {
        self.0.get_service_file()
    }
}

impl DerefMut for ServiceFile {
    fn deref_mut(&mut self) -> &mut ProtoServiceFile {
        self.0.mut_service_file()
    }
}

impl ServiceFile {
    /// Creates a new ServiceFile.
    pub fn new<S1, S2>(
        member_id: S1,
        service_group: ServiceGroup,
        filename: S2,
        body: Vec<u8>,
    ) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        let mut rumor = ProtoRumor::new();
        let from_id = member_id.into();
        rumor.set_from_id(from_id);
        rumor.set_field_type(ProtoRumor_Type::ServiceFile);

        let mut proto = ProtoServiceFile::new();
        proto.set_service_group(format!("{}", service_group));
        proto.set_incarnation(0);
        proto.set_filename(filename.into());
        proto.set_body(body);

        rumor.set_service_file(proto);
        ServiceFile(rumor)
    }

    /// Encrypt the contents of the service file
    pub fn encrypt(&mut self, user_pair: &BoxKeyPair, service_pair: &BoxKeyPair) -> Result<()> {
        let body = self.take_body();
        let encrypted_body = user_pair.encrypt(&body, service_pair)?;
        self.set_body(encrypted_body);
        self.set_encrypted(true);
        Ok(())
    }

    /// Return the body of the service file as a stream of bytes. Always returns a new copy, due to
    /// the fact that we might be encrypted.
    pub fn body(&self) -> Result<Vec<u8>> {
        if self.get_encrypted() {
            let bytes = BoxKeyPair::decrypt(self.get_body(), &default_cache_key_path(None))?;
            Ok(bytes)
        } else {
            Ok(self.get_body().to_vec())
        }
    }
}

impl Rumor for ServiceFile {
    fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let rumor = protobuf::parse_from_bytes::<ProtoRumor>(bytes)?;
        Ok(ServiceFile::from(rumor))
    }

    /// Follows a simple pattern; if we have a newer incarnation than the one we already have, the
    /// new one wins. So far, these never change.
    fn merge(&mut self, mut other: ServiceFile) -> bool {
        if *self >= other {
            false
        } else {
            mem::swap(self, &mut other);
            true
        }
    }

    fn kind(&self) -> ProtoRumor_Type {
        ProtoRumor_Type::ServiceFile
    }

    fn id(&self) -> &str {
        self.get_filename()
    }

    fn key(&self) -> &str {
        self.get_service_group()
    }

    fn write_to_bytes(&self) -> Result<Vec<u8>> {
        Ok(self.0.write_to_bytes()?)
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use habitat_core::service::ServiceGroup;

    use super::ServiceFile;
    use rumor::Rumor;

    fn create_service_file(member_id: &str, filename: &str, body: &str) -> ServiceFile {
        let body_bytes: Vec<u8> = Vec::from(body);
        ServiceFile::new(
            member_id,
            ServiceGroup::new(None, "neurosis", "production", None).unwrap(),
            filename,
            body_bytes,
        )
    }

    #[test]
    fn identical_service_file_are_equal() {
        let s1 = create_service_file("adam", "yep", "tcp-backlog = 128");
        let s2 = create_service_file("adam", "yep", "tcp-backlog = 128");
        assert_eq!(s1, s2);
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn service_files_with_different_incarnations_are_not_equal() {
        let s1 = create_service_file("adam", "yep", "tcp-backlog = 128");
        let mut s2 = create_service_file("adam", "yep", "tcp-backlog = 128");
        s2.set_incarnation(1);
        assert_eq!(s1, s2);
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn service_files_with_different_service_groups_are_not_equal() {
        let s1 = create_service_file("adam", "yep", "tcp-backlog = 128");
        let mut s2 = create_service_file("adam", "yep", "tcp-backlog = 128");
        s2.set_service_group(String::from("adam.fragile"));
        assert_eq!(s1, s2);
    }

    // Order
    #[test]
    fn service_files_that_are_identical_are_equal_via_cmp() {
        let s1 = create_service_file("adam", "yep", "tcp-backlog = 128");
        let s2 = create_service_file("adam", "yep", "tcp-backlog = 128");
        assert_eq!(s1.partial_cmp(&s2), Some(Ordering::Equal));
    }

    #[test]
    fn service_files_with_different_incarnations_are_not_equal_via_cmp() {
        let s1 = create_service_file("adam", "yep", "tcp-backlog = 128");
        let mut s2 = create_service_file("adam", "yep", "tcp-backlog = 128");
        s2.set_incarnation(1);
        assert_eq!(s1.partial_cmp(&s2), Some(Ordering::Less));
        assert_eq!(s2.partial_cmp(&s1), Some(Ordering::Greater));
    }

    #[test]
    fn merge_chooses_the_higher_incarnation() {
        let mut s1 = create_service_file("adam", "yep", "tcp-backlog = 128");
        let mut s2 = create_service_file("adam", "yep", "tcp-backlog = 128");
        s2.set_incarnation(1);
        let s2_check = s2.clone();
        assert_eq!(s1.merge(s2), true);
        assert_eq!(s1, s2_check);
    }

    #[test]
    fn merge_returns_false_if_nothing_changed() {
        let mut s1 = create_service_file("adam", "yep", "tcp-backlog = 128");
        s1.set_incarnation(1);
        let s1_check = s1.clone();
        let s2 = create_service_file("adam", "yep", "tcp-backlog = 128");
        assert_eq!(s1.merge(s2), false);
        assert_eq!(s1, s1_check);
    }

    #[test]
    fn config_comes_back_as_a_string() {
        let s1 = create_service_file("adam", "yep", "tcp-backlog = 128");
        assert_eq!(
            String::from_utf8(s1.body().unwrap()).expect("cannot get a utf-8 string for the body"),
            String::from("tcp-backlog = 128")
        );
    }
}
