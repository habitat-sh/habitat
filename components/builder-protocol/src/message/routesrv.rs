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
        }
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
        for v in &self.registration {
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
        if let Some(ref v) = self.registration.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.registration.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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
        }
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
                    }
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
        }
        if let Some(ref v) = self.endpoint.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if !self.shards.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_size(3, &self.shards);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.protocol {
            os.write_enum(1, v.value())?;
        }
        if let Some(ref v) = self.endpoint.as_ref() {
            os.write_string(2, &v)?;
        }
        if !self.shards.is_empty() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            // TODO: Data size is computed again, it should be cached
            os.write_raw_varint32(::protobuf::rt::vec_packed_varint_data_size(&self.shards))?;
            for v in &self.shards {
                os.write_uint32_no_tag(*v)?;
            };
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

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x18protocols/routesrv.proto\x12\x08routesrv\x1a\x13protocols/net.prot\
    o\"E\n\x07Connect\x12:\n\x0cregistration\x18\x01\x20\x01(\x0b2\x16.route\
    srv.RegistrationR\x0cregistration\"\x0b\n\tConnectOk\"\x0c\n\nDisconnect\
    \"q\n\x0cRegistration\x12)\n\x08protocol\x18\x01\x20\x01(\x0e2\r.net.Pro\
    tocolR\x08protocol\x12\x1a\n\x08endpoint\x18\x02\x20\x01(\tR\x08endpoint\
    \x12\x1a\n\x06shards\x18\x03\x20\x03(\rR\x06shardsB\x02\x10\x01J\xf9\x03\
    \n\x06\x12\x04\0\0\x0e\x01\n\t\n\x02\x03\0\x12\x03\0\x07\x1c\n\x08\n\x01\
    \x02\x12\x03\x01\x08\x10\n\n\n\x02\x04\0\x12\x04\x03\0\x05\x01\n\n\n\x03\
    \x04\0\x01\x12\x03\x03\x08\x0f\n\x0b\n\x04\x04\0\x02\0\x12\x03\x04\x02)\
    \n\x0c\n\x05\x04\0\x02\0\x04\x12\x03\x04\x02\n\n\x0c\n\x05\x04\0\x02\0\
    \x06\x12\x03\x04\x0b\x17\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x04\x18$\n\
    \x0c\n\x05\x04\0\x02\0\x03\x12\x03\x04'(\n\t\n\x02\x04\x01\x12\x03\x07\0\
    \x14\n\n\n\x03\x04\x01\x01\x12\x03\x07\x08\x11\n\t\n\x02\x04\x02\x12\x03\
    \x08\0\x15\n\n\n\x03\x04\x02\x01\x12\x03\x08\x08\x12\n\n\n\x02\x04\x03\
    \x12\x04\n\0\x0e\x01\n\n\n\x03\x04\x03\x01\x12\x03\n\x08\x14\n\x0b\n\x04\
    \x04\x03\x02\0\x12\x03\x0b\x02%\n\x0c\n\x05\x04\x03\x02\0\x04\x12\x03\
    \x0b\x02\n\n\x0c\n\x05\x04\x03\x02\0\x06\x12\x03\x0b\x0b\x17\n\x0c\n\x05\
    \x04\x03\x02\0\x01\x12\x03\x0b\x18\x20\n\x0c\n\x05\x04\x03\x02\0\x03\x12\
    \x03\x0b#$\n\x0b\n\x04\x04\x03\x02\x01\x12\x03\x0c\x02\x1f\n\x0c\n\x05\
    \x04\x03\x02\x01\x04\x12\x03\x0c\x02\n\n\x0c\n\x05\x04\x03\x02\x01\x05\
    \x12\x03\x0c\x0b\x11\n\x0c\n\x05\x04\x03\x02\x01\x01\x12\x03\x0c\x12\x1a\
    \n\x0c\n\x05\x04\x03\x02\x01\x03\x12\x03\x0c\x1d\x1e\n\x0b\n\x04\x04\x03\
    \x02\x02\x12\x03\r\x02+\n\x0c\n\x05\x04\x03\x02\x02\x04\x12\x03\r\x02\n\
    \n\x0c\n\x05\x04\x03\x02\x02\x05\x12\x03\r\x0b\x11\n\x0c\n\x05\x04\x03\
    \x02\x02\x01\x12\x03\r\x12\x18\n\x0c\n\x05\x04\x03\x02\x02\x03\x12\x03\r\
    \x1b\x1c\n\x0c\n\x05\x04\x03\x02\x02\x08\x12\x03\r\x1d*\n\x0f\n\x08\x04\
    \x03\x02\x02\x08\xe7\x07\0\x12\x03\r\x1e)\n\x10\n\t\x04\x03\x02\x02\x08\
    \xe7\x07\0\x02\x12\x03\r\x1e$\n\x11\n\n\x04\x03\x02\x02\x08\xe7\x07\0\
    \x02\0\x12\x03\r\x1e$\n\x12\n\x0b\x04\x03\x02\x02\x08\xe7\x07\0\x02\0\
    \x01\x12\x03\r\x1e$\n\x10\n\t\x04\x03\x02\x02\x08\xe7\x07\0\x03\x12\x03\
    \r%)\
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
