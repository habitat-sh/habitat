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
pub struct Connect {
    // message fields
    registration: ::protobuf::SingularPtrField<Registration>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Connect {}

impl Connect {
    pub fn new() -> Connect {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Connect {
        static mut instance: ::protobuf::lazy::Lazy<Connect> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Connect,
        };
        unsafe {
            instance.get(Connect::new)
        }
    }

    // optional .routesrv.Registration registration = 1;

    pub fn clear_registration(&mut self) {
        self.registration.clear();
    }

    pub fn has_registration(&self) -> bool {
        self.registration.is_some()
    }

    // Param is passed by value, moved
    pub fn set_registration(&mut self, v: Registration) {
        self.registration = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_registration(&mut self) -> &mut Registration {
        if self.registration.is_none() {
            self.registration.set_default();
        };
        self.registration.as_mut().unwrap()
    }

    // Take field
    pub fn take_registration(&mut self) -> Registration {
        self.registration.take().unwrap_or_else(|| Registration::new())
    }

    pub fn get_registration(&self) -> &Registration {
        self.registration.as_ref().unwrap_or_else(|| Registration::default_instance())
    }

    fn get_registration_for_reflect(&self) -> &::protobuf::SingularPtrField<Registration> {
        &self.registration
    }

    fn mut_registration_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Registration> {
        &mut self.registration
    }
}

impl ::protobuf::Message for Connect {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.registration)?;
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
        if let Some(v) = self.registration.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.registration.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
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

impl ::protobuf::MessageStatic for Connect {
    fn new() -> Connect {
        Connect::new()
    }

    fn descriptor_static(_: ::std::option::Option<Connect>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Registration>>(
                    "registration",
                    Connect::get_registration_for_reflect,
                    Connect::mut_registration_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Connect>(
                    "Connect",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Connect {
    fn clear(&mut self) {
        self.clear_registration();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Connect {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Connect {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ConnectOk {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ConnectOk {}

impl ConnectOk {
    pub fn new() -> ConnectOk {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ConnectOk {
        static mut instance: ::protobuf::lazy::Lazy<ConnectOk> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ConnectOk,
        };
        unsafe {
            instance.get(ConnectOk::new)
        }
    }
}

impl ::protobuf::Message for ConnectOk {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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

impl ::protobuf::MessageStatic for ConnectOk {
    fn new() -> ConnectOk {
        ConnectOk::new()
    }

    fn descriptor_static(_: ::std::option::Option<ConnectOk>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<ConnectOk>(
                    "ConnectOk",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ConnectOk {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ConnectOk {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ConnectOk {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Disconnect {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Disconnect {}

impl Disconnect {
    pub fn new() -> Disconnect {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Disconnect {
        static mut instance: ::protobuf::lazy::Lazy<Disconnect> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Disconnect,
        };
        unsafe {
            instance.get(Disconnect::new)
        }
    }
}

impl ::protobuf::Message for Disconnect {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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

impl ::protobuf::MessageStatic for Disconnect {
    fn new() -> Disconnect {
        Disconnect::new()
    }

    fn descriptor_static(_: ::std::option::Option<Disconnect>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<Disconnect>(
                    "Disconnect",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Disconnect {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Disconnect {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Disconnect {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Registration {
    // message fields
    protocol: ::std::option::Option<super::net::Protocol>,
    endpoint: ::protobuf::SingularField<::std::string::String>,
    shards: ::std::vec::Vec<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Registration {}

impl Registration {
    pub fn new() -> Registration {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Registration {
        static mut instance: ::protobuf::lazy::Lazy<Registration> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Registration,
        };
        unsafe {
            instance.get(Registration::new)
        }
    }

    // optional .net.Protocol protocol = 1;

    pub fn clear_protocol(&mut self) {
        self.protocol = ::std::option::Option::None;
    }

    pub fn has_protocol(&self) -> bool {
        self.protocol.is_some()
    }

    // Param is passed by value, moved
    pub fn set_protocol(&mut self, v: super::net::Protocol) {
        self.protocol = ::std::option::Option::Some(v);
    }

    pub fn get_protocol(&self) -> super::net::Protocol {
        self.protocol.unwrap_or(super::net::Protocol::Net)
    }

    fn get_protocol_for_reflect(&self) -> &::std::option::Option<super::net::Protocol> {
        &self.protocol
    }

    fn mut_protocol_for_reflect(&mut self) -> &mut ::std::option::Option<super::net::Protocol> {
        &mut self.protocol
    }

    // optional string endpoint = 2;

    pub fn clear_endpoint(&mut self) {
        self.endpoint.clear();
    }

    pub fn has_endpoint(&self) -> bool {
        self.endpoint.is_some()
    }

    // Param is passed by value, moved
    pub fn set_endpoint(&mut self, v: ::std::string::String) {
        self.endpoint = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_endpoint(&mut self) -> &mut ::std::string::String {
        if self.endpoint.is_none() {
            self.endpoint.set_default();
        };
        self.endpoint.as_mut().unwrap()
    }

    // Take field
    pub fn take_endpoint(&mut self) -> ::std::string::String {
        self.endpoint.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_endpoint(&self) -> &str {
        match self.endpoint.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_endpoint_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.endpoint
    }

    fn mut_endpoint_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.endpoint
    }

    // repeated uint32 shards = 3;

    pub fn clear_shards(&mut self) {
        self.shards.clear();
    }

    // Param is passed by value, moved
    pub fn set_shards(&mut self, v: ::std::vec::Vec<u32>) {
        self.shards = v;
    }

    // Mutable pointer to the field.
    pub fn mut_shards(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.shards
    }

    // Take field
    pub fn take_shards(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.shards, ::std::vec::Vec::new())
    }

    pub fn get_shards(&self) -> &[u32] {
        &self.shards
    }

    fn get_shards_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.shards
    }

    fn mut_shards_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.shards
    }
}

impl ::protobuf::Message for Registration {
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
                    };
                    let tmp = is.read_enum()?;
                    self.protocol = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.endpoint)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.shards)?;
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
        if let Some(v) = self.protocol {
            my_size += ::protobuf::rt::enum_size(1, v);
        };
        if let Some(v) = self.endpoint.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        if !self.shards.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_size(3, &self.shards);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.protocol {
            os.write_enum(1, v.value())?;
        };
        if let Some(v) = self.endpoint.as_ref() {
            os.write_string(2, &v)?;
        };
        if !self.shards.is_empty() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            // TODO: Data size is computed again, it should be cached
            os.write_raw_varint32(::protobuf::rt::vec_packed_varint_data_size(&self.shards))?;
            for v in &self.shards {
                os.write_uint32_no_tag(*v)?;
            };
        };
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

impl ::protobuf::MessageStatic for Registration {
    fn new() -> Registration {
        Registration::new()
    }

    fn descriptor_static(_: ::std::option::Option<Registration>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::net::Protocol>>(
                    "protocol",
                    Registration::get_protocol_for_reflect,
                    Registration::mut_protocol_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "endpoint",
                    Registration::get_endpoint_for_reflect,
                    Registration::mut_endpoint_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "shards",
                    Registration::get_shards_for_reflect,
                    Registration::mut_shards_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Registration>(
                    "Registration",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Registration {
    fn clear(&mut self) {
        self.clear_protocol();
        self.clear_endpoint();
        self.clear_shards();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Registration {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Registration {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x18, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x73, 0x2f, 0x72, 0x6f, 0x75, 0x74,
    0x65, 0x73, 0x72, 0x76, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x08, 0x72, 0x6f, 0x75, 0x74,
    0x65, 0x73, 0x72, 0x76, 0x1a, 0x13, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x73, 0x2f,
    0x6e, 0x65, 0x74, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x45, 0x0a, 0x07, 0x43, 0x6f, 0x6e,
    0x6e, 0x65, 0x63, 0x74, 0x12, 0x3a, 0x0a, 0x0c, 0x72, 0x65, 0x67, 0x69, 0x73, 0x74, 0x72, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x16, 0x2e, 0x72, 0x6f, 0x75,
    0x74, 0x65, 0x73, 0x72, 0x76, 0x2e, 0x52, 0x65, 0x67, 0x69, 0x73, 0x74, 0x72, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x52, 0x0c, 0x72, 0x65, 0x67, 0x69, 0x73, 0x74, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e,
    0x22, 0x0b, 0x0a, 0x09, 0x43, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x4f, 0x6b, 0x22, 0x0c, 0x0a,
    0x0a, 0x44, 0x69, 0x73, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x22, 0x71, 0x0a, 0x0c, 0x52,
    0x65, 0x67, 0x69, 0x73, 0x74, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x29, 0x0a, 0x08, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x0d, 0x2e,
    0x6e, 0x65, 0x74, 0x2e, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x52, 0x08, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x12, 0x1a, 0x0a, 0x08, 0x65, 0x6e, 0x64, 0x70, 0x6f, 0x69,
    0x6e, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x08, 0x65, 0x6e, 0x64, 0x70, 0x6f, 0x69,
    0x6e, 0x74, 0x12, 0x1a, 0x0a, 0x06, 0x73, 0x68, 0x61, 0x72, 0x64, 0x73, 0x18, 0x03, 0x20, 0x03,
    0x28, 0x0d, 0x52, 0x06, 0x73, 0x68, 0x61, 0x72, 0x64, 0x73, 0x42, 0x02, 0x10, 0x01, 0x4a, 0xf9,
    0x03, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x0e, 0x01, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12,
    0x03, 0x00, 0x07, 0x1c, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x01, 0x08, 0x10, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x03, 0x00, 0x05, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00,
    0x01, 0x12, 0x03, 0x03, 0x08, 0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03,
    0x04, 0x02, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x04, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x04, 0x0b, 0x17, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x04, 0x18, 0x24, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x04, 0x27, 0x28, 0x0a, 0x09, 0x0a, 0x02, 0x04,
    0x01, 0x12, 0x03, 0x07, 0x00, 0x14, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x07,
    0x08, 0x11, 0x0a, 0x09, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x03, 0x08, 0x00, 0x15, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x08, 0x08, 0x12, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12,
    0x04, 0x0a, 0x00, 0x0e, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x0a, 0x08,
    0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x0b, 0x02, 0x25, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x03, 0x0b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x00, 0x06, 0x12, 0x03, 0x0b, 0x0b, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x0b, 0x18, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x0b, 0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x01, 0x12, 0x03,
    0x0c, 0x02, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x04, 0x12, 0x03, 0x0c, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x05, 0x12, 0x03, 0x0c, 0x0b, 0x11, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0c, 0x12, 0x1a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0c, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x03, 0x02, 0x02, 0x12, 0x03, 0x0d, 0x02, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02,
    0x04, 0x12, 0x03, 0x0d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x05, 0x12,
    0x03, 0x0d, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0d,
    0x12, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x03, 0x12, 0x03, 0x0d, 0x1b, 0x1c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x08, 0x12, 0x03, 0x0d, 0x1d, 0x2a, 0x0a, 0x0f,
    0x0a, 0x08, 0x04, 0x03, 0x02, 0x02, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x0d, 0x1e, 0x29, 0x0a,
    0x10, 0x0a, 0x09, 0x04, 0x03, 0x02, 0x02, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x0d, 0x1e,
    0x24, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x03, 0x02, 0x02, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12,
    0x03, 0x0d, 0x1e, 0x24, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x03, 0x02, 0x02, 0x08, 0xe7, 0x07, 0x00,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x0d, 0x1e, 0x24, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x03, 0x02, 0x02,
    0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x0d, 0x25, 0x29,
];

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
