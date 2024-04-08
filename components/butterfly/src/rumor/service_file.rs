//! The ServiceFile rumor.
//!
//! Holds the toml configuration injected for a service.

use crate::{error::{Error,
                    Result},
            protocol::{self,
                       newscast::{self,
                                  Rumor as ProtoRumor},
                       FromProto},
            rumor::{Rumor,
                    RumorPayload,
                    RumorType}};
use habitat_core::{crypto::keys::{KeyCache,
                                  SignedBox},
                   service::ServiceGroup};
use serde::Serialize;
use std::{cmp::Ordering,
          fmt,
          mem,
          str::FromStr};

#[derive(Debug, Clone, Serialize)]
pub struct ServiceFile {
    pub from_id:       String,
    pub service_group: ServiceGroup,
    pub incarnation:   u64,
    pub encrypted:     bool,
    pub filename:      String,
    pub body:          Vec<u8>,
}

impl fmt::Display for ServiceFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,
               "ServiceFile i/{} m/{} sg/{} fn/{}",
               self.incarnation, self.from_id, self.service_group, self.filename)
    }
}

impl PartialOrd for ServiceFile {
    fn partial_cmp(&self, other: &ServiceFile) -> Option<Ordering> {
        if self.service_group != other.service_group {
            None
        } else {
            Some(self.incarnation.cmp(&other.incarnation))
        }
    }
}

impl PartialEq for ServiceFile {
    fn eq(&self, other: &ServiceFile) -> bool {
        self.service_group == other.service_group
        && self.incarnation == other.incarnation
        && self.encrypted == other.encrypted
        && self.filename == other.filename
        && self.body == other.body
    }
}

impl ServiceFile {
    /// Creates a new ServiceFile.
    pub fn new<S1, S2>(member_id: S1,
                       service_group: ServiceGroup,
                       filename: S2,
                       body: Vec<u8>)
                       -> Self
        where S1: Into<String>,
              S2: Into<String>
    {
        ServiceFile { from_id: member_id.into(),
                      service_group,
                      incarnation: 0,
                      encrypted: false,
                      filename: filename.into(),
                      body }
    }

    /// Return the body of the service file as a stream of bytes. Always returns a new copy, due to
    /// the fact that we might be encrypted.
    // TODO (CM): Technically, we could return a Cow here to achieve
    // the same effect a bit more efficiently, but the lifetime
    // parameter looks like it would complicate a lot of other
    // code. It's not clear that the gain in efficiency is worth the
    // conceptual overhead at this time... perhaps with future
    // refactoring, though.
    pub fn body(&self, key_cache: &KeyCache) -> Result<Vec<u8>> {
        if self.encrypted {
            let secret = SignedBox::from_bytes(&self.body)?;
            let user_public_key = key_cache.user_public_encryption_key(secret.encryptor())?;
            let service_secret_key = key_cache.service_secret_encryption_key(secret.decryptor())?;
            Ok(service_secret_key.decrypt_user_message(&secret, &user_public_key)?)
        } else {
            Ok(self.body.clone())
        }
    }
}

impl protocol::Message<ProtoRumor> for ServiceFile {
    const MESSAGE_ID: &'static str = "ServiceFile";
}

impl FromProto<ProtoRumor> for ServiceFile {
    fn from_proto(rumor: ProtoRumor) -> Result<Self> {
        let payload = match rumor.payload.ok_or(Error::ProtocolMismatch("payload"))? {
            RumorPayload::ServiceFile(payload) => payload,
            _ => panic!("from-bytes service-config"),
        };
        Ok(ServiceFile { from_id:       rumor.from_id.ok_or(Error::ProtocolMismatch("from-id"))?,
                         service_group:
                             payload.service_group
                                    .ok_or(Error::ProtocolMismatch("service-group"))
                                    .and_then(|s| ServiceGroup::from_str(&s).map_err(Error::from))?,
                         incarnation:   payload.incarnation.unwrap_or(0),
                         encrypted:     payload.encrypted.unwrap_or(false),
                         filename:      payload.filename
                                               .ok_or(Error::ProtocolMismatch("filename"))?,
                         body:          payload.body.unwrap_or_default(), })
    }
}

impl From<ServiceFile> for newscast::ServiceFile {
    fn from(value: ServiceFile) -> Self {
        newscast::ServiceFile { service_group: Some(value.service_group.to_string()),
                                incarnation:   Some(value.incarnation),
                                encrypted:     Some(value.encrypted),
                                filename:      Some(value.filename),
                                body:          Some(value.body), }
    }
}

impl Rumor for ServiceFile {
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

    fn kind(&self) -> RumorType { RumorType::ServiceFile }

    fn id(&self) -> &str { &self.filename }

    fn key(&self) -> &str { &self.service_group }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rumor::Rumor;
    use habitat_core::service::ServiceGroup;
    use std::{cmp::Ordering,
              str::FromStr};

    fn create_service_file(member_id: &str, filename: &str, body: &str) -> ServiceFile {
        let body_bytes: Vec<u8> = Vec::from(body);
        ServiceFile::new(member_id,
                         ServiceGroup::new("neurosis", "production", None).unwrap(),
                         filename,
                         body_bytes)
    }

    #[test]
    fn identical_service_file_are_equal() {
        let s1 = create_service_file("adam", "yep", "tcp-backlog = 128");
        let s2 = create_service_file("adam", "yep", "tcp-backlog = 128");
        assert_eq!(s1, s2);
    }

    #[test]
    fn service_files_with_different_incarnations_are_not_equal() {
        let s1 = create_service_file("adam", "yep", "tcp-backlog = 128");
        let mut s2 = create_service_file("adam", "yep", "tcp-backlog = 128");
        s2.incarnation = 1;
        assert_ne!(s1, s2);
    }

    #[test]
    fn service_files_with_different_service_groups_are_not_equal() {
        let s1 = create_service_file("adam", "yep", "tcp-backlog = 128");
        let mut s2 = create_service_file("adam", "yep", "tcp-backlog = 128");
        s2.service_group = ServiceGroup::from_str("adam.fragile").unwrap();
        assert_ne!(s1, s2);
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
        s2.incarnation = 1;
        assert_eq!(s1.partial_cmp(&s2), Some(Ordering::Less));
        assert_eq!(s2.partial_cmp(&s1), Some(Ordering::Greater));
    }

    #[test]
    fn merge_chooses_the_higher_incarnation() {
        let mut s1 = create_service_file("adam", "yep", "tcp-backlog = 128");
        let mut s2 = create_service_file("adam", "yep", "tcp-backlog = 128");
        s2.incarnation = 1;
        let s2_check = s2.clone();
        assert!(s1.merge(s2));
        assert_eq!(s1, s2_check);
    }

    #[test]
    fn merge_returns_false_if_nothing_changed() {
        let mut s1 = create_service_file("adam", "yep", "tcp-backlog = 128");
        s1.incarnation = 1;
        let s1_check = s1.clone();
        let s2 = create_service_file("adam", "yep", "tcp-backlog = 128");
        assert!(!s1.merge(s2));
        assert_eq!(s1, s1_check);
    }

    #[test]
    fn config_comes_back_as_a_string() {
        let s1 = create_service_file("adam", "yep", "tcp-backlog = 128");
        let mock_cache_key_path = KeyCache::new(std::path::PathBuf::new());
        assert_eq!(String::from_utf8(s1.body(&mock_cache_key_path).unwrap()).expect("cannot get a utf-8 string for \
                                                                 the body"),
                   String::from("tcp-backlog = 128"));
    }
}
