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
pub struct RouteInfo {
    // message fields
    protocol: ::std::option::Option<Protocol>,
    hash: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RouteInfo {}

impl RouteInfo {
    pub fn new() -> RouteInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RouteInfo {
        static mut instance: ::protobuf::lazy::Lazy<RouteInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RouteInfo,
        };
        unsafe {
            instance.get(RouteInfo::new)
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
    pub fn set_protocol(&mut self, v: Protocol) {
        self.protocol = ::std::option::Option::Some(v);
    }

    pub fn get_protocol(&self) -> Protocol {
        self.protocol.unwrap_or(Protocol::Net)
    }

    fn get_protocol_for_reflect(&self) -> &::std::option::Option<Protocol> {
        &self.protocol
    }

    fn mut_protocol_for_reflect(&mut self) -> &mut ::std::option::Option<Protocol> {
        &mut self.protocol
    }

    // optional uint64 hash = 2;

    pub fn clear_hash(&mut self) {
        self.hash = ::std::option::Option::None;
    }

    pub fn has_hash(&self) -> bool {
        self.hash.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hash(&mut self, v: u64) {
        self.hash = ::std::option::Option::Some(v);
    }

    pub fn get_hash(&self) -> u64 {
        self.hash.unwrap_or(0)
    }

    fn get_hash_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.hash
    }

    fn mut_hash_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.hash
    }
}

impl ::protobuf::Message for RouteInfo {
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.hash = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.hash {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.protocol {
            os.write_enum(1, v.value())?;
        };
        if let Some(v) = self.hash {
            os.write_uint64(2, v)?;
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

impl ::protobuf::MessageStatic for RouteInfo {
    fn new() -> RouteInfo {
        RouteInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<RouteInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Protocol>>(
                    "protocol",
                    RouteInfo::get_protocol_for_reflect,
                    RouteInfo::mut_protocol_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "hash",
                    RouteInfo::get_hash_for_reflect,
                    RouteInfo::mut_hash_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RouteInfo>(
                    "RouteInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RouteInfo {
    fn clear(&mut self) {
        self.clear_protocol();
        self.clear_hash();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RouteInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RouteInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Msg {
    // message fields
    message_id: ::protobuf::SingularField<::std::string::String>,
    body: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    route_info: ::protobuf::SingularPtrField<RouteInfo>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Msg {}

impl Msg {
    pub fn new() -> Msg {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Msg {
        static mut instance: ::protobuf::lazy::Lazy<Msg> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Msg,
        };
        unsafe {
            instance.get(Msg::new)
        }
    }

    // optional string message_id = 1;

    pub fn clear_message_id(&mut self) {
        self.message_id.clear();
    }

    pub fn has_message_id(&self) -> bool {
        self.message_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_message_id(&mut self, v: ::std::string::String) {
        self.message_id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_message_id(&mut self) -> &mut ::std::string::String {
        if self.message_id.is_none() {
            self.message_id.set_default();
        };
        self.message_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_message_id(&mut self) -> ::std::string::String {
        self.message_id.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_message_id(&self) -> &str {
        match self.message_id.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_message_id_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.message_id
    }

    fn mut_message_id_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.message_id
    }

    // optional bytes body = 2;

    pub fn clear_body(&mut self) {
        self.body.clear();
    }

    pub fn has_body(&self) -> bool {
        self.body.is_some()
    }

    // Param is passed by value, moved
    pub fn set_body(&mut self, v: ::std::vec::Vec<u8>) {
        self.body = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_body(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.body.is_none() {
            self.body.set_default();
        };
        self.body.as_mut().unwrap()
    }

    // Take field
    pub fn take_body(&mut self) -> ::std::vec::Vec<u8> {
        self.body.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_body(&self) -> &[u8] {
        match self.body.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_body_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.body
    }

    fn mut_body_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.body
    }

    // optional .net.RouteInfo route_info = 3;

    pub fn clear_route_info(&mut self) {
        self.route_info.clear();
    }

    pub fn has_route_info(&self) -> bool {
        self.route_info.is_some()
    }

    // Param is passed by value, moved
    pub fn set_route_info(&mut self, v: RouteInfo) {
        self.route_info = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_route_info(&mut self) -> &mut RouteInfo {
        if self.route_info.is_none() {
            self.route_info.set_default();
        };
        self.route_info.as_mut().unwrap()
    }

    // Take field
    pub fn take_route_info(&mut self) -> RouteInfo {
        self.route_info.take().unwrap_or_else(|| RouteInfo::new())
    }

    pub fn get_route_info(&self) -> &RouteInfo {
        self.route_info.as_ref().unwrap_or_else(|| RouteInfo::default_instance())
    }

    fn get_route_info_for_reflect(&self) -> &::protobuf::SingularPtrField<RouteInfo> {
        &self.route_info
    }

    fn mut_route_info_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<RouteInfo> {
        &mut self.route_info
    }
}

impl ::protobuf::Message for Msg {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.message_id)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.body)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.route_info)?;
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
        if let Some(v) = self.message_id.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.body.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        };
        if let Some(v) = self.route_info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.message_id.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.body.as_ref() {
            os.write_bytes(2, &v)?;
        };
        if let Some(v) = self.route_info.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for Msg {
    fn new() -> Msg {
        Msg::new()
    }

    fn descriptor_static(_: ::std::option::Option<Msg>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "message_id",
                    Msg::get_message_id_for_reflect,
                    Msg::mut_message_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "body",
                    Msg::get_body_for_reflect,
                    Msg::mut_body_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<RouteInfo>>(
                    "route_info",
                    Msg::get_route_info_for_reflect,
                    Msg::mut_route_info_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Msg>(
                    "Msg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Msg {
    fn clear(&mut self) {
        self.clear_message_id();
        self.clear_body();
        self.clear_route_info();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Msg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Msg {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct NetError {
    // message fields
    code: ::std::option::Option<ErrCode>,
    msg: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for NetError {}

impl NetError {
    pub fn new() -> NetError {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static NetError {
        static mut instance: ::protobuf::lazy::Lazy<NetError> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const NetError,
        };
        unsafe {
            instance.get(NetError::new)
        }
    }

    // optional .net.ErrCode code = 1;

    pub fn clear_code(&mut self) {
        self.code = ::std::option::Option::None;
    }

    pub fn has_code(&self) -> bool {
        self.code.is_some()
    }

    // Param is passed by value, moved
    pub fn set_code(&mut self, v: ErrCode) {
        self.code = ::std::option::Option::Some(v);
    }

    pub fn get_code(&self) -> ErrCode {
        self.code.unwrap_or(ErrCode::BUG)
    }

    fn get_code_for_reflect(&self) -> &::std::option::Option<ErrCode> {
        &self.code
    }

    fn mut_code_for_reflect(&mut self) -> &mut ::std::option::Option<ErrCode> {
        &mut self.code
    }

    // optional string msg = 2;

    pub fn clear_msg(&mut self) {
        self.msg.clear();
    }

    pub fn has_msg(&self) -> bool {
        self.msg.is_some()
    }

    // Param is passed by value, moved
    pub fn set_msg(&mut self, v: ::std::string::String) {
        self.msg = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_msg(&mut self) -> &mut ::std::string::String {
        if self.msg.is_none() {
            self.msg.set_default();
        };
        self.msg.as_mut().unwrap()
    }

    // Take field
    pub fn take_msg(&mut self) -> ::std::string::String {
        self.msg.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_msg(&self) -> &str {
        match self.msg.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_msg_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.msg
    }

    fn mut_msg_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.msg
    }
}

impl ::protobuf::Message for NetError {
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
                    self.code = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.msg)?;
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
        if let Some(v) = self.code {
            my_size += ::protobuf::rt::enum_size(1, v);
        };
        if let Some(v) = self.msg.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.code {
            os.write_enum(1, v.value())?;
        };
        if let Some(v) = self.msg.as_ref() {
            os.write_string(2, &v)?;
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

impl ::protobuf::MessageStatic for NetError {
    fn new() -> NetError {
        NetError::new()
    }

    fn descriptor_static(_: ::std::option::Option<NetError>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<ErrCode>>(
                    "code",
                    NetError::get_code_for_reflect,
                    NetError::mut_code_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "msg",
                    NetError::get_msg_for_reflect,
                    NetError::mut_msg_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<NetError>(
                    "NetError",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for NetError {
    fn clear(&mut self) {
        self.clear_code();
        self.clear_msg();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for NetError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for NetError {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct NetOk {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for NetOk {}

impl NetOk {
    pub fn new() -> NetOk {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static NetOk {
        static mut instance: ::protobuf::lazy::Lazy<NetOk> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const NetOk,
        };
        unsafe {
            instance.get(NetOk::new)
        }
    }
}

impl ::protobuf::Message for NetOk {
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

impl ::protobuf::MessageStatic for NetOk {
    fn new() -> NetOk {
        NetOk::new()
    }

    fn descriptor_static(_: ::std::option::Option<NetOk>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<NetOk>(
                    "NetOk",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for NetOk {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for NetOk {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for NetOk {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Ping {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Ping {}

impl Ping {
    pub fn new() -> Ping {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Ping {
        static mut instance: ::protobuf::lazy::Lazy<Ping> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Ping,
        };
        unsafe {
            instance.get(Ping::new)
        }
    }
}

impl ::protobuf::Message for Ping {
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

impl ::protobuf::MessageStatic for Ping {
    fn new() -> Ping {
        Ping::new()
    }

    fn descriptor_static(_: ::std::option::Option<Ping>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<Ping>(
                    "Ping",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Ping {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Ping {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Ping {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Pong {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Pong {}

impl Pong {
    pub fn new() -> Pong {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Pong {
        static mut instance: ::protobuf::lazy::Lazy<Pong> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Pong,
        };
        unsafe {
            instance.get(Pong::new)
        }
    }
}

impl ::protobuf::Message for Pong {
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

impl ::protobuf::MessageStatic for Pong {
    fn new() -> Pong {
        Pong::new()
    }

    fn descriptor_static(_: ::std::option::Option<Pong>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<Pong>(
                    "Pong",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Pong {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Pong {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Pong {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Protocol {
    Net = 0,
    RouteSrv = 1,
    SessionSrv = 2,
    OriginSrv = 3,
    JobSrv = 4,
    Scheduler = 5,
}

impl ::protobuf::ProtobufEnum for Protocol {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Protocol> {
        match value {
            0 => ::std::option::Option::Some(Protocol::Net),
            1 => ::std::option::Option::Some(Protocol::RouteSrv),
            2 => ::std::option::Option::Some(Protocol::SessionSrv),
            3 => ::std::option::Option::Some(Protocol::OriginSrv),
            4 => ::std::option::Option::Some(Protocol::JobSrv),
            5 => ::std::option::Option::Some(Protocol::Scheduler),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Protocol] = &[
            Protocol::Net,
            Protocol::RouteSrv,
            Protocol::SessionSrv,
            Protocol::OriginSrv,
            Protocol::JobSrv,
            Protocol::Scheduler,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<Protocol>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Protocol", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Protocol {
}

impl ::protobuf::reflect::ProtobufValue for Protocol {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ErrCode {
    BUG = 0,
    TIMEOUT = 1,
    REMOTE_REJECTED = 2,
    BAD_REMOTE_REPLY = 3,
    ENTITY_NOT_FOUND = 4,
    NO_SHARD = 6,
    ACCESS_DENIED = 7,
    SESSION_EXPIRED = 8,
    ENTITY_CONFLICT = 9,
    ZMQ = 10,
    DATA_STORE = 11,
    AUTH_SCOPE = 12,
    WORKSPACE_SETUP = 1000,
    SECRET_KEY_FETCH = 1001,
    SECRET_KEY_IMPORT = 1002,
    VCS_CLONE = 1003,
    BUILD = 1004,
    POST_PROCESSOR = 1005,
}

impl ::protobuf::ProtobufEnum for ErrCode {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ErrCode> {
        match value {
            0 => ::std::option::Option::Some(ErrCode::BUG),
            1 => ::std::option::Option::Some(ErrCode::TIMEOUT),
            2 => ::std::option::Option::Some(ErrCode::REMOTE_REJECTED),
            3 => ::std::option::Option::Some(ErrCode::BAD_REMOTE_REPLY),
            4 => ::std::option::Option::Some(ErrCode::ENTITY_NOT_FOUND),
            6 => ::std::option::Option::Some(ErrCode::NO_SHARD),
            7 => ::std::option::Option::Some(ErrCode::ACCESS_DENIED),
            8 => ::std::option::Option::Some(ErrCode::SESSION_EXPIRED),
            9 => ::std::option::Option::Some(ErrCode::ENTITY_CONFLICT),
            10 => ::std::option::Option::Some(ErrCode::ZMQ),
            11 => ::std::option::Option::Some(ErrCode::DATA_STORE),
            12 => ::std::option::Option::Some(ErrCode::AUTH_SCOPE),
            1000 => ::std::option::Option::Some(ErrCode::WORKSPACE_SETUP),
            1001 => ::std::option::Option::Some(ErrCode::SECRET_KEY_FETCH),
            1002 => ::std::option::Option::Some(ErrCode::SECRET_KEY_IMPORT),
            1003 => ::std::option::Option::Some(ErrCode::VCS_CLONE),
            1004 => ::std::option::Option::Some(ErrCode::BUILD),
            1005 => ::std::option::Option::Some(ErrCode::POST_PROCESSOR),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ErrCode] = &[
            ErrCode::BUG,
            ErrCode::TIMEOUT,
            ErrCode::REMOTE_REJECTED,
            ErrCode::BAD_REMOTE_REPLY,
            ErrCode::ENTITY_NOT_FOUND,
            ErrCode::NO_SHARD,
            ErrCode::ACCESS_DENIED,
            ErrCode::SESSION_EXPIRED,
            ErrCode::ENTITY_CONFLICT,
            ErrCode::ZMQ,
            ErrCode::DATA_STORE,
            ErrCode::AUTH_SCOPE,
            ErrCode::WORKSPACE_SETUP,
            ErrCode::SECRET_KEY_FETCH,
            ErrCode::SECRET_KEY_IMPORT,
            ErrCode::VCS_CLONE,
            ErrCode::BUILD,
            ErrCode::POST_PROCESSOR,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<ErrCode>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ErrCode", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ErrCode {
}

impl ::protobuf::reflect::ProtobufValue for ErrCode {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x13, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x73, 0x2f, 0x6e, 0x65, 0x74, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x03, 0x6e, 0x65, 0x74, 0x22, 0x4a, 0x0a, 0x09, 0x52, 0x6f,
    0x75, 0x74, 0x65, 0x49, 0x6e, 0x66, 0x6f, 0x12, 0x29, 0x0a, 0x08, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x63, 0x6f, 0x6c, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x0d, 0x2e, 0x6e, 0x65, 0x74, 0x2e,
    0x50, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x52, 0x08, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x63,
    0x6f, 0x6c, 0x12, 0x12, 0x0a, 0x04, 0x68, 0x61, 0x73, 0x68, 0x18, 0x02, 0x20, 0x01, 0x28, 0x04,
    0x52, 0x04, 0x68, 0x61, 0x73, 0x68, 0x22, 0x67, 0x0a, 0x03, 0x4d, 0x73, 0x67, 0x12, 0x1d, 0x0a,
    0x0a, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x09, 0x52, 0x09, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x49, 0x64, 0x12, 0x12, 0x0a, 0x04,
    0x62, 0x6f, 0x64, 0x79, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x04, 0x62, 0x6f, 0x64, 0x79,
    0x12, 0x2d, 0x0a, 0x0a, 0x72, 0x6f, 0x75, 0x74, 0x65, 0x5f, 0x69, 0x6e, 0x66, 0x6f, 0x18, 0x03,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x6e, 0x65, 0x74, 0x2e, 0x52, 0x6f, 0x75, 0x74, 0x65,
    0x49, 0x6e, 0x66, 0x6f, 0x52, 0x09, 0x72, 0x6f, 0x75, 0x74, 0x65, 0x49, 0x6e, 0x66, 0x6f, 0x22,
    0x3e, 0x0a, 0x08, 0x4e, 0x65, 0x74, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x12, 0x20, 0x0a, 0x04, 0x63,
    0x6f, 0x64, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x0c, 0x2e, 0x6e, 0x65, 0x74, 0x2e,
    0x45, 0x72, 0x72, 0x43, 0x6f, 0x64, 0x65, 0x52, 0x04, 0x63, 0x6f, 0x64, 0x65, 0x12, 0x10, 0x0a,
    0x03, 0x6d, 0x73, 0x67, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x03, 0x6d, 0x73, 0x67, 0x22,
    0x07, 0x0a, 0x05, 0x4e, 0x65, 0x74, 0x4f, 0x6b, 0x22, 0x06, 0x0a, 0x04, 0x50, 0x69, 0x6e, 0x67,
    0x22, 0x06, 0x0a, 0x04, 0x50, 0x6f, 0x6e, 0x67, 0x2a, 0x5b, 0x0a, 0x08, 0x50, 0x72, 0x6f, 0x74,
    0x6f, 0x63, 0x6f, 0x6c, 0x12, 0x07, 0x0a, 0x03, 0x4e, 0x65, 0x74, 0x10, 0x00, 0x12, 0x0c, 0x0a,
    0x08, 0x52, 0x6f, 0x75, 0x74, 0x65, 0x53, 0x72, 0x76, 0x10, 0x01, 0x12, 0x0e, 0x0a, 0x0a, 0x53,
    0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x53, 0x72, 0x76, 0x10, 0x02, 0x12, 0x0d, 0x0a, 0x09, 0x4f,
    0x72, 0x69, 0x67, 0x69, 0x6e, 0x53, 0x72, 0x76, 0x10, 0x03, 0x12, 0x0a, 0x0a, 0x06, 0x4a, 0x6f,
    0x62, 0x53, 0x72, 0x76, 0x10, 0x04, 0x12, 0x0d, 0x0a, 0x09, 0x53, 0x63, 0x68, 0x65, 0x64, 0x75,
    0x6c, 0x65, 0x72, 0x10, 0x05, 0x2a, 0xca, 0x02, 0x0a, 0x07, 0x45, 0x72, 0x72, 0x43, 0x6f, 0x64,
    0x65, 0x12, 0x07, 0x0a, 0x03, 0x42, 0x55, 0x47, 0x10, 0x00, 0x12, 0x0b, 0x0a, 0x07, 0x54, 0x49,
    0x4d, 0x45, 0x4f, 0x55, 0x54, 0x10, 0x01, 0x12, 0x13, 0x0a, 0x0f, 0x52, 0x45, 0x4d, 0x4f, 0x54,
    0x45, 0x5f, 0x52, 0x45, 0x4a, 0x45, 0x43, 0x54, 0x45, 0x44, 0x10, 0x02, 0x12, 0x14, 0x0a, 0x10,
    0x42, 0x41, 0x44, 0x5f, 0x52, 0x45, 0x4d, 0x4f, 0x54, 0x45, 0x5f, 0x52, 0x45, 0x50, 0x4c, 0x59,
    0x10, 0x03, 0x12, 0x14, 0x0a, 0x10, 0x45, 0x4e, 0x54, 0x49, 0x54, 0x59, 0x5f, 0x4e, 0x4f, 0x54,
    0x5f, 0x46, 0x4f, 0x55, 0x4e, 0x44, 0x10, 0x04, 0x12, 0x0c, 0x0a, 0x08, 0x4e, 0x4f, 0x5f, 0x53,
    0x48, 0x41, 0x52, 0x44, 0x10, 0x06, 0x12, 0x11, 0x0a, 0x0d, 0x41, 0x43, 0x43, 0x45, 0x53, 0x53,
    0x5f, 0x44, 0x45, 0x4e, 0x49, 0x45, 0x44, 0x10, 0x07, 0x12, 0x13, 0x0a, 0x0f, 0x53, 0x45, 0x53,
    0x53, 0x49, 0x4f, 0x4e, 0x5f, 0x45, 0x58, 0x50, 0x49, 0x52, 0x45, 0x44, 0x10, 0x08, 0x12, 0x13,
    0x0a, 0x0f, 0x45, 0x4e, 0x54, 0x49, 0x54, 0x59, 0x5f, 0x43, 0x4f, 0x4e, 0x46, 0x4c, 0x49, 0x43,
    0x54, 0x10, 0x09, 0x12, 0x07, 0x0a, 0x03, 0x5a, 0x4d, 0x51, 0x10, 0x0a, 0x12, 0x0e, 0x0a, 0x0a,
    0x44, 0x41, 0x54, 0x41, 0x5f, 0x53, 0x54, 0x4f, 0x52, 0x45, 0x10, 0x0b, 0x12, 0x0e, 0x0a, 0x0a,
    0x41, 0x55, 0x54, 0x48, 0x5f, 0x53, 0x43, 0x4f, 0x50, 0x45, 0x10, 0x0c, 0x12, 0x14, 0x0a, 0x0f,
    0x57, 0x4f, 0x52, 0x4b, 0x53, 0x50, 0x41, 0x43, 0x45, 0x5f, 0x53, 0x45, 0x54, 0x55, 0x50, 0x10,
    0xe8, 0x07, 0x12, 0x15, 0x0a, 0x10, 0x53, 0x45, 0x43, 0x52, 0x45, 0x54, 0x5f, 0x4b, 0x45, 0x59,
    0x5f, 0x46, 0x45, 0x54, 0x43, 0x48, 0x10, 0xe9, 0x07, 0x12, 0x16, 0x0a, 0x11, 0x53, 0x45, 0x43,
    0x52, 0x45, 0x54, 0x5f, 0x4b, 0x45, 0x59, 0x5f, 0x49, 0x4d, 0x50, 0x4f, 0x52, 0x54, 0x10, 0xea,
    0x07, 0x12, 0x0e, 0x0a, 0x09, 0x56, 0x43, 0x53, 0x5f, 0x43, 0x4c, 0x4f, 0x4e, 0x45, 0x10, 0xeb,
    0x07, 0x12, 0x0a, 0x0a, 0x05, 0x42, 0x55, 0x49, 0x4c, 0x44, 0x10, 0xec, 0x07, 0x12, 0x13, 0x0a,
    0x0e, 0x50, 0x4f, 0x53, 0x54, 0x5f, 0x50, 0x52, 0x4f, 0x43, 0x45, 0x53, 0x53, 0x4f, 0x52, 0x10,
    0xed, 0x07, 0x4a, 0x9f, 0x0d, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x35, 0x0f, 0x0a, 0x08, 0x0a,
    0x01, 0x02, 0x12, 0x03, 0x00, 0x08, 0x0b, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x00, 0x12, 0x04, 0x02,
    0x00, 0x09, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x00, 0x01, 0x12, 0x03, 0x02, 0x05, 0x0d, 0x0a,
    0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x00, 0x12, 0x03, 0x03, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x03, 0x02, 0x05, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00,
    0x02, 0x00, 0x02, 0x12, 0x03, 0x03, 0x08, 0x09, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x01,
    0x12, 0x03, 0x04, 0x02, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x04, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x04, 0x0d,
    0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x02, 0x12, 0x03, 0x05, 0x02, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x05, 0x02, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x05, 0x0f, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00,
    0x02, 0x03, 0x12, 0x03, 0x06, 0x02, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x03, 0x01,
    0x12, 0x03, 0x06, 0x02, 0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x03, 0x02, 0x12, 0x03,
    0x06, 0x0e, 0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x04, 0x12, 0x03, 0x07, 0x02, 0x0d,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x07, 0x02, 0x08, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x00, 0x02, 0x04, 0x02, 0x12, 0x03, 0x07, 0x0b, 0x0c, 0x0a, 0x0b, 0x0a, 0x04,
    0x05, 0x00, 0x02, 0x05, 0x12, 0x03, 0x08, 0x02, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02,
    0x05, 0x01, 0x12, 0x03, 0x08, 0x02, 0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x05, 0x02,
    0x12, 0x03, 0x08, 0x0e, 0x0f, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x0b, 0x00, 0x0e,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x0b, 0x08, 0x11, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x0c, 0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x04, 0x12, 0x03, 0x0c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x06, 0x12, 0x03, 0x0c, 0x0b, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x0c, 0x14, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0c,
    0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0d, 0x02, 0x1b, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x0d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x0d, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0d, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x0d, 0x19, 0x1a, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x10,
    0x00, 0x14, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x10, 0x08, 0x0b, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x11, 0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x11, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x00, 0x05, 0x12, 0x03, 0x11, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x11, 0x12, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x11, 0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x12, 0x02,
    0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x04, 0x12, 0x03, 0x12, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x12, 0x0b, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x12, 0x11, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x12, 0x18, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02,
    0x02, 0x12, 0x03, 0x13, 0x02, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x04, 0x12,
    0x03, 0x13, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x06, 0x12, 0x03, 0x13,
    0x0b, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x13, 0x15, 0x1f,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x03, 0x12, 0x03, 0x13, 0x22, 0x23, 0x0a, 0x0a,
    0x0a, 0x02, 0x05, 0x01, 0x12, 0x04, 0x16, 0x00, 0x2c, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x01,
    0x01, 0x12, 0x03, 0x16, 0x05, 0x0c, 0x0a, 0x16, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x00, 0x12, 0x03,
    0x18, 0x02, 0x0a, 0x1a, 0x09, 0x20, 0x47, 0x65, 0x6e, 0x65, 0x72, 0x69, 0x63, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x18, 0x02, 0x05, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x01, 0x02, 0x00, 0x02, 0x12, 0x03, 0x18, 0x08, 0x09, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01,
    0x02, 0x01, 0x12, 0x03, 0x19, 0x02, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x19, 0x02, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x01, 0x02, 0x12, 0x03,
    0x19, 0x0c, 0x0d, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x02, 0x12, 0x03, 0x1a, 0x02, 0x16,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x1a, 0x02, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x01, 0x02, 0x02, 0x02, 0x12, 0x03, 0x1a, 0x14, 0x15, 0x0a, 0x0b, 0x0a, 0x04,
    0x05, 0x01, 0x02, 0x03, 0x12, 0x03, 0x1b, 0x02, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02,
    0x03, 0x01, 0x12, 0x03, 0x1b, 0x02, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x03, 0x02,
    0x12, 0x03, 0x1b, 0x15, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x04, 0x12, 0x03, 0x1c,
    0x02, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x04, 0x01, 0x12, 0x03, 0x1c, 0x02, 0x12,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x04, 0x02, 0x12, 0x03, 0x1c, 0x15, 0x16, 0x0a, 0x0b,
    0x0a, 0x04, 0x05, 0x01, 0x02, 0x05, 0x12, 0x03, 0x1d, 0x02, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x01, 0x02, 0x05, 0x01, 0x12, 0x03, 0x1d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02,
    0x05, 0x02, 0x12, 0x03, 0x1d, 0x0d, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x06, 0x12,
    0x03, 0x1e, 0x02, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x06, 0x01, 0x12, 0x03, 0x1e,
    0x02, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x06, 0x02, 0x12, 0x03, 0x1e, 0x12, 0x13,
    0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x07, 0x12, 0x03, 0x1f, 0x02, 0x16, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x01, 0x02, 0x07, 0x01, 0x12, 0x03, 0x1f, 0x02, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x01, 0x02, 0x07, 0x02, 0x12, 0x03, 0x1f, 0x14, 0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02,
    0x08, 0x12, 0x03, 0x20, 0x02, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x08, 0x01, 0x12,
    0x03, 0x20, 0x02, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x08, 0x02, 0x12, 0x03, 0x20,
    0x14, 0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x09, 0x12, 0x03, 0x21, 0x02, 0x0b, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x09, 0x01, 0x12, 0x03, 0x21, 0x02, 0x05, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x01, 0x02, 0x09, 0x02, 0x12, 0x03, 0x21, 0x08, 0x0a, 0x0a, 0x0b, 0x0a, 0x04, 0x05,
    0x01, 0x02, 0x0a, 0x12, 0x03, 0x22, 0x02, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x0a,
    0x01, 0x12, 0x03, 0x22, 0x02, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x0a, 0x02, 0x12,
    0x03, 0x22, 0x0f, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x0b, 0x12, 0x03, 0x23, 0x02,
    0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x0b, 0x01, 0x12, 0x03, 0x23, 0x02, 0x0c, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x0b, 0x02, 0x12, 0x03, 0x23, 0x0f, 0x11, 0x0a, 0x15, 0x0a,
    0x04, 0x05, 0x01, 0x02, 0x0c, 0x12, 0x03, 0x26, 0x02, 0x19, 0x1a, 0x08, 0x20, 0x57, 0x6f, 0x72,
    0x6b, 0x65, 0x72, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x0c, 0x01, 0x12, 0x03, 0x26,
    0x02, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x0c, 0x02, 0x12, 0x03, 0x26, 0x14, 0x18,
    0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x0d, 0x12, 0x03, 0x27, 0x02, 0x1a, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x01, 0x02, 0x0d, 0x01, 0x12, 0x03, 0x27, 0x02, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x01, 0x02, 0x0d, 0x02, 0x12, 0x03, 0x27, 0x15, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02,
    0x0e, 0x12, 0x03, 0x28, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x0e, 0x01, 0x12,
    0x03, 0x28, 0x02, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x0e, 0x02, 0x12, 0x03, 0x28,
    0x16, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x0f, 0x12, 0x03, 0x29, 0x02, 0x13, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x0f, 0x01, 0x12, 0x03, 0x29, 0x02, 0x0b, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x01, 0x02, 0x0f, 0x02, 0x12, 0x03, 0x29, 0x0e, 0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x05,
    0x01, 0x02, 0x10, 0x12, 0x03, 0x2a, 0x02, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x10,
    0x01, 0x12, 0x03, 0x2a, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x10, 0x02, 0x12,
    0x03, 0x2a, 0x0a, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x11, 0x12, 0x03, 0x2b, 0x02,
    0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x11, 0x01, 0x12, 0x03, 0x2b, 0x02, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x11, 0x02, 0x12, 0x03, 0x2b, 0x13, 0x17, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x02, 0x12, 0x04, 0x2e, 0x00, 0x31, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01,
    0x12, 0x03, 0x2e, 0x08, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x2f,
    0x02, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x2f, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x06, 0x12, 0x03, 0x2f, 0x0b, 0x12, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2f, 0x13, 0x17, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2f, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02,
    0x02, 0x01, 0x12, 0x03, 0x30, 0x02, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x04,
    0x12, 0x03, 0x30, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x05, 0x12, 0x03,
    0x30, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x30, 0x12,
    0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x30, 0x18, 0x19, 0x0a,
    0x09, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x03, 0x33, 0x00, 0x10, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03,
    0x01, 0x12, 0x03, 0x33, 0x08, 0x0d, 0x0a, 0x09, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x03, 0x34, 0x00,
    0x0f, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x34, 0x08, 0x0c, 0x0a, 0x09, 0x0a,
    0x02, 0x04, 0x05, 0x12, 0x03, 0x35, 0x00, 0x0f, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01, 0x12,
    0x03, 0x35, 0x08, 0x0c,
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
