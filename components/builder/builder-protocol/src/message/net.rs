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
pub struct Header {
    // message fields
    message_id: ::protobuf::SingularField<::std::string::String>,
    route_info: ::std::option::Option<bool>,
    txn: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Header {}

impl Header {
    pub fn new() -> Header {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Header {
        static mut instance: ::protobuf::lazy::Lazy<Header> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Header,
        };
        unsafe {
            instance.get(Header::new)
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
        }
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

    // optional bool route_info = 2;

    pub fn clear_route_info(&mut self) {
        self.route_info = ::std::option::Option::None;
    }

    pub fn has_route_info(&self) -> bool {
        self.route_info.is_some()
    }

    // Param is passed by value, moved
    pub fn set_route_info(&mut self, v: bool) {
        self.route_info = ::std::option::Option::Some(v);
    }

    pub fn get_route_info(&self) -> bool {
        self.route_info.unwrap_or(false)
    }

    fn get_route_info_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.route_info
    }

    fn mut_route_info_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.route_info
    }

    // optional bool txn = 3;

    pub fn clear_txn(&mut self) {
        self.txn = ::std::option::Option::None;
    }

    pub fn has_txn(&self) -> bool {
        self.txn.is_some()
    }

    // Param is passed by value, moved
    pub fn set_txn(&mut self, v: bool) {
        self.txn = ::std::option::Option::Some(v);
    }

    pub fn get_txn(&self) -> bool {
        self.txn.unwrap_or(false)
    }

    fn get_txn_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.txn
    }

    fn mut_txn_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.txn
    }
}

impl ::protobuf::Message for Header {
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.route_info = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.txn = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.message_id.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(v) = self.route_info {
            my_size += 2;
        }
        if let Some(v) = self.txn {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.message_id.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(v) = self.route_info {
            os.write_bool(2, v)?;
        }
        if let Some(v) = self.txn {
            os.write_bool(3, v)?;
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

impl ::protobuf::MessageStatic for Header {
    fn new() -> Header {
        Header::new()
    }

    fn descriptor_static(_: ::std::option::Option<Header>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "message_id",
                    Header::get_message_id_for_reflect,
                    Header::mut_message_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "route_info",
                    Header::get_route_info_for_reflect,
                    Header::mut_route_info_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "txn",
                    Header::get_txn_for_reflect,
                    Header::mut_txn_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Header>(
                    "Header",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Header {
    fn clear(&mut self) {
        self.clear_message_id();
        self.clear_route_info();
        self.clear_txn();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Header {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Header {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

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
                    }
                    let tmp = is.read_enum()?;
                    self.protocol = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
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
        }
        if let Some(v) = self.hash {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.protocol {
            os.write_enum(1, v.value())?;
        }
        if let Some(v) = self.hash {
            os.write_uint64(2, v)?;
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
pub struct Txn {
    // message fields
    id: ::std::option::Option<u64>,
    complete: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Txn {}

impl Txn {
    pub fn new() -> Txn {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Txn {
        static mut instance: ::protobuf::lazy::Lazy<Txn> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Txn,
        };
        unsafe {
            instance.get(Txn::new)
        }
    }

    // optional uint64 id = 1;

    pub fn clear_id(&mut self) {
        self.id = ::std::option::Option::None;
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: u64) {
        self.id = ::std::option::Option::Some(v);
    }

    pub fn get_id(&self) -> u64 {
        self.id.unwrap_or(0)
    }

    fn get_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.id
    }

    // optional bool complete = 2;

    pub fn clear_complete(&mut self) {
        self.complete = ::std::option::Option::None;
    }

    pub fn has_complete(&self) -> bool {
        self.complete.is_some()
    }

    // Param is passed by value, moved
    pub fn set_complete(&mut self, v: bool) {
        self.complete = ::std::option::Option::Some(v);
    }

    pub fn get_complete(&self) -> bool {
        self.complete.unwrap_or(false)
    }

    fn get_complete_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.complete
    }

    fn mut_complete_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.complete
    }
}

impl ::protobuf::Message for Txn {
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
                    let tmp = is.read_uint64()?;
                    self.id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.complete = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.complete {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id {
            os.write_uint64(1, v)?;
        }
        if let Some(v) = self.complete {
            os.write_bool(2, v)?;
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

impl ::protobuf::MessageStatic for Txn {
    fn new() -> Txn {
        Txn::new()
    }

    fn descriptor_static(_: ::std::option::Option<Txn>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "id",
                    Txn::get_id_for_reflect,
                    Txn::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "complete",
                    Txn::get_complete_for_reflect,
                    Txn::mut_complete_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Txn>(
                    "Txn",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Txn {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_complete();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Txn {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Txn {
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
        }
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
                    }
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
        }
        if let Some(ref v) = self.msg.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.code {
            os.write_enum(1, v.value())?;
        }
        if let Some(ref v) = self.msg.as_ref() {
            os.write_string(2, &v)?;
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

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Protocol {
    Net = 0,
    RouteSrv = 1,
    SessionSrv = 2,
    OriginSrv = 3,
    JobSrv = 4,
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
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<Protocol>) -> &'static ::protobuf::reflect::EnumDescriptor {
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
    SOCK = 10,
    DATA_STORE = 11,
    BAD_TOKEN = 12,
    REMOTE_UNAVAILABLE = 13,
    SYS = 14,
    WORKSPACE_SETUP = 1000,
    SECRET_KEY_FETCH = 1001,
    SECRET_KEY_IMPORT = 1002,
    VCS_CLONE = 1003,
    BUILD = 1004,
    POST_PROCESSOR = 1005,
    INVALID_INTEGRATIONS = 1006,
    REG_CONFLICT = 2000,
    REG_NOT_FOUND = 2001,
    GROUP_NOT_COMPLETE = 3000,
    PARTIAL_JOB_GROUP_PROMOTE = 3001,
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
            10 => ::std::option::Option::Some(ErrCode::SOCK),
            11 => ::std::option::Option::Some(ErrCode::DATA_STORE),
            12 => ::std::option::Option::Some(ErrCode::BAD_TOKEN),
            13 => ::std::option::Option::Some(ErrCode::REMOTE_UNAVAILABLE),
            14 => ::std::option::Option::Some(ErrCode::SYS),
            1000 => ::std::option::Option::Some(ErrCode::WORKSPACE_SETUP),
            1001 => ::std::option::Option::Some(ErrCode::SECRET_KEY_FETCH),
            1002 => ::std::option::Option::Some(ErrCode::SECRET_KEY_IMPORT),
            1003 => ::std::option::Option::Some(ErrCode::VCS_CLONE),
            1004 => ::std::option::Option::Some(ErrCode::BUILD),
            1005 => ::std::option::Option::Some(ErrCode::POST_PROCESSOR),
            1006 => ::std::option::Option::Some(ErrCode::INVALID_INTEGRATIONS),
            2000 => ::std::option::Option::Some(ErrCode::REG_CONFLICT),
            2001 => ::std::option::Option::Some(ErrCode::REG_NOT_FOUND),
            3000 => ::std::option::Option::Some(ErrCode::GROUP_NOT_COMPLETE),
            3001 => ::std::option::Option::Some(ErrCode::PARTIAL_JOB_GROUP_PROMOTE),
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
            ErrCode::SOCK,
            ErrCode::DATA_STORE,
            ErrCode::BAD_TOKEN,
            ErrCode::REMOTE_UNAVAILABLE,
            ErrCode::SYS,
            ErrCode::WORKSPACE_SETUP,
            ErrCode::SECRET_KEY_FETCH,
            ErrCode::SECRET_KEY_IMPORT,
            ErrCode::VCS_CLONE,
            ErrCode::BUILD,
            ErrCode::POST_PROCESSOR,
            ErrCode::INVALID_INTEGRATIONS,
            ErrCode::REG_CONFLICT,
            ErrCode::REG_NOT_FOUND,
            ErrCode::GROUP_NOT_COMPLETE,
            ErrCode::PARTIAL_JOB_GROUP_PROMOTE,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<ErrCode>) -> &'static ::protobuf::reflect::EnumDescriptor {
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

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x13protocols/net.proto\x12\x03net\"X\n\x06Header\x12\x1d\n\nmessage_i\
    d\x18\x01\x20\x01(\tR\tmessageId\x12\x1d\n\nroute_info\x18\x02\x20\x01(\
    \x08R\trouteInfo\x12\x10\n\x03txn\x18\x03\x20\x01(\x08R\x03txn\"J\n\tRou\
    teInfo\x12)\n\x08protocol\x18\x01\x20\x01(\x0e2\r.net.ProtocolR\x08proto\
    col\x12\x12\n\x04hash\x18\x02\x20\x01(\x04R\x04hash\"1\n\x03Txn\x12\x0e\
    \n\x02id\x18\x01\x20\x01(\x04R\x02id\x12\x1a\n\x08complete\x18\x02\x20\
    \x01(\x08R\x08complete\">\n\x08NetError\x12\x20\n\x04code\x18\x01\x20\
    \x01(\x0e2\x0c.net.ErrCodeR\x04code\x12\x10\n\x03msg\x18\x02\x20\x01(\tR\
    \x03msg\"\x07\n\x05NetOk*L\n\x08Protocol\x12\x07\n\x03Net\x10\0\x12\x0c\
    \n\x08RouteSrv\x10\x01\x12\x0e\n\nSessionSrv\x10\x02\x12\r\n\tOriginSrv\
    \x10\x03\x12\n\n\x06JobSrv\x10\x04*\xe6\x03\n\x07ErrCode\x12\x07\n\x03BU\
    G\x10\0\x12\x0b\n\x07TIMEOUT\x10\x01\x12\x13\n\x0fREMOTE_REJECTED\x10\
    \x02\x12\x14\n\x10BAD_REMOTE_REPLY\x10\x03\x12\x14\n\x10ENTITY_NOT_FOUND\
    \x10\x04\x12\x0c\n\x08NO_SHARD\x10\x06\x12\x11\n\rACCESS_DENIED\x10\x07\
    \x12\x13\n\x0fSESSION_EXPIRED\x10\x08\x12\x13\n\x0fENTITY_CONFLICT\x10\t\
    \x12\x08\n\x04SOCK\x10\n\x12\x0e\n\nDATA_STORE\x10\x0b\x12\r\n\tBAD_TOKE\
    N\x10\x0c\x12\x16\n\x12REMOTE_UNAVAILABLE\x10\r\x12\x07\n\x03SYS\x10\x0e\
    \x12\x14\n\x0fWORKSPACE_SETUP\x10\xe8\x07\x12\x15\n\x10SECRET_KEY_FETCH\
    \x10\xe9\x07\x12\x16\n\x11SECRET_KEY_IMPORT\x10\xea\x07\x12\x0e\n\tVCS_C\
    LONE\x10\xeb\x07\x12\n\n\x05BUILD\x10\xec\x07\x12\x13\n\x0ePOST_PROCESSO\
    R\x10\xed\x07\x12\x19\n\x14INVALID_INTEGRATIONS\x10\xee\x07\x12\x11\n\
    \x0cREG_CONFLICT\x10\xd0\x0f\x12\x12\n\rREG_NOT_FOUND\x10\xd1\x0f\x12\
    \x17\n\x12GROUP_NOT_COMPLETE\x10\xb8\x17\x12\x1e\n\x19PARTIAL_JOB_GROUP_\
    PROMOTE\x10\xb9\x17J\xac\x10\n\x06\x12\x04\0\0C\x10\n\x08\n\x01\x0c\x12\
    \x03\0\0\x12\n\x08\n\x01\x02\x12\x03\x01\x08\x0b\n\n\n\x02\x05\0\x12\x04\
    \x03\0\t\x01\n\n\n\x03\x05\0\x01\x12\x03\x03\x05\r\n\x0b\n\x04\x05\0\x02\
    \0\x12\x03\x04\x02\n\n\x0c\n\x05\x05\0\x02\0\x01\x12\x03\x04\x02\x05\n\
    \x0c\n\x05\x05\0\x02\0\x02\x12\x03\x04\x08\t\n\x0b\n\x04\x05\0\x02\x01\
    \x12\x03\x05\x02\x0f\n\x0c\n\x05\x05\0\x02\x01\x01\x12\x03\x05\x02\n\n\
    \x0c\n\x05\x05\0\x02\x01\x02\x12\x03\x05\r\x0e\n\x0b\n\x04\x05\0\x02\x02\
    \x12\x03\x06\x02\x11\n\x0c\n\x05\x05\0\x02\x02\x01\x12\x03\x06\x02\x0c\n\
    \x0c\n\x05\x05\0\x02\x02\x02\x12\x03\x06\x0f\x10\n\x0b\n\x04\x05\0\x02\
    \x03\x12\x03\x07\x02\x10\n\x0c\n\x05\x05\0\x02\x03\x01\x12\x03\x07\x02\
    \x0b\n\x0c\n\x05\x05\0\x02\x03\x02\x12\x03\x07\x0e\x0f\n\x0b\n\x04\x05\0\
    \x02\x04\x12\x03\x08\x02\r\n\x0c\n\x05\x05\0\x02\x04\x01\x12\x03\x08\x02\
    \x08\n\x0c\n\x05\x05\0\x02\x04\x02\x12\x03\x08\x0b\x0c\n\n\n\x02\x05\x01\
    \x12\x04\x0b\0,\x01\n\n\n\x03\x05\x01\x01\x12\x03\x0b\x05\x0c\n\x16\n\
    \x04\x05\x01\x02\0\x12\x03\r\x02\n\x1a\t\x20Generic\n\n\x0c\n\x05\x05\
    \x01\x02\0\x01\x12\x03\r\x02\x05\n\x0c\n\x05\x05\x01\x02\0\x02\x12\x03\r\
    \x08\t\n\x0b\n\x04\x05\x01\x02\x01\x12\x03\x0e\x02\x0e\n\x0c\n\x05\x05\
    \x01\x02\x01\x01\x12\x03\x0e\x02\t\n\x0c\n\x05\x05\x01\x02\x01\x02\x12\
    \x03\x0e\x0c\r\n\x0b\n\x04\x05\x01\x02\x02\x12\x03\x0f\x02\x16\n\x0c\n\
    \x05\x05\x01\x02\x02\x01\x12\x03\x0f\x02\x11\n\x0c\n\x05\x05\x01\x02\x02\
    \x02\x12\x03\x0f\x14\x15\n\x0b\n\x04\x05\x01\x02\x03\x12\x03\x10\x02\x17\
    \n\x0c\n\x05\x05\x01\x02\x03\x01\x12\x03\x10\x02\x12\n\x0c\n\x05\x05\x01\
    \x02\x03\x02\x12\x03\x10\x15\x16\n\x0b\n\x04\x05\x01\x02\x04\x12\x03\x11\
    \x02\x17\n\x0c\n\x05\x05\x01\x02\x04\x01\x12\x03\x11\x02\x12\n\x0c\n\x05\
    \x05\x01\x02\x04\x02\x12\x03\x11\x15\x16\n\x0b\n\x04\x05\x01\x02\x05\x12\
    \x03\x12\x02\x0f\n\x0c\n\x05\x05\x01\x02\x05\x01\x12\x03\x12\x02\n\n\x0c\
    \n\x05\x05\x01\x02\x05\x02\x12\x03\x12\r\x0e\n\x0b\n\x04\x05\x01\x02\x06\
    \x12\x03\x13\x02\x14\n\x0c\n\x05\x05\x01\x02\x06\x01\x12\x03\x13\x02\x0f\
    \n\x0c\n\x05\x05\x01\x02\x06\x02\x12\x03\x13\x12\x13\n\x0b\n\x04\x05\x01\
    \x02\x07\x12\x03\x14\x02\x16\n\x0c\n\x05\x05\x01\x02\x07\x01\x12\x03\x14\
    \x02\x11\n\x0c\n\x05\x05\x01\x02\x07\x02\x12\x03\x14\x14\x15\n\x0b\n\x04\
    \x05\x01\x02\x08\x12\x03\x15\x02\x16\n\x0c\n\x05\x05\x01\x02\x08\x01\x12\
    \x03\x15\x02\x11\n\x0c\n\x05\x05\x01\x02\x08\x02\x12\x03\x15\x14\x15\n\
    \x0b\n\x04\x05\x01\x02\t\x12\x03\x16\x02\x0c\n\x0c\n\x05\x05\x01\x02\t\
    \x01\x12\x03\x16\x02\x06\n\x0c\n\x05\x05\x01\x02\t\x02\x12\x03\x16\t\x0b\
    \n\x0b\n\x04\x05\x01\x02\n\x12\x03\x17\x02\x12\n\x0c\n\x05\x05\x01\x02\n\
    \x01\x12\x03\x17\x02\x0c\n\x0c\n\x05\x05\x01\x02\n\x02\x12\x03\x17\x0f\
    \x11\n\x0b\n\x04\x05\x01\x02\x0b\x12\x03\x18\x02\x11\n\x0c\n\x05\x05\x01\
    \x02\x0b\x01\x12\x03\x18\x02\x0b\n\x0c\n\x05\x05\x01\x02\x0b\x02\x12\x03\
    \x18\x0e\x10\n\x0b\n\x04\x05\x01\x02\x0c\x12\x03\x19\x02\x1a\n\x0c\n\x05\
    \x05\x01\x02\x0c\x01\x12\x03\x19\x02\x14\n\x0c\n\x05\x05\x01\x02\x0c\x02\
    \x12\x03\x19\x17\x19\n\x0b\n\x04\x05\x01\x02\r\x12\x03\x1a\x02\x0b\n\x0c\
    \n\x05\x05\x01\x02\r\x01\x12\x03\x1a\x02\x05\n\x0c\n\x05\x05\x01\x02\r\
    \x02\x12\x03\x1a\x08\n\n\x15\n\x04\x05\x01\x02\x0e\x12\x03\x1d\x02\x19\
    \x1a\x08\x20Worker\n\n\x0c\n\x05\x05\x01\x02\x0e\x01\x12\x03\x1d\x02\x11\
    \n\x0c\n\x05\x05\x01\x02\x0e\x02\x12\x03\x1d\x14\x18\n\x0b\n\x04\x05\x01\
    \x02\x0f\x12\x03\x1e\x02\x1a\n\x0c\n\x05\x05\x01\x02\x0f\x01\x12\x03\x1e\
    \x02\x12\n\x0c\n\x05\x05\x01\x02\x0f\x02\x12\x03\x1e\x15\x19\n\x0b\n\x04\
    \x05\x01\x02\x10\x12\x03\x1f\x02\x1b\n\x0c\n\x05\x05\x01\x02\x10\x01\x12\
    \x03\x1f\x02\x13\n\x0c\n\x05\x05\x01\x02\x10\x02\x12\x03\x1f\x16\x1a\n\
    \x0b\n\x04\x05\x01\x02\x11\x12\x03\x20\x02\x13\n\x0c\n\x05\x05\x01\x02\
    \x11\x01\x12\x03\x20\x02\x0b\n\x0c\n\x05\x05\x01\x02\x11\x02\x12\x03\x20\
    \x0e\x12\n\x0b\n\x04\x05\x01\x02\x12\x12\x03!\x02\x0f\n\x0c\n\x05\x05\
    \x01\x02\x12\x01\x12\x03!\x02\x07\n\x0c\n\x05\x05\x01\x02\x12\x02\x12\
    \x03!\n\x0e\n\x0b\n\x04\x05\x01\x02\x13\x12\x03\"\x02\x18\n\x0c\n\x05\
    \x05\x01\x02\x13\x01\x12\x03\"\x02\x10\n\x0c\n\x05\x05\x01\x02\x13\x02\
    \x12\x03\"\x13\x17\n\x0b\n\x04\x05\x01\x02\x14\x12\x03#\x02\x1e\n\x0c\n\
    \x05\x05\x01\x02\x14\x01\x12\x03#\x02\x16\n\x0c\n\x05\x05\x01\x02\x14\
    \x02\x12\x03#\x19\x1d\n\x17\n\x04\x05\x01\x02\x15\x12\x03&\x02\x16\x1a\n\
    \x20RouteSrv\n\n\x0c\n\x05\x05\x01\x02\x15\x01\x12\x03&\x02\x0e\n\x0c\n\
    \x05\x05\x01\x02\x15\x02\x12\x03&\x11\x15\n\x0b\n\x04\x05\x01\x02\x16\
    \x12\x03'\x02\x17\n\x0c\n\x05\x05\x01\x02\x16\x01\x12\x03'\x02\x0f\n\x0c\
    \n\x05\x05\x01\x02\x16\x02\x12\x03'\x12\x16\n\x18\n\x04\x05\x01\x02\x17\
    \x12\x03*\x02\x1c\x1a\x0b\x20Scheduler\n\n\x0c\n\x05\x05\x01\x02\x17\x01\
    \x12\x03*\x02\x14\n\x0c\n\x05\x05\x01\x02\x17\x02\x12\x03*\x17\x1b\n\x0b\
    \n\x04\x05\x01\x02\x18\x12\x03+\x02#\n\x0c\n\x05\x05\x01\x02\x18\x01\x12\
    \x03+\x02\x1b\n\x0c\n\x05\x05\x01\x02\x18\x02\x12\x03+\x1e\"\n\n\n\x02\
    \x04\0\x12\x04.\02\x01\n\n\n\x03\x04\0\x01\x12\x03.\x08\x0e\n\x0b\n\x04\
    \x04\0\x02\0\x12\x03/\x02!\n\x0c\n\x05\x04\0\x02\0\x04\x12\x03/\x02\n\n\
    \x0c\n\x05\x04\0\x02\0\x05\x12\x03/\x0b\x11\n\x0c\n\x05\x04\0\x02\0\x01\
    \x12\x03/\x12\x1c\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03/\x1f\x20\n\x0b\n\
    \x04\x04\0\x02\x01\x12\x030\x02\x1f\n\x0c\n\x05\x04\0\x02\x01\x04\x12\
    \x030\x02\n\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x030\x0b\x0f\n\x0c\n\x05\
    \x04\0\x02\x01\x01\x12\x030\x10\x1a\n\x0c\n\x05\x04\0\x02\x01\x03\x12\
    \x030\x1d\x1e\n\x0b\n\x04\x04\0\x02\x02\x12\x031\x02\x18\n\x0c\n\x05\x04\
    \0\x02\x02\x04\x12\x031\x02\n\n\x0c\n\x05\x04\0\x02\x02\x05\x12\x031\x0b\
    \x0f\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x031\x10\x13\n\x0c\n\x05\x04\0\
    \x02\x02\x03\x12\x031\x16\x17\n\n\n\x02\x04\x01\x12\x044\07\x01\n\n\n\
    \x03\x04\x01\x01\x12\x034\x08\x11\n\x0b\n\x04\x04\x01\x02\0\x12\x035\x02\
    !\n\x0c\n\x05\x04\x01\x02\0\x04\x12\x035\x02\n\n\x0c\n\x05\x04\x01\x02\0\
    \x06\x12\x035\x0b\x13\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x035\x14\x1c\n\
    \x0c\n\x05\x04\x01\x02\0\x03\x12\x035\x1f\x20\n\x0b\n\x04\x04\x01\x02\
    \x01\x12\x036\x02\x1b\n\x0c\n\x05\x04\x01\x02\x01\x04\x12\x036\x02\n\n\
    \x0c\n\x05\x04\x01\x02\x01\x05\x12\x036\x0b\x11\n\x0c\n\x05\x04\x01\x02\
    \x01\x01\x12\x036\x12\x16\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x036\x19\
    \x1a\n\n\n\x02\x04\x02\x12\x049\0<\x01\n\n\n\x03\x04\x02\x01\x12\x039\
    \x08\x0b\n\x0b\n\x04\x04\x02\x02\0\x12\x03:\x02\x19\n\x0c\n\x05\x04\x02\
    \x02\0\x04\x12\x03:\x02\n\n\x0c\n\x05\x04\x02\x02\0\x05\x12\x03:\x0b\x11\
    \n\x0c\n\x05\x04\x02\x02\0\x01\x12\x03:\x12\x14\n\x0c\n\x05\x04\x02\x02\
    \0\x03\x12\x03:\x17\x18\n\x0b\n\x04\x04\x02\x02\x01\x12\x03;\x02\x1d\n\
    \x0c\n\x05\x04\x02\x02\x01\x04\x12\x03;\x02\n\n\x0c\n\x05\x04\x02\x02\
    \x01\x05\x12\x03;\x0b\x0f\n\x0c\n\x05\x04\x02\x02\x01\x01\x12\x03;\x10\
    \x18\n\x0c\n\x05\x04\x02\x02\x01\x03\x12\x03;\x1b\x1c\n\n\n\x02\x04\x03\
    \x12\x04>\0A\x01\n\n\n\x03\x04\x03\x01\x12\x03>\x08\x10\n\x0b\n\x04\x04\
    \x03\x02\0\x12\x03?\x02\x1c\n\x0c\n\x05\x04\x03\x02\0\x04\x12\x03?\x02\n\
    \n\x0c\n\x05\x04\x03\x02\0\x06\x12\x03?\x0b\x12\n\x0c\n\x05\x04\x03\x02\
    \0\x01\x12\x03?\x13\x17\n\x0c\n\x05\x04\x03\x02\0\x03\x12\x03?\x1a\x1b\n\
    \x0b\n\x04\x04\x03\x02\x01\x12\x03@\x02\x1a\n\x0c\n\x05\x04\x03\x02\x01\
    \x04\x12\x03@\x02\n\n\x0c\n\x05\x04\x03\x02\x01\x05\x12\x03@\x0b\x11\n\
    \x0c\n\x05\x04\x03\x02\x01\x01\x12\x03@\x12\x15\n\x0c\n\x05\x04\x03\x02\
    \x01\x03\x12\x03@\x18\x19\n\t\n\x02\x04\x04\x12\x03C\0\x10\n\n\n\x03\x04\
    \x04\x01\x12\x03C\x08\r\
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
