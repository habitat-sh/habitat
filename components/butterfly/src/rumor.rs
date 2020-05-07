//! Tracks rumors for distribution.
//!
//! Each rumor is represented by a `RumorKey`, which has a unique key and a "kind", which
//! represents what "kind" of rumor it is (for example, a "member").
//!
//! New rumors need to implement the `From` trait for `RumorKey`, and then can track the arrival of
//! new rumors, and dispatch them according to their `kind`.

pub mod dat_file;
pub mod departure;
pub mod election;
pub mod heat;
pub mod service;
pub mod service_config;
pub mod service_file;
pub mod service_health;

use crate::{error::{Error,
                    Result},
            member::Membership,
            protocol::{FromProto,
                       Message},
            rumor::election::ElectionRumor};
use bytes::BytesMut;
use prometheus::IntCounterVec;
use prost::Message as ProstMessage;
use serde;
use std::{collections::{hash_map::Entry,
                        HashMap},
          default::Default,
          fmt,
          result,
          sync::{atomic::{AtomicUsize,
                          Ordering},
                 Arc}};

pub use self::{departure::Departure,
               election::{Election,
                          ElectionUpdate},
               service::Service,
               service_config::ServiceConfig,
               service_file::ServiceFile,
               service_health::ServiceHealth,
               storage::{RumorStore,
                         RumorStoreProxy}};
pub use crate::protocol::newscast::{Rumor as ProtoRumor,
                                    RumorPayload,
                                    RumorType};

lazy_static! {
    static ref IGNORED_RUMOR_COUNT: IntCounterVec =
        register_int_counter_vec!("hab_butterfly_ignored_rumor_total",
                                  "How many rumors we ignore",
                                  &["rumor"]).unwrap();
}

#[derive(Debug, Clone, Serialize)]
pub enum RumorKind {
    Departure(Departure),
    Election(Election),
    ElectionUpdate(ElectionUpdate),
    Membership(Membership),
    Service(Box<Service>), // Boxed due to clippy::large_enum_variant
    ServiceConfig(ServiceConfig),
    ServiceFile(ServiceFile),
    ServiceHealth(ServiceHealth),
}

impl From<RumorKind> for RumorPayload {
    fn from(value: RumorKind) -> Self {
        match value {
            RumorKind::Departure(departure) => RumorPayload::Departure(departure.into()),
            RumorKind::Election(election) => RumorPayload::Election(election.into()),
            RumorKind::ElectionUpdate(election) => RumorPayload::Election(election.into()),
            RumorKind::Membership(membership) => RumorPayload::Member(membership.into()),
            RumorKind::Service(service) => RumorPayload::Service((*service).into()),
            RumorKind::ServiceConfig(service_config) => {
                RumorPayload::ServiceConfig(service_config.into())
            }
            RumorKind::ServiceFile(service_file) => RumorPayload::ServiceFile(service_file.into()),
            RumorKind::ServiceHealth(service_health) => {
                RumorPayload::ServiceHealth(service_health.into())
            }
        }
    }
}

/// This is used differently by different rumors, but when not a constant, its value is the name of
/// a service group. See the various `impl`s of Rumor::key.
type RumorKeyKey = String;
/// This is used differently by different rumors, but when not a constant, its value is most often
/// the member ID. See the various `impl`s of Rumor::id.
type RumorKeyId = String;

/// The description of a `RumorKey`.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct RumorKey {
    pub kind: RumorType,
    pub id:   RumorKeyId,
    pub key:  RumorKeyKey,
}

impl RumorKey {
    pub fn new(kind: RumorType,
               id: impl Into<RumorKeyId>,
               key: impl Into<RumorKeyKey>)
               -> RumorKey {
        RumorKey { kind,
                   id: id.into(),
                   key: key.into() }
    }
}

impl fmt::Display for RumorKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.key.is_empty() {
            write!(f, "{}-{}", self.id, self.key)
        } else {
            write!(f, "{}", self.id)
        }
    }
}

/// A representation of a Rumor; implemented by all the concrete types we share as rumors. The
/// exception is the Membership rumor, since it's not actually a rumor in the same vein.
pub trait Rumor: Message<ProtoRumor> + Sized {
    fn kind(&self) -> RumorType;
    fn key(&self) -> &str;
    fn id(&self) -> &str;
    fn merge(&mut self, other: Self) -> bool;
}

pub trait ConstKeyRumor: Rumor {
    fn const_key() -> &'static str;
}

pub trait ConstIdRumor: Rumor {
    fn const_id() -> &'static str;
}

impl<'a, T: Rumor> From<&'a T> for RumorKey {
    fn from(rumor: &'a T) -> RumorKey { RumorKey::new(rumor.kind(), rumor.id(), rumor.key()) }
}

type RumorSubMap<T> = HashMap<RumorKeyId, T>;
type RumorMap<T> = HashMap<RumorKeyKey, RumorSubMap<T>>;

/// To keep the details of the locking from being directly accessible to all the code in the
/// rumor submodule.
mod storage {
    use super::*;
    use habitat_common::sync::{Lock,
                               ReadGuard};
    use serde::{ser::{SerializeMap,
                      SerializeSeq,
                      SerializeStruct},
                Serialize,
                Serializer};

    /// Provides access to the rumors for a particular service group bounded by the
    /// lifetime of the service group key.
    pub struct ServiceGroupRumors<'sg_key, R>(Option<&'sg_key RumorSubMap<R>>);

    impl<'sg_key, R> ServiceGroupRumors<'sg_key, R> {
        /// Allows iterator access to the rumors in a particular service group:
        /// ```
        /// # use habitat_butterfly::rumor::{RumorStore,
        /// #                                Service};
        /// let service_store: RumorStore<Service> = RumorStore::default();
        /// for s in service_store.lock_rsr()
        ///                       .service_group("redis.default")
        ///                       .rumors()
        /// {
        ///     println!("{:?}", s);
        /// }
        /// ```
        pub fn rumors(&self) -> impl Iterator<Item = &R> {
            self.0.iter().map(|m| m.values()).flatten()
        }

        /// Return the result of applying `f` to the rumor in this service_group from
        /// `member_id`, or `None` if no such rumor is present.
        pub fn map_rumor<OUT>(&self, member_id: &str, f: impl Fn(&R) -> OUT) -> Option<OUT> {
            self.0.and_then(|m| m.get(member_id).map(f))
        }

        pub fn contains_id(&self, member_id: &str) -> bool {
            self.map_rumor(member_id, |_| true).unwrap_or(false)
        }
    }

    pub struct IterableGuard<'a, T>(ReadGuard<'a, T>);

    // This impl block covers a `ReadGuard` over a `RumorMap` structure, but none of these
    // functions require the contained value to be a rumor, so we use T, not R. Rumor-specific
    // functionality is a different impl block.
    impl<'a, T> IterableGuard<'a, RumorMap<T>> {
        fn read(lock: &'a Lock<RumorMap<T>>) -> Self { IterableGuard(lock.read()) }

        /// Allows iterator access to the rumors in to the `RumorMap` while holding its lock:
        /// ```
        /// # use habitat_butterfly::rumor::{Departure,
        /// #                                RumorStore};
        /// let rs: RumorStore<Departure> = RumorStore::default();
        /// for rumor in rs.lock_rsr().rumors() {
        ///     println!("{:?}", rumor);
        /// }
        /// ```
        pub fn rumors(&self) -> impl Iterator<Item = &T> {
            self.values().map(HashMap::values).flatten()
        }

        /// Allows iterator access to the rumors in to the `RumorMap` for a particular service group
        /// while holding its lock.
        pub fn service_group(&self, service_group: &str) -> ServiceGroupRumors<T> {
            ServiceGroupRumors(self.get(service_group))
        }

        /// Return the result of applying `f` to the rumor accessible with the provided key or
        /// `None` if no such rumor is present.
        fn map_key<OUT>(&self,
                        RumorKey { key, id, .. }: &RumorKey,
                        f: impl Fn(&T) -> OUT)
                        -> Option<OUT> {
            self.service_group(key).0.and_then(|m| m.get(id).map(f))
        }
    }

    impl<'a, R: Rumor> IterableGuard<'a, RumorMap<R>> {
        pub fn contains_rumor(&self, rumor: &R) -> bool {
            let RumorKey { key, id, .. } = rumor.into();

            self.service_group(&key).contains_id(&id)
        }

        /// Return the bytesteam encoding of the rumor for the given key if present
        ///
        /// # Errors
        /// * Error::NonExistentRumor if no rumor is stored for the key
        pub fn encode_rumor_for(&self, key: &RumorKey) -> Result<Vec<u8>> {
            self.map_key(key, R::write_to_bytes)
                .unwrap_or_else(|| {
                    Err(Error::NonExistentRumor(String::from(&key.id), String::from(&key.key)))
                })
        }
    }

    impl<'a, C: ConstKeyRumor> IterableGuard<'a, RumorMap<C>> {
        pub fn contains_id(&self, member_id: &str) -> bool {
            self.get(C::const_key())
                .map(|rumors| rumors.contains_key(member_id))
                .unwrap_or(false)
        }
    }

    impl<'a, E: ElectionRumor> IterableGuard<'a, RumorMap<E>> {
        pub fn get_term(&self, service_group: &str) -> Option<u64> {
            self.get(service_group)
                .map(|sg| sg.get(E::const_id()).map(ElectionRumor::term))
                .unwrap_or(None)
        }
    }

    /// Allows ergonomic use of the guard for accessing the guarded `RumorMap`:
    /// ```
    /// # use habitat_butterfly::rumor::{Departure,
    /// #                                RumorStore};
    /// let rs: RumorStore<Departure> = RumorStore::default();
    /// assert_eq!(rs.lock_rsr().len(), 0);
    /// ```
    impl<'a, R> std::ops::Deref for IterableGuard<'a, RumorMap<R>> {
        type Target = RumorMap<R>;

        fn deref(&self) -> &Self::Target { &self.0 }
    }

    /// Storage for Rumors. It takes a rumor and stores it according to the member that produced it,
    /// and the service group it is related to.
    ///
    /// Generic over the type of rumor it stores.
    #[derive(Debug, Clone)]
    pub struct RumorStore<T> {
        list:           Arc<Lock<RumorMap<T>>>,
        update_counter: Arc<AtomicUsize>,
    }

    impl<T> RumorStore<T> {
        pub fn get_update_counter(&self) -> usize { self.update_counter.load(Ordering::Relaxed) }

        /// Increment the update counter for this store.
        ///
        /// We don't care if this repeats - it just needs to be unique for any given two states,
        /// which it will be.
        fn increment_update_counter(&self) { self.update_counter.fetch_add(1, Ordering::Relaxed); }

        /// # Locking (see locking.md)
        /// * `RumorStore::list` (read)
        /// * IterableGuard contains an instance of a held lock in order to facilitate ergonomic
        ///   access to collection data inside the lock. However, this means that the lock will be
        ///   held until the `IterableGuard` goes out of scope. In general, it's best to avoid
        ///   binding the return of `lock_rsr` in favor of using it as the first link in a chain of
        ///   functions that will be consumed by an iterator adapter or `for` loop.
        pub fn lock_rsr(&self) -> IterableGuard<RumorMap<T>> { IterableGuard::read(&self.list) }

        /// # Locking (see locking.md)
        /// * `RumorStore::list` (write)
        pub fn remove_rsw(&self, key: &str, id: &str) {
            let mut list = self.list.write();
            list.get_mut(key).and_then(|r| r.remove(id));
        }
    }

    impl<R: Rumor> RumorStore<R> {
        /// Insert a rumor into the Rumor Store. Returns true if the value didn't exist or if it was
        /// mutated; if nothing changed, returns false.
        ///
        /// # Locking (see locking.md)
        /// * `RumorStore::list` (write)
        pub fn insert_rsw(&self, rumor: R) -> bool {
            let mut list = self.list.write();
            let rumors = list.entry(String::from(rumor.key()))
                             .or_insert_with(HashMap::new);
            let kind_ignored_count =
                IGNORED_RUMOR_COUNT.with_label_values(&[&rumor.kind().to_string()]);
            // Result reveals if there was a change so we can increment the counter if needed.
            let result = match rumors.entry(rumor.id().into()) {
                Entry::Occupied(mut entry) => entry.get_mut().merge(rumor),
                Entry::Vacant(entry) => {
                    entry.insert(rumor);
                    true
                }
            };
            if result {
                self.increment_update_counter();
            } else {
                // If we get here, it means nothing changed, which means we effectively ignored the
                // rumor. Let's track that.
                kind_ignored_count.inc();
            }
            result
        }
    }

    impl<T> Default for RumorStore<T> {
        fn default() -> RumorStore<T> {
            RumorStore { list:           Arc::default(),
                         update_counter: Arc::default(), }
        }
    }

    impl<T> Serialize for RumorStore<T> where T: Rumor
    {
        /// # Locking (see locking.md)
        /// * `RumorStore::list` (read)
        fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
            where S: Serializer
        {
            let mut strukt = serializer.serialize_struct("rumor_store", 2)?;
            strukt.serialize_field("list", &*(self.list.read()))?;
            strukt.serialize_field("update_counter", &self.get_update_counter())?;
            strukt.end()
        }
    }

    /// This proxy wraps a RumorStore so that we can control its serialization at a more granular
    /// level. Note that we don't implement a generic version of this, on purpose. Rather, due to
    /// the interaction between the 'key()' and 'id()' functions on the Rumor trait, each rumor
    /// type needs to be custom serialized if we want to avoid irrelevant implementation details
    /// leaking into the JSON output.
    pub struct RumorStoreProxy<'a, T: Rumor>(&'a RumorStore<T>);

    impl<'a, T> RumorStoreProxy<'a, T> where T: Rumor
    {
        pub fn new(r: &'a RumorStore<T>) -> Self { RumorStoreProxy(r) }
    }

    impl<'a> Serialize for RumorStoreProxy<'a, Departure> {
        /// # Locking (see locking.md)
        /// * `RumorStore::list` (read)
        fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
            where S: Serializer
        {
            let map = self.0.list.read();
            let inner_map = map.get(Departure::const_key());
            let len = inner_map.map_or(0, HashMap::len);
            let mut s = serializer.serialize_seq(Some(len))?;

            if let Some(im) = inner_map {
                for k in im.keys() {
                    s.serialize_element(k)?;
                }
            }

            s.end()
        }
    }

    impl<'a, C: ConstIdRumor> Serialize for RumorStoreProxy<'a, C> {
        /// # Locking (see locking.md)
        /// * `RumorStore::list` (read)
        fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
            where S: Serializer
        {
            let map = self.0.list.read();
            let mut new_map = HashMap::new();

            for (k, v) in map.iter() {
                let rumor = v.get(C::const_id());
                let _service_group = new_map.entry(k).or_insert(rumor);
            }

            let mut m = serializer.serialize_map(Some(new_map.len()))?;

            for (key, val) in new_map {
                m.serialize_entry(key, &val)?;
            }

            m.end()
        }
    }

    impl<'a> Serialize for RumorStoreProxy<'a, Service> {
        /// # Locking (see locking.md)
        /// * `RumorStore::list` (read)
        fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
            where S: Serializer
        {
            let map = self.0.list.read();
            let mut m = serializer.serialize_map(Some(map.len()))?;

            for (key, val) in map.iter() {
                m.serialize_entry(key, &val)?;
            }

            m.end()
        }
    }

    impl<'a> Serialize for RumorStoreProxy<'a, ServiceFile> {
        /// # Locking (see locking.md)
        /// * `RumorStore::list` (read)
        fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
            where S: Serializer
        {
            let map = self.0.list.read();
            let mut m = serializer.serialize_map(Some(map.len()))?;

            for (key, val) in map.iter() {
                m.serialize_entry(key, &val)?;
            }

            m.end()
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn update_counter_overflows_safely() {
            let rs = RumorStore::<()> { update_counter:
                                            Arc::new(AtomicUsize::new(usize::max_value())),
                                        ..Default::default() };
            rs.increment_update_counter();
            assert_eq!(rs.get_update_counter(), 0);
        }

        #[test]
        fn update_counter() {
            let rs = RumorStore::<()>::default();
            rs.increment_update_counter();
            assert_eq!(rs.get_update_counter(), 1);
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct RumorEnvelope {
    pub r#type:  RumorType,
    pub from_id: String,
    pub kind:    RumorKind,
}

impl RumorEnvelope {
    pub fn decode(bytes: &[u8]) -> Result<Self> {
        let proto = ProtoRumor::decode(bytes)?;
        let r#type = RumorType::from_i32(proto.r#type).ok_or(Error::ProtocolMismatch("type"))?;
        let from_id = proto.from_id
                           .clone()
                           .ok_or(Error::ProtocolMismatch("from-id"))?;
        let kind = match r#type {
            RumorType::Departure => RumorKind::Departure(Departure::from_proto(proto)?),
            RumorType::Election => RumorKind::Election(Election::from_proto(proto)?),
            RumorType::ElectionUpdate => {
                RumorKind::ElectionUpdate(ElectionUpdate::from_proto(proto)?)
            }
            RumorType::Member => RumorKind::Membership(Membership::from_proto(proto)?),
            RumorType::Service => RumorKind::Service(Box::new(Service::from_proto(proto)?)),
            RumorType::ServiceConfig => RumorKind::ServiceConfig(ServiceConfig::from_proto(proto)?),
            RumorType::ServiceFile => RumorKind::ServiceFile(ServiceFile::from_proto(proto)?),
            RumorType::ServiceHealth => RumorKind::ServiceHealth(ServiceHealth::from_proto(proto)?),
            RumorType::Fake | RumorType::Fake2 => panic!("fake rumor"),
        };
        Ok(RumorEnvelope { r#type,
                           from_id,
                           kind })
    }

    pub fn encode(self) -> Result<Vec<u8>> {
        let proto: ProtoRumor = self.into();
        let mut buf = BytesMut::with_capacity(proto.encoded_len());
        proto.encode(&mut buf)?;
        Ok(buf.to_vec())
    }
}

impl From<RumorEnvelope> for ProtoRumor {
    fn from(value: RumorEnvelope) -> ProtoRumor {
        ProtoRumor { r#type:  value.r#type as i32,
                     tag:     vec![],
                     from_id: Some(value.from_id),
                     payload: Some(value.kind.into()), }
    }
}

#[cfg(test)]
mod tests {
    use crate::{error::Result,
                protocol::{self,
                           newscast},
                rumor::{Rumor,
                        RumorKey,
                        RumorType}};
    use uuid::Uuid;

    #[derive(Clone, Debug, Serialize)]
    struct FakeRumor {
        pub id:  String,
        pub key: String,
    }

    impl Default for FakeRumor {
        fn default() -> FakeRumor {
            FakeRumor { id:  format!("{}", Uuid::new_v4().to_simple_ref()),
                        key: String::from("fakerton"), }
        }
    }

    #[derive(Clone, Debug, Serialize)]
    struct TrumpRumor {
        pub id:  String,
        pub key: String,
    }

    impl Rumor for FakeRumor {
        fn kind(&self) -> RumorType { RumorType::Fake }

        fn key(&self) -> &str { &self.key }

        fn id(&self) -> &str { &self.id }

        fn merge(&mut self, mut _other: FakeRumor) -> bool { false }
    }

    impl protocol::FromProto<newscast::Rumor> for FakeRumor {
        fn from_proto(_other: newscast::Rumor) -> Result<Self> { Ok(FakeRumor::default()) }
    }

    impl From<FakeRumor> for newscast::Rumor {
        fn from(_other: FakeRumor) -> newscast::Rumor { newscast::Rumor::default() }
    }

    impl protocol::Message<newscast::Rumor> for FakeRumor {
        const MESSAGE_ID: &'static str = "FakeRumor";

        fn from_bytes(_bytes: &[u8]) -> Result<Self> { Ok(FakeRumor::default()) }

        fn write_to_bytes(&self) -> Result<Vec<u8>> {
            Ok(Vec::from(format!("{}-{}", self.id, self.key).as_bytes()))
        }
    }

    impl Default for TrumpRumor {
        fn default() -> TrumpRumor {
            TrumpRumor { id:  format!("{}", Uuid::new_v4().to_simple_ref()),
                         key: String::from("fakerton"), }
        }
    }

    impl Rumor for TrumpRumor {
        fn kind(&self) -> RumorType { RumorType::Fake2 }

        fn key(&self) -> &str { &self.key }

        fn id(&self) -> &str { &self.id }

        fn merge(&mut self, mut _other: TrumpRumor) -> bool { false }
    }

    impl protocol::FromProto<newscast::Rumor> for TrumpRumor {
        fn from_proto(_other: newscast::Rumor) -> Result<Self> { Ok(TrumpRumor::default()) }
    }

    impl From<TrumpRumor> for newscast::Rumor {
        fn from(_other: TrumpRumor) -> newscast::Rumor { newscast::Rumor::default() }
    }

    impl protocol::Message<newscast::Rumor> for TrumpRumor {
        const MESSAGE_ID: &'static str = "TrumpRumor";

        fn from_bytes(_bytes: &[u8]) -> Result<Self> { Ok(TrumpRumor::default()) }

        fn write_to_bytes(&self) -> Result<Vec<u8>> {
            Ok(Vec::from(format!("{}-{}", self.id, self.key).as_bytes()))
        }
    }

    #[test]
    fn rumor_keys_kind_can_be_represented_as_a_string() {
        let r = RumorKey::new(RumorType::Member, "my-sweet-id", "my-sweet-key");
        assert_eq!(r.kind.to_string(), "member");
    }

    mod rumor_store {
        use super::*;
        use crate::{error::Error,
                    rumor::{Rumor,
                            RumorStore}};

        #[test]
        fn insert_adds_rumor_when_empty() {
            let rs = RumorStore::default();
            let f = FakeRumor::default();
            assert!(rs.insert_rsw(f));
            assert_eq!(rs.get_update_counter(), 1);
        }

        #[test]
        fn encode_ok() {
            let rs = RumorStore::default();
            let f = FakeRumor { id:  "foo".to_string(),
                                key: "bar".to_string(), };
            let key = RumorKey::from(&f);
            rs.insert_rsw(f);

            assert_eq!(rs.lock_rsr().encode_rumor_for(&key).unwrap(),
                       b"foo-bar".to_vec());
        }

        #[test]
        fn encode_non_existant() {
            let rs = RumorStore::<FakeRumor>::default();
            let f = FakeRumor { id:  "foo".to_string(),
                                key: "bar".to_string(), };
            let key = RumorKey::from(&f);
            // Note, f is never inserted into rs

            let result = rs.lock_rsr().encode_rumor_for(&key);
            if let Err(Error::NonExistentRumor(..)) = result {
                // pass
            } else {
                panic!("Expected Error:NonExistentRumor");
            }
        }

        #[test]
        fn insert_adds_multiple_rumors_for_same_key() {
            let rs = RumorStore::default();
            let f1 = FakeRumor::default();
            let key = String::from(f1.key());
            let f1_id = String::from(f1.id());
            let f2 = FakeRumor::default();
            let f2_id = String::from(f2.id());

            assert!(rs.insert_rsw(f1));
            assert!(rs.insert_rsw(f2));
            assert_eq!(rs.lock_rsr().len(), 1);
            assert_eq!(rs.lock_rsr()
                         .service_group(&key)
                         .map_rumor(&f1_id, |r| r.id.clone()),
                       Some(f1_id));
            assert_eq!(rs.lock_rsr()
                         .service_group(&key)
                         .map_rumor(&f2_id, |r| r.id.clone()),
                       Some(f2_id));
        }

        #[test]
        fn insert_adds_multiple_members() {
            let rs = RumorStore::default();
            let f1 = FakeRumor::default();
            let key = String::from(f1.key());
            let f2 = FakeRumor::default();
            assert!(rs.insert_rsw(f1));
            assert!(rs.insert_rsw(f2));
            assert_eq!(rs.lock_rsr().get(&key).unwrap().len(), 2);
        }

        #[test]
        fn insert_returns_false_on_no_changes() {
            let rs = RumorStore::default();
            let f1 = FakeRumor::default();
            let f2 = f1.clone();
            assert!(rs.insert_rsw(f1));
            assert_eq!(rs.insert_rsw(f2), false);
        }

        #[test]
        fn map_rumor_calls_closure_with_rumor() {
            let rs = RumorStore::default();
            let f1 = FakeRumor::default();
            let member_id = f1.id.clone();
            let key = f1.key.clone();
            rs.insert_rsw(f1);
            rs.lock_rsr()
              .service_group(&key)
              .map_rumor(&member_id, |o| assert_eq!(o.id, member_id));
        }
    }
}
