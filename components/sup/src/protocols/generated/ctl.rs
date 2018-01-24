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
pub struct NetProgress {
    // message fields
    total: ::std::option::Option<u64>,
    position: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for NetProgress {}

impl NetProgress {
    pub fn new() -> NetProgress {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static NetProgress {
        static mut instance: ::protobuf::lazy::Lazy<NetProgress> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const NetProgress,
        };
        unsafe {
            instance.get(NetProgress::new)
        }
    }

    // optional uint64 total = 1;

    pub fn clear_total(&mut self) {
        self.total = ::std::option::Option::None;
    }

    pub fn has_total(&self) -> bool {
        self.total.is_some()
    }

    // Param is passed by value, moved
    pub fn set_total(&mut self, v: u64) {
        self.total = ::std::option::Option::Some(v);
    }

    pub fn get_total(&self) -> u64 {
        self.total.unwrap_or(0)
    }

    fn get_total_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.total
    }

    fn mut_total_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.total
    }

    // optional uint64 position = 2;

    pub fn clear_position(&mut self) {
        self.position = ::std::option::Option::None;
    }

    pub fn has_position(&self) -> bool {
        self.position.is_some()
    }

    // Param is passed by value, moved
    pub fn set_position(&mut self, v: u64) {
        self.position = ::std::option::Option::Some(v);
    }

    pub fn get_position(&self) -> u64 {
        self.position.unwrap_or(0)
    }

    fn get_position_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.position
    }

    fn mut_position_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.position
    }
}

impl ::protobuf::Message for NetProgress {
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
                    self.total = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.position = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.total {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.position {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.total {
            os.write_uint64(1, v)?;
        }
        if let Some(v) = self.position {
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

impl ::protobuf::MessageStatic for NetProgress {
    fn new() -> NetProgress {
        NetProgress::new()
    }

    fn descriptor_static(_: ::std::option::Option<NetProgress>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "total",
                    NetProgress::get_total_for_reflect,
                    NetProgress::mut_total_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "position",
                    NetProgress::get_position_for_reflect,
                    NetProgress::mut_position_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<NetProgress>(
                    "NetProgress",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for NetProgress {
    fn clear(&mut self) {
        self.clear_total();
        self.clear_position();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for NetProgress {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for NetProgress {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Handshake {
    // message fields
    secret_key: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Handshake {}

impl Handshake {
    pub fn new() -> Handshake {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Handshake {
        static mut instance: ::protobuf::lazy::Lazy<Handshake> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Handshake,
        };
        unsafe {
            instance.get(Handshake::new)
        }
    }

    // optional string secret_key = 1;

    pub fn clear_secret_key(&mut self) {
        self.secret_key.clear();
    }

    pub fn has_secret_key(&self) -> bool {
        self.secret_key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_secret_key(&mut self, v: ::std::string::String) {
        self.secret_key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_secret_key(&mut self) -> &mut ::std::string::String {
        if self.secret_key.is_none() {
            self.secret_key.set_default();
        }
        self.secret_key.as_mut().unwrap()
    }

    // Take field
    pub fn take_secret_key(&mut self) -> ::std::string::String {
        self.secret_key.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_secret_key(&self) -> &str {
        match self.secret_key.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_secret_key_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.secret_key
    }

    fn mut_secret_key_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.secret_key
    }
}

impl ::protobuf::Message for Handshake {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.secret_key)?;
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
        if let Some(ref v) = self.secret_key.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.secret_key.as_ref() {
            os.write_string(1, &v)?;
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

impl ::protobuf::MessageStatic for Handshake {
    fn new() -> Handshake {
        Handshake::new()
    }

    fn descriptor_static(_: ::std::option::Option<Handshake>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "secret_key",
                    Handshake::get_secret_key_for_reflect,
                    Handshake::mut_secret_key_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Handshake>(
                    "Handshake",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Handshake {
    fn clear(&mut self) {
        self.clear_secret_key();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Handshake {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Handshake {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ServiceBindList {
    // message fields
    binds: ::protobuf::RepeatedField<super::types::ServiceBind>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ServiceBindList {}

impl ServiceBindList {
    pub fn new() -> ServiceBindList {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ServiceBindList {
        static mut instance: ::protobuf::lazy::Lazy<ServiceBindList> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ServiceBindList,
        };
        unsafe {
            instance.get(ServiceBindList::new)
        }
    }

    // repeated .ServiceBind binds = 1;

    pub fn clear_binds(&mut self) {
        self.binds.clear();
    }

    // Param is passed by value, moved
    pub fn set_binds(&mut self, v: ::protobuf::RepeatedField<super::types::ServiceBind>) {
        self.binds = v;
    }

    // Mutable pointer to the field.
    pub fn mut_binds(&mut self) -> &mut ::protobuf::RepeatedField<super::types::ServiceBind> {
        &mut self.binds
    }

    // Take field
    pub fn take_binds(&mut self) -> ::protobuf::RepeatedField<super::types::ServiceBind> {
        ::std::mem::replace(&mut self.binds, ::protobuf::RepeatedField::new())
    }

    pub fn get_binds(&self) -> &[super::types::ServiceBind] {
        &self.binds
    }

    fn get_binds_for_reflect(&self) -> &::protobuf::RepeatedField<super::types::ServiceBind> {
        &self.binds
    }

    fn mut_binds_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::types::ServiceBind> {
        &mut self.binds
    }
}

impl ::protobuf::Message for ServiceBindList {
    fn is_initialized(&self) -> bool {
        for v in &self.binds {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.binds)?;
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
        for value in &self.binds {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.binds {
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

impl ::protobuf::MessageStatic for ServiceBindList {
    fn new() -> ServiceBindList {
        ServiceBindList::new()
    }

    fn descriptor_static(_: ::std::option::Option<ServiceBindList>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::types::ServiceBind>>(
                    "binds",
                    ServiceBindList::get_binds_for_reflect,
                    ServiceBindList::mut_binds_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ServiceBindList>(
                    "ServiceBindList",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ServiceBindList {
    fn clear(&mut self) {
        self.clear_binds();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ServiceBindList {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ServiceBindList {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SvcGetDefaultCfg {
    // message fields
    ident: ::protobuf::SingularPtrField<super::types::PackageIdent>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SvcGetDefaultCfg {}

impl SvcGetDefaultCfg {
    pub fn new() -> SvcGetDefaultCfg {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SvcGetDefaultCfg {
        static mut instance: ::protobuf::lazy::Lazy<SvcGetDefaultCfg> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SvcGetDefaultCfg,
        };
        unsafe {
            instance.get(SvcGetDefaultCfg::new)
        }
    }

    // optional .PackageIdent ident = 1;

    pub fn clear_ident(&mut self) {
        self.ident.clear();
    }

    pub fn has_ident(&self) -> bool {
        self.ident.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ident(&mut self, v: super::types::PackageIdent) {
        self.ident = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ident(&mut self) -> &mut super::types::PackageIdent {
        if self.ident.is_none() {
            self.ident.set_default();
        }
        self.ident.as_mut().unwrap()
    }

    // Take field
    pub fn take_ident(&mut self) -> super::types::PackageIdent {
        self.ident.take().unwrap_or_else(|| super::types::PackageIdent::new())
    }

    pub fn get_ident(&self) -> &super::types::PackageIdent {
        self.ident.as_ref().unwrap_or_else(|| super::types::PackageIdent::default_instance())
    }

    fn get_ident_for_reflect(&self) -> &::protobuf::SingularPtrField<super::types::PackageIdent> {
        &self.ident
    }

    fn mut_ident_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::types::PackageIdent> {
        &mut self.ident
    }
}

impl ::protobuf::Message for SvcGetDefaultCfg {
    fn is_initialized(&self) -> bool {
        for v in &self.ident {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.ident)?;
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
        if let Some(ref v) = self.ident.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.ident.as_ref() {
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

impl ::protobuf::MessageStatic for SvcGetDefaultCfg {
    fn new() -> SvcGetDefaultCfg {
        SvcGetDefaultCfg::new()
    }

    fn descriptor_static(_: ::std::option::Option<SvcGetDefaultCfg>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::types::PackageIdent>>(
                    "ident",
                    SvcGetDefaultCfg::get_ident_for_reflect,
                    SvcGetDefaultCfg::mut_ident_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SvcGetDefaultCfg>(
                    "SvcGetDefaultCfg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SvcGetDefaultCfg {
    fn clear(&mut self) {
        self.clear_ident();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SvcGetDefaultCfg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SvcGetDefaultCfg {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SvcLoad {
    // message fields
    ident: ::protobuf::SingularPtrField<super::types::PackageIdent>,
    application_environment: ::protobuf::SingularPtrField<super::types::ApplicationEnvironment>,
    binds: ::protobuf::RepeatedField<super::types::ServiceBind>,
    specified_binds: ::std::option::Option<bool>,
    bldr_url: ::protobuf::SingularField<::std::string::String>,
    bldr_channel: ::protobuf::SingularField<::std::string::String>,
    config_from: ::protobuf::SingularField<::std::string::String>,
    force: ::std::option::Option<bool>,
    group: ::protobuf::SingularField<::std::string::String>,
    svc_encrypted_password: ::protobuf::SingularField<::std::string::String>,
    topology: ::std::option::Option<super::types::Topology>,
    update_strategy: ::std::option::Option<super::types::UpdateStrategy>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SvcLoad {}

impl SvcLoad {
    pub fn new() -> SvcLoad {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SvcLoad {
        static mut instance: ::protobuf::lazy::Lazy<SvcLoad> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SvcLoad,
        };
        unsafe {
            instance.get(SvcLoad::new)
        }
    }

    // optional .PackageIdent ident = 1;

    pub fn clear_ident(&mut self) {
        self.ident.clear();
    }

    pub fn has_ident(&self) -> bool {
        self.ident.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ident(&mut self, v: super::types::PackageIdent) {
        self.ident = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ident(&mut self) -> &mut super::types::PackageIdent {
        if self.ident.is_none() {
            self.ident.set_default();
        }
        self.ident.as_mut().unwrap()
    }

    // Take field
    pub fn take_ident(&mut self) -> super::types::PackageIdent {
        self.ident.take().unwrap_or_else(|| super::types::PackageIdent::new())
    }

    pub fn get_ident(&self) -> &super::types::PackageIdent {
        self.ident.as_ref().unwrap_or_else(|| super::types::PackageIdent::default_instance())
    }

    fn get_ident_for_reflect(&self) -> &::protobuf::SingularPtrField<super::types::PackageIdent> {
        &self.ident
    }

    fn mut_ident_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::types::PackageIdent> {
        &mut self.ident
    }

    // optional .ApplicationEnvironment application_environment = 2;

    pub fn clear_application_environment(&mut self) {
        self.application_environment.clear();
    }

    pub fn has_application_environment(&self) -> bool {
        self.application_environment.is_some()
    }

    // Param is passed by value, moved
    pub fn set_application_environment(&mut self, v: super::types::ApplicationEnvironment) {
        self.application_environment = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_application_environment(&mut self) -> &mut super::types::ApplicationEnvironment {
        if self.application_environment.is_none() {
            self.application_environment.set_default();
        }
        self.application_environment.as_mut().unwrap()
    }

    // Take field
    pub fn take_application_environment(&mut self) -> super::types::ApplicationEnvironment {
        self.application_environment.take().unwrap_or_else(|| super::types::ApplicationEnvironment::new())
    }

    pub fn get_application_environment(&self) -> &super::types::ApplicationEnvironment {
        self.application_environment.as_ref().unwrap_or_else(|| super::types::ApplicationEnvironment::default_instance())
    }

    fn get_application_environment_for_reflect(&self) -> &::protobuf::SingularPtrField<super::types::ApplicationEnvironment> {
        &self.application_environment
    }

    fn mut_application_environment_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::types::ApplicationEnvironment> {
        &mut self.application_environment
    }

    // repeated .ServiceBind binds = 3;

    pub fn clear_binds(&mut self) {
        self.binds.clear();
    }

    // Param is passed by value, moved
    pub fn set_binds(&mut self, v: ::protobuf::RepeatedField<super::types::ServiceBind>) {
        self.binds = v;
    }

    // Mutable pointer to the field.
    pub fn mut_binds(&mut self) -> &mut ::protobuf::RepeatedField<super::types::ServiceBind> {
        &mut self.binds
    }

    // Take field
    pub fn take_binds(&mut self) -> ::protobuf::RepeatedField<super::types::ServiceBind> {
        ::std::mem::replace(&mut self.binds, ::protobuf::RepeatedField::new())
    }

    pub fn get_binds(&self) -> &[super::types::ServiceBind] {
        &self.binds
    }

    fn get_binds_for_reflect(&self) -> &::protobuf::RepeatedField<super::types::ServiceBind> {
        &self.binds
    }

    fn mut_binds_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::types::ServiceBind> {
        &mut self.binds
    }

    // optional bool specified_binds = 5;

    pub fn clear_specified_binds(&mut self) {
        self.specified_binds = ::std::option::Option::None;
    }

    pub fn has_specified_binds(&self) -> bool {
        self.specified_binds.is_some()
    }

    // Param is passed by value, moved
    pub fn set_specified_binds(&mut self, v: bool) {
        self.specified_binds = ::std::option::Option::Some(v);
    }

    pub fn get_specified_binds(&self) -> bool {
        self.specified_binds.unwrap_or(false)
    }

    fn get_specified_binds_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.specified_binds
    }

    fn mut_specified_binds_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.specified_binds
    }

    // optional string bldr_url = 6;

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

    // optional string bldr_channel = 7;

    pub fn clear_bldr_channel(&mut self) {
        self.bldr_channel.clear();
    }

    pub fn has_bldr_channel(&self) -> bool {
        self.bldr_channel.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bldr_channel(&mut self, v: ::std::string::String) {
        self.bldr_channel = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_bldr_channel(&mut self) -> &mut ::std::string::String {
        if self.bldr_channel.is_none() {
            self.bldr_channel.set_default();
        }
        self.bldr_channel.as_mut().unwrap()
    }

    // Take field
    pub fn take_bldr_channel(&mut self) -> ::std::string::String {
        self.bldr_channel.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_bldr_channel(&self) -> &str {
        match self.bldr_channel.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_bldr_channel_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.bldr_channel
    }

    fn mut_bldr_channel_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.bldr_channel
    }

    // optional string config_from = 8;

    pub fn clear_config_from(&mut self) {
        self.config_from.clear();
    }

    pub fn has_config_from(&self) -> bool {
        self.config_from.is_some()
    }

    // Param is passed by value, moved
    pub fn set_config_from(&mut self, v: ::std::string::String) {
        self.config_from = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_config_from(&mut self) -> &mut ::std::string::String {
        if self.config_from.is_none() {
            self.config_from.set_default();
        }
        self.config_from.as_mut().unwrap()
    }

    // Take field
    pub fn take_config_from(&mut self) -> ::std::string::String {
        self.config_from.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_config_from(&self) -> &str {
        match self.config_from.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_config_from_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.config_from
    }

    fn mut_config_from_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.config_from
    }

    // optional bool force = 9;

    pub fn clear_force(&mut self) {
        self.force = ::std::option::Option::None;
    }

    pub fn has_force(&self) -> bool {
        self.force.is_some()
    }

    // Param is passed by value, moved
    pub fn set_force(&mut self, v: bool) {
        self.force = ::std::option::Option::Some(v);
    }

    pub fn get_force(&self) -> bool {
        self.force.unwrap_or(false)
    }

    fn get_force_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.force
    }

    fn mut_force_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.force
    }

    // optional string group = 10;

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

    // optional string svc_encrypted_password = 11;

    pub fn clear_svc_encrypted_password(&mut self) {
        self.svc_encrypted_password.clear();
    }

    pub fn has_svc_encrypted_password(&self) -> bool {
        self.svc_encrypted_password.is_some()
    }

    // Param is passed by value, moved
    pub fn set_svc_encrypted_password(&mut self, v: ::std::string::String) {
        self.svc_encrypted_password = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_svc_encrypted_password(&mut self) -> &mut ::std::string::String {
        if self.svc_encrypted_password.is_none() {
            self.svc_encrypted_password.set_default();
        }
        self.svc_encrypted_password.as_mut().unwrap()
    }

    // Take field
    pub fn take_svc_encrypted_password(&mut self) -> ::std::string::String {
        self.svc_encrypted_password.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_svc_encrypted_password(&self) -> &str {
        match self.svc_encrypted_password.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_svc_encrypted_password_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.svc_encrypted_password
    }

    fn mut_svc_encrypted_password_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.svc_encrypted_password
    }

    // optional .Topology topology = 12;

    pub fn clear_topology(&mut self) {
        self.topology = ::std::option::Option::None;
    }

    pub fn has_topology(&self) -> bool {
        self.topology.is_some()
    }

    // Param is passed by value, moved
    pub fn set_topology(&mut self, v: super::types::Topology) {
        self.topology = ::std::option::Option::Some(v);
    }

    pub fn get_topology(&self) -> super::types::Topology {
        self.topology.unwrap_or(super::types::Topology::Standalone)
    }

    fn get_topology_for_reflect(&self) -> &::std::option::Option<super::types::Topology> {
        &self.topology
    }

    fn mut_topology_for_reflect(&mut self) -> &mut ::std::option::Option<super::types::Topology> {
        &mut self.topology
    }

    // optional .UpdateStrategy update_strategy = 13;

    pub fn clear_update_strategy(&mut self) {
        self.update_strategy = ::std::option::Option::None;
    }

    pub fn has_update_strategy(&self) -> bool {
        self.update_strategy.is_some()
    }

    // Param is passed by value, moved
    pub fn set_update_strategy(&mut self, v: super::types::UpdateStrategy) {
        self.update_strategy = ::std::option::Option::Some(v);
    }

    pub fn get_update_strategy(&self) -> super::types::UpdateStrategy {
        self.update_strategy.unwrap_or(super::types::UpdateStrategy::None)
    }

    fn get_update_strategy_for_reflect(&self) -> &::std::option::Option<super::types::UpdateStrategy> {
        &self.update_strategy
    }

    fn mut_update_strategy_for_reflect(&mut self) -> &mut ::std::option::Option<super::types::UpdateStrategy> {
        &mut self.update_strategy
    }
}

impl ::protobuf::Message for SvcLoad {
    fn is_initialized(&self) -> bool {
        for v in &self.ident {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.application_environment {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.binds {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.ident)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.application_environment)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.binds)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.specified_binds = ::std::option::Option::Some(tmp);
                },
                6 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.bldr_url)?;
                },
                7 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.bldr_channel)?;
                },
                8 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.config_from)?;
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.force = ::std::option::Option::Some(tmp);
                },
                10 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.group)?;
                },
                11 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.svc_encrypted_password)?;
                },
                12 => {
                    ::protobuf::rt::read_proto2_enum_with_unknown_fields_into(wire_type, is, &mut self.topology, 12, &mut self.unknown_fields)?
                },
                13 => {
                    ::protobuf::rt::read_proto2_enum_with_unknown_fields_into(wire_type, is, &mut self.update_strategy, 13, &mut self.unknown_fields)?
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
        if let Some(ref v) = self.ident.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.application_environment.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        for value in &self.binds {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.specified_binds {
            my_size += 2;
        }
        if let Some(ref v) = self.bldr_url.as_ref() {
            my_size += ::protobuf::rt::string_size(6, &v);
        }
        if let Some(ref v) = self.bldr_channel.as_ref() {
            my_size += ::protobuf::rt::string_size(7, &v);
        }
        if let Some(ref v) = self.config_from.as_ref() {
            my_size += ::protobuf::rt::string_size(8, &v);
        }
        if let Some(v) = self.force {
            my_size += 2;
        }
        if let Some(ref v) = self.group.as_ref() {
            my_size += ::protobuf::rt::string_size(10, &v);
        }
        if let Some(ref v) = self.svc_encrypted_password.as_ref() {
            my_size += ::protobuf::rt::string_size(11, &v);
        }
        if let Some(v) = self.topology {
            my_size += ::protobuf::rt::enum_size(12, v);
        }
        if let Some(v) = self.update_strategy {
            my_size += ::protobuf::rt::enum_size(13, v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.ident.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.application_environment.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        for v in &self.binds {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.specified_binds {
            os.write_bool(5, v)?;
        }
        if let Some(ref v) = self.bldr_url.as_ref() {
            os.write_string(6, &v)?;
        }
        if let Some(ref v) = self.bldr_channel.as_ref() {
            os.write_string(7, &v)?;
        }
        if let Some(ref v) = self.config_from.as_ref() {
            os.write_string(8, &v)?;
        }
        if let Some(v) = self.force {
            os.write_bool(9, v)?;
        }
        if let Some(ref v) = self.group.as_ref() {
            os.write_string(10, &v)?;
        }
        if let Some(ref v) = self.svc_encrypted_password.as_ref() {
            os.write_string(11, &v)?;
        }
        if let Some(v) = self.topology {
            os.write_enum(12, v.value())?;
        }
        if let Some(v) = self.update_strategy {
            os.write_enum(13, v.value())?;
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

impl ::protobuf::MessageStatic for SvcLoad {
    fn new() -> SvcLoad {
        SvcLoad::new()
    }

    fn descriptor_static(_: ::std::option::Option<SvcLoad>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::types::PackageIdent>>(
                    "ident",
                    SvcLoad::get_ident_for_reflect,
                    SvcLoad::mut_ident_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::types::ApplicationEnvironment>>(
                    "application_environment",
                    SvcLoad::get_application_environment_for_reflect,
                    SvcLoad::mut_application_environment_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::types::ServiceBind>>(
                    "binds",
                    SvcLoad::get_binds_for_reflect,
                    SvcLoad::mut_binds_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "specified_binds",
                    SvcLoad::get_specified_binds_for_reflect,
                    SvcLoad::mut_specified_binds_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "bldr_url",
                    SvcLoad::get_bldr_url_for_reflect,
                    SvcLoad::mut_bldr_url_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "bldr_channel",
                    SvcLoad::get_bldr_channel_for_reflect,
                    SvcLoad::mut_bldr_channel_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "config_from",
                    SvcLoad::get_config_from_for_reflect,
                    SvcLoad::mut_config_from_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "force",
                    SvcLoad::get_force_for_reflect,
                    SvcLoad::mut_force_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "group",
                    SvcLoad::get_group_for_reflect,
                    SvcLoad::mut_group_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "svc_encrypted_password",
                    SvcLoad::get_svc_encrypted_password_for_reflect,
                    SvcLoad::mut_svc_encrypted_password_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::types::Topology>>(
                    "topology",
                    SvcLoad::get_topology_for_reflect,
                    SvcLoad::mut_topology_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::types::UpdateStrategy>>(
                    "update_strategy",
                    SvcLoad::get_update_strategy_for_reflect,
                    SvcLoad::mut_update_strategy_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SvcLoad>(
                    "SvcLoad",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SvcLoad {
    fn clear(&mut self) {
        self.clear_ident();
        self.clear_application_environment();
        self.clear_binds();
        self.clear_specified_binds();
        self.clear_bldr_url();
        self.clear_bldr_channel();
        self.clear_config_from();
        self.clear_force();
        self.clear_group();
        self.clear_svc_encrypted_password();
        self.clear_topology();
        self.clear_update_strategy();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SvcLoad {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SvcLoad {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SvcUnload {
    // message fields
    ident: ::protobuf::SingularPtrField<super::types::PackageIdent>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SvcUnload {}

impl SvcUnload {
    pub fn new() -> SvcUnload {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SvcUnload {
        static mut instance: ::protobuf::lazy::Lazy<SvcUnload> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SvcUnload,
        };
        unsafe {
            instance.get(SvcUnload::new)
        }
    }

    // optional .PackageIdent ident = 1;

    pub fn clear_ident(&mut self) {
        self.ident.clear();
    }

    pub fn has_ident(&self) -> bool {
        self.ident.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ident(&mut self, v: super::types::PackageIdent) {
        self.ident = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ident(&mut self) -> &mut super::types::PackageIdent {
        if self.ident.is_none() {
            self.ident.set_default();
        }
        self.ident.as_mut().unwrap()
    }

    // Take field
    pub fn take_ident(&mut self) -> super::types::PackageIdent {
        self.ident.take().unwrap_or_else(|| super::types::PackageIdent::new())
    }

    pub fn get_ident(&self) -> &super::types::PackageIdent {
        self.ident.as_ref().unwrap_or_else(|| super::types::PackageIdent::default_instance())
    }

    fn get_ident_for_reflect(&self) -> &::protobuf::SingularPtrField<super::types::PackageIdent> {
        &self.ident
    }

    fn mut_ident_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::types::PackageIdent> {
        &mut self.ident
    }
}

impl ::protobuf::Message for SvcUnload {
    fn is_initialized(&self) -> bool {
        for v in &self.ident {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.ident)?;
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
        if let Some(ref v) = self.ident.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.ident.as_ref() {
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

impl ::protobuf::MessageStatic for SvcUnload {
    fn new() -> SvcUnload {
        SvcUnload::new()
    }

    fn descriptor_static(_: ::std::option::Option<SvcUnload>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::types::PackageIdent>>(
                    "ident",
                    SvcUnload::get_ident_for_reflect,
                    SvcUnload::mut_ident_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SvcUnload>(
                    "SvcUnload",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SvcUnload {
    fn clear(&mut self) {
        self.clear_ident();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SvcUnload {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SvcUnload {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SvcStart {
    // message fields
    ident: ::protobuf::SingularPtrField<super::types::PackageIdent>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SvcStart {}

impl SvcStart {
    pub fn new() -> SvcStart {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SvcStart {
        static mut instance: ::protobuf::lazy::Lazy<SvcStart> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SvcStart,
        };
        unsafe {
            instance.get(SvcStart::new)
        }
    }

    // optional .PackageIdent ident = 1;

    pub fn clear_ident(&mut self) {
        self.ident.clear();
    }

    pub fn has_ident(&self) -> bool {
        self.ident.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ident(&mut self, v: super::types::PackageIdent) {
        self.ident = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ident(&mut self) -> &mut super::types::PackageIdent {
        if self.ident.is_none() {
            self.ident.set_default();
        }
        self.ident.as_mut().unwrap()
    }

    // Take field
    pub fn take_ident(&mut self) -> super::types::PackageIdent {
        self.ident.take().unwrap_or_else(|| super::types::PackageIdent::new())
    }

    pub fn get_ident(&self) -> &super::types::PackageIdent {
        self.ident.as_ref().unwrap_or_else(|| super::types::PackageIdent::default_instance())
    }

    fn get_ident_for_reflect(&self) -> &::protobuf::SingularPtrField<super::types::PackageIdent> {
        &self.ident
    }

    fn mut_ident_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::types::PackageIdent> {
        &mut self.ident
    }
}

impl ::protobuf::Message for SvcStart {
    fn is_initialized(&self) -> bool {
        for v in &self.ident {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.ident)?;
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
        if let Some(ref v) = self.ident.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.ident.as_ref() {
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

impl ::protobuf::MessageStatic for SvcStart {
    fn new() -> SvcStart {
        SvcStart::new()
    }

    fn descriptor_static(_: ::std::option::Option<SvcStart>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::types::PackageIdent>>(
                    "ident",
                    SvcStart::get_ident_for_reflect,
                    SvcStart::mut_ident_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SvcStart>(
                    "SvcStart",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SvcStart {
    fn clear(&mut self) {
        self.clear_ident();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SvcStart {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SvcStart {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SvcStop {
    // message fields
    ident: ::protobuf::SingularPtrField<super::types::PackageIdent>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SvcStop {}

impl SvcStop {
    pub fn new() -> SvcStop {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SvcStop {
        static mut instance: ::protobuf::lazy::Lazy<SvcStop> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SvcStop,
        };
        unsafe {
            instance.get(SvcStop::new)
        }
    }

    // optional .PackageIdent ident = 1;

    pub fn clear_ident(&mut self) {
        self.ident.clear();
    }

    pub fn has_ident(&self) -> bool {
        self.ident.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ident(&mut self, v: super::types::PackageIdent) {
        self.ident = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ident(&mut self) -> &mut super::types::PackageIdent {
        if self.ident.is_none() {
            self.ident.set_default();
        }
        self.ident.as_mut().unwrap()
    }

    // Take field
    pub fn take_ident(&mut self) -> super::types::PackageIdent {
        self.ident.take().unwrap_or_else(|| super::types::PackageIdent::new())
    }

    pub fn get_ident(&self) -> &super::types::PackageIdent {
        self.ident.as_ref().unwrap_or_else(|| super::types::PackageIdent::default_instance())
    }

    fn get_ident_for_reflect(&self) -> &::protobuf::SingularPtrField<super::types::PackageIdent> {
        &self.ident
    }

    fn mut_ident_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::types::PackageIdent> {
        &mut self.ident
    }
}

impl ::protobuf::Message for SvcStop {
    fn is_initialized(&self) -> bool {
        for v in &self.ident {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.ident)?;
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
        if let Some(ref v) = self.ident.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.ident.as_ref() {
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

impl ::protobuf::MessageStatic for SvcStop {
    fn new() -> SvcStop {
        SvcStop::new()
    }

    fn descriptor_static(_: ::std::option::Option<SvcStop>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::types::PackageIdent>>(
                    "ident",
                    SvcStop::get_ident_for_reflect,
                    SvcStop::mut_ident_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SvcStop>(
                    "SvcStop",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SvcStop {
    fn clear(&mut self) {
        self.clear_ident();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SvcStop {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SvcStop {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SvcStatus {
    // message fields
    ident: ::protobuf::SingularPtrField<super::types::PackageIdent>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SvcStatus {}

impl SvcStatus {
    pub fn new() -> SvcStatus {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SvcStatus {
        static mut instance: ::protobuf::lazy::Lazy<SvcStatus> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SvcStatus,
        };
        unsafe {
            instance.get(SvcStatus::new)
        }
    }

    // optional .PackageIdent ident = 1;

    pub fn clear_ident(&mut self) {
        self.ident.clear();
    }

    pub fn has_ident(&self) -> bool {
        self.ident.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ident(&mut self, v: super::types::PackageIdent) {
        self.ident = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ident(&mut self) -> &mut super::types::PackageIdent {
        if self.ident.is_none() {
            self.ident.set_default();
        }
        self.ident.as_mut().unwrap()
    }

    // Take field
    pub fn take_ident(&mut self) -> super::types::PackageIdent {
        self.ident.take().unwrap_or_else(|| super::types::PackageIdent::new())
    }

    pub fn get_ident(&self) -> &super::types::PackageIdent {
        self.ident.as_ref().unwrap_or_else(|| super::types::PackageIdent::default_instance())
    }

    fn get_ident_for_reflect(&self) -> &::protobuf::SingularPtrField<super::types::PackageIdent> {
        &self.ident
    }

    fn mut_ident_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::types::PackageIdent> {
        &mut self.ident
    }
}

impl ::protobuf::Message for SvcStatus {
    fn is_initialized(&self) -> bool {
        for v in &self.ident {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.ident)?;
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
        if let Some(ref v) = self.ident.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.ident.as_ref() {
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

impl ::protobuf::MessageStatic for SvcStatus {
    fn new() -> SvcStatus {
        SvcStatus::new()
    }

    fn descriptor_static(_: ::std::option::Option<SvcStatus>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::types::PackageIdent>>(
                    "ident",
                    SvcStatus::get_ident_for_reflect,
                    SvcStatus::mut_ident_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SvcStatus>(
                    "SvcStatus",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SvcStatus {
    fn clear(&mut self) {
        self.clear_ident();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SvcStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SvcStatus {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ConsoleLine {
    // message fields
    line: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ConsoleLine {}

impl ConsoleLine {
    pub fn new() -> ConsoleLine {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ConsoleLine {
        static mut instance: ::protobuf::lazy::Lazy<ConsoleLine> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ConsoleLine,
        };
        unsafe {
            instance.get(ConsoleLine::new)
        }
    }

    // optional string line = 2;

    pub fn clear_line(&mut self) {
        self.line.clear();
    }

    pub fn has_line(&self) -> bool {
        self.line.is_some()
    }

    // Param is passed by value, moved
    pub fn set_line(&mut self, v: ::std::string::String) {
        self.line = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_line(&mut self) -> &mut ::std::string::String {
        if self.line.is_none() {
            self.line.set_default();
        }
        self.line.as_mut().unwrap()
    }

    // Take field
    pub fn take_line(&mut self) -> ::std::string::String {
        self.line.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_line(&self) -> &str {
        match self.line.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_line_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.line
    }

    fn mut_line_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.line
    }
}

impl ::protobuf::Message for ConsoleLine {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.line)?;
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
        if let Some(ref v) = self.line.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.line.as_ref() {
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

impl ::protobuf::MessageStatic for ConsoleLine {
    fn new() -> ConsoleLine {
        ConsoleLine::new()
    }

    fn descriptor_static(_: ::std::option::Option<ConsoleLine>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "line",
                    ConsoleLine::get_line_for_reflect,
                    ConsoleLine::mut_line_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ConsoleLine>(
                    "ConsoleLine",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ConsoleLine {
    fn clear(&mut self) {
        self.clear_line();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ConsoleLine {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ConsoleLine {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\tctl.proto\x1a\x0btypes.proto\"?\n\x0bNetProgress\x12\x14\n\x05total\
    \x18\x01\x20\x01(\x04R\x05total\x12\x1a\n\x08position\x18\x02\x20\x01(\
    \x04R\x08position\"*\n\tHandshake\x12\x1d\n\nsecret_key\x18\x01\x20\x01(\
    \tR\tsecretKey\"5\n\x0fServiceBindList\x12\"\n\x05binds\x18\x01\x20\x03(\
    \x0b2\x0c.ServiceBindR\x05binds\"7\n\x10SvcGetDefaultCfg\x12#\n\x05ident\
    \x18\x01\x20\x01(\x0b2\r.PackageIdentR\x05ident\"\xf6\x03\n\x07SvcLoad\
    \x12#\n\x05ident\x18\x01\x20\x01(\x0b2\r.PackageIdentR\x05ident\x12P\n\
    \x17application_environment\x18\x02\x20\x01(\x0b2\x17.ApplicationEnviron\
    mentR\x16applicationEnvironment\x12\"\n\x05binds\x18\x03\x20\x03(\x0b2\
    \x0c.ServiceBindR\x05binds\x12'\n\x0fspecified_binds\x18\x05\x20\x01(\
    \x08R\x0especifiedBinds\x12\x19\n\x08bldr_url\x18\x06\x20\x01(\tR\x07bld\
    rUrl\x12!\n\x0cbldr_channel\x18\x07\x20\x01(\tR\x0bbldrChannel\x12\x1f\n\
    \x0bconfig_from\x18\x08\x20\x01(\tR\nconfigFrom\x12\x1b\n\x05force\x18\t\
    \x20\x01(\x08:\x05falseR\x05force\x12\x14\n\x05group\x18\n\x20\x01(\tR\
    \x05group\x124\n\x16svc_encrypted_password\x18\x0b\x20\x01(\tR\x14svcEnc\
    ryptedPassword\x12%\n\x08topology\x18\x0c\x20\x01(\x0e2\t.TopologyR\x08t\
    opology\x128\n\x0fupdate_strategy\x18\r\x20\x01(\x0e2\x0f.UpdateStrategy\
    R\x0eupdateStrategy\"0\n\tSvcUnload\x12#\n\x05ident\x18\x01\x20\x01(\x0b\
    2\r.PackageIdentR\x05ident\"/\n\x08SvcStart\x12#\n\x05ident\x18\x01\x20\
    \x01(\x0b2\r.PackageIdentR\x05ident\".\n\x07SvcStop\x12#\n\x05ident\x18\
    \x01\x20\x01(\x0b2\r.PackageIdentR\x05ident\"0\n\tSvcStatus\x12#\n\x05id\
    ent\x18\x01\x20\x01(\x0b2\r.PackageIdentR\x05ident\"!\n\x0bConsoleLine\
    \x12\x12\n\x04line\x18\x02\x20\x01(\tR\x04line\
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
