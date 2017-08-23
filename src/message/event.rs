// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct EventEnvelope {
    // message fields
    field_type: ::std::option::Option<EventEnvelope_Type>,
    payload: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    timestamp: ::std::option::Option<u64>,
    member_id: ::protobuf::SingularField<::std::string::String>,
    service: ::protobuf::SingularField<::std::string::String>,
    incarnation: ::std::option::Option<u64>,
    sequence_id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for EventEnvelope {}

impl EventEnvelope {
    pub fn new() -> EventEnvelope {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static EventEnvelope {
        static mut instance: ::protobuf::lazy::Lazy<EventEnvelope> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const EventEnvelope,
        };
        unsafe {
            instance.get(EventEnvelope::new)
        }
    }

    // optional .habitat.eventsrv.EventEnvelope.Type type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: EventEnvelope_Type) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type(&self) -> EventEnvelope_Type {
        self.field_type.unwrap_or(EventEnvelope_Type::ProtoBuf)
    }

    fn get_field_type_for_reflect(&self) -> &::std::option::Option<EventEnvelope_Type> {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut ::std::option::Option<EventEnvelope_Type> {
        &mut self.field_type
    }

    // optional bytes payload = 2;

    pub fn clear_payload(&mut self) {
        self.payload.clear();
    }

    pub fn has_payload(&self) -> bool {
        self.payload.is_some()
    }

    // Param is passed by value, moved
    pub fn set_payload(&mut self, v: ::std::vec::Vec<u8>) {
        self.payload = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_payload(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.payload.is_none() {
            self.payload.set_default();
        }
        self.payload.as_mut().unwrap()
    }

    // Take field
    pub fn take_payload(&mut self) -> ::std::vec::Vec<u8> {
        self.payload.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_payload(&self) -> &[u8] {
        match self.payload.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_payload_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.payload
    }

    fn mut_payload_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.payload
    }

    // optional uint64 timestamp = 3;

    pub fn clear_timestamp(&mut self) {
        self.timestamp = ::std::option::Option::None;
    }

    pub fn has_timestamp(&self) -> bool {
        self.timestamp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_timestamp(&mut self, v: u64) {
        self.timestamp = ::std::option::Option::Some(v);
    }

    pub fn get_timestamp(&self) -> u64 {
        self.timestamp.unwrap_or(0)
    }

    fn get_timestamp_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.timestamp
    }

    fn mut_timestamp_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.timestamp
    }

    // optional string member_id = 4;

    pub fn clear_member_id(&mut self) {
        self.member_id.clear();
    }

    pub fn has_member_id(&self) -> bool {
        self.member_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_member_id(&mut self, v: ::std::string::String) {
        self.member_id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_member_id(&mut self) -> &mut ::std::string::String {
        if self.member_id.is_none() {
            self.member_id.set_default();
        }
        self.member_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_member_id(&mut self) -> ::std::string::String {
        self.member_id.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_member_id(&self) -> &str {
        match self.member_id.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_member_id_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.member_id
    }

    fn mut_member_id_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.member_id
    }

    // optional string service = 5;

    pub fn clear_service(&mut self) {
        self.service.clear();
    }

    pub fn has_service(&self) -> bool {
        self.service.is_some()
    }

    // Param is passed by value, moved
    pub fn set_service(&mut self, v: ::std::string::String) {
        self.service = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_service(&mut self) -> &mut ::std::string::String {
        if self.service.is_none() {
            self.service.set_default();
        }
        self.service.as_mut().unwrap()
    }

    // Take field
    pub fn take_service(&mut self) -> ::std::string::String {
        self.service.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_service(&self) -> &str {
        match self.service.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_service_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.service
    }

    fn mut_service_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.service
    }

    // optional uint64 incarnation = 6;

    pub fn clear_incarnation(&mut self) {
        self.incarnation = ::std::option::Option::None;
    }

    pub fn has_incarnation(&self) -> bool {
        self.incarnation.is_some()
    }

    // Param is passed by value, moved
    pub fn set_incarnation(&mut self, v: u64) {
        self.incarnation = ::std::option::Option::Some(v);
    }

    pub fn get_incarnation(&self) -> u64 {
        self.incarnation.unwrap_or(0)
    }

    fn get_incarnation_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.incarnation
    }

    fn mut_incarnation_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.incarnation
    }

    // optional uint64 sequence_id = 7;

    pub fn clear_sequence_id(&mut self) {
        self.sequence_id = ::std::option::Option::None;
    }

    pub fn has_sequence_id(&self) -> bool {
        self.sequence_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sequence_id(&mut self, v: u64) {
        self.sequence_id = ::std::option::Option::Some(v);
    }

    pub fn get_sequence_id(&self) -> u64 {
        self.sequence_id.unwrap_or(0)
    }

    fn get_sequence_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.sequence_id
    }

    fn mut_sequence_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.sequence_id
    }
}

impl ::protobuf::Message for EventEnvelope {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.field_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.payload)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.timestamp = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.member_id)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.service)?;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.incarnation = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.sequence_id = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.field_type {
            my_size += ::protobuf::rt::enum_size(1, v);
        }
        if let Some(ref v) = self.payload.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        }
        if let Some(v) = self.timestamp {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.member_id.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        }
        if let Some(ref v) = self.service.as_ref() {
            my_size += ::protobuf::rt::string_size(5, &v);
        }
        if let Some(v) = self.incarnation {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.sequence_id {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.field_type {
            os.write_enum(1, v.value())?;
        }
        if let Some(ref v) = self.payload.as_ref() {
            os.write_bytes(2, &v)?;
        }
        if let Some(v) = self.timestamp {
            os.write_uint64(3, v)?;
        }
        if let Some(ref v) = self.member_id.as_ref() {
            os.write_string(4, &v)?;
        }
        if let Some(ref v) = self.service.as_ref() {
            os.write_string(5, &v)?;
        }
        if let Some(v) = self.incarnation {
            os.write_uint64(6, v)?;
        }
        if let Some(v) = self.sequence_id {
            os.write_uint64(7, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for EventEnvelope {
    fn new() -> EventEnvelope {
        EventEnvelope::new()
    }

    fn descriptor_static(_: ::std::option::Option<EventEnvelope>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<EventEnvelope_Type>>(
                    "type",
                    EventEnvelope::get_field_type_for_reflect,
                    EventEnvelope::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "payload",
                    EventEnvelope::get_payload_for_reflect,
                    EventEnvelope::mut_payload_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "timestamp",
                    EventEnvelope::get_timestamp_for_reflect,
                    EventEnvelope::mut_timestamp_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "member_id",
                    EventEnvelope::get_member_id_for_reflect,
                    EventEnvelope::mut_member_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "service",
                    EventEnvelope::get_service_for_reflect,
                    EventEnvelope::mut_service_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "incarnation",
                    EventEnvelope::get_incarnation_for_reflect,
                    EventEnvelope::mut_incarnation_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "sequence_id",
                    EventEnvelope::get_sequence_id_for_reflect,
                    EventEnvelope::mut_sequence_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<EventEnvelope>(
                    "EventEnvelope",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for EventEnvelope {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_payload();
        self.clear_timestamp();
        self.clear_member_id();
        self.clear_service();
        self.clear_incarnation();
        self.clear_sequence_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for EventEnvelope {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EventEnvelope {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum EventEnvelope_Type {
    ProtoBuf = 1,
    JSON = 2,
    TOML = 3,
}

impl ::protobuf::ProtobufEnum for EventEnvelope_Type {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EventEnvelope_Type> {
        match value {
            1 => ::std::option::Option::Some(EventEnvelope_Type::ProtoBuf),
            2 => ::std::option::Option::Some(EventEnvelope_Type::JSON),
            3 => ::std::option::Option::Some(EventEnvelope_Type::TOML),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [EventEnvelope_Type] = &[
            EventEnvelope_Type::ProtoBuf,
            EventEnvelope_Type::JSON,
            EventEnvelope_Type::TOML,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<EventEnvelope_Type>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("EventEnvelope_Type", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for EventEnvelope_Type {
}

impl ::protobuf::reflect::ProtobufValue for EventEnvelope_Type {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SysInfo {
    // message fields
    ip: ::protobuf::SingularField<::std::string::String>,
    hostname: ::protobuf::SingularField<::std::string::String>,
    gossip_ip: ::protobuf::SingularField<::std::string::String>,
    gossip_port: ::protobuf::SingularField<::std::string::String>,
    http_gateway_ip: ::protobuf::SingularField<::std::string::String>,
    http_gateway_port: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SysInfo {}

impl SysInfo {
    pub fn new() -> SysInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SysInfo {
        static mut instance: ::protobuf::lazy::Lazy<SysInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SysInfo,
        };
        unsafe {
            instance.get(SysInfo::new)
        }
    }

    // optional string ip = 1;

    pub fn clear_ip(&mut self) {
        self.ip.clear();
    }

    pub fn has_ip(&self) -> bool {
        self.ip.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ip(&mut self, v: ::std::string::String) {
        self.ip = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ip(&mut self) -> &mut ::std::string::String {
        if self.ip.is_none() {
            self.ip.set_default();
        }
        self.ip.as_mut().unwrap()
    }

    // Take field
    pub fn take_ip(&mut self) -> ::std::string::String {
        self.ip.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_ip(&self) -> &str {
        match self.ip.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_ip_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.ip
    }

    fn mut_ip_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.ip
    }

    // optional string hostname = 2;

    pub fn clear_hostname(&mut self) {
        self.hostname.clear();
    }

    pub fn has_hostname(&self) -> bool {
        self.hostname.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hostname(&mut self, v: ::std::string::String) {
        self.hostname = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_hostname(&mut self) -> &mut ::std::string::String {
        if self.hostname.is_none() {
            self.hostname.set_default();
        }
        self.hostname.as_mut().unwrap()
    }

    // Take field
    pub fn take_hostname(&mut self) -> ::std::string::String {
        self.hostname.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_hostname(&self) -> &str {
        match self.hostname.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_hostname_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.hostname
    }

    fn mut_hostname_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.hostname
    }

    // optional string gossip_ip = 3;

    pub fn clear_gossip_ip(&mut self) {
        self.gossip_ip.clear();
    }

    pub fn has_gossip_ip(&self) -> bool {
        self.gossip_ip.is_some()
    }

    // Param is passed by value, moved
    pub fn set_gossip_ip(&mut self, v: ::std::string::String) {
        self.gossip_ip = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_gossip_ip(&mut self) -> &mut ::std::string::String {
        if self.gossip_ip.is_none() {
            self.gossip_ip.set_default();
        }
        self.gossip_ip.as_mut().unwrap()
    }

    // Take field
    pub fn take_gossip_ip(&mut self) -> ::std::string::String {
        self.gossip_ip.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_gossip_ip(&self) -> &str {
        match self.gossip_ip.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_gossip_ip_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.gossip_ip
    }

    fn mut_gossip_ip_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.gossip_ip
    }

    // optional string gossip_port = 4;

    pub fn clear_gossip_port(&mut self) {
        self.gossip_port.clear();
    }

    pub fn has_gossip_port(&self) -> bool {
        self.gossip_port.is_some()
    }

    // Param is passed by value, moved
    pub fn set_gossip_port(&mut self, v: ::std::string::String) {
        self.gossip_port = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_gossip_port(&mut self) -> &mut ::std::string::String {
        if self.gossip_port.is_none() {
            self.gossip_port.set_default();
        }
        self.gossip_port.as_mut().unwrap()
    }

    // Take field
    pub fn take_gossip_port(&mut self) -> ::std::string::String {
        self.gossip_port.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_gossip_port(&self) -> &str {
        match self.gossip_port.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_gossip_port_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.gossip_port
    }

    fn mut_gossip_port_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.gossip_port
    }

    // optional string http_gateway_ip = 5;

    pub fn clear_http_gateway_ip(&mut self) {
        self.http_gateway_ip.clear();
    }

    pub fn has_http_gateway_ip(&self) -> bool {
        self.http_gateway_ip.is_some()
    }

    // Param is passed by value, moved
    pub fn set_http_gateway_ip(&mut self, v: ::std::string::String) {
        self.http_gateway_ip = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_http_gateway_ip(&mut self) -> &mut ::std::string::String {
        if self.http_gateway_ip.is_none() {
            self.http_gateway_ip.set_default();
        }
        self.http_gateway_ip.as_mut().unwrap()
    }

    // Take field
    pub fn take_http_gateway_ip(&mut self) -> ::std::string::String {
        self.http_gateway_ip.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_http_gateway_ip(&self) -> &str {
        match self.http_gateway_ip.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_http_gateway_ip_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.http_gateway_ip
    }

    fn mut_http_gateway_ip_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.http_gateway_ip
    }

    // optional string http_gateway_port = 6;

    pub fn clear_http_gateway_port(&mut self) {
        self.http_gateway_port.clear();
    }

    pub fn has_http_gateway_port(&self) -> bool {
        self.http_gateway_port.is_some()
    }

    // Param is passed by value, moved
    pub fn set_http_gateway_port(&mut self, v: ::std::string::String) {
        self.http_gateway_port = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_http_gateway_port(&mut self) -> &mut ::std::string::String {
        if self.http_gateway_port.is_none() {
            self.http_gateway_port.set_default();
        }
        self.http_gateway_port.as_mut().unwrap()
    }

    // Take field
    pub fn take_http_gateway_port(&mut self) -> ::std::string::String {
        self.http_gateway_port.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_http_gateway_port(&self) -> &str {
        match self.http_gateway_port.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_http_gateway_port_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.http_gateway_port
    }

    fn mut_http_gateway_port_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.http_gateway_port
    }
}

impl ::protobuf::Message for SysInfo {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.ip)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.hostname)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.gossip_ip)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.gossip_port)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.http_gateway_ip)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.http_gateway_port)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.ip.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.hostname.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(ref v) = self.gossip_ip.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        if let Some(ref v) = self.gossip_port.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        }
        if let Some(ref v) = self.http_gateway_ip.as_ref() {
            my_size += ::protobuf::rt::string_size(5, &v);
        }
        if let Some(ref v) = self.http_gateway_port.as_ref() {
            my_size += ::protobuf::rt::string_size(6, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.ip.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.hostname.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(ref v) = self.gossip_ip.as_ref() {
            os.write_string(3, &v)?;
        }
        if let Some(ref v) = self.gossip_port.as_ref() {
            os.write_string(4, &v)?;
        }
        if let Some(ref v) = self.http_gateway_ip.as_ref() {
            os.write_string(5, &v)?;
        }
        if let Some(ref v) = self.http_gateway_port.as_ref() {
            os.write_string(6, &v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SysInfo {
    fn new() -> SysInfo {
        SysInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<SysInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "ip",
                    SysInfo::get_ip_for_reflect,
                    SysInfo::mut_ip_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "hostname",
                    SysInfo::get_hostname_for_reflect,
                    SysInfo::mut_hostname_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "gossip_ip",
                    SysInfo::get_gossip_ip_for_reflect,
                    SysInfo::mut_gossip_ip_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "gossip_port",
                    SysInfo::get_gossip_port_for_reflect,
                    SysInfo::mut_gossip_port_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "http_gateway_ip",
                    SysInfo::get_http_gateway_ip_for_reflect,
                    SysInfo::mut_http_gateway_ip_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "http_gateway_port",
                    SysInfo::get_http_gateway_port_for_reflect,
                    SysInfo::mut_http_gateway_port_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SysInfo>(
                    "SysInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SysInfo {
    fn clear(&mut self) {
        self.clear_ip();
        self.clear_hostname();
        self.clear_gossip_ip();
        self.clear_gossip_port();
        self.clear_http_gateway_ip();
        self.clear_http_gateway_port();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SysInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SysInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PackageIdent {
    // message fields
    origin: ::protobuf::SingularField<::std::string::String>,
    name: ::protobuf::SingularField<::std::string::String>,
    version: ::protobuf::SingularField<::std::string::String>,
    release: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PackageIdent {}

impl PackageIdent {
    pub fn new() -> PackageIdent {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PackageIdent {
        static mut instance: ::protobuf::lazy::Lazy<PackageIdent> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PackageIdent,
        };
        unsafe {
            instance.get(PackageIdent::new)
        }
    }

    // optional string origin = 1;

    pub fn clear_origin(&mut self) {
        self.origin.clear();
    }

    pub fn has_origin(&self) -> bool {
        self.origin.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin(&mut self, v: ::std::string::String) {
        self.origin = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_origin(&mut self) -> &mut ::std::string::String {
        if self.origin.is_none() {
            self.origin.set_default();
        }
        self.origin.as_mut().unwrap()
    }

    // Take field
    pub fn take_origin(&mut self) -> ::std::string::String {
        self.origin.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_origin(&self) -> &str {
        match self.origin.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_origin_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.origin
    }

    fn mut_origin_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.origin
    }

    // optional string name = 2;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        }
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.name
    }

    // optional string version = 3;

    pub fn clear_version(&mut self) {
        self.version.clear();
    }

    pub fn has_version(&self) -> bool {
        self.version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: ::std::string::String) {
        self.version = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_version(&mut self) -> &mut ::std::string::String {
        if self.version.is_none() {
            self.version.set_default();
        }
        self.version.as_mut().unwrap()
    }

    // Take field
    pub fn take_version(&mut self) -> ::std::string::String {
        self.version.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_version(&self) -> &str {
        match self.version.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_version_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.version
    }

    fn mut_version_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.version
    }

    // optional string release = 4;

    pub fn clear_release(&mut self) {
        self.release.clear();
    }

    pub fn has_release(&self) -> bool {
        self.release.is_some()
    }

    // Param is passed by value, moved
    pub fn set_release(&mut self, v: ::std::string::String) {
        self.release = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_release(&mut self) -> &mut ::std::string::String {
        if self.release.is_none() {
            self.release.set_default();
        }
        self.release.as_mut().unwrap()
    }

    // Take field
    pub fn take_release(&mut self) -> ::std::string::String {
        self.release.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_release(&self) -> &str {
        match self.release.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_release_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.release
    }

    fn mut_release_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.release
    }
}

impl ::protobuf::Message for PackageIdent {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.origin)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.version)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.release)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.origin.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(ref v) = self.version.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        if let Some(ref v) = self.release.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.origin.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(ref v) = self.version.as_ref() {
            os.write_string(3, &v)?;
        }
        if let Some(ref v) = self.release.as_ref() {
            os.write_string(4, &v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PackageIdent {
    fn new() -> PackageIdent {
        PackageIdent::new()
    }

    fn descriptor_static(_: ::std::option::Option<PackageIdent>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "origin",
                    PackageIdent::get_origin_for_reflect,
                    PackageIdent::mut_origin_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    PackageIdent::get_name_for_reflect,
                    PackageIdent::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "version",
                    PackageIdent::get_version_for_reflect,
                    PackageIdent::mut_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "release",
                    PackageIdent::get_release_for_reflect,
                    PackageIdent::mut_release_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PackageIdent>(
                    "PackageIdent",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PackageIdent {
    fn clear(&mut self) {
        self.clear_origin();
        self.clear_name();
        self.clear_version();
        self.clear_release();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PackageIdent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PackageIdent {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CensusEntry {
    // message fields
    member_id: ::protobuf::SingularField<::std::string::String>,
    service: ::protobuf::SingularField<::std::string::String>,
    group: ::protobuf::SingularField<::std::string::String>,
    org: ::protobuf::SingularField<::std::string::String>,
    cfg: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    sys: ::protobuf::SingularPtrField<SysInfo>,
    pkg: ::protobuf::SingularPtrField<PackageIdent>,
    leader: ::std::option::Option<bool>,
    follower: ::std::option::Option<bool>,
    update_leader: ::std::option::Option<bool>,
    update_follower: ::std::option::Option<bool>,
    election_is_running: ::std::option::Option<bool>,
    election_is_no_quorum: ::std::option::Option<bool>,
    election_is_finished: ::std::option::Option<bool>,
    update_election_is_running: ::std::option::Option<bool>,
    update_election_is_no_quorum: ::std::option::Option<bool>,
    update_election_is_finished: ::std::option::Option<bool>,
    initialized: ::std::option::Option<bool>,
    alive: ::std::option::Option<bool>,
    suspect: ::std::option::Option<bool>,
    confirmed: ::std::option::Option<bool>,
    persistent: ::std::option::Option<bool>,
    departed: ::std::option::Option<bool>,
    application: ::protobuf::SingularField<::std::string::String>,
    environment: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CensusEntry {}

impl CensusEntry {
    pub fn new() -> CensusEntry {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CensusEntry {
        static mut instance: ::protobuf::lazy::Lazy<CensusEntry> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CensusEntry,
        };
        unsafe {
            instance.get(CensusEntry::new)
        }
    }

    // optional string member_id = 1;

    pub fn clear_member_id(&mut self) {
        self.member_id.clear();
    }

    pub fn has_member_id(&self) -> bool {
        self.member_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_member_id(&mut self, v: ::std::string::String) {
        self.member_id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_member_id(&mut self) -> &mut ::std::string::String {
        if self.member_id.is_none() {
            self.member_id.set_default();
        }
        self.member_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_member_id(&mut self) -> ::std::string::String {
        self.member_id.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_member_id(&self) -> &str {
        match self.member_id.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_member_id_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.member_id
    }

    fn mut_member_id_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.member_id
    }

    // optional string service = 2;

    pub fn clear_service(&mut self) {
        self.service.clear();
    }

    pub fn has_service(&self) -> bool {
        self.service.is_some()
    }

    // Param is passed by value, moved
    pub fn set_service(&mut self, v: ::std::string::String) {
        self.service = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_service(&mut self) -> &mut ::std::string::String {
        if self.service.is_none() {
            self.service.set_default();
        }
        self.service.as_mut().unwrap()
    }

    // Take field
    pub fn take_service(&mut self) -> ::std::string::String {
        self.service.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_service(&self) -> &str {
        match self.service.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_service_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.service
    }

    fn mut_service_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.service
    }

    // optional string group = 3;

    pub fn clear_group(&mut self) {
        self.group.clear();
    }

    pub fn has_group(&self) -> bool {
        self.group.is_some()
    }

    // Param is passed by value, moved
    pub fn set_group(&mut self, v: ::std::string::String) {
        self.group = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_group(&mut self) -> &mut ::std::string::String {
        if self.group.is_none() {
            self.group.set_default();
        }
        self.group.as_mut().unwrap()
    }

    // Take field
    pub fn take_group(&mut self) -> ::std::string::String {
        self.group.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_group(&self) -> &str {
        match self.group.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_group_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.group
    }

    fn mut_group_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.group
    }

    // optional string org = 4;

    pub fn clear_org(&mut self) {
        self.org.clear();
    }

    pub fn has_org(&self) -> bool {
        self.org.is_some()
    }

    // Param is passed by value, moved
    pub fn set_org(&mut self, v: ::std::string::String) {
        self.org = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_org(&mut self) -> &mut ::std::string::String {
        if self.org.is_none() {
            self.org.set_default();
        }
        self.org.as_mut().unwrap()
    }

    // Take field
    pub fn take_org(&mut self) -> ::std::string::String {
        self.org.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_org(&self) -> &str {
        match self.org.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_org_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.org
    }

    fn mut_org_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.org
    }

    // optional bytes cfg = 5;

    pub fn clear_cfg(&mut self) {
        self.cfg.clear();
    }

    pub fn has_cfg(&self) -> bool {
        self.cfg.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cfg(&mut self, v: ::std::vec::Vec<u8>) {
        self.cfg = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cfg(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.cfg.is_none() {
            self.cfg.set_default();
        }
        self.cfg.as_mut().unwrap()
    }

    // Take field
    pub fn take_cfg(&mut self) -> ::std::vec::Vec<u8> {
        self.cfg.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_cfg(&self) -> &[u8] {
        match self.cfg.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_cfg_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.cfg
    }

    fn mut_cfg_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.cfg
    }

    // optional .habitat.eventsrv.SysInfo sys = 6;

    pub fn clear_sys(&mut self) {
        self.sys.clear();
    }

    pub fn has_sys(&self) -> bool {
        self.sys.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sys(&mut self, v: SysInfo) {
        self.sys = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_sys(&mut self) -> &mut SysInfo {
        if self.sys.is_none() {
            self.sys.set_default();
        }
        self.sys.as_mut().unwrap()
    }

    // Take field
    pub fn take_sys(&mut self) -> SysInfo {
        self.sys.take().unwrap_or_else(|| SysInfo::new())
    }

    pub fn get_sys(&self) -> &SysInfo {
        self.sys.as_ref().unwrap_or_else(|| SysInfo::default_instance())
    }

    fn get_sys_for_reflect(&self) -> &::protobuf::SingularPtrField<SysInfo> {
        &self.sys
    }

    fn mut_sys_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<SysInfo> {
        &mut self.sys
    }

    // optional .habitat.eventsrv.PackageIdent pkg = 7;

    pub fn clear_pkg(&mut self) {
        self.pkg.clear();
    }

    pub fn has_pkg(&self) -> bool {
        self.pkg.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pkg(&mut self, v: PackageIdent) {
        self.pkg = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_pkg(&mut self) -> &mut PackageIdent {
        if self.pkg.is_none() {
            self.pkg.set_default();
        }
        self.pkg.as_mut().unwrap()
    }

    // Take field
    pub fn take_pkg(&mut self) -> PackageIdent {
        self.pkg.take().unwrap_or_else(|| PackageIdent::new())
    }

    pub fn get_pkg(&self) -> &PackageIdent {
        self.pkg.as_ref().unwrap_or_else(|| PackageIdent::default_instance())
    }

    fn get_pkg_for_reflect(&self) -> &::protobuf::SingularPtrField<PackageIdent> {
        &self.pkg
    }

    fn mut_pkg_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<PackageIdent> {
        &mut self.pkg
    }

    // optional bool leader = 8;

    pub fn clear_leader(&mut self) {
        self.leader = ::std::option::Option::None;
    }

    pub fn has_leader(&self) -> bool {
        self.leader.is_some()
    }

    // Param is passed by value, moved
    pub fn set_leader(&mut self, v: bool) {
        self.leader = ::std::option::Option::Some(v);
    }

    pub fn get_leader(&self) -> bool {
        self.leader.unwrap_or(false)
    }

    fn get_leader_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.leader
    }

    fn mut_leader_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.leader
    }

    // optional bool follower = 9;

    pub fn clear_follower(&mut self) {
        self.follower = ::std::option::Option::None;
    }

    pub fn has_follower(&self) -> bool {
        self.follower.is_some()
    }

    // Param is passed by value, moved
    pub fn set_follower(&mut self, v: bool) {
        self.follower = ::std::option::Option::Some(v);
    }

    pub fn get_follower(&self) -> bool {
        self.follower.unwrap_or(false)
    }

    fn get_follower_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.follower
    }

    fn mut_follower_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.follower
    }

    // optional bool update_leader = 10;

    pub fn clear_update_leader(&mut self) {
        self.update_leader = ::std::option::Option::None;
    }

    pub fn has_update_leader(&self) -> bool {
        self.update_leader.is_some()
    }

    // Param is passed by value, moved
    pub fn set_update_leader(&mut self, v: bool) {
        self.update_leader = ::std::option::Option::Some(v);
    }

    pub fn get_update_leader(&self) -> bool {
        self.update_leader.unwrap_or(false)
    }

    fn get_update_leader_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.update_leader
    }

    fn mut_update_leader_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.update_leader
    }

    // optional bool update_follower = 11;

    pub fn clear_update_follower(&mut self) {
        self.update_follower = ::std::option::Option::None;
    }

    pub fn has_update_follower(&self) -> bool {
        self.update_follower.is_some()
    }

    // Param is passed by value, moved
    pub fn set_update_follower(&mut self, v: bool) {
        self.update_follower = ::std::option::Option::Some(v);
    }

    pub fn get_update_follower(&self) -> bool {
        self.update_follower.unwrap_or(false)
    }

    fn get_update_follower_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.update_follower
    }

    fn mut_update_follower_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.update_follower
    }

    // optional bool election_is_running = 12;

    pub fn clear_election_is_running(&mut self) {
        self.election_is_running = ::std::option::Option::None;
    }

    pub fn has_election_is_running(&self) -> bool {
        self.election_is_running.is_some()
    }

    // Param is passed by value, moved
    pub fn set_election_is_running(&mut self, v: bool) {
        self.election_is_running = ::std::option::Option::Some(v);
    }

    pub fn get_election_is_running(&self) -> bool {
        self.election_is_running.unwrap_or(false)
    }

    fn get_election_is_running_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.election_is_running
    }

    fn mut_election_is_running_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.election_is_running
    }

    // optional bool election_is_no_quorum = 13;

    pub fn clear_election_is_no_quorum(&mut self) {
        self.election_is_no_quorum = ::std::option::Option::None;
    }

    pub fn has_election_is_no_quorum(&self) -> bool {
        self.election_is_no_quorum.is_some()
    }

    // Param is passed by value, moved
    pub fn set_election_is_no_quorum(&mut self, v: bool) {
        self.election_is_no_quorum = ::std::option::Option::Some(v);
    }

    pub fn get_election_is_no_quorum(&self) -> bool {
        self.election_is_no_quorum.unwrap_or(false)
    }

    fn get_election_is_no_quorum_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.election_is_no_quorum
    }

    fn mut_election_is_no_quorum_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.election_is_no_quorum
    }

    // optional bool election_is_finished = 14;

    pub fn clear_election_is_finished(&mut self) {
        self.election_is_finished = ::std::option::Option::None;
    }

    pub fn has_election_is_finished(&self) -> bool {
        self.election_is_finished.is_some()
    }

    // Param is passed by value, moved
    pub fn set_election_is_finished(&mut self, v: bool) {
        self.election_is_finished = ::std::option::Option::Some(v);
    }

    pub fn get_election_is_finished(&self) -> bool {
        self.election_is_finished.unwrap_or(false)
    }

    fn get_election_is_finished_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.election_is_finished
    }

    fn mut_election_is_finished_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.election_is_finished
    }

    // optional bool update_election_is_running = 15;

    pub fn clear_update_election_is_running(&mut self) {
        self.update_election_is_running = ::std::option::Option::None;
    }

    pub fn has_update_election_is_running(&self) -> bool {
        self.update_election_is_running.is_some()
    }

    // Param is passed by value, moved
    pub fn set_update_election_is_running(&mut self, v: bool) {
        self.update_election_is_running = ::std::option::Option::Some(v);
    }

    pub fn get_update_election_is_running(&self) -> bool {
        self.update_election_is_running.unwrap_or(false)
    }

    fn get_update_election_is_running_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.update_election_is_running
    }

    fn mut_update_election_is_running_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.update_election_is_running
    }

    // optional bool update_election_is_no_quorum = 16;

    pub fn clear_update_election_is_no_quorum(&mut self) {
        self.update_election_is_no_quorum = ::std::option::Option::None;
    }

    pub fn has_update_election_is_no_quorum(&self) -> bool {
        self.update_election_is_no_quorum.is_some()
    }

    // Param is passed by value, moved
    pub fn set_update_election_is_no_quorum(&mut self, v: bool) {
        self.update_election_is_no_quorum = ::std::option::Option::Some(v);
    }

    pub fn get_update_election_is_no_quorum(&self) -> bool {
        self.update_election_is_no_quorum.unwrap_or(false)
    }

    fn get_update_election_is_no_quorum_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.update_election_is_no_quorum
    }

    fn mut_update_election_is_no_quorum_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.update_election_is_no_quorum
    }

    // optional bool update_election_is_finished = 17;

    pub fn clear_update_election_is_finished(&mut self) {
        self.update_election_is_finished = ::std::option::Option::None;
    }

    pub fn has_update_election_is_finished(&self) -> bool {
        self.update_election_is_finished.is_some()
    }

    // Param is passed by value, moved
    pub fn set_update_election_is_finished(&mut self, v: bool) {
        self.update_election_is_finished = ::std::option::Option::Some(v);
    }

    pub fn get_update_election_is_finished(&self) -> bool {
        self.update_election_is_finished.unwrap_or(false)
    }

    fn get_update_election_is_finished_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.update_election_is_finished
    }

    fn mut_update_election_is_finished_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.update_election_is_finished
    }

    // optional bool initialized = 18;

    pub fn clear_initialized(&mut self) {
        self.initialized = ::std::option::Option::None;
    }

    pub fn has_initialized(&self) -> bool {
        self.initialized.is_some()
    }

    // Param is passed by value, moved
    pub fn set_initialized(&mut self, v: bool) {
        self.initialized = ::std::option::Option::Some(v);
    }

    pub fn get_initialized(&self) -> bool {
        self.initialized.unwrap_or(false)
    }

    fn get_initialized_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.initialized
    }

    fn mut_initialized_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.initialized
    }

    // optional bool alive = 19;

    pub fn clear_alive(&mut self) {
        self.alive = ::std::option::Option::None;
    }

    pub fn has_alive(&self) -> bool {
        self.alive.is_some()
    }

    // Param is passed by value, moved
    pub fn set_alive(&mut self, v: bool) {
        self.alive = ::std::option::Option::Some(v);
    }

    pub fn get_alive(&self) -> bool {
        self.alive.unwrap_or(false)
    }

    fn get_alive_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.alive
    }

    fn mut_alive_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.alive
    }

    // optional bool suspect = 20;

    pub fn clear_suspect(&mut self) {
        self.suspect = ::std::option::Option::None;
    }

    pub fn has_suspect(&self) -> bool {
        self.suspect.is_some()
    }

    // Param is passed by value, moved
    pub fn set_suspect(&mut self, v: bool) {
        self.suspect = ::std::option::Option::Some(v);
    }

    pub fn get_suspect(&self) -> bool {
        self.suspect.unwrap_or(false)
    }

    fn get_suspect_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.suspect
    }

    fn mut_suspect_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.suspect
    }

    // optional bool confirmed = 21;

    pub fn clear_confirmed(&mut self) {
        self.confirmed = ::std::option::Option::None;
    }

    pub fn has_confirmed(&self) -> bool {
        self.confirmed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_confirmed(&mut self, v: bool) {
        self.confirmed = ::std::option::Option::Some(v);
    }

    pub fn get_confirmed(&self) -> bool {
        self.confirmed.unwrap_or(false)
    }

    fn get_confirmed_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.confirmed
    }

    fn mut_confirmed_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.confirmed
    }

    // optional bool persistent = 22;

    pub fn clear_persistent(&mut self) {
        self.persistent = ::std::option::Option::None;
    }

    pub fn has_persistent(&self) -> bool {
        self.persistent.is_some()
    }

    // Param is passed by value, moved
    pub fn set_persistent(&mut self, v: bool) {
        self.persistent = ::std::option::Option::Some(v);
    }

    pub fn get_persistent(&self) -> bool {
        self.persistent.unwrap_or(false)
    }

    fn get_persistent_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.persistent
    }

    fn mut_persistent_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.persistent
    }

    // optional bool departed = 23;

    pub fn clear_departed(&mut self) {
        self.departed = ::std::option::Option::None;
    }

    pub fn has_departed(&self) -> bool {
        self.departed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_departed(&mut self, v: bool) {
        self.departed = ::std::option::Option::Some(v);
    }

    pub fn get_departed(&self) -> bool {
        self.departed.unwrap_or(false)
    }

    fn get_departed_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.departed
    }

    fn mut_departed_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.departed
    }

    // optional string application = 24;

    pub fn clear_application(&mut self) {
        self.application.clear();
    }

    pub fn has_application(&self) -> bool {
        self.application.is_some()
    }

    // Param is passed by value, moved
    pub fn set_application(&mut self, v: ::std::string::String) {
        self.application = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_application(&mut self) -> &mut ::std::string::String {
        if self.application.is_none() {
            self.application.set_default();
        }
        self.application.as_mut().unwrap()
    }

    // Take field
    pub fn take_application(&mut self) -> ::std::string::String {
        self.application.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_application(&self) -> &str {
        match self.application.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_application_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.application
    }

    fn mut_application_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.application
    }

    // optional string environment = 25;

    pub fn clear_environment(&mut self) {
        self.environment.clear();
    }

    pub fn has_environment(&self) -> bool {
        self.environment.is_some()
    }

    // Param is passed by value, moved
    pub fn set_environment(&mut self, v: ::std::string::String) {
        self.environment = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_environment(&mut self) -> &mut ::std::string::String {
        if self.environment.is_none() {
            self.environment.set_default();
        }
        self.environment.as_mut().unwrap()
    }

    // Take field
    pub fn take_environment(&mut self) -> ::std::string::String {
        self.environment.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_environment(&self) -> &str {
        match self.environment.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_environment_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.environment
    }

    fn mut_environment_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.environment
    }
}

impl ::protobuf::Message for CensusEntry {
    fn is_initialized(&self) -> bool {
        for v in &self.sys {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.pkg {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.member_id)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.service)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.group)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.org)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.cfg)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.sys)?;
                },
                7 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.pkg)?;
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.leader = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.follower = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.update_leader = ::std::option::Option::Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.update_follower = ::std::option::Option::Some(tmp);
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.election_is_running = ::std::option::Option::Some(tmp);
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.election_is_no_quorum = ::std::option::Option::Some(tmp);
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.election_is_finished = ::std::option::Option::Some(tmp);
                },
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.update_election_is_running = ::std::option::Option::Some(tmp);
                },
                16 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.update_election_is_no_quorum = ::std::option::Option::Some(tmp);
                },
                17 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.update_election_is_finished = ::std::option::Option::Some(tmp);
                },
                18 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.initialized = ::std::option::Option::Some(tmp);
                },
                19 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.alive = ::std::option::Option::Some(tmp);
                },
                20 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.suspect = ::std::option::Option::Some(tmp);
                },
                21 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.confirmed = ::std::option::Option::Some(tmp);
                },
                22 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.persistent = ::std::option::Option::Some(tmp);
                },
                23 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.departed = ::std::option::Option::Some(tmp);
                },
                24 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.application)?;
                },
                25 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.environment)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.member_id.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.service.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(ref v) = self.group.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        if let Some(ref v) = self.org.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        }
        if let Some(ref v) = self.cfg.as_ref() {
            my_size += ::protobuf::rt::bytes_size(5, &v);
        }
        if let Some(ref v) = self.sys.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.pkg.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.leader {
            my_size += 2;
        }
        if let Some(v) = self.follower {
            my_size += 2;
        }
        if let Some(v) = self.update_leader {
            my_size += 2;
        }
        if let Some(v) = self.update_follower {
            my_size += 2;
        }
        if let Some(v) = self.election_is_running {
            my_size += 2;
        }
        if let Some(v) = self.election_is_no_quorum {
            my_size += 2;
        }
        if let Some(v) = self.election_is_finished {
            my_size += 2;
        }
        if let Some(v) = self.update_election_is_running {
            my_size += 2;
        }
        if let Some(v) = self.update_election_is_no_quorum {
            my_size += 3;
        }
        if let Some(v) = self.update_election_is_finished {
            my_size += 3;
        }
        if let Some(v) = self.initialized {
            my_size += 3;
        }
        if let Some(v) = self.alive {
            my_size += 3;
        }
        if let Some(v) = self.suspect {
            my_size += 3;
        }
        if let Some(v) = self.confirmed {
            my_size += 3;
        }
        if let Some(v) = self.persistent {
            my_size += 3;
        }
        if let Some(v) = self.departed {
            my_size += 3;
        }
        if let Some(ref v) = self.application.as_ref() {
            my_size += ::protobuf::rt::string_size(24, &v);
        }
        if let Some(ref v) = self.environment.as_ref() {
            my_size += ::protobuf::rt::string_size(25, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.member_id.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.service.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(ref v) = self.group.as_ref() {
            os.write_string(3, &v)?;
        }
        if let Some(ref v) = self.org.as_ref() {
            os.write_string(4, &v)?;
        }
        if let Some(ref v) = self.cfg.as_ref() {
            os.write_bytes(5, &v)?;
        }
        if let Some(ref v) = self.sys.as_ref() {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.pkg.as_ref() {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.leader {
            os.write_bool(8, v)?;
        }
        if let Some(v) = self.follower {
            os.write_bool(9, v)?;
        }
        if let Some(v) = self.update_leader {
            os.write_bool(10, v)?;
        }
        if let Some(v) = self.update_follower {
            os.write_bool(11, v)?;
        }
        if let Some(v) = self.election_is_running {
            os.write_bool(12, v)?;
        }
        if let Some(v) = self.election_is_no_quorum {
            os.write_bool(13, v)?;
        }
        if let Some(v) = self.election_is_finished {
            os.write_bool(14, v)?;
        }
        if let Some(v) = self.update_election_is_running {
            os.write_bool(15, v)?;
        }
        if let Some(v) = self.update_election_is_no_quorum {
            os.write_bool(16, v)?;
        }
        if let Some(v) = self.update_election_is_finished {
            os.write_bool(17, v)?;
        }
        if let Some(v) = self.initialized {
            os.write_bool(18, v)?;
        }
        if let Some(v) = self.alive {
            os.write_bool(19, v)?;
        }
        if let Some(v) = self.suspect {
            os.write_bool(20, v)?;
        }
        if let Some(v) = self.confirmed {
            os.write_bool(21, v)?;
        }
        if let Some(v) = self.persistent {
            os.write_bool(22, v)?;
        }
        if let Some(v) = self.departed {
            os.write_bool(23, v)?;
        }
        if let Some(ref v) = self.application.as_ref() {
            os.write_string(24, &v)?;
        }
        if let Some(ref v) = self.environment.as_ref() {
            os.write_string(25, &v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CensusEntry {
    fn new() -> CensusEntry {
        CensusEntry::new()
    }

    fn descriptor_static(_: ::std::option::Option<CensusEntry>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "member_id",
                    CensusEntry::get_member_id_for_reflect,
                    CensusEntry::mut_member_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "service",
                    CensusEntry::get_service_for_reflect,
                    CensusEntry::mut_service_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "group",
                    CensusEntry::get_group_for_reflect,
                    CensusEntry::mut_group_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "org",
                    CensusEntry::get_org_for_reflect,
                    CensusEntry::mut_org_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "cfg",
                    CensusEntry::get_cfg_for_reflect,
                    CensusEntry::mut_cfg_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<SysInfo>>(
                    "sys",
                    CensusEntry::get_sys_for_reflect,
                    CensusEntry::mut_sys_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<PackageIdent>>(
                    "pkg",
                    CensusEntry::get_pkg_for_reflect,
                    CensusEntry::mut_pkg_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "leader",
                    CensusEntry::get_leader_for_reflect,
                    CensusEntry::mut_leader_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "follower",
                    CensusEntry::get_follower_for_reflect,
                    CensusEntry::mut_follower_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "update_leader",
                    CensusEntry::get_update_leader_for_reflect,
                    CensusEntry::mut_update_leader_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "update_follower",
                    CensusEntry::get_update_follower_for_reflect,
                    CensusEntry::mut_update_follower_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "election_is_running",
                    CensusEntry::get_election_is_running_for_reflect,
                    CensusEntry::mut_election_is_running_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "election_is_no_quorum",
                    CensusEntry::get_election_is_no_quorum_for_reflect,
                    CensusEntry::mut_election_is_no_quorum_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "election_is_finished",
                    CensusEntry::get_election_is_finished_for_reflect,
                    CensusEntry::mut_election_is_finished_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "update_election_is_running",
                    CensusEntry::get_update_election_is_running_for_reflect,
                    CensusEntry::mut_update_election_is_running_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "update_election_is_no_quorum",
                    CensusEntry::get_update_election_is_no_quorum_for_reflect,
                    CensusEntry::mut_update_election_is_no_quorum_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "update_election_is_finished",
                    CensusEntry::get_update_election_is_finished_for_reflect,
                    CensusEntry::mut_update_election_is_finished_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "initialized",
                    CensusEntry::get_initialized_for_reflect,
                    CensusEntry::mut_initialized_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "alive",
                    CensusEntry::get_alive_for_reflect,
                    CensusEntry::mut_alive_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "suspect",
                    CensusEntry::get_suspect_for_reflect,
                    CensusEntry::mut_suspect_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "confirmed",
                    CensusEntry::get_confirmed_for_reflect,
                    CensusEntry::mut_confirmed_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "persistent",
                    CensusEntry::get_persistent_for_reflect,
                    CensusEntry::mut_persistent_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "departed",
                    CensusEntry::get_departed_for_reflect,
                    CensusEntry::mut_departed_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "application",
                    CensusEntry::get_application_for_reflect,
                    CensusEntry::mut_application_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "environment",
                    CensusEntry::get_environment_for_reflect,
                    CensusEntry::mut_environment_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CensusEntry>(
                    "CensusEntry",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CensusEntry {
    fn clear(&mut self) {
        self.clear_member_id();
        self.clear_service();
        self.clear_group();
        self.clear_org();
        self.clear_cfg();
        self.clear_sys();
        self.clear_pkg();
        self.clear_leader();
        self.clear_follower();
        self.clear_update_leader();
        self.clear_update_follower();
        self.clear_election_is_running();
        self.clear_election_is_no_quorum();
        self.clear_election_is_finished();
        self.clear_update_election_is_running();
        self.clear_update_election_is_no_quorum();
        self.clear_update_election_is_finished();
        self.clear_initialized();
        self.clear_alive();
        self.clear_suspect();
        self.clear_confirmed();
        self.clear_persistent();
        self.clear_departed();
        self.clear_application();
        self.clear_environment();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CensusEntry {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CensusEntry {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x15protocols/event.proto\x12\x10habitat.eventsrv\"\xa5\x02\n\rEventEn\
    velope\x128\n\x04type\x18\x01\x20\x01(\x0e2$.habitat.eventsrv.EventEnvel\
    ope.TypeR\x04type\x12\x18\n\x07payload\x18\x02\x20\x01(\x0cR\x07payload\
    \x12\x1c\n\ttimestamp\x18\x03\x20\x01(\x04R\ttimestamp\x12\x1b\n\tmember\
    _id\x18\x04\x20\x01(\tR\x08memberId\x12\x18\n\x07service\x18\x05\x20\x01\
    (\tR\x07service\x12\x20\n\x0bincarnation\x18\x06\x20\x01(\x04R\x0bincarn\
    ation\x12\x1f\n\x0bsequence_id\x18\x07\x20\x01(\x04R\nsequenceId\"(\n\
    \x04Type\x12\x0c\n\x08ProtoBuf\x10\x01\x12\x08\n\x04JSON\x10\x02\x12\x08\
    \n\x04TOML\x10\x03\"\xc7\x01\n\x07SysInfo\x12\x0e\n\x02ip\x18\x01\x20\
    \x01(\tR\x02ip\x12\x1a\n\x08hostname\x18\x02\x20\x01(\tR\x08hostname\x12\
    \x1b\n\tgossip_ip\x18\x03\x20\x01(\tR\x08gossipIp\x12\x1f\n\x0bgossip_po\
    rt\x18\x04\x20\x01(\tR\ngossipPort\x12&\n\x0fhttp_gateway_ip\x18\x05\x20\
    \x01(\tR\rhttpGatewayIp\x12*\n\x11http_gateway_port\x18\x06\x20\x01(\tR\
    \x0fhttpGatewayPort\"n\n\x0cPackageIdent\x12\x16\n\x06origin\x18\x01\x20\
    \x01(\tR\x06origin\x12\x12\n\x04name\x18\x02\x20\x01(\tR\x04name\x12\x18\
    \n\x07version\x18\x03\x20\x01(\tR\x07version\x12\x18\n\x07release\x18\
    \x04\x20\x01(\tR\x07release\"\xa0\x07\n\x0bCensusEntry\x12\x1b\n\tmember\
    _id\x18\x01\x20\x01(\tR\x08memberId\x12\x18\n\x07service\x18\x02\x20\x01\
    (\tR\x07service\x12\x14\n\x05group\x18\x03\x20\x01(\tR\x05group\x12\x10\
    \n\x03org\x18\x04\x20\x01(\tR\x03org\x12\x10\n\x03cfg\x18\x05\x20\x01(\
    \x0cR\x03cfg\x12+\n\x03sys\x18\x06\x20\x01(\x0b2\x19.habitat.eventsrv.Sy\
    sInfoR\x03sys\x120\n\x03pkg\x18\x07\x20\x01(\x0b2\x1e.habitat.eventsrv.P\
    ackageIdentR\x03pkg\x12\x16\n\x06leader\x18\x08\x20\x01(\x08R\x06leader\
    \x12\x1a\n\x08follower\x18\t\x20\x01(\x08R\x08follower\x12#\n\rupdate_le\
    ader\x18\n\x20\x01(\x08R\x0cupdateLeader\x12'\n\x0fupdate_follower\x18\
    \x0b\x20\x01(\x08R\x0eupdateFollower\x12.\n\x13election_is_running\x18\
    \x0c\x20\x01(\x08R\x11electionIsRunning\x121\n\x15election_is_no_quorum\
    \x18\r\x20\x01(\x08R\x12electionIsNoQuorum\x120\n\x14election_is_finishe\
    d\x18\x0e\x20\x01(\x08R\x12electionIsFinished\x12;\n\x1aupdate_election_\
    is_running\x18\x0f\x20\x01(\x08R\x17updateElectionIsRunning\x12>\n\x1cup\
    date_election_is_no_quorum\x18\x10\x20\x01(\x08R\x18updateElectionIsNoQu\
    orum\x12=\n\x1bupdate_election_is_finished\x18\x11\x20\x01(\x08R\x18upda\
    teElectionIsFinished\x12\x20\n\x0binitialized\x18\x12\x20\x01(\x08R\x0bi\
    nitialized\x12\x14\n\x05alive\x18\x13\x20\x01(\x08R\x05alive\x12\x18\n\
    \x07suspect\x18\x14\x20\x01(\x08R\x07suspect\x12\x1c\n\tconfirmed\x18\
    \x15\x20\x01(\x08R\tconfirmed\x12\x1e\n\npersistent\x18\x16\x20\x01(\x08\
    R\npersistent\x12\x1a\n\x08departed\x18\x17\x20\x01(\x08R\x08departed\
    \x12\x20\n\x0bapplication\x18\x18\x20\x01(\tR\x0bapplication\x12\x20\n\
    \x0benvironment\x18\x19\x20\x01(\tR\x0benvironmentJ\xa81\n\x06\x12\x04\0\
    \0{\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\nD\n\x01\x02\x12\x03\x03\x08\x18\
    \x1a:/\x20Messages\x20sent\x20to\x20an\x20EventSrv\x20from\x20a\x20Habit\
    at\x20Supervisor.\n\n\xdd\x01\n\x02\x04\0\x12\x04\x08\0!\x01\x1a\xd0\x01\
    /\x20The\x20base\x20for\x20all\x20messages\x20generated\x20by\x20an\x20E\
    ventSrv.\x20This\x20message\x20contains\x20framing\n/\x20to\x20hint\x20t\
    o\x20a\x20consumer\x20how\x20to\x20encode/decode\x20the\x20message's\x20\
    payload\x20and\x20information\x20for\n/\x20how\x20to\x20route\x20or\x20i\
    ndex\x20the\x20message.\n\n\n\n\x03\x04\0\x01\x12\x03\x08\x08\x15\nR\n\
    \x04\x04\0\x04\0\x12\x04\n\x02\x11\x03\x1aD/\x20Enumerator\x20of\x20pote\
    ntial\x20encoding\x20types\x20for\x20the\x20Envelope's\x20payload\n\n\
    \x0c\n\x05\x04\0\x04\0\x01\x12\x03\n\x07\x0b\n0\n\x06\x04\0\x04\0\x02\0\
    \x12\x03\x0c\x04\x11\x1a!/\x20Encoded\x20with\x20a\x20Google\x20Protobuf\
    \n\n\x0e\n\x07\x04\0\x04\0\x02\0\x01\x12\x03\x0c\x04\x0c\n\x0e\n\x07\x04\
    \0\x04\0\x02\0\x02\x12\x03\x0c\x0f\x10\n#\n\x06\x04\0\x04\0\x02\x01\x12\
    \x03\x0e\x04\r\x1a\x14/\x20Encoded\x20with\x20JSON\n\n\x0e\n\x07\x04\0\
    \x04\0\x02\x01\x01\x12\x03\x0e\x04\x08\n\x0e\n\x07\x04\0\x04\0\x02\x01\
    \x02\x12\x03\x0e\x0b\x0c\n#\n\x06\x04\0\x04\0\x02\x02\x12\x03\x10\x04\r\
    \x1a\x14/\x20Encoded\x20with\x20TOML\n\n\x0e\n\x07\x04\0\x04\0\x02\x02\
    \x01\x12\x03\x10\x04\x08\n\x0e\n\x07\x04\0\x04\0\x02\x02\x02\x12\x03\x10\
    \x0b\x0c\n1\n\x04\x04\0\x02\0\x12\x03\x14\x02\x19\x1a$/\x20Message\x20pa\
    yload\x20hint\x20to\x20a\x20decoder\n\n\x0c\n\x05\x04\0\x02\0\x04\x12\
    \x03\x14\x02\n\n\x0c\n\x05\x04\0\x02\0\x06\x12\x03\x14\x0b\x0f\n\x0c\n\
    \x05\x04\0\x02\0\x01\x12\x03\x14\x10\x14\n\x0c\n\x05\x04\0\x02\0\x03\x12\
    \x03\x14\x17\x18\n#\n\x04\x04\0\x02\x01\x12\x03\x16\x02\x1d\x1a\x16/\x20\
    Contents\x20of\x20message\n\n\x0c\n\x05\x04\0\x02\x01\x04\x12\x03\x16\
    \x02\n\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\x16\x0b\x10\n\x0c\n\x05\x04\
    \0\x02\x01\x01\x12\x03\x16\x11\x18\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\
    \x16\x1b\x1c\nc\n\x04\x04\0\x02\x02\x12\x03\x18\x02\x20\x1aV/\x20Time\
    \x20of\x20message\x20origination\x20in\x20milliseconds\x20since\x20the\
    \x20Epoch\x20(1970-01-01T00:00:00Z).\n\n\x0c\n\x05\x04\0\x02\x02\x04\x12\
    \x03\x18\x02\n\n\x0c\n\x05\x04\0\x02\x02\x05\x12\x03\x18\x0b\x11\n\x0c\n\
    \x05\x04\0\x02\x02\x01\x12\x03\x18\x12\x1b\n\x0c\n\x05\x04\0\x02\x02\x03\
    \x12\x03\x18\x1e\x1f\n3\n\x04\x04\0\x02\x03\x12\x03\x1a\x02\x20\x1a&/\
    \x20Member-ID\x20of\x20originating\x20Supervisor\n\n\x0c\n\x05\x04\0\x02\
    \x03\x04\x12\x03\x1a\x02\n\n\x0c\n\x05\x04\0\x02\x03\x05\x12\x03\x1a\x0b\
    \x11\n\x0c\n\x05\x04\0\x02\x03\x01\x12\x03\x1a\x12\x1b\n\x0c\n\x05\x04\0\
    \x02\x03\x03\x12\x03\x1a\x1e\x1f\n>\n\x04\x04\0\x02\x04\x12\x03\x1c\x02\
    \x1e\x1a1/\x20Service\x20name\x20of\x20originating\x20Supervisor\x20serv\
    ice\n\n\x0c\n\x05\x04\0\x02\x04\x04\x12\x03\x1c\x02\n\n\x0c\n\x05\x04\0\
    \x02\x04\x05\x12\x03\x1c\x0b\x11\n\x0c\n\x05\x04\0\x02\x04\x01\x12\x03\
    \x1c\x12\x19\n\x0c\n\x05\x04\0\x02\x04\x03\x12\x03\x1c\x1c\x1d\n?\n\x04\
    \x04\0\x02\x05\x12\x03\x1e\x02\"\x1a2/\x20Supervisor's\x20incarnation\
    \x20at\x20message\x20origination\n\n\x0c\n\x05\x04\0\x02\x05\x04\x12\x03\
    \x1e\x02\n\n\x0c\n\x05\x04\0\x02\x05\x05\x12\x03\x1e\x0b\x11\n\x0c\n\x05\
    \x04\0\x02\x05\x01\x12\x03\x1e\x12\x1d\n\x0c\n\x05\x04\0\x02\x05\x03\x12\
    \x03\x1e\x20!\n%\n\x04\x04\0\x02\x06\x12\x03\x20\x02\"\x1a\x18/\x20Messa\
    ge's\x20sequence\x20ID\n\n\x0c\n\x05\x04\0\x02\x06\x04\x12\x03\x20\x02\n\
    \n\x0c\n\x05\x04\0\x02\x06\x05\x12\x03\x20\x0b\x11\n\x0c\n\x05\x04\0\x02\
    \x06\x01\x12\x03\x20\x12\x1d\n\x0c\n\x05\x04\0\x02\x06\x03\x12\x03\x20\
    \x20!\n^\n\x02\x04\x01\x12\x04$\01\x01\x1aR/\x20System\x20information\
    \x20generated\x20by\x20the\x20Supervisor\x20of\x20the\x20machine\x20it\
    \x20is\x20running\x20on.\n\n\n\n\x03\x04\x01\x01\x12\x03$\x08\x0f\n6\n\
    \x04\x04\x01\x02\0\x12\x03&\x02\x19\x1a)/\x20Public\x20facing\x20IP\x20a\
    ddress\x20of\x20Supervisor\n\n\x0c\n\x05\x04\x01\x02\0\x04\x12\x03&\x02\
    \n\n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03&\x0b\x11\n\x0c\n\x05\x04\x01\
    \x02\0\x01\x12\x03&\x12\x14\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03&\x17\
    \x18\n.\n\x04\x04\x01\x02\x01\x12\x03(\x02\x1f\x1a!/\x20Network\x20hostn\
    ame\x20of\x20Supervisor\n\n\x0c\n\x05\x04\x01\x02\x01\x04\x12\x03(\x02\n\
    \n\x0c\n\x05\x04\x01\x02\x01\x05\x12\x03(\x0b\x11\n\x0c\n\x05\x04\x01\
    \x02\x01\x01\x12\x03(\x12\x1a\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x03(\
    \x1d\x1e\nD\n\x04\x04\x01\x02\x02\x12\x03*\x02\x20\x1a7/\x20Listening\
    \x20address\x20for\x20Supervisor's\x20gossip\x20connection\n\n\x0c\n\x05\
    \x04\x01\x02\x02\x04\x12\x03*\x02\n\n\x0c\n\x05\x04\x01\x02\x02\x05\x12\
    \x03*\x0b\x11\n\x0c\n\x05\x04\x01\x02\x02\x01\x12\x03*\x12\x1b\n\x0c\n\
    \x05\x04\x01\x02\x02\x03\x12\x03*\x1e\x1f\nA\n\x04\x04\x01\x02\x03\x12\
    \x03,\x02\"\x1a4/\x20Listening\x20port\x20for\x20Supervisor's\x20gossip\
    \x20connection\n\n\x0c\n\x05\x04\x01\x02\x03\x04\x12\x03,\x02\n\n\x0c\n\
    \x05\x04\x01\x02\x03\x05\x12\x03,\x0b\x11\n\x0c\n\x05\x04\x01\x02\x03\
    \x01\x12\x03,\x12\x1d\n\x0c\n\x05\x04\x01\x02\x03\x03\x12\x03,\x20!\n?\n\
    \x04\x04\x01\x02\x04\x12\x03.\x02&\x1a2/\x20Listening\x20address\x20for\
    \x20Supervisor's\x20http\x20gateway\n\n\x0c\n\x05\x04\x01\x02\x04\x04\
    \x12\x03.\x02\n\n\x0c\n\x05\x04\x01\x02\x04\x05\x12\x03.\x0b\x11\n\x0c\n\
    \x05\x04\x01\x02\x04\x01\x12\x03.\x12!\n\x0c\n\x05\x04\x01\x02\x04\x03\
    \x12\x03.$%\n<\n\x04\x04\x01\x02\x05\x12\x030\x02(\x1a//\x20Listening\
    \x20port\x20for\x20Supervisor's\x20http\x20gateway\n\n\x0c\n\x05\x04\x01\
    \x02\x05\x04\x12\x030\x02\n\n\x0c\n\x05\x04\x01\x02\x05\x05\x12\x030\x0b\
    \x11\n\x0c\n\x05\x04\x01\x02\x05\x01\x12\x030\x12#\n\x0c\n\x05\x04\x01\
    \x02\x05\x03\x12\x030&'\nG\n\x02\x04\x02\x12\x044\0=\x01\x1a;/\x20Inform\
    ation\x20describing\x20the\x20package\x20a\x20service\x20is\x20running.\
    \n\n\n\n\x03\x04\x02\x01\x12\x034\x08\x14\n&\n\x04\x04\x02\x02\0\x12\x03\
    6\x02\x1d\x1a\x19/\x20Origin\x20name\x20of\x20package\n\n\x0c\n\x05\x04\
    \x02\x02\0\x04\x12\x036\x02\n\n\x0c\n\x05\x04\x02\x02\0\x05\x12\x036\x0b\
    \x11\n\x0c\n\x05\x04\x02\x02\0\x01\x12\x036\x12\x18\n\x0c\n\x05\x04\x02\
    \x02\0\x03\x12\x036\x1b\x1c\n(\n\x04\x04\x02\x02\x01\x12\x038\x02\x1b\
    \x1a\x1b/\x20Software\x20name\x20of\x20package\n\n\x0c\n\x05\x04\x02\x02\
    \x01\x04\x12\x038\x02\n\n\x0c\n\x05\x04\x02\x02\x01\x05\x12\x038\x0b\x11\
    \n\x0c\n\x05\x04\x02\x02\x01\x01\x12\x038\x12\x16\n\x0c\n\x05\x04\x02\
    \x02\x01\x03\x12\x038\x19\x1a\n+\n\x04\x04\x02\x02\x02\x12\x03:\x02\x1e\
    \x1a\x1e/\x20Software\x20version\x20of\x20package\n\n\x0c\n\x05\x04\x02\
    \x02\x02\x04\x12\x03:\x02\n\n\x0c\n\x05\x04\x02\x02\x02\x05\x12\x03:\x0b\
    \x11\n\x0c\n\x05\x04\x02\x02\x02\x01\x12\x03:\x12\x19\n\x0c\n\x05\x04\
    \x02\x02\x02\x03\x12\x03:\x1c\x1d\n2\n\x04\x04\x02\x02\x03\x12\x03<\x02\
    \x1e\x1a%/\x20Build\x20release\x20timestamp\x20of\x20package\n\n\x0c\n\
    \x05\x04\x02\x02\x03\x04\x12\x03<\x02\n\n\x0c\n\x05\x04\x02\x02\x03\x05\
    \x12\x03<\x0b\x11\n\x0c\n\x05\x04\x02\x02\x03\x01\x12\x03<\x12\x19\n\x0c\
    \n\x05\x04\x02\x02\x03\x03\x12\x03<\x1c\x1d\nk\n\x02\x04\x03\x12\x04@\0{\
    \x01\x1a_/\x20Generated\x20by\x20gossip\x20information\x20from\x20Superv\
    isors\x20representing\x20a\x20single\x20member\x20of\x20the\x20Census.\n\
    \n\n\n\x03\x04\x03\x01\x12\x03@\x08\x13\n3\n\x04\x04\x03\x02\0\x12\x03B\
    \x02\x20\x1a&/\x20Member-ID\x20of\x20originating\x20Supervisor\n\n\x0c\n\
    \x05\x04\x03\x02\0\x04\x12\x03B\x02\n\n\x0c\n\x05\x04\x03\x02\0\x05\x12\
    \x03B\x0b\x11\n\x0c\n\x05\x04\x03\x02\0\x01\x12\x03B\x12\x1b\n\x0c\n\x05\
    \x04\x03\x02\0\x03\x12\x03B\x1e\x1f\n\x1c\n\x04\x04\x03\x02\x01\x12\x03E\
    \x02\x1e\x1a\x0f/\x20Service\x20name\n\n\x0c\n\x05\x04\x03\x02\x01\x04\
    \x12\x03E\x02\n\n\x0c\n\x05\x04\x03\x02\x01\x05\x12\x03E\x0b\x11\n\x0c\n\
    \x05\x04\x03\x02\x01\x01\x12\x03E\x12\x19\n\x0c\n\x05\x04\x03\x02\x01\
    \x03\x12\x03E\x1c\x1d\n\"\n\x04\x04\x03\x02\x02\x12\x03G\x02\x1c\x1a\x15\
    /\x20Service\x20group\x20name\n\n\x0c\n\x05\x04\x03\x02\x02\x04\x12\x03G\
    \x02\n\n\x0c\n\x05\x04\x03\x02\x02\x05\x12\x03G\x0b\x11\n\x0c\n\x05\x04\
    \x03\x02\x02\x01\x12\x03G\x12\x17\n\x0c\n\x05\x04\x03\x02\x02\x03\x12\
    \x03G\x1a\x1b\n\x82\x01\n\x04\x04\x03\x02\x03\x12\x03I\x02\x1a\x1a\"/\
    \x20Service\x20group\x20organization\x20name\n\"Q\x20NOTE:\x20service\
    \x20group\x20application\x20and\x20environment\x20are\x20further\x20down\
    \n\x20in\x20this\x20file.\n\n\x0c\n\x05\x04\x03\x02\x03\x04\x12\x03I\x02\
    \n\n\x0c\n\x05\x04\x03\x02\x03\x05\x12\x03I\x0b\x11\n\x0c\n\x05\x04\x03\
    \x02\x03\x01\x12\x03I\x12\x15\n\x0c\n\x05\x04\x03\x02\x03\x03\x12\x03I\
    \x18\x19\n1\n\x04\x04\x03\x02\x04\x12\x03N\x02\x19\x1a$/\x20Gossiped\x20\
    configuration\x20of\x20service\n\n\x0c\n\x05\x04\x03\x02\x04\x04\x12\x03\
    N\x02\n\n\x0c\n\x05\x04\x03\x02\x04\x05\x12\x03N\x0b\x10\n\x0c\n\x05\x04\
    \x03\x02\x04\x01\x12\x03N\x11\x14\n\x0c\n\x05\x04\x03\x02\x04\x03\x12\
    \x03N\x17\x18\n0\n\x04\x04\x03\x02\x05\x12\x03P\x02\x1b\x1a#/\x20System\
    \x20information\x20of\x20Supervisor\n\n\x0c\n\x05\x04\x03\x02\x05\x04\
    \x12\x03P\x02\n\n\x0c\n\x05\x04\x03\x02\x05\x06\x12\x03P\x0b\x12\n\x0c\n\
    \x05\x04\x03\x02\x05\x01\x12\x03P\x13\x16\n\x0c\n\x05\x04\x03\x02\x05\
    \x03\x12\x03P\x19\x1a\n.\n\x04\x04\x03\x02\x06\x12\x03R\x02\x20\x1a!/\
    \x20Package\x20information\x20of\x20service\n\n\x0c\n\x05\x04\x03\x02\
    \x06\x04\x12\x03R\x02\n\n\x0c\n\x05\x04\x03\x02\x06\x06\x12\x03R\x0b\x17\
    \n\x0c\n\x05\x04\x03\x02\x06\x01\x12\x03R\x18\x1b\n\x0c\n\x05\x04\x03\
    \x02\x06\x03\x12\x03R\x1e\x1f\nW\n\x04\x04\x03\x02\x07\x12\x03T\x02\x1b\
    \x1aJ/\x20`true`\x20if\x20this\x20service\x20instance\x20is\x20the\x20le\
    ader\x20when\x20in\x20a\x20leader\x20topology\n\n\x0c\n\x05\x04\x03\x02\
    \x07\x04\x12\x03T\x02\n\n\x0c\n\x05\x04\x03\x02\x07\x05\x12\x03T\x0b\x0f\
    \n\x0c\n\x05\x04\x03\x02\x07\x01\x12\x03T\x10\x16\n\x0c\n\x05\x04\x03\
    \x02\x07\x03\x12\x03T\x19\x1a\nW\n\x04\x04\x03\x02\x08\x12\x03V\x02\x1d\
    \x1aJ/\x20`true`\x20if\x20this\x20service\x20instance\x20is\x20a\x20foll\
    ower\x20when\x20in\x20a\x20leader\x20topology\n\n\x0c\n\x05\x04\x03\x02\
    \x08\x04\x12\x03V\x02\n\n\x0c\n\x05\x04\x03\x02\x08\x05\x12\x03V\x0b\x0f\
    \n\x0c\n\x05\x04\x03\x02\x08\x01\x12\x03V\x10\x18\n\x0c\n\x05\x04\x03\
    \x02\x08\x03\x12\x03V\x1b\x1c\nj\n\x04\x04\x03\x02\t\x12\x03X\x02#\x1a]/\
    \x20`true`\x20if\x20this\x20service\x20instance\x20is\x20the\x20update\
    \x20leader\x20when\x20in\x20a\x20coordinated\x20update\x20topology\n\n\
    \x0c\n\x05\x04\x03\x02\t\x04\x12\x03X\x02\n\n\x0c\n\x05\x04\x03\x02\t\
    \x05\x12\x03X\x0b\x0f\n\x0c\n\x05\x04\x03\x02\t\x01\x12\x03X\x10\x1d\n\
    \x0c\n\x05\x04\x03\x02\t\x03\x12\x03X\x20\"\ni\n\x04\x04\x03\x02\n\x12\
    \x03Z\x02%\x1a\\/\x20`true`\x20if\x20this\x20service\x20instance\x20is\
    \x20an\x20update\x20leader\x20when\x20in\x20a\x20coordinated\x20update\
    \x20topology\n\n\x0c\n\x05\x04\x03\x02\n\x04\x12\x03Z\x02\n\n\x0c\n\x05\
    \x04\x03\x02\n\x05\x12\x03Z\x0b\x0f\n\x0c\n\x05\x04\x03\x02\n\x01\x12\
    \x03Z\x10\x1f\n\x0c\n\x05\x04\x03\x02\n\x03\x12\x03Z\"$\nl\n\x04\x04\x03\
    \x02\x0b\x12\x03\\\x02)\x1a_/\x20`true`\x20if\x20this\x20service\x20inst\
    ance\x20is\x20part\x20of\x20a\x20topology\x20and\x20an\x20election\x20is\
    \x20currently\x20under\x20way\n\n\x0c\n\x05\x04\x03\x02\x0b\x04\x12\x03\
    \\\x02\n\n\x0c\n\x05\x04\x03\x02\x0b\x05\x12\x03\\\x0b\x0f\n\x0c\n\x05\
    \x04\x03\x02\x0b\x01\x12\x03\\\x10#\n\x0c\n\x05\x04\x03\x02\x0b\x03\x12\
    \x03\\&(\n\xa5\x01\n\x04\x04\x03\x02\x0c\x12\x03_\x02+\x1a\x97\x01/\x20`\
    true`\x20if\x20this\x20service\x20instance\x20is\x20part\x20of\x20a\x20t\
    opology\x20and\x20an\x20election\x20is\x20currently\x20under\x20way\n/\
    \x20but\x20has\x20come\x20to\x20a\x20stop\x20because\x20a\x20quorum\x20c\
    annot\x20be\x20met\n\n\x0c\n\x05\x04\x03\x02\x0c\x04\x12\x03_\x02\n\n\
    \x0c\n\x05\x04\x03\x02\x0c\x05\x12\x03_\x0b\x0f\n\x0c\n\x05\x04\x03\x02\
    \x0c\x01\x12\x03_\x10%\n\x0c\n\x05\x04\x03\x02\x0c\x03\x12\x03_(*\na\n\
    \x04\x04\x03\x02\r\x12\x03a\x02*\x1aT/\x20`true`\x20if\x20this\x20servic\
    e\x20instance\x20is\x20part\x20of\x20a\x20topology\x20and\x20an\x20elect\
    ion\x20is\x20finished\n\n\x0c\n\x05\x04\x03\x02\r\x04\x12\x03a\x02\n\n\
    \x0c\n\x05\x04\x03\x02\r\x05\x12\x03a\x0b\x0f\n\x0c\n\x05\x04\x03\x02\r\
    \x01\x12\x03a\x10$\n\x0c\n\x05\x04\x03\x02\r\x03\x12\x03a')\nv\n\x04\x04\
    \x03\x02\x0e\x12\x03d\x020\x1ai/\x20`true`\x20if\x20this\x20service\x20i\
    nstance\x20is\x20part\x20of\x20an\x20update\x20topology\x20and\x20an\x20\
    election\x20is\x20currently\n/\x20under\x20way\n\n\x0c\n\x05\x04\x03\x02\
    \x0e\x04\x12\x03d\x02\n\n\x0c\n\x05\x04\x03\x02\x0e\x05\x12\x03d\x0b\x0f\
    \n\x0c\n\x05\x04\x03\x02\x0e\x01\x12\x03d\x10*\n\x0c\n\x05\x04\x03\x02\
    \x0e\x03\x12\x03d-/\n\xad\x01\n\x04\x04\x03\x02\x0f\x12\x03g\x022\x1a\
    \x9f\x01/\x20`true`\x20if\x20this\x20service\x20instance\x20is\x20part\
    \x20of\x20an\x20update\x20topology\x20and\x20an\x20election\x20is\x20cur\
    rently\n/\x20under\x20way\x20but\x20has\x20come\x20to\x20a\x20stop\x20be\
    cause\x20a\x20quorum\x20cannot\x20be\x20met\n\n\x0c\n\x05\x04\x03\x02\
    \x0f\x04\x12\x03g\x02\n\n\x0c\n\x05\x04\x03\x02\x0f\x05\x12\x03g\x0b\x0f\
    \n\x0c\n\x05\x04\x03\x02\x0f\x01\x12\x03g\x10,\n\x0c\n\x05\x04\x03\x02\
    \x0f\x03\x12\x03g/1\ni\n\x04\x04\x03\x02\x10\x12\x03i\x021\x1a\\/\x20`tr\
    ue`\x20if\x20this\x20service\x20instance\x20is\x20part\x20of\x20an\x20up\
    date\x20topology\x20and\x20an\x20election\x20is\x20finished\n\n\x0c\n\
    \x05\x04\x03\x02\x10\x04\x12\x03i\x02\n\n\x0c\n\x05\x04\x03\x02\x10\x05\
    \x12\x03i\x0b\x0f\n\x0c\n\x05\x04\x03\x02\x10\x01\x12\x03i\x10+\n\x0c\n\
    \x05\x04\x03\x02\x10\x03\x12\x03i.0\nB\n\x04\x04\x03\x02\x11\x12\x03k\
    \x02!\x1a5/\x20`true`\x20if\x20the\x20service\x20has\x20successfully\x20\
    initialized\n\n\x0c\n\x05\x04\x03\x02\x11\x04\x12\x03k\x02\n\n\x0c\n\x05\
    \x04\x03\x02\x11\x05\x12\x03k\x0b\x0f\n\x0c\n\x05\x04\x03\x02\x11\x01\
    \x12\x03k\x10\x1b\n\x0c\n\x05\x04\x03\x02\x11\x03\x12\x03k\x1e\x20\n.\n\
    \x04\x04\x03\x02\x12\x12\x03m\x02\x1b\x1a!/\x20`true`\x20if\x20the\x20se\
    rvice\x20is\x20alive\n\n\x0c\n\x05\x04\x03\x02\x12\x04\x12\x03m\x02\n\n\
    \x0c\n\x05\x04\x03\x02\x12\x05\x12\x03m\x0b\x0f\n\x0c\n\x05\x04\x03\x02\
    \x12\x01\x12\x03m\x10\x15\n\x0c\n\x05\x04\x03\x02\x12\x03\x12\x03m\x18\
    \x1a\n=\n\x04\x04\x03\x02\x13\x12\x03o\x02\x1d\x1a0/\x20`true`\x20if\x20\
    the\x20service\x20is\x20suspected\x20to\x20be\x20dead\n\n\x0c\n\x05\x04\
    \x03\x02\x13\x04\x12\x03o\x02\n\n\x0c\n\x05\x04\x03\x02\x13\x05\x12\x03o\
    \x0b\x0f\n\x0c\n\x05\x04\x03\x02\x13\x01\x12\x03o\x10\x17\n\x0c\n\x05\
    \x04\x03\x02\x13\x03\x12\x03o\x1a\x1c\n=\n\x04\x04\x03\x02\x14\x12\x03q\
    \x02\x1f\x1a0/\x20`true`\x20if\x20the\x20service\x20is\x20confirmed\x20t\
    o\x20be\x20dead\n\n\x0c\n\x05\x04\x03\x02\x14\x04\x12\x03q\x02\n\n\x0c\n\
    \x05\x04\x03\x02\x14\x05\x12\x03q\x0b\x0f\n\x0c\n\x05\x04\x03\x02\x14\
    \x01\x12\x03q\x10\x19\n\x0c\n\x05\x04\x03\x02\x14\x03\x12\x03q\x1c\x1e\n\
    H\n\x04\x04\x03\x02\x15\x12\x03s\x02\x20\x1a;/\x20`true`\x20if\x20the\
    \x20originating\x20Supervisor\x20is\x20a\x20permanent\x20peer\n\n\x0c\n\
    \x05\x04\x03\x02\x15\x04\x12\x03s\x02\n\n\x0c\n\x05\x04\x03\x02\x15\x05\
    \x12\x03s\x0b\x0f\n\x0c\n\x05\x04\x03\x02\x15\x01\x12\x03s\x10\x1a\n\x0c\
    \n\x05\x04\x03\x02\x15\x03\x12\x03s\x1d\x1f\n;\n\x04\x04\x03\x02\x16\x12\
    \x03u\x02\x1e\x1a./\x20`true`\x20if\x20the\x20service\x20is\x20marked\
    \x20as\x20departed\n\n\x0c\n\x05\x04\x03\x02\x16\x04\x12\x03u\x02\n\n\
    \x0c\n\x05\x04\x03\x02\x16\x05\x12\x03u\x0b\x0f\n\x0c\n\x05\x04\x03\x02\
    \x16\x01\x12\x03u\x10\x18\n\x0c\n\x05\x04\x03\x02\x16\x03\x12\x03u\x1b\
    \x1d\n.\n\x04\x04\x03\x02\x17\x12\x03x\x02#\x1a!/\x20Service\x20group\
    \x20application\x20name\n\n\x0c\n\x05\x04\x03\x02\x17\x04\x12\x03x\x02\n\
    \n\x0c\n\x05\x04\x03\x02\x17\x05\x12\x03x\x0b\x11\n\x0c\n\x05\x04\x03\
    \x02\x17\x01\x12\x03x\x12\x1d\n\x0c\n\x05\x04\x03\x02\x17\x03\x12\x03x\
    \x20\"\n.\n\x04\x04\x03\x02\x18\x12\x03z\x02#\x1a!/\x20Service\x20group\
    \x20environment\x20name\n\n\x0c\n\x05\x04\x03\x02\x18\x04\x12\x03z\x02\n\
    \n\x0c\n\x05\x04\x03\x02\x18\x05\x12\x03z\x0b\x11\n\x0c\n\x05\x04\x03\
    \x02\x18\x01\x12\x03z\x12\x1d\n\x0c\n\x05\x04\x03\x02\x18\x03\x12\x03z\
    \x20\"\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
