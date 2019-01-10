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
                    ::protobuf::rt::read_proto2_enum_with_unknown_fields_into(wire_type, is, &mut self.field_type, 1, &mut self.unknown_fields)?
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
pub struct ServiceUpdate {
    // message fields
    member_id: ::protobuf::SingularField<::std::string::String>,
    service: ::protobuf::SingularField<::std::string::String>,
    group: ::protobuf::SingularField<::std::string::String>,
    org: ::protobuf::SingularField<::std::string::String>,
    cfg: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    sys: ::protobuf::SingularPtrField<SysInfo>,
    pkg: ::protobuf::SingularPtrField<PackageIdent>,
    initialized: ::std::option::Option<bool>,
    bldr_url: ::protobuf::SingularField<::std::string::String>,
    channel: ::protobuf::SingularField<::std::string::String>,
    start_style: ::protobuf::SingularField<::std::string::String>,
    topology: ::protobuf::SingularField<::std::string::String>,
    update_strategy: ::protobuf::SingularField<::std::string::String>,
    application: ::protobuf::SingularField<::std::string::String>,
    environment: ::protobuf::SingularField<::std::string::String>,
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
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ServiceUpdate {}

impl ServiceUpdate {
    pub fn new() -> ServiceUpdate {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ServiceUpdate {
        static mut instance: ::protobuf::lazy::Lazy<ServiceUpdate> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ServiceUpdate,
        };
        unsafe {
            instance.get(ServiceUpdate::new)
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

    // optional bool initialized = 8;

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

    // optional string bldr_url = 9;

    pub fn clear_bldr_url(&mut self) {
        self.bldr_url.clear();
    }

    pub fn has_bldr_url(&self) -> bool {
        self.bldr_url.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bldr_url(&mut self, v: ::std::string::String) {
        self.bldr_url = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_bldr_url(&mut self) -> &mut ::std::string::String {
        if self.bldr_url.is_none() {
            self.bldr_url.set_default();
        }
        self.bldr_url.as_mut().unwrap()
    }

    // Take field
    pub fn take_bldr_url(&mut self) -> ::std::string::String {
        self.bldr_url.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_bldr_url(&self) -> &str {
        match self.bldr_url.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_bldr_url_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.bldr_url
    }

    fn mut_bldr_url_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.bldr_url
    }

    // optional string channel = 10;

    pub fn clear_channel(&mut self) {
        self.channel.clear();
    }

    pub fn has_channel(&self) -> bool {
        self.channel.is_some()
    }

    // Param is passed by value, moved
    pub fn set_channel(&mut self, v: ::std::string::String) {
        self.channel = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_channel(&mut self) -> &mut ::std::string::String {
        if self.channel.is_none() {
            self.channel.set_default();
        }
        self.channel.as_mut().unwrap()
    }

    // Take field
    pub fn take_channel(&mut self) -> ::std::string::String {
        self.channel.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_channel(&self) -> &str {
        match self.channel.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_channel_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.channel
    }

    fn mut_channel_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.channel
    }

    // optional string start_style = 11;

    pub fn clear_start_style(&mut self) {
        self.start_style.clear();
    }

    pub fn has_start_style(&self) -> bool {
        self.start_style.is_some()
    }

    // Param is passed by value, moved
    pub fn set_start_style(&mut self, v: ::std::string::String) {
        self.start_style = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_start_style(&mut self) -> &mut ::std::string::String {
        if self.start_style.is_none() {
            self.start_style.set_default();
        }
        self.start_style.as_mut().unwrap()
    }

    // Take field
    pub fn take_start_style(&mut self) -> ::std::string::String {
        self.start_style.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_start_style(&self) -> &str {
        match self.start_style.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_start_style_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.start_style
    }

    fn mut_start_style_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.start_style
    }

    // optional string topology = 12;

    pub fn clear_topology(&mut self) {
        self.topology.clear();
    }

    pub fn has_topology(&self) -> bool {
        self.topology.is_some()
    }

    // Param is passed by value, moved
    pub fn set_topology(&mut self, v: ::std::string::String) {
        self.topology = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_topology(&mut self) -> &mut ::std::string::String {
        if self.topology.is_none() {
            self.topology.set_default();
        }
        self.topology.as_mut().unwrap()
    }

    // Take field
    pub fn take_topology(&mut self) -> ::std::string::String {
        self.topology.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_topology(&self) -> &str {
        match self.topology.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_topology_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.topology
    }

    fn mut_topology_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.topology
    }

    // optional string update_strategy = 13;

    pub fn clear_update_strategy(&mut self) {
        self.update_strategy.clear();
    }

    pub fn has_update_strategy(&self) -> bool {
        self.update_strategy.is_some()
    }

    // Param is passed by value, moved
    pub fn set_update_strategy(&mut self, v: ::std::string::String) {
        self.update_strategy = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_update_strategy(&mut self) -> &mut ::std::string::String {
        if self.update_strategy.is_none() {
            self.update_strategy.set_default();
        }
        self.update_strategy.as_mut().unwrap()
    }

    // Take field
    pub fn take_update_strategy(&mut self) -> ::std::string::String {
        self.update_strategy.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_update_strategy(&self) -> &str {
        match self.update_strategy.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_update_strategy_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.update_strategy
    }

    fn mut_update_strategy_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.update_strategy
    }

    // optional string application = 14;

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

    // optional string environment = 15;

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

    // optional bool leader = 16;

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

    // optional bool follower = 17;

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

    // optional bool update_leader = 18;

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

    // optional bool update_follower = 19;

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

    // optional bool election_is_running = 20;

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

    // optional bool election_is_no_quorum = 21;

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

    // optional bool election_is_finished = 22;

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

    // optional bool update_election_is_running = 23;

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

    // optional bool update_election_is_no_quorum = 24;

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

    // optional bool update_election_is_finished = 25;

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
}

impl ::protobuf::Message for ServiceUpdate {
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
                    self.initialized = ::std::option::Option::Some(tmp);
                },
                9 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.bldr_url)?;
                },
                10 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.channel)?;
                },
                11 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.start_style)?;
                },
                12 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.topology)?;
                },
                13 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.update_strategy)?;
                },
                14 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.application)?;
                },
                15 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.environment)?;
                },
                16 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.leader = ::std::option::Option::Some(tmp);
                },
                17 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.follower = ::std::option::Option::Some(tmp);
                },
                18 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.update_leader = ::std::option::Option::Some(tmp);
                },
                19 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.update_follower = ::std::option::Option::Some(tmp);
                },
                20 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.election_is_running = ::std::option::Option::Some(tmp);
                },
                21 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.election_is_no_quorum = ::std::option::Option::Some(tmp);
                },
                22 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.election_is_finished = ::std::option::Option::Some(tmp);
                },
                23 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.update_election_is_running = ::std::option::Option::Some(tmp);
                },
                24 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.update_election_is_no_quorum = ::std::option::Option::Some(tmp);
                },
                25 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.update_election_is_finished = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.initialized {
            my_size += 2;
        }
        if let Some(ref v) = self.bldr_url.as_ref() {
            my_size += ::protobuf::rt::string_size(9, &v);
        }
        if let Some(ref v) = self.channel.as_ref() {
            my_size += ::protobuf::rt::string_size(10, &v);
        }
        if let Some(ref v) = self.start_style.as_ref() {
            my_size += ::protobuf::rt::string_size(11, &v);
        }
        if let Some(ref v) = self.topology.as_ref() {
            my_size += ::protobuf::rt::string_size(12, &v);
        }
        if let Some(ref v) = self.update_strategy.as_ref() {
            my_size += ::protobuf::rt::string_size(13, &v);
        }
        if let Some(ref v) = self.application.as_ref() {
            my_size += ::protobuf::rt::string_size(14, &v);
        }
        if let Some(ref v) = self.environment.as_ref() {
            my_size += ::protobuf::rt::string_size(15, &v);
        }
        if let Some(v) = self.leader {
            my_size += 3;
        }
        if let Some(v) = self.follower {
            my_size += 3;
        }
        if let Some(v) = self.update_leader {
            my_size += 3;
        }
        if let Some(v) = self.update_follower {
            my_size += 3;
        }
        if let Some(v) = self.election_is_running {
            my_size += 3;
        }
        if let Some(v) = self.election_is_no_quorum {
            my_size += 3;
        }
        if let Some(v) = self.election_is_finished {
            my_size += 3;
        }
        if let Some(v) = self.update_election_is_running {
            my_size += 3;
        }
        if let Some(v) = self.update_election_is_no_quorum {
            my_size += 3;
        }
        if let Some(v) = self.update_election_is_finished {
            my_size += 3;
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
        if let Some(v) = self.initialized {
            os.write_bool(8, v)?;
        }
        if let Some(ref v) = self.bldr_url.as_ref() {
            os.write_string(9, &v)?;
        }
        if let Some(ref v) = self.channel.as_ref() {
            os.write_string(10, &v)?;
        }
        if let Some(ref v) = self.start_style.as_ref() {
            os.write_string(11, &v)?;
        }
        if let Some(ref v) = self.topology.as_ref() {
            os.write_string(12, &v)?;
        }
        if let Some(ref v) = self.update_strategy.as_ref() {
            os.write_string(13, &v)?;
        }
        if let Some(ref v) = self.application.as_ref() {
            os.write_string(14, &v)?;
        }
        if let Some(ref v) = self.environment.as_ref() {
            os.write_string(15, &v)?;
        }
        if let Some(v) = self.leader {
            os.write_bool(16, v)?;
        }
        if let Some(v) = self.follower {
            os.write_bool(17, v)?;
        }
        if let Some(v) = self.update_leader {
            os.write_bool(18, v)?;
        }
        if let Some(v) = self.update_follower {
            os.write_bool(19, v)?;
        }
        if let Some(v) = self.election_is_running {
            os.write_bool(20, v)?;
        }
        if let Some(v) = self.election_is_no_quorum {
            os.write_bool(21, v)?;
        }
        if let Some(v) = self.election_is_finished {
            os.write_bool(22, v)?;
        }
        if let Some(v) = self.update_election_is_running {
            os.write_bool(23, v)?;
        }
        if let Some(v) = self.update_election_is_no_quorum {
            os.write_bool(24, v)?;
        }
        if let Some(v) = self.update_election_is_finished {
            os.write_bool(25, v)?;
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

impl ::protobuf::MessageStatic for ServiceUpdate {
    fn new() -> ServiceUpdate {
        ServiceUpdate::new()
    }

    fn descriptor_static(_: ::std::option::Option<ServiceUpdate>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "member_id",
                    ServiceUpdate::get_member_id_for_reflect,
                    ServiceUpdate::mut_member_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "service",
                    ServiceUpdate::get_service_for_reflect,
                    ServiceUpdate::mut_service_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "group",
                    ServiceUpdate::get_group_for_reflect,
                    ServiceUpdate::mut_group_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "org",
                    ServiceUpdate::get_org_for_reflect,
                    ServiceUpdate::mut_org_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "cfg",
                    ServiceUpdate::get_cfg_for_reflect,
                    ServiceUpdate::mut_cfg_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<SysInfo>>(
                    "sys",
                    ServiceUpdate::get_sys_for_reflect,
                    ServiceUpdate::mut_sys_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<PackageIdent>>(
                    "pkg",
                    ServiceUpdate::get_pkg_for_reflect,
                    ServiceUpdate::mut_pkg_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "initialized",
                    ServiceUpdate::get_initialized_for_reflect,
                    ServiceUpdate::mut_initialized_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "bldr_url",
                    ServiceUpdate::get_bldr_url_for_reflect,
                    ServiceUpdate::mut_bldr_url_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "channel",
                    ServiceUpdate::get_channel_for_reflect,
                    ServiceUpdate::mut_channel_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "start_style",
                    ServiceUpdate::get_start_style_for_reflect,
                    ServiceUpdate::mut_start_style_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "topology",
                    ServiceUpdate::get_topology_for_reflect,
                    ServiceUpdate::mut_topology_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "update_strategy",
                    ServiceUpdate::get_update_strategy_for_reflect,
                    ServiceUpdate::mut_update_strategy_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "application",
                    ServiceUpdate::get_application_for_reflect,
                    ServiceUpdate::mut_application_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "environment",
                    ServiceUpdate::get_environment_for_reflect,
                    ServiceUpdate::mut_environment_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "leader",
                    ServiceUpdate::get_leader_for_reflect,
                    ServiceUpdate::mut_leader_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "follower",
                    ServiceUpdate::get_follower_for_reflect,
                    ServiceUpdate::mut_follower_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "update_leader",
                    ServiceUpdate::get_update_leader_for_reflect,
                    ServiceUpdate::mut_update_leader_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "update_follower",
                    ServiceUpdate::get_update_follower_for_reflect,
                    ServiceUpdate::mut_update_follower_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "election_is_running",
                    ServiceUpdate::get_election_is_running_for_reflect,
                    ServiceUpdate::mut_election_is_running_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "election_is_no_quorum",
                    ServiceUpdate::get_election_is_no_quorum_for_reflect,
                    ServiceUpdate::mut_election_is_no_quorum_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "election_is_finished",
                    ServiceUpdate::get_election_is_finished_for_reflect,
                    ServiceUpdate::mut_election_is_finished_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "update_election_is_running",
                    ServiceUpdate::get_update_election_is_running_for_reflect,
                    ServiceUpdate::mut_update_election_is_running_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "update_election_is_no_quorum",
                    ServiceUpdate::get_update_election_is_no_quorum_for_reflect,
                    ServiceUpdate::mut_update_election_is_no_quorum_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "update_election_is_finished",
                    ServiceUpdate::get_update_election_is_finished_for_reflect,
                    ServiceUpdate::mut_update_election_is_finished_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ServiceUpdate>(
                    "ServiceUpdate",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ServiceUpdate {
    fn clear(&mut self) {
        self.clear_member_id();
        self.clear_service();
        self.clear_group();
        self.clear_org();
        self.clear_cfg();
        self.clear_sys();
        self.clear_pkg();
        self.clear_initialized();
        self.clear_bldr_url();
        self.clear_channel();
        self.clear_start_style();
        self.clear_topology();
        self.clear_update_strategy();
        self.clear_application();
        self.clear_environment();
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
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ServiceUpdate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ServiceUpdate {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0bevent.proto\x12\x10habitat.eventsrv\"\xdf\x01\n\rEventEnvelope\x12\
    2\n\x04type\x18\x01\x20\x01(\x0e2$.habitat.eventsrv.EventEnvelope.Type\
    \x12\x0f\n\x07payload\x18\x02\x20\x01(\x0c\x12\x11\n\ttimestamp\x18\x03\
    \x20\x01(\x04\x12\x11\n\tmember_id\x18\x04\x20\x01(\t\x12\x0f\n\x07servi\
    ce\x18\x05\x20\x01(\t\x12\x13\n\x0bincarnation\x18\x06\x20\x01(\x04\x12\
    \x13\n\x0bsequence_id\x18\x07\x20\x01(\x04\"(\n\x04Type\x12\x0c\n\x08Pro\
    toBuf\x10\x01\x12\x08\n\x04JSON\x10\x02\x12\x08\n\x04TOML\x10\x03\"\x83\
    \x01\n\x07SysInfo\x12\n\n\x02ip\x18\x01\x20\x01(\t\x12\x10\n\x08hostname\
    \x18\x02\x20\x01(\t\x12\x11\n\tgossip_ip\x18\x03\x20\x01(\t\x12\x13\n\
    \x0bgossip_port\x18\x04\x20\x01(\t\x12\x17\n\x0fhttp_gateway_ip\x18\x05\
    \x20\x01(\t\x12\x19\n\x11http_gateway_port\x18\x06\x20\x01(\t\"N\n\x0cPa\
    ckageIdent\x12\x0e\n\x06origin\x18\x01\x20\x01(\t\x12\x0c\n\x04name\x18\
    \x02\x20\x01(\t\x12\x0f\n\x07version\x18\x03\x20\x01(\t\x12\x0f\n\x07rel\
    ease\x18\x04\x20\x01(\t\"\xee\x04\n\rServiceUpdate\x12\x11\n\tmember_id\
    \x18\x01\x20\x01(\t\x12\x0f\n\x07service\x18\x02\x20\x01(\t\x12\r\n\x05g\
    roup\x18\x03\x20\x01(\t\x12\x0b\n\x03org\x18\x04\x20\x01(\t\x12\x0b\n\
    \x03cfg\x18\x05\x20\x01(\x0c\x12&\n\x03sys\x18\x06\x20\x01(\x0b2\x19.hab\
    itat.eventsrv.SysInfo\x12+\n\x03pkg\x18\x07\x20\x01(\x0b2\x1e.habitat.ev\
    entsrv.PackageIdent\x12\x13\n\x0binitialized\x18\x08\x20\x01(\x08\x12\
    \x10\n\x08bldr_url\x18\t\x20\x01(\t\x12\x0f\n\x07channel\x18\n\x20\x01(\
    \t\x12\x13\n\x0bstart_style\x18\x0b\x20\x01(\t\x12\x10\n\x08topology\x18\
    \x0c\x20\x01(\t\x12\x17\n\x0fupdate_strategy\x18\r\x20\x01(\t\x12\x13\n\
    \x0bapplication\x18\x0e\x20\x01(\t\x12\x13\n\x0benvironment\x18\x0f\x20\
    \x01(\t\x12\x0e\n\x06leader\x18\x10\x20\x01(\x08\x12\x10\n\x08follower\
    \x18\x11\x20\x01(\x08\x12\x15\n\rupdate_leader\x18\x12\x20\x01(\x08\x12\
    \x17\n\x0fupdate_follower\x18\x13\x20\x01(\x08\x12\x1b\n\x13election_is_\
    running\x18\x14\x20\x01(\x08\x12\x1d\n\x15election_is_no_quorum\x18\x15\
    \x20\x01(\x08\x12\x1c\n\x14election_is_finished\x18\x16\x20\x01(\x08\x12\
    \"\n\x1aupdate_election_is_running\x18\x17\x20\x01(\x08\x12$\n\x1cupdate\
    _election_is_no_quorum\x18\x18\x20\x01(\x08\x12#\n\x1bupdate_election_is\
    _finished\x18\x19\x20\x01(\x08\
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
