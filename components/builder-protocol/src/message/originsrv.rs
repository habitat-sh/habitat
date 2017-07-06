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
pub struct AccountInvitationListRequest {
    // message fields
    account_id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AccountInvitationListRequest {}

impl AccountInvitationListRequest {
    pub fn new() -> AccountInvitationListRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AccountInvitationListRequest {
        static mut instance: ::protobuf::lazy::Lazy<AccountInvitationListRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AccountInvitationListRequest,
        };
        unsafe {
            instance.get(AccountInvitationListRequest::new)
        }
    }

    // optional uint64 account_id = 1;

    pub fn clear_account_id(&mut self) {
        self.account_id = ::std::option::Option::None;
    }

    pub fn has_account_id(&self) -> bool {
        self.account_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_account_id(&mut self, v: u64) {
        self.account_id = ::std::option::Option::Some(v);
    }

    pub fn get_account_id(&self) -> u64 {
        self.account_id.unwrap_or(0)
    }

    fn get_account_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.account_id
    }

    fn mut_account_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.account_id
    }
}

impl ::protobuf::Message for AccountInvitationListRequest {
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
                    self.account_id = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.account_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.account_id {
            os.write_uint64(1, v)?;
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

impl ::protobuf::MessageStatic for AccountInvitationListRequest {
    fn new() -> AccountInvitationListRequest {
        AccountInvitationListRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<AccountInvitationListRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "account_id",
                    AccountInvitationListRequest::get_account_id_for_reflect,
                    AccountInvitationListRequest::mut_account_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AccountInvitationListRequest>(
                    "AccountInvitationListRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AccountInvitationListRequest {
    fn clear(&mut self) {
        self.clear_account_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AccountInvitationListRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AccountInvitationListRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AccountInvitationListResponse {
    // message fields
    account_id: ::std::option::Option<u64>,
    invitations: ::protobuf::RepeatedField<OriginInvitation>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AccountInvitationListResponse {}

impl AccountInvitationListResponse {
    pub fn new() -> AccountInvitationListResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AccountInvitationListResponse {
        static mut instance: ::protobuf::lazy::Lazy<AccountInvitationListResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AccountInvitationListResponse,
        };
        unsafe {
            instance.get(AccountInvitationListResponse::new)
        }
    }

    // optional uint64 account_id = 1;

    pub fn clear_account_id(&mut self) {
        self.account_id = ::std::option::Option::None;
    }

    pub fn has_account_id(&self) -> bool {
        self.account_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_account_id(&mut self, v: u64) {
        self.account_id = ::std::option::Option::Some(v);
    }

    pub fn get_account_id(&self) -> u64 {
        self.account_id.unwrap_or(0)
    }

    fn get_account_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.account_id
    }

    fn mut_account_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.account_id
    }

    // repeated .originsrv.OriginInvitation invitations = 2;

    pub fn clear_invitations(&mut self) {
        self.invitations.clear();
    }

    // Param is passed by value, moved
    pub fn set_invitations(&mut self, v: ::protobuf::RepeatedField<OriginInvitation>) {
        self.invitations = v;
    }

    // Mutable pointer to the field.
    pub fn mut_invitations(&mut self) -> &mut ::protobuf::RepeatedField<OriginInvitation> {
        &mut self.invitations
    }

    // Take field
    pub fn take_invitations(&mut self) -> ::protobuf::RepeatedField<OriginInvitation> {
        ::std::mem::replace(&mut self.invitations, ::protobuf::RepeatedField::new())
    }

    pub fn get_invitations(&self) -> &[OriginInvitation] {
        &self.invitations
    }

    fn get_invitations_for_reflect(&self) -> &::protobuf::RepeatedField<OriginInvitation> {
        &self.invitations
    }

    fn mut_invitations_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<OriginInvitation> {
        &mut self.invitations
    }
}

impl ::protobuf::Message for AccountInvitationListResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.invitations {
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.account_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.invitations)?;
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
        if let Some(v) = self.account_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.invitations {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.account_id {
            os.write_uint64(1, v)?;
        }
        for v in &self.invitations {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for AccountInvitationListResponse {
    fn new() -> AccountInvitationListResponse {
        AccountInvitationListResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<AccountInvitationListResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "account_id",
                    AccountInvitationListResponse::get_account_id_for_reflect,
                    AccountInvitationListResponse::mut_account_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<OriginInvitation>>(
                    "invitations",
                    AccountInvitationListResponse::get_invitations_for_reflect,
                    AccountInvitationListResponse::mut_invitations_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AccountInvitationListResponse>(
                    "AccountInvitationListResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AccountInvitationListResponse {
    fn clear(&mut self) {
        self.clear_account_id();
        self.clear_invitations();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AccountInvitationListResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AccountInvitationListResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CheckOriginAccessRequest {
    // message oneof groups
    account_info: ::std::option::Option<CheckOriginAccessRequest_oneof_account_info>,
    origin_info: ::std::option::Option<CheckOriginAccessRequest_oneof_origin_info>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CheckOriginAccessRequest {}

#[derive(Clone,PartialEq)]
pub enum CheckOriginAccessRequest_oneof_account_info {
    account_id(u64),
    account_name(::std::string::String),
}

#[derive(Clone,PartialEq)]
pub enum CheckOriginAccessRequest_oneof_origin_info {
    origin_id(u64),
    origin_name(::std::string::String),
}

impl CheckOriginAccessRequest {
    pub fn new() -> CheckOriginAccessRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CheckOriginAccessRequest {
        static mut instance: ::protobuf::lazy::Lazy<CheckOriginAccessRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CheckOriginAccessRequest,
        };
        unsafe {
            instance.get(CheckOriginAccessRequest::new)
        }
    }

    // optional uint64 account_id = 1;

    pub fn clear_account_id(&mut self) {
        self.account_info = ::std::option::Option::None;
    }

    pub fn has_account_id(&self) -> bool {
        match self.account_info {
            ::std::option::Option::Some(CheckOriginAccessRequest_oneof_account_info::account_id(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_account_id(&mut self, v: u64) {
        self.account_info = ::std::option::Option::Some(CheckOriginAccessRequest_oneof_account_info::account_id(v))
    }

    pub fn get_account_id(&self) -> u64 {
        match self.account_info {
            ::std::option::Option::Some(CheckOriginAccessRequest_oneof_account_info::account_id(v)) => v,
            _ => 0,
        }
    }

    // optional string account_name = 2;

    pub fn clear_account_name(&mut self) {
        self.account_info = ::std::option::Option::None;
    }

    pub fn has_account_name(&self) -> bool {
        match self.account_info {
            ::std::option::Option::Some(CheckOriginAccessRequest_oneof_account_info::account_name(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_account_name(&mut self, v: ::std::string::String) {
        self.account_info = ::std::option::Option::Some(CheckOriginAccessRequest_oneof_account_info::account_name(v))
    }

    // Mutable pointer to the field.
    pub fn mut_account_name(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(CheckOriginAccessRequest_oneof_account_info::account_name(_)) = self.account_info {
        } else {
            self.account_info = ::std::option::Option::Some(CheckOriginAccessRequest_oneof_account_info::account_name(::std::string::String::new()));
        }
        match self.account_info {
            ::std::option::Option::Some(CheckOriginAccessRequest_oneof_account_info::account_name(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_account_name(&mut self) -> ::std::string::String {
        if self.has_account_name() {
            match self.account_info.take() {
                ::std::option::Option::Some(CheckOriginAccessRequest_oneof_account_info::account_name(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    pub fn get_account_name(&self) -> &str {
        match self.account_info {
            ::std::option::Option::Some(CheckOriginAccessRequest_oneof_account_info::account_name(ref v)) => v,
            _ => "",
        }
    }

    // optional uint64 origin_id = 3;

    pub fn clear_origin_id(&mut self) {
        self.origin_info = ::std::option::Option::None;
    }

    pub fn has_origin_id(&self) -> bool {
        match self.origin_info {
            ::std::option::Option::Some(CheckOriginAccessRequest_oneof_origin_info::origin_id(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_origin_id(&mut self, v: u64) {
        self.origin_info = ::std::option::Option::Some(CheckOriginAccessRequest_oneof_origin_info::origin_id(v))
    }

    pub fn get_origin_id(&self) -> u64 {
        match self.origin_info {
            ::std::option::Option::Some(CheckOriginAccessRequest_oneof_origin_info::origin_id(v)) => v,
            _ => 0,
        }
    }

    // optional string origin_name = 4;

    pub fn clear_origin_name(&mut self) {
        self.origin_info = ::std::option::Option::None;
    }

    pub fn has_origin_name(&self) -> bool {
        match self.origin_info {
            ::std::option::Option::Some(CheckOriginAccessRequest_oneof_origin_info::origin_name(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_origin_name(&mut self, v: ::std::string::String) {
        self.origin_info = ::std::option::Option::Some(CheckOriginAccessRequest_oneof_origin_info::origin_name(v))
    }

    // Mutable pointer to the field.
    pub fn mut_origin_name(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(CheckOriginAccessRequest_oneof_origin_info::origin_name(_)) = self.origin_info {
        } else {
            self.origin_info = ::std::option::Option::Some(CheckOriginAccessRequest_oneof_origin_info::origin_name(::std::string::String::new()));
        }
        match self.origin_info {
            ::std::option::Option::Some(CheckOriginAccessRequest_oneof_origin_info::origin_name(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_origin_name(&mut self) -> ::std::string::String {
        if self.has_origin_name() {
            match self.origin_info.take() {
                ::std::option::Option::Some(CheckOriginAccessRequest_oneof_origin_info::origin_name(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    pub fn get_origin_name(&self) -> &str {
        match self.origin_info {
            ::std::option::Option::Some(CheckOriginAccessRequest_oneof_origin_info::origin_name(ref v)) => v,
            _ => "",
        }
    }
}

impl ::protobuf::Message for CheckOriginAccessRequest {
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
                    self.account_info = ::std::option::Option::Some(CheckOriginAccessRequest_oneof_account_info::account_id(is.read_uint64()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.account_info = ::std::option::Option::Some(CheckOriginAccessRequest_oneof_account_info::account_name(is.read_string()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.origin_info = ::std::option::Option::Some(CheckOriginAccessRequest_oneof_origin_info::origin_id(is.read_uint64()?));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.origin_info = ::std::option::Option::Some(CheckOriginAccessRequest_oneof_origin_info::origin_name(is.read_string()?));
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
        if let ::std::option::Option::Some(ref v) = self.account_info {
            match v {
                &CheckOriginAccessRequest_oneof_account_info::account_id(v) => {
                    my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
                },
                &CheckOriginAccessRequest_oneof_account_info::account_name(ref v) => {
                    my_size += ::protobuf::rt::string_size(2, &v);
                },
            };
        }
        if let ::std::option::Option::Some(ref v) = self.origin_info {
            match v {
                &CheckOriginAccessRequest_oneof_origin_info::origin_id(v) => {
                    my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
                },
                &CheckOriginAccessRequest_oneof_origin_info::origin_name(ref v) => {
                    my_size += ::protobuf::rt::string_size(4, &v);
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let ::std::option::Option::Some(ref v) = self.account_info {
            match v {
                &CheckOriginAccessRequest_oneof_account_info::account_id(v) => {
                    os.write_uint64(1, v)?;
                },
                &CheckOriginAccessRequest_oneof_account_info::account_name(ref v) => {
                    os.write_string(2, v)?;
                },
            };
        }
        if let ::std::option::Option::Some(ref v) = self.origin_info {
            match v {
                &CheckOriginAccessRequest_oneof_origin_info::origin_id(v) => {
                    os.write_uint64(3, v)?;
                },
                &CheckOriginAccessRequest_oneof_origin_info::origin_name(ref v) => {
                    os.write_string(4, v)?;
                },
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

impl ::protobuf::MessageStatic for CheckOriginAccessRequest {
    fn new() -> CheckOriginAccessRequest {
        CheckOriginAccessRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CheckOriginAccessRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor::<_>(
                    "account_id",
                    CheckOriginAccessRequest::has_account_id,
                    CheckOriginAccessRequest::get_account_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor::<_>(
                    "account_name",
                    CheckOriginAccessRequest::has_account_name,
                    CheckOriginAccessRequest::get_account_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor::<_>(
                    "origin_id",
                    CheckOriginAccessRequest::has_origin_id,
                    CheckOriginAccessRequest::get_origin_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor::<_>(
                    "origin_name",
                    CheckOriginAccessRequest::has_origin_name,
                    CheckOriginAccessRequest::get_origin_name,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CheckOriginAccessRequest>(
                    "CheckOriginAccessRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CheckOriginAccessRequest {
    fn clear(&mut self) {
        self.clear_account_id();
        self.clear_account_name();
        self.clear_origin_id();
        self.clear_origin_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CheckOriginAccessRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CheckOriginAccessRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CheckOriginAccessResponse {
    // message fields
    has_access: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CheckOriginAccessResponse {}

impl CheckOriginAccessResponse {
    pub fn new() -> CheckOriginAccessResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CheckOriginAccessResponse {
        static mut instance: ::protobuf::lazy::Lazy<CheckOriginAccessResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CheckOriginAccessResponse,
        };
        unsafe {
            instance.get(CheckOriginAccessResponse::new)
        }
    }

    // optional bool has_access = 1;

    pub fn clear_has_access(&mut self) {
        self.has_access = ::std::option::Option::None;
    }

    pub fn has_has_access(&self) -> bool {
        self.has_access.is_some()
    }

    // Param is passed by value, moved
    pub fn set_has_access(&mut self, v: bool) {
        self.has_access = ::std::option::Option::Some(v);
    }

    pub fn get_has_access(&self) -> bool {
        self.has_access.unwrap_or(false)
    }

    fn get_has_access_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.has_access
    }

    fn mut_has_access_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.has_access
    }
}

impl ::protobuf::Message for CheckOriginAccessResponse {
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
                    let tmp = is.read_bool()?;
                    self.has_access = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.has_access {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.has_access {
            os.write_bool(1, v)?;
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

impl ::protobuf::MessageStatic for CheckOriginAccessResponse {
    fn new() -> CheckOriginAccessResponse {
        CheckOriginAccessResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CheckOriginAccessResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "has_access",
                    CheckOriginAccessResponse::get_has_access_for_reflect,
                    CheckOriginAccessResponse::mut_has_access_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CheckOriginAccessResponse>(
                    "CheckOriginAccessResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CheckOriginAccessResponse {
    fn clear(&mut self) {
        self.clear_has_access();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CheckOriginAccessResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CheckOriginAccessResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Origin {
    // message fields
    id: ::std::option::Option<u64>,
    name: ::protobuf::SingularField<::std::string::String>,
    owner_id: ::std::option::Option<u64>,
    private_key_name: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Origin {}

impl Origin {
    pub fn new() -> Origin {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Origin {
        static mut instance: ::protobuf::lazy::Lazy<Origin> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Origin,
        };
        unsafe {
            instance.get(Origin::new)
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

    // optional uint64 owner_id = 3;

    pub fn clear_owner_id(&mut self) {
        self.owner_id = ::std::option::Option::None;
    }

    pub fn has_owner_id(&self) -> bool {
        self.owner_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_owner_id(&mut self, v: u64) {
        self.owner_id = ::std::option::Option::Some(v);
    }

    pub fn get_owner_id(&self) -> u64 {
        self.owner_id.unwrap_or(0)
    }

    fn get_owner_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.owner_id
    }

    fn mut_owner_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.owner_id
    }

    // optional string private_key_name = 4;

    pub fn clear_private_key_name(&mut self) {
        self.private_key_name.clear();
    }

    pub fn has_private_key_name(&self) -> bool {
        self.private_key_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_private_key_name(&mut self, v: ::std::string::String) {
        self.private_key_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_private_key_name(&mut self) -> &mut ::std::string::String {
        if self.private_key_name.is_none() {
            self.private_key_name.set_default();
        }
        self.private_key_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_private_key_name(&mut self) -> ::std::string::String {
        self.private_key_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_private_key_name(&self) -> &str {
        match self.private_key_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_private_key_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.private_key_name
    }

    fn mut_private_key_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.private_key_name
    }
}

impl ::protobuf::Message for Origin {
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.owner_id = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.private_key_name)?;
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
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(v) = self.owner_id {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.private_key_name.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id {
            os.write_uint64(1, v)?;
        }
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(v) = self.owner_id {
            os.write_uint64(3, v)?;
        }
        if let Some(ref v) = self.private_key_name.as_ref() {
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

impl ::protobuf::MessageStatic for Origin {
    fn new() -> Origin {
        Origin::new()
    }

    fn descriptor_static(_: ::std::option::Option<Origin>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "id",
                    Origin::get_id_for_reflect,
                    Origin::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    Origin::get_name_for_reflect,
                    Origin::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "owner_id",
                    Origin::get_owner_id_for_reflect,
                    Origin::mut_owner_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "private_key_name",
                    Origin::get_private_key_name_for_reflect,
                    Origin::mut_private_key_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Origin>(
                    "Origin",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Origin {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_name();
        self.clear_owner_id();
        self.clear_private_key_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Origin {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Origin {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OriginCreate {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    owner_id: ::std::option::Option<u64>,
    owner_name: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OriginCreate {}

impl OriginCreate {
    pub fn new() -> OriginCreate {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OriginCreate {
        static mut instance: ::protobuf::lazy::Lazy<OriginCreate> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OriginCreate,
        };
        unsafe {
            instance.get(OriginCreate::new)
        }
    }

    // optional string name = 1;

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

    // optional uint64 owner_id = 2;

    pub fn clear_owner_id(&mut self) {
        self.owner_id = ::std::option::Option::None;
    }

    pub fn has_owner_id(&self) -> bool {
        self.owner_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_owner_id(&mut self, v: u64) {
        self.owner_id = ::std::option::Option::Some(v);
    }

    pub fn get_owner_id(&self) -> u64 {
        self.owner_id.unwrap_or(0)
    }

    fn get_owner_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.owner_id
    }

    fn mut_owner_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.owner_id
    }

    // optional string owner_name = 3;

    pub fn clear_owner_name(&mut self) {
        self.owner_name.clear();
    }

    pub fn has_owner_name(&self) -> bool {
        self.owner_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_owner_name(&mut self, v: ::std::string::String) {
        self.owner_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_owner_name(&mut self) -> &mut ::std::string::String {
        if self.owner_name.is_none() {
            self.owner_name.set_default();
        }
        self.owner_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_owner_name(&mut self) -> ::std::string::String {
        self.owner_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_owner_name(&self) -> &str {
        match self.owner_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_owner_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.owner_name
    }

    fn mut_owner_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.owner_name
    }
}

impl ::protobuf::Message for OriginCreate {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.owner_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.owner_name)?;
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
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(v) = self.owner_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.owner_name.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(v) = self.owner_id {
            os.write_uint64(2, v)?;
        }
        if let Some(ref v) = self.owner_name.as_ref() {
            os.write_string(3, &v)?;
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

impl ::protobuf::MessageStatic for OriginCreate {
    fn new() -> OriginCreate {
        OriginCreate::new()
    }

    fn descriptor_static(_: ::std::option::Option<OriginCreate>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    OriginCreate::get_name_for_reflect,
                    OriginCreate::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "owner_id",
                    OriginCreate::get_owner_id_for_reflect,
                    OriginCreate::mut_owner_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "owner_name",
                    OriginCreate::get_owner_name_for_reflect,
                    OriginCreate::mut_owner_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OriginCreate>(
                    "OriginCreate",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OriginCreate {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_owner_id();
        self.clear_owner_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OriginCreate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OriginCreate {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OriginDelete {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OriginDelete {}

impl OriginDelete {
    pub fn new() -> OriginDelete {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OriginDelete {
        static mut instance: ::protobuf::lazy::Lazy<OriginDelete> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OriginDelete,
        };
        unsafe {
            instance.get(OriginDelete::new)
        }
    }

    // optional string name = 1;

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
}

impl ::protobuf::Message for OriginDelete {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
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
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.name.as_ref() {
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

impl ::protobuf::MessageStatic for OriginDelete {
    fn new() -> OriginDelete {
        OriginDelete::new()
    }

    fn descriptor_static(_: ::std::option::Option<OriginDelete>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    OriginDelete::get_name_for_reflect,
                    OriginDelete::mut_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OriginDelete>(
                    "OriginDelete",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OriginDelete {
    fn clear(&mut self) {
        self.clear_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OriginDelete {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OriginDelete {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OriginGet {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OriginGet {}

impl OriginGet {
    pub fn new() -> OriginGet {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OriginGet {
        static mut instance: ::protobuf::lazy::Lazy<OriginGet> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OriginGet,
        };
        unsafe {
            instance.get(OriginGet::new)
        }
    }

    // optional string name = 1;

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
}

impl ::protobuf::Message for OriginGet {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
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
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.name.as_ref() {
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

impl ::protobuf::MessageStatic for OriginGet {
    fn new() -> OriginGet {
        OriginGet::new()
    }

    fn descriptor_static(_: ::std::option::Option<OriginGet>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    OriginGet::get_name_for_reflect,
                    OriginGet::mut_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OriginGet>(
                    "OriginGet",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OriginGet {
    fn clear(&mut self) {
        self.clear_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OriginGet {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OriginGet {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OriginChannel {
    // message fields
    id: ::std::option::Option<u64>,
    origin_id: ::std::option::Option<u64>,
    name: ::protobuf::SingularField<::std::string::String>,
    owner_id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OriginChannel {}

impl OriginChannel {
    pub fn new() -> OriginChannel {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OriginChannel {
        static mut instance: ::protobuf::lazy::Lazy<OriginChannel> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OriginChannel,
        };
        unsafe {
            instance.get(OriginChannel::new)
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

    // optional uint64 origin_id = 2;

    pub fn clear_origin_id(&mut self) {
        self.origin_id = ::std::option::Option::None;
    }

    pub fn has_origin_id(&self) -> bool {
        self.origin_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin_id(&mut self, v: u64) {
        self.origin_id = ::std::option::Option::Some(v);
    }

    pub fn get_origin_id(&self) -> u64 {
        self.origin_id.unwrap_or(0)
    }

    fn get_origin_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.origin_id
    }

    fn mut_origin_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.origin_id
    }

    // optional string name = 3;

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

    // optional uint64 owner_id = 4;

    pub fn clear_owner_id(&mut self) {
        self.owner_id = ::std::option::Option::None;
    }

    pub fn has_owner_id(&self) -> bool {
        self.owner_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_owner_id(&mut self, v: u64) {
        self.owner_id = ::std::option::Option::Some(v);
    }

    pub fn get_owner_id(&self) -> u64 {
        self.owner_id.unwrap_or(0)
    }

    fn get_owner_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.owner_id
    }

    fn mut_owner_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.owner_id
    }
}

impl ::protobuf::Message for OriginChannel {
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
                    let tmp = is.read_uint64()?;
                    self.origin_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.owner_id = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.origin_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        if let Some(v) = self.owner_id {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id {
            os.write_uint64(1, v)?;
        }
        if let Some(v) = self.origin_id {
            os.write_uint64(2, v)?;
        }
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(3, &v)?;
        }
        if let Some(v) = self.owner_id {
            os.write_uint64(4, v)?;
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

impl ::protobuf::MessageStatic for OriginChannel {
    fn new() -> OriginChannel {
        OriginChannel::new()
    }

    fn descriptor_static(_: ::std::option::Option<OriginChannel>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "id",
                    OriginChannel::get_id_for_reflect,
                    OriginChannel::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "origin_id",
                    OriginChannel::get_origin_id_for_reflect,
                    OriginChannel::mut_origin_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    OriginChannel::get_name_for_reflect,
                    OriginChannel::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "owner_id",
                    OriginChannel::get_owner_id_for_reflect,
                    OriginChannel::mut_owner_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OriginChannel>(
                    "OriginChannel",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OriginChannel {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_origin_id();
        self.clear_name();
        self.clear_owner_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OriginChannel {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OriginChannel {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OriginChannelIdent {
    // message fields
    origin: ::protobuf::SingularField<::std::string::String>,
    name: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OriginChannelIdent {}

impl OriginChannelIdent {
    pub fn new() -> OriginChannelIdent {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OriginChannelIdent {
        static mut instance: ::protobuf::lazy::Lazy<OriginChannelIdent> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OriginChannelIdent,
        };
        unsafe {
            instance.get(OriginChannelIdent::new)
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
}

impl ::protobuf::Message for OriginChannelIdent {
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

impl ::protobuf::MessageStatic for OriginChannelIdent {
    fn new() -> OriginChannelIdent {
        OriginChannelIdent::new()
    }

    fn descriptor_static(_: ::std::option::Option<OriginChannelIdent>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "origin",
                    OriginChannelIdent::get_origin_for_reflect,
                    OriginChannelIdent::mut_origin_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    OriginChannelIdent::get_name_for_reflect,
                    OriginChannelIdent::mut_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OriginChannelIdent>(
                    "OriginChannelIdent",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OriginChannelIdent {
    fn clear(&mut self) {
        self.clear_origin();
        self.clear_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OriginChannelIdent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OriginChannelIdent {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OriginChannelCreate {
    // message fields
    origin_id: ::std::option::Option<u64>,
    origin_name: ::protobuf::SingularField<::std::string::String>,
    name: ::protobuf::SingularField<::std::string::String>,
    owner_id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OriginChannelCreate {}

impl OriginChannelCreate {
    pub fn new() -> OriginChannelCreate {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OriginChannelCreate {
        static mut instance: ::protobuf::lazy::Lazy<OriginChannelCreate> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OriginChannelCreate,
        };
        unsafe {
            instance.get(OriginChannelCreate::new)
        }
    }

    // optional uint64 origin_id = 1;

    pub fn clear_origin_id(&mut self) {
        self.origin_id = ::std::option::Option::None;
    }

    pub fn has_origin_id(&self) -> bool {
        self.origin_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin_id(&mut self, v: u64) {
        self.origin_id = ::std::option::Option::Some(v);
    }

    pub fn get_origin_id(&self) -> u64 {
        self.origin_id.unwrap_or(0)
    }

    fn get_origin_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.origin_id
    }

    fn mut_origin_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.origin_id
    }

    // optional string origin_name = 2;

    pub fn clear_origin_name(&mut self) {
        self.origin_name.clear();
    }

    pub fn has_origin_name(&self) -> bool {
        self.origin_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin_name(&mut self, v: ::std::string::String) {
        self.origin_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_origin_name(&mut self) -> &mut ::std::string::String {
        if self.origin_name.is_none() {
            self.origin_name.set_default();
        }
        self.origin_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_origin_name(&mut self) -> ::std::string::String {
        self.origin_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_origin_name(&self) -> &str {
        match self.origin_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_origin_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.origin_name
    }

    fn mut_origin_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.origin_name
    }

    // optional string name = 3;

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

    // optional uint64 owner_id = 4;

    pub fn clear_owner_id(&mut self) {
        self.owner_id = ::std::option::Option::None;
    }

    pub fn has_owner_id(&self) -> bool {
        self.owner_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_owner_id(&mut self, v: u64) {
        self.owner_id = ::std::option::Option::Some(v);
    }

    pub fn get_owner_id(&self) -> u64 {
        self.owner_id.unwrap_or(0)
    }

    fn get_owner_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.owner_id
    }

    fn mut_owner_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.owner_id
    }
}

impl ::protobuf::Message for OriginChannelCreate {
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
                    self.origin_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.origin_name)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.owner_id = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.origin_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.origin_name.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        if let Some(v) = self.owner_id {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.origin_id {
            os.write_uint64(1, v)?;
        }
        if let Some(ref v) = self.origin_name.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(3, &v)?;
        }
        if let Some(v) = self.owner_id {
            os.write_uint64(4, v)?;
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

impl ::protobuf::MessageStatic for OriginChannelCreate {
    fn new() -> OriginChannelCreate {
        OriginChannelCreate::new()
    }

    fn descriptor_static(_: ::std::option::Option<OriginChannelCreate>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "origin_id",
                    OriginChannelCreate::get_origin_id_for_reflect,
                    OriginChannelCreate::mut_origin_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "origin_name",
                    OriginChannelCreate::get_origin_name_for_reflect,
                    OriginChannelCreate::mut_origin_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    OriginChannelCreate::get_name_for_reflect,
                    OriginChannelCreate::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "owner_id",
                    OriginChannelCreate::get_owner_id_for_reflect,
                    OriginChannelCreate::mut_owner_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OriginChannelCreate>(
                    "OriginChannelCreate",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OriginChannelCreate {
    fn clear(&mut self) {
        self.clear_origin_id();
        self.clear_origin_name();
        self.clear_name();
        self.clear_owner_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OriginChannelCreate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OriginChannelCreate {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OriginChannelGet {
    // message fields
    origin_name: ::protobuf::SingularField<::std::string::String>,
    name: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OriginChannelGet {}

impl OriginChannelGet {
    pub fn new() -> OriginChannelGet {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OriginChannelGet {
        static mut instance: ::protobuf::lazy::Lazy<OriginChannelGet> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OriginChannelGet,
        };
        unsafe {
            instance.get(OriginChannelGet::new)
        }
    }

    // optional string origin_name = 1;

    pub fn clear_origin_name(&mut self) {
        self.origin_name.clear();
    }

    pub fn has_origin_name(&self) -> bool {
        self.origin_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin_name(&mut self, v: ::std::string::String) {
        self.origin_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_origin_name(&mut self) -> &mut ::std::string::String {
        if self.origin_name.is_none() {
            self.origin_name.set_default();
        }
        self.origin_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_origin_name(&mut self) -> ::std::string::String {
        self.origin_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_origin_name(&self) -> &str {
        match self.origin_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_origin_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.origin_name
    }

    fn mut_origin_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.origin_name
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
}

impl ::protobuf::Message for OriginChannelGet {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.origin_name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
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
        if let Some(ref v) = self.origin_name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.origin_name.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.name.as_ref() {
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

impl ::protobuf::MessageStatic for OriginChannelGet {
    fn new() -> OriginChannelGet {
        OriginChannelGet::new()
    }

    fn descriptor_static(_: ::std::option::Option<OriginChannelGet>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "origin_name",
                    OriginChannelGet::get_origin_name_for_reflect,
                    OriginChannelGet::mut_origin_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    OriginChannelGet::get_name_for_reflect,
                    OriginChannelGet::mut_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OriginChannelGet>(
                    "OriginChannelGet",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OriginChannelGet {
    fn clear(&mut self) {
        self.clear_origin_name();
        self.clear_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OriginChannelGet {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OriginChannelGet {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OriginChannelListRequest {
    // message fields
    origin_id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OriginChannelListRequest {}

impl OriginChannelListRequest {
    pub fn new() -> OriginChannelListRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OriginChannelListRequest {
        static mut instance: ::protobuf::lazy::Lazy<OriginChannelListRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OriginChannelListRequest,
        };
        unsafe {
            instance.get(OriginChannelListRequest::new)
        }
    }

    // optional uint64 origin_id = 1;

    pub fn clear_origin_id(&mut self) {
        self.origin_id = ::std::option::Option::None;
    }

    pub fn has_origin_id(&self) -> bool {
        self.origin_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin_id(&mut self, v: u64) {
        self.origin_id = ::std::option::Option::Some(v);
    }

    pub fn get_origin_id(&self) -> u64 {
        self.origin_id.unwrap_or(0)
    }

    fn get_origin_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.origin_id
    }

    fn mut_origin_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.origin_id
    }
}

impl ::protobuf::Message for OriginChannelListRequest {
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
                    self.origin_id = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.origin_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.origin_id {
            os.write_uint64(1, v)?;
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

impl ::protobuf::MessageStatic for OriginChannelListRequest {
    fn new() -> OriginChannelListRequest {
        OriginChannelListRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<OriginChannelListRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "origin_id",
                    OriginChannelListRequest::get_origin_id_for_reflect,
                    OriginChannelListRequest::mut_origin_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OriginChannelListRequest>(
                    "OriginChannelListRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OriginChannelListRequest {
    fn clear(&mut self) {
        self.clear_origin_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OriginChannelListRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OriginChannelListRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OriginChannelListResponse {
    // message fields
    origin_id: ::std::option::Option<u64>,
    channels: ::protobuf::RepeatedField<OriginChannel>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OriginChannelListResponse {}

impl OriginChannelListResponse {
    pub fn new() -> OriginChannelListResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OriginChannelListResponse {
        static mut instance: ::protobuf::lazy::Lazy<OriginChannelListResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OriginChannelListResponse,
        };
        unsafe {
            instance.get(OriginChannelListResponse::new)
        }
    }

    // optional uint64 origin_id = 1;

    pub fn clear_origin_id(&mut self) {
        self.origin_id = ::std::option::Option::None;
    }

    pub fn has_origin_id(&self) -> bool {
        self.origin_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin_id(&mut self, v: u64) {
        self.origin_id = ::std::option::Option::Some(v);
    }

    pub fn get_origin_id(&self) -> u64 {
        self.origin_id.unwrap_or(0)
    }

    fn get_origin_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.origin_id
    }

    fn mut_origin_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.origin_id
    }

    // repeated .originsrv.OriginChannel channels = 2;

    pub fn clear_channels(&mut self) {
        self.channels.clear();
    }

    // Param is passed by value, moved
    pub fn set_channels(&mut self, v: ::protobuf::RepeatedField<OriginChannel>) {
        self.channels = v;
    }

    // Mutable pointer to the field.
    pub fn mut_channels(&mut self) -> &mut ::protobuf::RepeatedField<OriginChannel> {
        &mut self.channels
    }

    // Take field
    pub fn take_channels(&mut self) -> ::protobuf::RepeatedField<OriginChannel> {
        ::std::mem::replace(&mut self.channels, ::protobuf::RepeatedField::new())
    }

    pub fn get_channels(&self) -> &[OriginChannel] {
        &self.channels
    }

    fn get_channels_for_reflect(&self) -> &::protobuf::RepeatedField<OriginChannel> {
        &self.channels
    }

    fn mut_channels_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<OriginChannel> {
        &mut self.channels
    }
}

impl ::protobuf::Message for OriginChannelListResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.channels {
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.origin_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.channels)?;
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
        if let Some(v) = self.origin_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.channels {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.origin_id {
            os.write_uint64(1, v)?;
        }
        for v in &self.channels {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for OriginChannelListResponse {
    fn new() -> OriginChannelListResponse {
        OriginChannelListResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<OriginChannelListResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "origin_id",
                    OriginChannelListResponse::get_origin_id_for_reflect,
                    OriginChannelListResponse::mut_origin_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<OriginChannel>>(
                    "channels",
                    OriginChannelListResponse::get_channels_for_reflect,
                    OriginChannelListResponse::mut_channels_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OriginChannelListResponse>(
                    "OriginChannelListResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OriginChannelListResponse {
    fn clear(&mut self) {
        self.clear_origin_id();
        self.clear_channels();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OriginChannelListResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OriginChannelListResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OriginChannelPackageGet {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    ident: ::protobuf::SingularPtrField<OriginPackageIdent>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OriginChannelPackageGet {}

impl OriginChannelPackageGet {
    pub fn new() -> OriginChannelPackageGet {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OriginChannelPackageGet {
        static mut instance: ::protobuf::lazy::Lazy<OriginChannelPackageGet> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OriginChannelPackageGet,
        };
        unsafe {
            instance.get(OriginChannelPackageGet::new)
        }
    }

    // optional string name = 1;

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

    // optional .originsrv.OriginPackageIdent ident = 2;

    pub fn clear_ident(&mut self) {
        self.ident.clear();
    }

    pub fn has_ident(&self) -> bool {
        self.ident.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ident(&mut self, v: OriginPackageIdent) {
        self.ident = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ident(&mut self) -> &mut OriginPackageIdent {
        if self.ident.is_none() {
            self.ident.set_default();
        }
        self.ident.as_mut().unwrap()
    }

    // Take field
    pub fn take_ident(&mut self) -> OriginPackageIdent {
        self.ident.take().unwrap_or_else(|| OriginPackageIdent::new())
    }

    pub fn get_ident(&self) -> &OriginPackageIdent {
        self.ident.as_ref().unwrap_or_else(|| OriginPackageIdent::default_instance())
    }

    fn get_ident_for_reflect(&self) -> &::protobuf::SingularPtrField<OriginPackageIdent> {
        &self.ident
    }

    fn mut_ident_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<OriginPackageIdent> {
        &mut self.ident
    }
}

impl ::protobuf::Message for OriginChannelPackageGet {
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
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
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.ident.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.ident.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for OriginChannelPackageGet {
    fn new() -> OriginChannelPackageGet {
        OriginChannelPackageGet::new()
    }

    fn descriptor_static(_: ::std::option::Option<OriginChannelPackageGet>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    OriginChannelPackageGet::get_name_for_reflect,
                    OriginChannelPackageGet::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<OriginPackageIdent>>(
                    "ident",
                    OriginChannelPackageGet::get_ident_for_reflect,
                    OriginChannelPackageGet::mut_ident_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OriginChannelPackageGet>(
                    "OriginChannelPackageGet",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OriginChannelPackageGet {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_ident();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OriginChannelPackageGet {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OriginChannelPackageGet {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OriginChannelPackageLatestGet {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    ident: ::protobuf::SingularPtrField<OriginPackageIdent>,
    target: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OriginChannelPackageLatestGet {}

impl OriginChannelPackageLatestGet {
    pub fn new() -> OriginChannelPackageLatestGet {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OriginChannelPackageLatestGet {
        static mut instance: ::protobuf::lazy::Lazy<OriginChannelPackageLatestGet> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OriginChannelPackageLatestGet,
        };
        unsafe {
            instance.get(OriginChannelPackageLatestGet::new)
        }
    }

    // optional string name = 1;

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

    // optional .originsrv.OriginPackageIdent ident = 2;

    pub fn clear_ident(&mut self) {
        self.ident.clear();
    }

    pub fn has_ident(&self) -> bool {
        self.ident.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ident(&mut self, v: OriginPackageIdent) {
        self.ident = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ident(&mut self) -> &mut OriginPackageIdent {
        if self.ident.is_none() {
            self.ident.set_default();
        }
        self.ident.as_mut().unwrap()
    }

    // Take field
    pub fn take_ident(&mut self) -> OriginPackageIdent {
        self.ident.take().unwrap_or_else(|| OriginPackageIdent::new())
    }

    pub fn get_ident(&self) -> &OriginPackageIdent {
        self.ident.as_ref().unwrap_or_else(|| OriginPackageIdent::default_instance())
    }

    fn get_ident_for_reflect(&self) -> &::protobuf::SingularPtrField<OriginPackageIdent> {
        &self.ident
    }

    fn mut_ident_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<OriginPackageIdent> {
        &mut self.ident
    }

    // optional string target = 3;

    pub fn clear_target(&mut self) {
        self.target.clear();
    }

    pub fn has_target(&self) -> bool {
        self.target.is_some()
    }

    // Param is passed by value, moved
    pub fn set_target(&mut self, v: ::std::string::String) {
        self.target = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_target(&mut self) -> &mut ::std::string::String {
        if self.target.is_none() {
            self.target.set_default();
        }
        self.target.as_mut().unwrap()
    }

    // Take field
    pub fn take_target(&mut self) -> ::std::string::String {
        self.target.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_target(&self) -> &str {
        match self.target.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_target_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.target
    }

    fn mut_target_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.target
    }
}

impl ::protobuf::Message for OriginChannelPackageLatestGet {
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.ident)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.target)?;
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
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.ident.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.target.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.ident.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.target.as_ref() {
            os.write_string(3, &v)?;
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

impl ::protobuf::MessageStatic for OriginChannelPackageLatestGet {
    fn new() -> OriginChannelPackageLatestGet {
        OriginChannelPackageLatestGet::new()
    }

    fn descriptor_static(_: ::std::option::Option<OriginChannelPackageLatestGet>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    OriginChannelPackageLatestGet::get_name_for_reflect,
                    OriginChannelPackageLatestGet::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<OriginPackageIdent>>(
                    "ident",
                    OriginChannelPackageLatestGet::get_ident_for_reflect,
                    OriginChannelPackageLatestGet::mut_ident_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "target",
                    OriginChannelPackageLatestGet::get_target_for_reflect,
                    OriginChannelPackageLatestGet::mut_target_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OriginChannelPackageLatestGet>(
                    "OriginChannelPackageLatestGet",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OriginChannelPackageLatestGet {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_ident();
        self.clear_target();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OriginChannelPackageLatestGet {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OriginChannelPackageLatestGet {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OriginChannelPackageListRequest {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    ident: ::protobuf::SingularPtrField<OriginPackageIdent>,
    start: ::std::option::Option<u64>,
    stop: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OriginChannelPackageListRequest {}

impl OriginChannelPackageListRequest {
    pub fn new() -> OriginChannelPackageListRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OriginChannelPackageListRequest {
        static mut instance: ::protobuf::lazy::Lazy<OriginChannelPackageListRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OriginChannelPackageListRequest,
        };
        unsafe {
            instance.get(OriginChannelPackageListRequest::new)
        }
    }

    // optional string name = 1;

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

    // optional .originsrv.OriginPackageIdent ident = 2;

    pub fn clear_ident(&mut self) {
        self.ident.clear();
    }

    pub fn has_ident(&self) -> bool {
        self.ident.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ident(&mut self, v: OriginPackageIdent) {
        self.ident = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ident(&mut self) -> &mut OriginPackageIdent {
        if self.ident.is_none() {
            self.ident.set_default();
        }
        self.ident.as_mut().unwrap()
    }

    // Take field
    pub fn take_ident(&mut self) -> OriginPackageIdent {
        self.ident.take().unwrap_or_else(|| OriginPackageIdent::new())
    }

    pub fn get_ident(&self) -> &OriginPackageIdent {
        self.ident.as_ref().unwrap_or_else(|| OriginPackageIdent::default_instance())
    }

    fn get_ident_for_reflect(&self) -> &::protobuf::SingularPtrField<OriginPackageIdent> {
        &self.ident
    }

    fn mut_ident_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<OriginPackageIdent> {
        &mut self.ident
    }

    // optional uint64 start = 3;

    pub fn clear_start(&mut self) {
        self.start = ::std::option::Option::None;
    }

    pub fn has_start(&self) -> bool {
        self.start.is_some()
    }

    // Param is passed by value, moved
    pub fn set_start(&mut self, v: u64) {
        self.start = ::std::option::Option::Some(v);
    }

    pub fn get_start(&self) -> u64 {
        self.start.unwrap_or(0)
    }

    fn get_start_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.start
    }

    fn mut_start_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.start
    }

    // optional uint64 stop = 4;

    pub fn clear_stop(&mut self) {
        self.stop = ::std::option::Option::None;
    }

    pub fn has_stop(&self) -> bool {
        self.stop.is_some()
    }

    // Param is passed by value, moved
    pub fn set_stop(&mut self, v: u64) {
        self.stop = ::std::option::Option::Some(v);
    }

    pub fn get_stop(&self) -> u64 {
        self.stop.unwrap_or(0)
    }

    fn get_stop_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.stop
    }

    fn mut_stop_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.stop
    }
}

impl ::protobuf::Message for OriginChannelPackageListRequest {
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.ident)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.start = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.stop = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.ident.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.start {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.stop {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.ident.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.start {
            os.write_uint64(3, v)?;
        }
        if let Some(v) = self.stop {
            os.write_uint64(4, v)?;
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

impl ::protobuf::MessageStatic for OriginChannelPackageListRequest {
    fn new() -> OriginChannelPackageListRequest {
        OriginChannelPackageListRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<OriginChannelPackageListRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    OriginChannelPackageListRequest::get_name_for_reflect,
                    OriginChannelPackageListRequest::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<OriginPackageIdent>>(
                    "ident",
                    OriginChannelPackageListRequest::get_ident_for_reflect,
                    OriginChannelPackageListRequest::mut_ident_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "start",
                    OriginChannelPackageListRequest::get_start_for_reflect,
                    OriginChannelPackageListRequest::mut_start_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "stop",
                    OriginChannelPackageListRequest::get_stop_for_reflect,
                    OriginChannelPackageListRequest::mut_stop_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OriginChannelPackageListRequest>(
                    "OriginChannelPackageListRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OriginChannelPackageListRequest {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_ident();
        self.clear_start();
        self.clear_stop();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OriginChannelPackageListRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OriginChannelPackageListRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OriginChannelDelete {
    // message fields
    id: ::std::option::Option<u64>,
    origin_id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OriginChannelDelete {}

impl OriginChannelDelete {
    pub fn new() -> OriginChannelDelete {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OriginChannelDelete {
        static mut instance: ::protobuf::lazy::Lazy<OriginChannelDelete> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OriginChannelDelete,
        };
        unsafe {
            instance.get(OriginChannelDelete::new)
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

    // optional uint64 origin_id = 2;

    pub fn clear_origin_id(&mut self) {
        self.origin_id = ::std::option::Option::None;
    }

    pub fn has_origin_id(&self) -> bool {
        self.origin_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin_id(&mut self, v: u64) {
        self.origin_id = ::std::option::Option::Some(v);
    }

    pub fn get_origin_id(&self) -> u64 {
        self.origin_id.unwrap_or(0)
    }

    fn get_origin_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.origin_id
    }

    fn mut_origin_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.origin_id
    }
}

impl ::protobuf::Message for OriginChannelDelete {
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
                    let tmp = is.read_uint64()?;
                    self.origin_id = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.origin_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id {
            os.write_uint64(1, v)?;
        }
        if let Some(v) = self.origin_id {
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

impl ::protobuf::MessageStatic for OriginChannelDelete {
    fn new() -> OriginChannelDelete {
        OriginChannelDelete::new()
    }

    fn descriptor_static(_: ::std::option::Option<OriginChannelDelete>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "id",
                    OriginChannelDelete::get_id_for_reflect,
                    OriginChannelDelete::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "origin_id",
                    OriginChannelDelete::get_origin_id_for_reflect,
                    OriginChannelDelete::mut_origin_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OriginChannelDelete>(
                    "OriginChannelDelete",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OriginChannelDelete {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_origin_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OriginChannelDelete {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OriginChannelDelete {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OriginInvitation {
    // message fields
    id: ::std::option::Option<u64>,
    account_id: ::std::option::Option<u64>,
    account_name: ::protobuf::SingularField<::std::string::String>,
    origin_id: ::std::option::Option<u64>,
    origin_name: ::protobuf::SingularField<::std::string::String>,
    owner_id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OriginInvitation {}

impl OriginInvitation {
    pub fn new() -> OriginInvitation {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OriginInvitation {
        static mut instance: ::protobuf::lazy::Lazy<OriginInvitation> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OriginInvitation,
        };
        unsafe {
            instance.get(OriginInvitation::new)
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

    // optional uint64 account_id = 2;

    pub fn clear_account_id(&mut self) {
        self.account_id = ::std::option::Option::None;
    }

    pub fn has_account_id(&self) -> bool {
        self.account_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_account_id(&mut self, v: u64) {
        self.account_id = ::std::option::Option::Some(v);
    }

    pub fn get_account_id(&self) -> u64 {
        self.account_id.unwrap_or(0)
    }

    fn get_account_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.account_id
    }

    fn mut_account_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.account_id
    }

    // optional string account_name = 3;

    pub fn clear_account_name(&mut self) {
        self.account_name.clear();
    }

    pub fn has_account_name(&self) -> bool {
        self.account_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_account_name(&mut self, v: ::std::string::String) {
        self.account_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_account_name(&mut self) -> &mut ::std::string::String {
        if self.account_name.is_none() {
            self.account_name.set_default();
        }
        self.account_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_account_name(&mut self) -> ::std::string::String {
        self.account_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_account_name(&self) -> &str {
        match self.account_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_account_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.account_name
    }

    fn mut_account_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.account_name
    }

    // optional uint64 origin_id = 4;

    pub fn clear_origin_id(&mut self) {
        self.origin_id = ::std::option::Option::None;
    }

    pub fn has_origin_id(&self) -> bool {
        self.origin_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin_id(&mut self, v: u64) {
        self.origin_id = ::std::option::Option::Some(v);
    }

    pub fn get_origin_id(&self) -> u64 {
        self.origin_id.unwrap_or(0)
    }

    fn get_origin_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.origin_id
    }

    fn mut_origin_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.origin_id
    }

    // optional string origin_name = 5;

    pub fn clear_origin_name(&mut self) {
        self.origin_name.clear();
    }

    pub fn has_origin_name(&self) -> bool {
        self.origin_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin_name(&mut self, v: ::std::string::String) {
        self.origin_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_origin_name(&mut self) -> &mut ::std::string::String {
        if self.origin_name.is_none() {
            self.origin_name.set_default();
        }
        self.origin_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_origin_name(&mut self) -> ::std::string::String {
        self.origin_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_origin_name(&self) -> &str {
        match self.origin_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_origin_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.origin_name
    }

    fn mut_origin_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.origin_name
    }

    // optional uint64 owner_id = 6;

    pub fn clear_owner_id(&mut self) {
        self.owner_id = ::std::option::Option::None;
    }

    pub fn has_owner_id(&self) -> bool {
        self.owner_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_owner_id(&mut self, v: u64) {
        self.owner_id = ::std::option::Option::Some(v);
    }

    pub fn get_owner_id(&self) -> u64 {
        self.owner_id.unwrap_or(0)
    }

    fn get_owner_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.owner_id
    }

    fn mut_owner_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.owner_id
    }
}

impl ::protobuf::Message for OriginInvitation {
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
                    let tmp = is.read_uint64()?;
                    self.account_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.account_name)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.origin_id = ::std::option::Option::Some(tmp);
                },
                5 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.origin_name)?;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.owner_id = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.account_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.account_name.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        if let Some(v) = self.origin_id {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.origin_name.as_ref() {
            my_size += ::protobuf::rt::string_size(5, &v);
        }
        if let Some(v) = self.owner_id {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id {
            os.write_uint64(1, v)?;
        }
        if let Some(v) = self.account_id {
            os.write_uint64(2, v)?;
        }
        if let Some(ref v) = self.account_name.as_ref() {
            os.write_string(3, &v)?;
        }
        if let Some(v) = self.origin_id {
            os.write_uint64(4, v)?;
        }
        if let Some(ref v) = self.origin_name.as_ref() {
            os.write_string(5, &v)?;
        }
        if let Some(v) = self.owner_id {
            os.write_uint64(6, v)?;
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

impl ::protobuf::MessageStatic for OriginInvitation {
    fn new() -> OriginInvitation {
        OriginInvitation::new()
    }

    fn descriptor_static(_: ::std::option::Option<OriginInvitation>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "id",
                    OriginInvitation::get_id_for_reflect,
                    OriginInvitation::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "account_id",
                    OriginInvitation::get_account_id_for_reflect,
                    OriginInvitation::mut_account_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "account_name",
                    OriginInvitation::get_account_name_for_reflect,
                    OriginInvitation::mut_account_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "origin_id",
                    OriginInvitation::get_origin_id_for_reflect,
                    OriginInvitation::mut_origin_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "origin_name",
                    OriginInvitation::get_origin_name_for_reflect,
                    OriginInvitation::mut_origin_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "owner_id",
                    OriginInvitation::get_owner_id_for_reflect,
                    OriginInvitation::mut_owner_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OriginInvitation>(
                    "OriginInvitation",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OriginInvitation {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_account_id();
        self.clear_account_name();
        self.clear_origin_id();
        self.clear_origin_name();
        self.clear_owner_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OriginInvitation {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OriginInvitation {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OriginInvitationAcceptRequest {
    // message fields
    account_id: ::std::option::Option<u64>,
    invite_id: ::std::option::Option<u64>,
    origin_name: ::protobuf::SingularField<::std::string::String>,
    ignore: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OriginInvitationAcceptRequest {}

impl OriginInvitationAcceptRequest {
    pub fn new() -> OriginInvitationAcceptRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OriginInvitationAcceptRequest {
        static mut instance: ::protobuf::lazy::Lazy<OriginInvitationAcceptRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OriginInvitationAcceptRequest,
        };
        unsafe {
            instance.get(OriginInvitationAcceptRequest::new)
        }
    }

    // optional uint64 account_id = 1;

    pub fn clear_account_id(&mut self) {
        self.account_id = ::std::option::Option::None;
    }

    pub fn has_account_id(&self) -> bool {
        self.account_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_account_id(&mut self, v: u64) {
        self.account_id = ::std::option::Option::Some(v);
    }

    pub fn get_account_id(&self) -> u64 {
        self.account_id.unwrap_or(0)
    }

    fn get_account_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.account_id
    }

    fn mut_account_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.account_id
    }

    // optional uint64 invite_id = 2;

    pub fn clear_invite_id(&mut self) {
        self.invite_id = ::std::option::Option::None;
    }

    pub fn has_invite_id(&self) -> bool {
        self.invite_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_invite_id(&mut self, v: u64) {
        self.invite_id = ::std::option::Option::Some(v);
    }

    pub fn get_invite_id(&self) -> u64 {
        self.invite_id.unwrap_or(0)
    }

    fn get_invite_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.invite_id
    }

    fn mut_invite_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.invite_id
    }

    // optional string origin_name = 3;

    pub fn clear_origin_name(&mut self) {
        self.origin_name.clear();
    }

    pub fn has_origin_name(&self) -> bool {
        self.origin_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin_name(&mut self, v: ::std::string::String) {
        self.origin_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_origin_name(&mut self) -> &mut ::std::string::String {
        if self.origin_name.is_none() {
            self.origin_name.set_default();
        }
        self.origin_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_origin_name(&mut self) -> ::std::string::String {
        self.origin_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_origin_name(&self) -> &str {
        match self.origin_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_origin_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.origin_name
    }

    fn mut_origin_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.origin_name
    }

    // optional bool ignore = 4;

    pub fn clear_ignore(&mut self) {
        self.ignore = ::std::option::Option::None;
    }

    pub fn has_ignore(&self) -> bool {
        self.ignore.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ignore(&mut self, v: bool) {
        self.ignore = ::std::option::Option::Some(v);
    }

    pub fn get_ignore(&self) -> bool {
        self.ignore.unwrap_or(false)
    }

    fn get_ignore_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.ignore
    }

    fn mut_ignore_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.ignore
    }
}

impl ::protobuf::Message for OriginInvitationAcceptRequest {
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
                    self.account_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.invite_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.origin_name)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.ignore = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.account_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.invite_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.origin_name.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        if let Some(v) = self.ignore {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.account_id {
            os.write_uint64(1, v)?;
        }
        if let Some(v) = self.invite_id {
            os.write_uint64(2, v)?;
        }
        if let Some(ref v) = self.origin_name.as_ref() {
            os.write_string(3, &v)?;
        }
        if let Some(v) = self.ignore {
            os.write_bool(4, v)?;
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

impl ::protobuf::MessageStatic for OriginInvitationAcceptRequest {
    fn new() -> OriginInvitationAcceptRequest {
        OriginInvitationAcceptRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<OriginInvitationAcceptRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "account_id",
                    OriginInvitationAcceptRequest::get_account_id_for_reflect,
                    OriginInvitationAcceptRequest::mut_account_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "invite_id",
                    OriginInvitationAcceptRequest::get_invite_id_for_reflect,
                    OriginInvitationAcceptRequest::mut_invite_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "origin_name",
                    OriginInvitationAcceptRequest::get_origin_name_for_reflect,
                    OriginInvitationAcceptRequest::mut_origin_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "ignore",
                    OriginInvitationAcceptRequest::get_ignore_for_reflect,
                    OriginInvitationAcceptRequest::mut_ignore_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OriginInvitationAcceptRequest>(
                    "OriginInvitationAcceptRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OriginInvitationAcceptRequest {
    fn clear(&mut self) {
        self.clear_account_id();
        self.clear_invite_id();
        self.clear_origin_name();
        self.clear_ignore();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OriginInvitationAcceptRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OriginInvitationAcceptRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OriginInvitationCreate {
    // message fields
    account_id: ::std::option::Option<u64>,
    account_name: ::protobuf::SingularField<::std::string::String>,
    origin_id: ::std::option::Option<u64>,
    origin_name: ::protobuf::SingularField<::std::string::String>,
    owner_id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OriginInvitationCreate {}

impl OriginInvitationCreate {
    pub fn new() -> OriginInvitationCreate {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OriginInvitationCreate {
        static mut instance: ::protobuf::lazy::Lazy<OriginInvitationCreate> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OriginInvitationCreate,
        };
        unsafe {
            instance.get(OriginInvitationCreate::new)
        }
    }

    // optional uint64 account_id = 1;

    pub fn clear_account_id(&mut self) {
        self.account_id = ::std::option::Option::None;
    }

    pub fn has_account_id(&self) -> bool {
        self.account_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_account_id(&mut self, v: u64) {
        self.account_id = ::std::option::Option::Some(v);
    }

    pub fn get_account_id(&self) -> u64 {
        self.account_id.unwrap_or(0)
    }

    fn get_account_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.account_id
    }

    fn mut_account_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.account_id
    }

    // optional string account_name = 2;

    pub fn clear_account_name(&mut self) {
        self.account_name.clear();
    }

    pub fn has_account_name(&self) -> bool {
        self.account_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_account_name(&mut self, v: ::std::string::String) {
        self.account_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_account_name(&mut self) -> &mut ::std::string::String {
        if self.account_name.is_none() {
            self.account_name.set_default();
        }
        self.account_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_account_name(&mut self) -> ::std::string::String {
        self.account_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_account_name(&self) -> &str {
        match self.account_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_account_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.account_name
    }

    fn mut_account_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.account_name
    }

    // optional uint64 origin_id = 3;

    pub fn clear_origin_id(&mut self) {
        self.origin_id = ::std::option::Option::None;
    }

    pub fn has_origin_id(&self) -> bool {
        self.origin_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin_id(&mut self, v: u64) {
        self.origin_id = ::std::option::Option::Some(v);
    }

    pub fn get_origin_id(&self) -> u64 {
        self.origin_id.unwrap_or(0)
    }

    fn get_origin_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.origin_id
    }

    fn mut_origin_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.origin_id
    }

    // optional string origin_name = 4;

    pub fn clear_origin_name(&mut self) {
        self.origin_name.clear();
    }

    pub fn has_origin_name(&self) -> bool {
        self.origin_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin_name(&mut self, v: ::std::string::String) {
        self.origin_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_origin_name(&mut self) -> &mut ::std::string::String {
        if self.origin_name.is_none() {
            self.origin_name.set_default();
        }
        self.origin_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_origin_name(&mut self) -> ::std::string::String {
        self.origin_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_origin_name(&self) -> &str {
        match self.origin_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_origin_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.origin_name
    }

    fn mut_origin_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.origin_name
    }

    // optional uint64 owner_id = 5;

    pub fn clear_owner_id(&mut self) {
        self.owner_id = ::std::option::Option::None;
    }

    pub fn has_owner_id(&self) -> bool {
        self.owner_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_owner_id(&mut self, v: u64) {
        self.owner_id = ::std::option::Option::Some(v);
    }

    pub fn get_owner_id(&self) -> u64 {
        self.owner_id.unwrap_or(0)
    }

    fn get_owner_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.owner_id
    }

    fn mut_owner_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.owner_id
    }
}

impl ::protobuf::Message for OriginInvitationCreate {
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
                    self.account_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.account_name)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.origin_id = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.origin_name)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.owner_id = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.account_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.account_name.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(v) = self.origin_id {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.origin_name.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        }
        if let Some(v) = self.owner_id {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.account_id {
            os.write_uint64(1, v)?;
        }
        if let Some(ref v) = self.account_name.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(v) = self.origin_id {
            os.write_uint64(3, v)?;
        }
        if let Some(ref v) = self.origin_name.as_ref() {
            os.write_string(4, &v)?;
        }
        if let Some(v) = self.owner_id {
            os.write_uint64(5, v)?;
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

impl ::protobuf::MessageStatic for OriginInvitationCreate {
    fn new() -> OriginInvitationCreate {
        OriginInvitationCreate::new()
    }

    fn descriptor_static(_: ::std::option::Option<OriginInvitationCreate>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "account_id",
                    OriginInvitationCreate::get_account_id_for_reflect,
                    OriginInvitationCreate::mut_account_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "account_name",
                    OriginInvitationCreate::get_account_name_for_reflect,
                    OriginInvitationCreate::mut_account_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "origin_id",
                    OriginInvitationCreate::get_origin_id_for_reflect,
                    OriginInvitationCreate::mut_origin_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "origin_name",
                    OriginInvitationCreate::get_origin_name_for_reflect,
                    OriginInvitationCreate::mut_origin_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "owner_id",
                    OriginInvitationCreate::get_owner_id_for_reflect,
                    OriginInvitationCreate::mut_owner_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OriginInvitationCreate>(
                    "OriginInvitationCreate",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OriginInvitationCreate {
    fn clear(&mut self) {
        self.clear_account_id();
        self.clear_account_name();
        self.clear_origin_id();
        self.clear_origin_name();
        self.clear_owner_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OriginInvitationCreate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OriginInvitationCreate {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OriginInvitationListRequest {
    // message fields
    origin_id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OriginInvitationListRequest {}

impl OriginInvitationListRequest {
    pub fn new() -> OriginInvitationListRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OriginInvitationListRequest {
        static mut instance: ::protobuf::lazy::Lazy<OriginInvitationListRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OriginInvitationListRequest,
        };
        unsafe {
            instance.get(OriginInvitationListRequest::new)
        }
    }

    // optional uint64 origin_id = 1;

    pub fn clear_origin_id(&mut self) {
        self.origin_id = ::std::option::Option::None;
    }

    pub fn has_origin_id(&self) -> bool {
        self.origin_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin_id(&mut self, v: u64) {
        self.origin_id = ::std::option::Option::Some(v);
    }

    pub fn get_origin_id(&self) -> u64 {
        self.origin_id.unwrap_or(0)
    }

    fn get_origin_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.origin_id
    }

    fn mut_origin_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.origin_id
    }
}

impl ::protobuf::Message for OriginInvitationListRequest {
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
                    self.origin_id = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.origin_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.origin_id {
            os.write_uint64(1, v)?;
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

impl ::protobuf::MessageStatic for OriginInvitationListRequest {
    fn new() -> OriginInvitationListRequest {
        OriginInvitationListRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<OriginInvitationListRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "origin_id",
                    OriginInvitationListRequest::get_origin_id_for_reflect,
                    OriginInvitationListRequest::mut_origin_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OriginInvitationListRequest>(
                    "OriginInvitationListRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OriginInvitationListRequest {
    fn clear(&mut self) {
        self.clear_origin_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OriginInvitationListRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OriginInvitationListRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OriginInvitationListResponse {
    // message fields
    origin_id: ::std::option::Option<u64>,
    invitations: ::protobuf::RepeatedField<OriginInvitation>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OriginInvitationListResponse {}

impl OriginInvitationListResponse {
    pub fn new() -> OriginInvitationListResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OriginInvitationListResponse {
        static mut instance: ::protobuf::lazy::Lazy<OriginInvitationListResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OriginInvitationListResponse,
        };
        unsafe {
            instance.get(OriginInvitationListResponse::new)
        }
    }

    // optional uint64 origin_id = 1;

    pub fn clear_origin_id(&mut self) {
        self.origin_id = ::std::option::Option::None;
    }

    pub fn has_origin_id(&self) -> bool {
        self.origin_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin_id(&mut self, v: u64) {
        self.origin_id = ::std::option::Option::Some(v);
    }

    pub fn get_origin_id(&self) -> u64 {
        self.origin_id.unwrap_or(0)
    }

    fn get_origin_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.origin_id
    }

    fn mut_origin_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.origin_id
    }

    // repeated .originsrv.OriginInvitation invitations = 2;

    pub fn clear_invitations(&mut self) {
        self.invitations.clear();
    }

    // Param is passed by value, moved
    pub fn set_invitations(&mut self, v: ::protobuf::RepeatedField<OriginInvitation>) {
        self.invitations = v;
    }

    // Mutable pointer to the field.
    pub fn mut_invitations(&mut self) -> &mut ::protobuf::RepeatedField<OriginInvitation> {
        &mut self.invitations
    }

    // Take field
    pub fn take_invitations(&mut self) -> ::protobuf::RepeatedField<OriginInvitation> {
        ::std::mem::replace(&mut self.invitations, ::protobuf::RepeatedField::new())
    }

    pub fn get_invitations(&self) -> &[OriginInvitation] {
        &self.invitations
    }

    fn get_invitations_for_reflect(&self) -> &::protobuf::RepeatedField<OriginInvitation> {
        &self.invitations
    }

    fn mut_invitations_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<OriginInvitation> {
        &mut self.invitations
    }
}

impl ::protobuf::Message for OriginInvitationListResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.invitations {
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.origin_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.invitations)?;
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
        if let Some(v) = self.origin_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.invitations {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.origin_id {
            os.write_uint64(1, v)?;
        }
        for v in &self.invitations {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for OriginInvitationListResponse {
    fn new() -> OriginInvitationListResponse {
        OriginInvitationListResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<OriginInvitationListResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "origin_id",
                    OriginInvitationListResponse::get_origin_id_for_reflect,
                    OriginInvitationListResponse::mut_origin_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<OriginInvitation>>(
                    "invitations",
                    OriginInvitationListResponse::get_invitations_for_reflect,
                    OriginInvitationListResponse::mut_invitations_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OriginInvitationListResponse>(
                    "OriginInvitationListResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OriginInvitationListResponse {
    fn clear(&mut self) {
        self.clear_origin_id();
        self.clear_invitations();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OriginInvitationListResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OriginInvitationListResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OriginKeyIdent {
    // message fields
    origin: ::protobuf::SingularField<::std::string::String>,
    revision: ::protobuf::SingularField<::std::string::String>,
    location: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OriginKeyIdent {}

impl OriginKeyIdent {
    pub fn new() -> OriginKeyIdent {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OriginKeyIdent {
        static mut instance: ::protobuf::lazy::Lazy<OriginKeyIdent> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OriginKeyIdent,
        };
        unsafe {
            instance.get(OriginKeyIdent::new)
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

    // optional string revision = 2;

    pub fn clear_revision(&mut self) {
        self.revision.clear();
    }

    pub fn has_revision(&self) -> bool {
        self.revision.is_some()
    }

    // Param is passed by value, moved
    pub fn set_revision(&mut self, v: ::std::string::String) {
        self.revision = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_revision(&mut self) -> &mut ::std::string::String {
        if self.revision.is_none() {
            self.revision.set_default();
        }
        self.revision.as_mut().unwrap()
    }

    // Take field
    pub fn take_revision(&mut self) -> ::std::string::String {
        self.revision.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_revision(&self) -> &str {
        match self.revision.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_revision_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.revision
    }

    fn mut_revision_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.revision
    }

    // optional string location = 3;

    pub fn clear_location(&mut self) {
        self.location.clear();
    }

    pub fn has_location(&self) -> bool {
        self.location.is_some()
    }

    // Param is passed by value, moved
    pub fn set_location(&mut self, v: ::std::string::String) {
        self.location = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_location(&mut self) -> &mut ::std::string::String {
        if self.location.is_none() {
            self.location.set_default();
        }
        self.location.as_mut().unwrap()
    }

    // Take field
    pub fn take_location(&mut self) -> ::std::string::String {
        self.location.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_location(&self) -> &str {
        match self.location.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_location_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.location
    }

    fn mut_location_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.location
    }
}

impl ::protobuf::Message for OriginKeyIdent {
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.revision)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.location)?;
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
        if let Some(ref v) = self.revision.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(ref v) = self.location.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.origin.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.revision.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(ref v) = self.location.as_ref() {
            os.write_string(3, &v)?;
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

impl ::protobuf::MessageStatic for OriginKeyIdent {
    fn new() -> OriginKeyIdent {
        OriginKeyIdent::new()
    }

    fn descriptor_static(_: ::std::option::Option<OriginKeyIdent>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "origin",
                    OriginKeyIdent::get_origin_for_reflect,
                    OriginKeyIdent::mut_origin_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "revision",
                    OriginKeyIdent::get_revision_for_reflect,
                    OriginKeyIdent::mut_revision_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "location",
                    OriginKeyIdent::get_location_for_reflect,
                    OriginKeyIdent::mut_location_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OriginKeyIdent>(
                    "OriginKeyIdent",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OriginKeyIdent {
    fn clear(&mut self) {
        self.clear_origin();
        self.clear_revision();
        self.clear_location();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OriginKeyIdent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OriginKeyIdent {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OriginMemberListRequest {
    // message fields
    origin_id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OriginMemberListRequest {}

impl OriginMemberListRequest {
    pub fn new() -> OriginMemberListRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OriginMemberListRequest {
        static mut instance: ::protobuf::lazy::Lazy<OriginMemberListRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OriginMemberListRequest,
        };
        unsafe {
            instance.get(OriginMemberListRequest::new)
        }
    }

    // optional uint64 origin_id = 1;

    pub fn clear_origin_id(&mut self) {
        self.origin_id = ::std::option::Option::None;
    }

    pub fn has_origin_id(&self) -> bool {
        self.origin_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin_id(&mut self, v: u64) {
        self.origin_id = ::std::option::Option::Some(v);
    }

    pub fn get_origin_id(&self) -> u64 {
        self.origin_id.unwrap_or(0)
    }

    fn get_origin_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.origin_id
    }

    fn mut_origin_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.origin_id
    }
}

impl ::protobuf::Message for OriginMemberListRequest {
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
                    self.origin_id = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.origin_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.origin_id {
            os.write_uint64(1, v)?;
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

impl ::protobuf::MessageStatic for OriginMemberListRequest {
    fn new() -> OriginMemberListRequest {
        OriginMemberListRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<OriginMemberListRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "origin_id",
                    OriginMemberListRequest::get_origin_id_for_reflect,
                    OriginMemberListRequest::mut_origin_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OriginMemberListRequest>(
                    "OriginMemberListRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OriginMemberListRequest {
    fn clear(&mut self) {
        self.clear_origin_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OriginMemberListRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OriginMemberListRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OriginMemberListResponse {
    // message fields
    origin_id: ::std::option::Option<u64>,
    members: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OriginMemberListResponse {}

impl OriginMemberListResponse {
    pub fn new() -> OriginMemberListResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OriginMemberListResponse {
        static mut instance: ::protobuf::lazy::Lazy<OriginMemberListResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OriginMemberListResponse,
        };
        unsafe {
            instance.get(OriginMemberListResponse::new)
        }
    }

    // optional uint64 origin_id = 1;

    pub fn clear_origin_id(&mut self) {
        self.origin_id = ::std::option::Option::None;
    }

    pub fn has_origin_id(&self) -> bool {
        self.origin_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin_id(&mut self, v: u64) {
        self.origin_id = ::std::option::Option::Some(v);
    }

    pub fn get_origin_id(&self) -> u64 {
        self.origin_id.unwrap_or(0)
    }

    fn get_origin_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.origin_id
    }

    fn mut_origin_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.origin_id
    }

    // repeated string members = 2;

    pub fn clear_members(&mut self) {
        self.members.clear();
    }

    // Param is passed by value, moved
    pub fn set_members(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.members = v;
    }

    // Mutable pointer to the field.
    pub fn mut_members(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.members
    }

    // Take field
    pub fn take_members(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.members, ::protobuf::RepeatedField::new())
    }

    pub fn get_members(&self) -> &[::std::string::String] {
        &self.members
    }

    fn get_members_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.members
    }

    fn mut_members_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.members
    }
}

impl ::protobuf::Message for OriginMemberListResponse {
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
                    self.origin_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.members)?;
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
        if let Some(v) = self.origin_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.members {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.origin_id {
            os.write_uint64(1, v)?;
        }
        for v in &self.members {
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

impl ::protobuf::MessageStatic for OriginMemberListResponse {
    fn new() -> OriginMemberListResponse {
        OriginMemberListResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<OriginMemberListResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "origin_id",
                    OriginMemberListResponse::get_origin_id_for_reflect,
                    OriginMemberListResponse::mut_origin_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "members",
                    OriginMemberListResponse::get_members_for_reflect,
                    OriginMemberListResponse::mut_members_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OriginMemberListResponse>(
                    "OriginMemberListResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OriginMemberListResponse {
    fn clear(&mut self) {
        self.clear_origin_id();
        self.clear_members();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OriginMemberListResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OriginMemberListResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OriginMemberRemove {
    // message fields
    origin_id: ::std::option::Option<u64>,
    user_id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OriginMemberRemove {}

impl OriginMemberRemove {
    pub fn new() -> OriginMemberRemove {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OriginMemberRemove {
        static mut instance: ::protobuf::lazy::Lazy<OriginMemberRemove> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OriginMemberRemove,
        };
        unsafe {
            instance.get(OriginMemberRemove::new)
        }
    }

    // optional uint64 origin_id = 1;

    pub fn clear_origin_id(&mut self) {
        self.origin_id = ::std::option::Option::None;
    }

    pub fn has_origin_id(&self) -> bool {
        self.origin_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin_id(&mut self, v: u64) {
        self.origin_id = ::std::option::Option::Some(v);
    }

    pub fn get_origin_id(&self) -> u64 {
        self.origin_id.unwrap_or(0)
    }

    fn get_origin_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.origin_id
    }

    fn mut_origin_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.origin_id
    }

    // optional uint64 user_id = 2;

    pub fn clear_user_id(&mut self) {
        self.user_id = ::std::option::Option::None;
    }

    pub fn has_user_id(&self) -> bool {
        self.user_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_user_id(&mut self, v: u64) {
        self.user_id = ::std::option::Option::Some(v);
    }

    pub fn get_user_id(&self) -> u64 {
        self.user_id.unwrap_or(0)
    }

    fn get_user_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.user_id
    }

    fn mut_user_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.user_id
    }
}

impl ::protobuf::Message for OriginMemberRemove {
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
                    self.origin_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.user_id = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.origin_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.user_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.origin_id {
            os.write_uint64(1, v)?;
        }
        if let Some(v) = self.user_id {
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

impl ::protobuf::MessageStatic for OriginMemberRemove {
    fn new() -> OriginMemberRemove {
        OriginMemberRemove::new()
    }

    fn descriptor_static(_: ::std::option::Option<OriginMemberRemove>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "origin_id",
                    OriginMemberRemove::get_origin_id_for_reflect,
                    OriginMemberRemove::mut_origin_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "user_id",
                    OriginMemberRemove::get_user_id_for_reflect,
                    OriginMemberRemove::mut_user_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OriginMemberRemove>(
                    "OriginMemberRemove",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OriginMemberRemove {
    fn clear(&mut self) {
        self.clear_origin_id();
        self.clear_user_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OriginMemberRemove {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OriginMemberRemove {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OriginPackage {
    // message fields
    id: ::std::option::Option<u64>,
    owner_id: ::std::option::Option<u64>,
    origin_id: ::std::option::Option<u64>,
    ident: ::protobuf::SingularPtrField<OriginPackageIdent>,
    checksum: ::protobuf::SingularField<::std::string::String>,
    manifest: ::protobuf::SingularField<::std::string::String>,
    deps: ::protobuf::RepeatedField<OriginPackageIdent>,
    tdeps: ::protobuf::RepeatedField<OriginPackageIdent>,
    exposes: ::std::vec::Vec<u32>,
    config: ::protobuf::SingularField<::std::string::String>,
    target: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OriginPackage {}

impl OriginPackage {
    pub fn new() -> OriginPackage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OriginPackage {
        static mut instance: ::protobuf::lazy::Lazy<OriginPackage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OriginPackage,
        };
        unsafe {
            instance.get(OriginPackage::new)
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

    // optional uint64 owner_id = 2;

    pub fn clear_owner_id(&mut self) {
        self.owner_id = ::std::option::Option::None;
    }

    pub fn has_owner_id(&self) -> bool {
        self.owner_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_owner_id(&mut self, v: u64) {
        self.owner_id = ::std::option::Option::Some(v);
    }

    pub fn get_owner_id(&self) -> u64 {
        self.owner_id.unwrap_or(0)
    }

    fn get_owner_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.owner_id
    }

    fn mut_owner_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.owner_id
    }

    // optional uint64 origin_id = 3;

    pub fn clear_origin_id(&mut self) {
        self.origin_id = ::std::option::Option::None;
    }

    pub fn has_origin_id(&self) -> bool {
        self.origin_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin_id(&mut self, v: u64) {
        self.origin_id = ::std::option::Option::Some(v);
    }

    pub fn get_origin_id(&self) -> u64 {
        self.origin_id.unwrap_or(0)
    }

    fn get_origin_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.origin_id
    }

    fn mut_origin_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.origin_id
    }

    // optional .originsrv.OriginPackageIdent ident = 4;

    pub fn clear_ident(&mut self) {
        self.ident.clear();
    }

    pub fn has_ident(&self) -> bool {
        self.ident.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ident(&mut self, v: OriginPackageIdent) {
        self.ident = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ident(&mut self) -> &mut OriginPackageIdent {
        if self.ident.is_none() {
            self.ident.set_default();
        }
        self.ident.as_mut().unwrap()
    }

    // Take field
    pub fn take_ident(&mut self) -> OriginPackageIdent {
        self.ident.take().unwrap_or_else(|| OriginPackageIdent::new())
    }

    pub fn get_ident(&self) -> &OriginPackageIdent {
        self.ident.as_ref().unwrap_or_else(|| OriginPackageIdent::default_instance())
    }

    fn get_ident_for_reflect(&self) -> &::protobuf::SingularPtrField<OriginPackageIdent> {
        &self.ident
    }

    fn mut_ident_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<OriginPackageIdent> {
        &mut self.ident
    }

    // optional string checksum = 5;

    pub fn clear_checksum(&mut self) {
        self.checksum.clear();
    }

    pub fn has_checksum(&self) -> bool {
        self.checksum.is_some()
    }

    // Param is passed by value, moved
    pub fn set_checksum(&mut self, v: ::std::string::String) {
        self.checksum = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_checksum(&mut self) -> &mut ::std::string::String {
        if self.checksum.is_none() {
            self.checksum.set_default();
        }
        self.checksum.as_mut().unwrap()
    }

    // Take field
    pub fn take_checksum(&mut self) -> ::std::string::String {
        self.checksum.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_checksum(&self) -> &str {
        match self.checksum.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_checksum_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.checksum
    }

    fn mut_checksum_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.checksum
    }

    // optional string manifest = 6;

    pub fn clear_manifest(&mut self) {
        self.manifest.clear();
    }

    pub fn has_manifest(&self) -> bool {
        self.manifest.is_some()
    }

    // Param is passed by value, moved
    pub fn set_manifest(&mut self, v: ::std::string::String) {
        self.manifest = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_manifest(&mut self) -> &mut ::std::string::String {
        if self.manifest.is_none() {
            self.manifest.set_default();
        }
        self.manifest.as_mut().unwrap()
    }

    // Take field
    pub fn take_manifest(&mut self) -> ::std::string::String {
        self.manifest.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_manifest(&self) -> &str {
        match self.manifest.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_manifest_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.manifest
    }

    fn mut_manifest_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.manifest
    }

    // repeated .originsrv.OriginPackageIdent deps = 7;

    pub fn clear_deps(&mut self) {
        self.deps.clear();
    }

    // Param is passed by value, moved
    pub fn set_deps(&mut self, v: ::protobuf::RepeatedField<OriginPackageIdent>) {
        self.deps = v;
    }

    // Mutable pointer to the field.
    pub fn mut_deps(&mut self) -> &mut ::protobuf::RepeatedField<OriginPackageIdent> {
        &mut self.deps
    }

    // Take field
    pub fn take_deps(&mut self) -> ::protobuf::RepeatedField<OriginPackageIdent> {
        ::std::mem::replace(&mut self.deps, ::protobuf::RepeatedField::new())
    }

    pub fn get_deps(&self) -> &[OriginPackageIdent] {
        &self.deps
    }

    fn get_deps_for_reflect(&self) -> &::protobuf::RepeatedField<OriginPackageIdent> {
        &self.deps
    }

    fn mut_deps_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<OriginPackageIdent> {
        &mut self.deps
    }

    // repeated .originsrv.OriginPackageIdent tdeps = 8;

    pub fn clear_tdeps(&mut self) {
        self.tdeps.clear();
    }

    // Param is passed by value, moved
    pub fn set_tdeps(&mut self, v: ::protobuf::RepeatedField<OriginPackageIdent>) {
        self.tdeps = v;
    }

    // Mutable pointer to the field.
    pub fn mut_tdeps(&mut self) -> &mut ::protobuf::RepeatedField<OriginPackageIdent> {
        &mut self.tdeps
    }

    // Take field
    pub fn take_tdeps(&mut self) -> ::protobuf::RepeatedField<OriginPackageIdent> {
        ::std::mem::replace(&mut self.tdeps, ::protobuf::RepeatedField::new())
    }

    pub fn get_tdeps(&self) -> &[OriginPackageIdent] {
        &self.tdeps
    }

    fn get_tdeps_for_reflect(&self) -> &::protobuf::RepeatedField<OriginPackageIdent> {
        &self.tdeps
    }

    fn mut_tdeps_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<OriginPackageIdent> {
        &mut self.tdeps
    }

    // repeated uint32 exposes = 9;

    pub fn clear_exposes(&mut self) {
        self.exposes.clear();
    }

    // Param is passed by value, moved
    pub fn set_exposes(&mut self, v: ::std::vec::Vec<u32>) {
        self.exposes = v;
    }

    // Mutable pointer to the field.
    pub fn mut_exposes(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.exposes
    }

    // Take field
    pub fn take_exposes(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.exposes, ::std::vec::Vec::new())
    }

    pub fn get_exposes(&self) -> &[u32] {
        &self.exposes
    }

    fn get_exposes_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.exposes
    }

    fn mut_exposes_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.exposes
    }

    // optional string config = 10;

    pub fn clear_config(&mut self) {
        self.config.clear();
    }

    pub fn has_config(&self) -> bool {
        self.config.is_some()
    }

    // Param is passed by value, moved
    pub fn set_config(&mut self, v: ::std::string::String) {
        self.config = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_config(&mut self) -> &mut ::std::string::String {
        if self.config.is_none() {
            self.config.set_default();
        }
        self.config.as_mut().unwrap()
    }

    // Take field
    pub fn take_config(&mut self) -> ::std::string::String {
        self.config.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_config(&self) -> &str {
        match self.config.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_config_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.config
    }

    fn mut_config_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.config
    }

    // optional string target = 11;

    pub fn clear_target(&mut self) {
        self.target.clear();
    }

    pub fn has_target(&self) -> bool {
        self.target.is_some()
    }

    // Param is passed by value, moved
    pub fn set_target(&mut self, v: ::std::string::String) {
        self.target = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_target(&mut self) -> &mut ::std::string::String {
        if self.target.is_none() {
            self.target.set_default();
        }
        self.target.as_mut().unwrap()
    }

    // Take field
    pub fn take_target(&mut self) -> ::std::string::String {
        self.target.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_target(&self) -> &str {
        match self.target.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_target_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.target
    }

    fn mut_target_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.target
    }
}

impl ::protobuf::Message for OriginPackage {
    fn is_initialized(&self) -> bool {
        for v in &self.ident {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.deps {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.tdeps {
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
                    let tmp = is.read_uint64()?;
                    self.owner_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.origin_id = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.ident)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.checksum)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.manifest)?;
                },
                7 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.deps)?;
                },
                8 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.tdeps)?;
                },
                9 => {
                    ::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.exposes)?;
                },
                10 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.config)?;
                },
                11 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.target)?;
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
        if let Some(v) = self.owner_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.origin_id {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.ident.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.checksum.as_ref() {
            my_size += ::protobuf::rt::string_size(5, &v);
        }
        if let Some(ref v) = self.manifest.as_ref() {
            my_size += ::protobuf::rt::string_size(6, &v);
        }
        for value in &self.deps {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.tdeps {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if !self.exposes.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_size(9, &self.exposes);
        }
        if let Some(ref v) = self.config.as_ref() {
            my_size += ::protobuf::rt::string_size(10, &v);
        }
        if let Some(ref v) = self.target.as_ref() {
            my_size += ::protobuf::rt::string_size(11, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id {
            os.write_uint64(1, v)?;
        }
        if let Some(v) = self.owner_id {
            os.write_uint64(2, v)?;
        }
        if let Some(v) = self.origin_id {
            os.write_uint64(3, v)?;
        }
        if let Some(ref v) = self.ident.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.checksum.as_ref() {
            os.write_string(5, &v)?;
        }
        if let Some(ref v) = self.manifest.as_ref() {
            os.write_string(6, &v)?;
        }
        for v in &self.deps {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.tdeps {
            os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if !self.exposes.is_empty() {
            os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            // TODO: Data size is computed again, it should be cached
            os.write_raw_varint32(::protobuf::rt::vec_packed_varint_data_size(&self.exposes))?;
            for v in &self.exposes {
                os.write_uint32_no_tag(*v)?;
            };
        }
        if let Some(ref v) = self.config.as_ref() {
            os.write_string(10, &v)?;
        }
        if let Some(ref v) = self.target.as_ref() {
            os.write_string(11, &v)?;
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

impl ::protobuf::MessageStatic for OriginPackage {
    fn new() -> OriginPackage {
        OriginPackage::new()
    }

    fn descriptor_static(_: ::std::option::Option<OriginPackage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "id",
                    OriginPackage::get_id_for_reflect,
                    OriginPackage::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "owner_id",
                    OriginPackage::get_owner_id_for_reflect,
                    OriginPackage::mut_owner_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "origin_id",
                    OriginPackage::get_origin_id_for_reflect,
                    OriginPackage::mut_origin_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<OriginPackageIdent>>(
                    "ident",
                    OriginPackage::get_ident_for_reflect,
                    OriginPackage::mut_ident_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "checksum",
                    OriginPackage::get_checksum_for_reflect,
                    OriginPackage::mut_checksum_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "manifest",
                    OriginPackage::get_manifest_for_reflect,
                    OriginPackage::mut_manifest_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<OriginPackageIdent>>(
                    "deps",
                    OriginPackage::get_deps_for_reflect,
                    OriginPackage::mut_deps_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<OriginPackageIdent>>(
                    "tdeps",
                    OriginPackage::get_tdeps_for_reflect,
                    OriginPackage::mut_tdeps_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "exposes",
                    OriginPackage::get_exposes_for_reflect,
                    OriginPackage::mut_exposes_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "config",
                    OriginPackage::get_config_for_reflect,
                    OriginPackage::mut_config_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "target",
                    OriginPackage::get_target_for_reflect,
                    OriginPackage::mut_target_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OriginPackage>(
                    "OriginPackage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OriginPackage {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_owner_id();
        self.clear_origin_id();
        self.clear_ident();
        self.clear_checksum();
        self.clear_manifest();
        self.clear_deps();
        self.clear_tdeps();
        self.clear_exposes();
        self.clear_config();
        self.clear_target();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OriginPackage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OriginPackage {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OriginPackageIdent {
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
unsafe impl ::std::marker::Sync for OriginPackageIdent {}

impl OriginPackageIdent {
    pub fn new() -> OriginPackageIdent {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OriginPackageIdent {
        static mut instance: ::protobuf::lazy::Lazy<OriginPackageIdent> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OriginPackageIdent,
        };
        unsafe {
            instance.get(OriginPackageIdent::new)
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

impl ::protobuf::Message for OriginPackageIdent {
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

impl ::protobuf::MessageStatic for OriginPackageIdent {
    fn new() -> OriginPackageIdent {
        OriginPackageIdent::new()
    }

    fn descriptor_static(_: ::std::option::Option<OriginPackageIdent>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "origin",
                    OriginPackageIdent::get_origin_for_reflect,
                    OriginPackageIdent::mut_origin_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    OriginPackageIdent::get_name_for_reflect,
                    OriginPackageIdent::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "version",
                    OriginPackageIdent::get_version_for_reflect,
                    OriginPackageIdent::mut_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "release",
                    OriginPackageIdent::get_release_for_reflect,
                    OriginPackageIdent::mut_release_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OriginPackageIdent>(
                    "OriginPackageIdent",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OriginPackageIdent {
    fn clear(&mut self) {
        self.clear_origin();
        self.clear_name();
        self.clear_version();
        self.clear_release();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OriginPackageIdent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OriginPackageIdent {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OriginPackageVersion {
    // message fields
    origin: ::protobuf::SingularField<::std::string::String>,
    name: ::protobuf::SingularField<::std::string::String>,
    version: ::protobuf::SingularField<::std::string::String>,
    release_count: ::std::option::Option<u64>,
    latest: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OriginPackageVersion {}

impl OriginPackageVersion {
    pub fn new() -> OriginPackageVersion {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OriginPackageVersion {
        static mut instance: ::protobuf::lazy::Lazy<OriginPackageVersion> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OriginPackageVersion,
        };
        unsafe {
            instance.get(OriginPackageVersion::new)
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

    // optional uint64 release_count = 4;

    pub fn clear_release_count(&mut self) {
        self.release_count = ::std::option::Option::None;
    }

    pub fn has_release_count(&self) -> bool {
        self.release_count.is_some()
    }

    // Param is passed by value, moved
    pub fn set_release_count(&mut self, v: u64) {
        self.release_count = ::std::option::Option::Some(v);
    }

    pub fn get_release_count(&self) -> u64 {
        self.release_count.unwrap_or(0)
    }

    fn get_release_count_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.release_count
    }

    fn mut_release_count_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.release_count
    }

    // optional string latest = 5;

    pub fn clear_latest(&mut self) {
        self.latest.clear();
    }

    pub fn has_latest(&self) -> bool {
        self.latest.is_some()
    }

    // Param is passed by value, moved
    pub fn set_latest(&mut self, v: ::std::string::String) {
        self.latest = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_latest(&mut self) -> &mut ::std::string::String {
        if self.latest.is_none() {
            self.latest.set_default();
        }
        self.latest.as_mut().unwrap()
    }

    // Take field
    pub fn take_latest(&mut self) -> ::std::string::String {
        self.latest.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_latest(&self) -> &str {
        match self.latest.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_latest_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.latest
    }

    fn mut_latest_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.latest
    }
}

impl ::protobuf::Message for OriginPackageVersion {
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.release_count = ::std::option::Option::Some(tmp);
                },
                5 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.latest)?;
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
        if let Some(v) = self.release_count {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.latest.as_ref() {
            my_size += ::protobuf::rt::string_size(5, &v);
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
        if let Some(v) = self.release_count {
            os.write_uint64(4, v)?;
        }
        if let Some(ref v) = self.latest.as_ref() {
            os.write_string(5, &v)?;
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

impl ::protobuf::MessageStatic for OriginPackageVersion {
    fn new() -> OriginPackageVersion {
        OriginPackageVersion::new()
    }

    fn descriptor_static(_: ::std::option::Option<OriginPackageVersion>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "origin",
                    OriginPackageVersion::get_origin_for_reflect,
                    OriginPackageVersion::mut_origin_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    OriginPackageVersion::get_name_for_reflect,
                    OriginPackageVersion::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "version",
                    OriginPackageVersion::get_version_for_reflect,
                    OriginPackageVersion::mut_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "release_count",
                    OriginPackageVersion::get_release_count_for_reflect,
                    OriginPackageVersion::mut_release_count_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "latest",
                    OriginPackageVersion::get_latest_for_reflect,
                    OriginPackageVersion::mut_latest_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OriginPackageVersion>(
                    "OriginPackageVersion",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OriginPackageVersion {
    fn clear(&mut self) {
        self.clear_origin();
        self.clear_name();
        self.clear_version();
        self.clear_release_count();
        self.clear_latest();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OriginPackageVersion {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OriginPackageVersion {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OriginPackageCreate {
    // message fields
    owner_id: ::std::option::Option<u64>,
    origin_id: ::std::option::Option<u64>,
    ident: ::protobuf::SingularPtrField<OriginPackageIdent>,
    checksum: ::protobuf::SingularField<::std::string::String>,
    manifest: ::protobuf::SingularField<::std::string::String>,
    deps: ::protobuf::RepeatedField<OriginPackageIdent>,
    tdeps: ::protobuf::RepeatedField<OriginPackageIdent>,
    exposes: ::std::vec::Vec<u32>,
    config: ::protobuf::SingularField<::std::string::String>,
    target: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OriginPackageCreate {}

impl OriginPackageCreate {
    pub fn new() -> OriginPackageCreate {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OriginPackageCreate {
        static mut instance: ::protobuf::lazy::Lazy<OriginPackageCreate> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OriginPackageCreate,
        };
        unsafe {
            instance.get(OriginPackageCreate::new)
        }
    }

    // optional uint64 owner_id = 1;

    pub fn clear_owner_id(&mut self) {
        self.owner_id = ::std::option::Option::None;
    }

    pub fn has_owner_id(&self) -> bool {
        self.owner_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_owner_id(&mut self, v: u64) {
        self.owner_id = ::std::option::Option::Some(v);
    }

    pub fn get_owner_id(&self) -> u64 {
        self.owner_id.unwrap_or(0)
    }

    fn get_owner_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.owner_id
    }

    fn mut_owner_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.owner_id
    }

    // optional uint64 origin_id = 2;

    pub fn clear_origin_id(&mut self) {
        self.origin_id = ::std::option::Option::None;
    }

    pub fn has_origin_id(&self) -> bool {
        self.origin_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin_id(&mut self, v: u64) {
        self.origin_id = ::std::option::Option::Some(v);
    }

    pub fn get_origin_id(&self) -> u64 {
        self.origin_id.unwrap_or(0)
    }

    fn get_origin_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.origin_id
    }

    fn mut_origin_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.origin_id
    }

    // optional .originsrv.OriginPackageIdent ident = 3;

    pub fn clear_ident(&mut self) {
        self.ident.clear();
    }

    pub fn has_ident(&self) -> bool {
        self.ident.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ident(&mut self, v: OriginPackageIdent) {
        self.ident = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ident(&mut self) -> &mut OriginPackageIdent {
        if self.ident.is_none() {
            self.ident.set_default();
        }
        self.ident.as_mut().unwrap()
    }

    // Take field
    pub fn take_ident(&mut self) -> OriginPackageIdent {
        self.ident.take().unwrap_or_else(|| OriginPackageIdent::new())
    }

    pub fn get_ident(&self) -> &OriginPackageIdent {
        self.ident.as_ref().unwrap_or_else(|| OriginPackageIdent::default_instance())
    }

    fn get_ident_for_reflect(&self) -> &::protobuf::SingularPtrField<OriginPackageIdent> {
        &self.ident
    }

    fn mut_ident_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<OriginPackageIdent> {
        &mut self.ident
    }

    // optional string checksum = 4;

    pub fn clear_checksum(&mut self) {
        self.checksum.clear();
    }

    pub fn has_checksum(&self) -> bool {
        self.checksum.is_some()
    }

    // Param is passed by value, moved
    pub fn set_checksum(&mut self, v: ::std::string::String) {
        self.checksum = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_checksum(&mut self) -> &mut ::std::string::String {
        if self.checksum.is_none() {
            self.checksum.set_default();
        }
        self.checksum.as_mut().unwrap()
    }

    // Take field
    pub fn take_checksum(&mut self) -> ::std::string::String {
        self.checksum.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_checksum(&self) -> &str {
        match self.checksum.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_checksum_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.checksum
    }

    fn mut_checksum_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.checksum
    }

    // optional string manifest = 5;

    pub fn clear_manifest(&mut self) {
        self.manifest.clear();
    }

    pub fn has_manifest(&self) -> bool {
        self.manifest.is_some()
    }

    // Param is passed by value, moved
    pub fn set_manifest(&mut self, v: ::std::string::String) {
        self.manifest = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_manifest(&mut self) -> &mut ::std::string::String {
        if self.manifest.is_none() {
            self.manifest.set_default();
        }
        self.manifest.as_mut().unwrap()
    }

    // Take field
    pub fn take_manifest(&mut self) -> ::std::string::String {
        self.manifest.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_manifest(&self) -> &str {
        match self.manifest.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_manifest_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.manifest
    }

    fn mut_manifest_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.manifest
    }

    // repeated .originsrv.OriginPackageIdent deps = 6;

    pub fn clear_deps(&mut self) {
        self.deps.clear();
    }

    // Param is passed by value, moved
    pub fn set_deps(&mut self, v: ::protobuf::RepeatedField<OriginPackageIdent>) {
        self.deps = v;
    }

    // Mutable pointer to the field.
    pub fn mut_deps(&mut self) -> &mut ::protobuf::RepeatedField<OriginPackageIdent> {
        &mut self.deps
    }

    // Take field
    pub fn take_deps(&mut self) -> ::protobuf::RepeatedField<OriginPackageIdent> {
        ::std::mem::replace(&mut self.deps, ::protobuf::RepeatedField::new())
    }

    pub fn get_deps(&self) -> &[OriginPackageIdent] {
        &self.deps
    }

    fn get_deps_for_reflect(&self) -> &::protobuf::RepeatedField<OriginPackageIdent> {
        &self.deps
    }

    fn mut_deps_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<OriginPackageIdent> {
        &mut self.deps
    }

    // repeated .originsrv.OriginPackageIdent tdeps = 7;

    pub fn clear_tdeps(&mut self) {
        self.tdeps.clear();
    }

    // Param is passed by value, moved
    pub fn set_tdeps(&mut self, v: ::protobuf::RepeatedField<OriginPackageIdent>) {
        self.tdeps = v;
    }

    // Mutable pointer to the field.
    pub fn mut_tdeps(&mut self) -> &mut ::protobuf::RepeatedField<OriginPackageIdent> {
        &mut self.tdeps
    }

    // Take field
    pub fn take_tdeps(&mut self) -> ::protobuf::RepeatedField<OriginPackageIdent> {
        ::std::mem::replace(&mut self.tdeps, ::protobuf::RepeatedField::new())
    }

    pub fn get_tdeps(&self) -> &[OriginPackageIdent] {
        &self.tdeps
    }

    fn get_tdeps_for_reflect(&self) -> &::protobuf::RepeatedField<OriginPackageIdent> {
        &self.tdeps
    }

    fn mut_tdeps_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<OriginPackageIdent> {
        &mut self.tdeps
    }

    // repeated uint32 exposes = 8;

    pub fn clear_exposes(&mut self) {
        self.exposes.clear();
    }

    // Param is passed by value, moved
    pub fn set_exposes(&mut self, v: ::std::vec::Vec<u32>) {
        self.exposes = v;
    }

    // Mutable pointer to the field.
    pub fn mut_exposes(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.exposes
    }

    // Take field
    pub fn take_exposes(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.exposes, ::std::vec::Vec::new())
    }

    pub fn get_exposes(&self) -> &[u32] {
        &self.exposes
    }

    fn get_exposes_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.exposes
    }

    fn mut_exposes_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.exposes
    }

    // optional string config = 9;

    pub fn clear_config(&mut self) {
        self.config.clear();
    }

    pub fn has_config(&self) -> bool {
        self.config.is_some()
    }

    // Param is passed by value, moved
    pub fn set_config(&mut self, v: ::std::string::String) {
        self.config = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_config(&mut self) -> &mut ::std::string::String {
        if self.config.is_none() {
            self.config.set_default();
        }
        self.config.as_mut().unwrap()
    }

    // Take field
    pub fn take_config(&mut self) -> ::std::string::String {
        self.config.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_config(&self) -> &str {
        match self.config.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_config_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.config
    }

    fn mut_config_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.config
    }

    // optional string target = 10;

    pub fn clear_target(&mut self) {
        self.target.clear();
    }

    pub fn has_target(&self) -> bool {
        self.target.is_some()
    }

    // Param is passed by value, moved
    pub fn set_target(&mut self, v: ::std::string::String) {
        self.target = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_target(&mut self) -> &mut ::std::string::String {
        if self.target.is_none() {
            self.target.set_default();
        }
        self.target.as_mut().unwrap()
    }

    // Take field
    pub fn take_target(&mut self) -> ::std::string::String {
        self.target.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_target(&self) -> &str {
        match self.target.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_target_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.target
    }

    fn mut_target_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.target
    }
}

impl ::protobuf::Message for OriginPackageCreate {
    fn is_initialized(&self) -> bool {
        for v in &self.ident {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.deps {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.tdeps {
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.owner_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.origin_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.ident)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.checksum)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.manifest)?;
                },
                6 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.deps)?;
                },
                7 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.tdeps)?;
                },
                8 => {
                    ::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.exposes)?;
                },
                9 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.config)?;
                },
                10 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.target)?;
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
        if let Some(v) = self.owner_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.origin_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.ident.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.checksum.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        }
        if let Some(ref v) = self.manifest.as_ref() {
            my_size += ::protobuf::rt::string_size(5, &v);
        }
        for value in &self.deps {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.tdeps {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if !self.exposes.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_size(8, &self.exposes);
        }
        if let Some(ref v) = self.config.as_ref() {
            my_size += ::protobuf::rt::string_size(9, &v);
        }
        if let Some(ref v) = self.target.as_ref() {
            my_size += ::protobuf::rt::string_size(10, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.owner_id {
            os.write_uint64(1, v)?;
        }
        if let Some(v) = self.origin_id {
            os.write_uint64(2, v)?;
        }
        if let Some(ref v) = self.ident.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.checksum.as_ref() {
            os.write_string(4, &v)?;
        }
        if let Some(ref v) = self.manifest.as_ref() {
            os.write_string(5, &v)?;
        }
        for v in &self.deps {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.tdeps {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if !self.exposes.is_empty() {
            os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            // TODO: Data size is computed again, it should be cached
            os.write_raw_varint32(::protobuf::rt::vec_packed_varint_data_size(&self.exposes))?;
            for v in &self.exposes {
                os.write_uint32_no_tag(*v)?;
            };
        }
        if let Some(ref v) = self.config.as_ref() {
            os.write_string(9, &v)?;
        }
        if let Some(ref v) = self.target.as_ref() {
            os.write_string(10, &v)?;
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

impl ::protobuf::MessageStatic for OriginPackageCreate {
    fn new() -> OriginPackageCreate {
        OriginPackageCreate::new()
    }

    fn descriptor_static(_: ::std::option::Option<OriginPackageCreate>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "owner_id",
                    OriginPackageCreate::get_owner_id_for_reflect,
                    OriginPackageCreate::mut_owner_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "origin_id",
                    OriginPackageCreate::get_origin_id_for_reflect,
                    OriginPackageCreate::mut_origin_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<OriginPackageIdent>>(
                    "ident",
                    OriginPackageCreate::get_ident_for_reflect,
                    OriginPackageCreate::mut_ident_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "checksum",
                    OriginPackageCreate::get_checksum_for_reflect,
                    OriginPackageCreate::mut_checksum_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "manifest",
                    OriginPackageCreate::get_manifest_for_reflect,
                    OriginPackageCreate::mut_manifest_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<OriginPackageIdent>>(
                    "deps",
                    OriginPackageCreate::get_deps_for_reflect,
                    OriginPackageCreate::mut_deps_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<OriginPackageIdent>>(
                    "tdeps",
                    OriginPackageCreate::get_tdeps_for_reflect,
                    OriginPackageCreate::mut_tdeps_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "exposes",
                    OriginPackageCreate::get_exposes_for_reflect,
                    OriginPackageCreate::mut_exposes_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "config",
                    OriginPackageCreate::get_config_for_reflect,
                    OriginPackageCreate::mut_config_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "target",
                    OriginPackageCreate::get_target_for_reflect,
                    OriginPackageCreate::mut_target_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OriginPackageCreate>(
                    "OriginPackageCreate",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OriginPackageCreate {
    fn clear(&mut self) {
        self.clear_owner_id();
        self.clear_origin_id();
        self.clear_ident();
        self.clear_checksum();
        self.clear_manifest();
        self.clear_deps();
        self.clear_tdeps();
        self.clear_exposes();
        self.clear_config();
        self.clear_target();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OriginPackageCreate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OriginPackageCreate {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OriginPackageGet {
    // message fields
    ident: ::protobuf::SingularPtrField<OriginPackageIdent>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OriginPackageGet {}

impl OriginPackageGet {
    pub fn new() -> OriginPackageGet {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OriginPackageGet {
        static mut instance: ::protobuf::lazy::Lazy<OriginPackageGet> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OriginPackageGet,
        };
        unsafe {
            instance.get(OriginPackageGet::new)
        }
    }

    // optional .originsrv.OriginPackageIdent ident = 1;

    pub fn clear_ident(&mut self) {
        self.ident.clear();
    }

    pub fn has_ident(&self) -> bool {
        self.ident.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ident(&mut self, v: OriginPackageIdent) {
        self.ident = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ident(&mut self) -> &mut OriginPackageIdent {
        if self.ident.is_none() {
            self.ident.set_default();
        }
        self.ident.as_mut().unwrap()
    }

    // Take field
    pub fn take_ident(&mut self) -> OriginPackageIdent {
        self.ident.take().unwrap_or_else(|| OriginPackageIdent::new())
    }

    pub fn get_ident(&self) -> &OriginPackageIdent {
        self.ident.as_ref().unwrap_or_else(|| OriginPackageIdent::default_instance())
    }

    fn get_ident_for_reflect(&self) -> &::protobuf::SingularPtrField<OriginPackageIdent> {
        &self.ident
    }

    fn mut_ident_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<OriginPackageIdent> {
        &mut self.ident
    }
}

impl ::protobuf::Message for OriginPackageGet {
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

impl ::protobuf::MessageStatic for OriginPackageGet {
    fn new() -> OriginPackageGet {
        OriginPackageGet::new()
    }

    fn descriptor_static(_: ::std::option::Option<OriginPackageGet>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<OriginPackageIdent>>(
                    "ident",
                    OriginPackageGet::get_ident_for_reflect,
                    OriginPackageGet::mut_ident_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OriginPackageGet>(
                    "OriginPackageGet",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OriginPackageGet {
    fn clear(&mut self) {
        self.clear_ident();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OriginPackageGet {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OriginPackageGet {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OriginPackageLatestGet {
    // message fields
    ident: ::protobuf::SingularPtrField<OriginPackageIdent>,
    target: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OriginPackageLatestGet {}

impl OriginPackageLatestGet {
    pub fn new() -> OriginPackageLatestGet {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OriginPackageLatestGet {
        static mut instance: ::protobuf::lazy::Lazy<OriginPackageLatestGet> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OriginPackageLatestGet,
        };
        unsafe {
            instance.get(OriginPackageLatestGet::new)
        }
    }

    // optional .originsrv.OriginPackageIdent ident = 1;

    pub fn clear_ident(&mut self) {
        self.ident.clear();
    }

    pub fn has_ident(&self) -> bool {
        self.ident.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ident(&mut self, v: OriginPackageIdent) {
        self.ident = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ident(&mut self) -> &mut OriginPackageIdent {
        if self.ident.is_none() {
            self.ident.set_default();
        }
        self.ident.as_mut().unwrap()
    }

    // Take field
    pub fn take_ident(&mut self) -> OriginPackageIdent {
        self.ident.take().unwrap_or_else(|| OriginPackageIdent::new())
    }

    pub fn get_ident(&self) -> &OriginPackageIdent {
        self.ident.as_ref().unwrap_or_else(|| OriginPackageIdent::default_instance())
    }

    fn get_ident_for_reflect(&self) -> &::protobuf::SingularPtrField<OriginPackageIdent> {
        &self.ident
    }

    fn mut_ident_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<OriginPackageIdent> {
        &mut self.ident
    }

    // optional string target = 2;

    pub fn clear_target(&mut self) {
        self.target.clear();
    }

    pub fn has_target(&self) -> bool {
        self.target.is_some()
    }

    // Param is passed by value, moved
    pub fn set_target(&mut self, v: ::std::string::String) {
        self.target = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_target(&mut self) -> &mut ::std::string::String {
        if self.target.is_none() {
            self.target.set_default();
        }
        self.target.as_mut().unwrap()
    }

    // Take field
    pub fn take_target(&mut self) -> ::std::string::String {
        self.target.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_target(&self) -> &str {
        match self.target.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_target_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.target
    }

    fn mut_target_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.target
    }
}

impl ::protobuf::Message for OriginPackageLatestGet {
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
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.target)?;
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
        if let Some(ref v) = self.target.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
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
        if let Some(ref v) = self.target.as_ref() {
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

impl ::protobuf::MessageStatic for OriginPackageLatestGet {
    fn new() -> OriginPackageLatestGet {
        OriginPackageLatestGet::new()
    }

    fn descriptor_static(_: ::std::option::Option<OriginPackageLatestGet>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<OriginPackageIdent>>(
                    "ident",
                    OriginPackageLatestGet::get_ident_for_reflect,
                    OriginPackageLatestGet::mut_ident_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "target",
                    OriginPackageLatestGet::get_target_for_reflect,
                    OriginPackageLatestGet::mut_target_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OriginPackageLatestGet>(
                    "OriginPackageLatestGet",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OriginPackageLatestGet {
    fn clear(&mut self) {
        self.clear_ident();
        self.clear_target();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OriginPackageLatestGet {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OriginPackageLatestGet {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OriginPackageListRequest {
    // message fields
    ident: ::protobuf::SingularPtrField<OriginPackageIdent>,
    start: ::std::option::Option<u64>,
    stop: ::std::option::Option<u64>,
    distinct: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OriginPackageListRequest {}

impl OriginPackageListRequest {
    pub fn new() -> OriginPackageListRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OriginPackageListRequest {
        static mut instance: ::protobuf::lazy::Lazy<OriginPackageListRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OriginPackageListRequest,
        };
        unsafe {
            instance.get(OriginPackageListRequest::new)
        }
    }

    // optional .originsrv.OriginPackageIdent ident = 1;

    pub fn clear_ident(&mut self) {
        self.ident.clear();
    }

    pub fn has_ident(&self) -> bool {
        self.ident.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ident(&mut self, v: OriginPackageIdent) {
        self.ident = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ident(&mut self) -> &mut OriginPackageIdent {
        if self.ident.is_none() {
            self.ident.set_default();
        }
        self.ident.as_mut().unwrap()
    }

    // Take field
    pub fn take_ident(&mut self) -> OriginPackageIdent {
        self.ident.take().unwrap_or_else(|| OriginPackageIdent::new())
    }

    pub fn get_ident(&self) -> &OriginPackageIdent {
        self.ident.as_ref().unwrap_or_else(|| OriginPackageIdent::default_instance())
    }

    fn get_ident_for_reflect(&self) -> &::protobuf::SingularPtrField<OriginPackageIdent> {
        &self.ident
    }

    fn mut_ident_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<OriginPackageIdent> {
        &mut self.ident
    }

    // optional uint64 start = 2;

    pub fn clear_start(&mut self) {
        self.start = ::std::option::Option::None;
    }

    pub fn has_start(&self) -> bool {
        self.start.is_some()
    }

    // Param is passed by value, moved
    pub fn set_start(&mut self, v: u64) {
        self.start = ::std::option::Option::Some(v);
    }

    pub fn get_start(&self) -> u64 {
        self.start.unwrap_or(0)
    }

    fn get_start_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.start
    }

    fn mut_start_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.start
    }

    // optional uint64 stop = 3;

    pub fn clear_stop(&mut self) {
        self.stop = ::std::option::Option::None;
    }

    pub fn has_stop(&self) -> bool {
        self.stop.is_some()
    }

    // Param is passed by value, moved
    pub fn set_stop(&mut self, v: u64) {
        self.stop = ::std::option::Option::Some(v);
    }

    pub fn get_stop(&self) -> u64 {
        self.stop.unwrap_or(0)
    }

    fn get_stop_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.stop
    }

    fn mut_stop_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.stop
    }

    // optional bool distinct = 4;

    pub fn clear_distinct(&mut self) {
        self.distinct = ::std::option::Option::None;
    }

    pub fn has_distinct(&self) -> bool {
        self.distinct.is_some()
    }

    // Param is passed by value, moved
    pub fn set_distinct(&mut self, v: bool) {
        self.distinct = ::std::option::Option::Some(v);
    }

    pub fn get_distinct(&self) -> bool {
        self.distinct.unwrap_or(false)
    }

    fn get_distinct_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.distinct
    }

    fn mut_distinct_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.distinct
    }
}

impl ::protobuf::Message for OriginPackageListRequest {
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
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.start = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.stop = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.distinct = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.start {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.stop {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.distinct {
            my_size += 2;
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
        if let Some(v) = self.start {
            os.write_uint64(2, v)?;
        }
        if let Some(v) = self.stop {
            os.write_uint64(3, v)?;
        }
        if let Some(v) = self.distinct {
            os.write_bool(4, v)?;
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

impl ::protobuf::MessageStatic for OriginPackageListRequest {
    fn new() -> OriginPackageListRequest {
        OriginPackageListRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<OriginPackageListRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<OriginPackageIdent>>(
                    "ident",
                    OriginPackageListRequest::get_ident_for_reflect,
                    OriginPackageListRequest::mut_ident_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "start",
                    OriginPackageListRequest::get_start_for_reflect,
                    OriginPackageListRequest::mut_start_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "stop",
                    OriginPackageListRequest::get_stop_for_reflect,
                    OriginPackageListRequest::mut_stop_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "distinct",
                    OriginPackageListRequest::get_distinct_for_reflect,
                    OriginPackageListRequest::mut_distinct_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OriginPackageListRequest>(
                    "OriginPackageListRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OriginPackageListRequest {
    fn clear(&mut self) {
        self.clear_ident();
        self.clear_start();
        self.clear_stop();
        self.clear_distinct();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OriginPackageListRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OriginPackageListRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OriginPackageListResponse {
    // message fields
    start: ::std::option::Option<u64>,
    stop: ::std::option::Option<u64>,
    count: ::std::option::Option<u64>,
    idents: ::protobuf::RepeatedField<OriginPackageIdent>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OriginPackageListResponse {}

impl OriginPackageListResponse {
    pub fn new() -> OriginPackageListResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OriginPackageListResponse {
        static mut instance: ::protobuf::lazy::Lazy<OriginPackageListResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OriginPackageListResponse,
        };
        unsafe {
            instance.get(OriginPackageListResponse::new)
        }
    }

    // optional uint64 start = 1;

    pub fn clear_start(&mut self) {
        self.start = ::std::option::Option::None;
    }

    pub fn has_start(&self) -> bool {
        self.start.is_some()
    }

    // Param is passed by value, moved
    pub fn set_start(&mut self, v: u64) {
        self.start = ::std::option::Option::Some(v);
    }

    pub fn get_start(&self) -> u64 {
        self.start.unwrap_or(0)
    }

    fn get_start_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.start
    }

    fn mut_start_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.start
    }

    // optional uint64 stop = 2;

    pub fn clear_stop(&mut self) {
        self.stop = ::std::option::Option::None;
    }

    pub fn has_stop(&self) -> bool {
        self.stop.is_some()
    }

    // Param is passed by value, moved
    pub fn set_stop(&mut self, v: u64) {
        self.stop = ::std::option::Option::Some(v);
    }

    pub fn get_stop(&self) -> u64 {
        self.stop.unwrap_or(0)
    }

    fn get_stop_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.stop
    }

    fn mut_stop_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.stop
    }

    // optional uint64 count = 3;

    pub fn clear_count(&mut self) {
        self.count = ::std::option::Option::None;
    }

    pub fn has_count(&self) -> bool {
        self.count.is_some()
    }

    // Param is passed by value, moved
    pub fn set_count(&mut self, v: u64) {
        self.count = ::std::option::Option::Some(v);
    }

    pub fn get_count(&self) -> u64 {
        self.count.unwrap_or(0)
    }

    fn get_count_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.count
    }

    fn mut_count_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.count
    }

    // repeated .originsrv.OriginPackageIdent idents = 4;

    pub fn clear_idents(&mut self) {
        self.idents.clear();
    }

    // Param is passed by value, moved
    pub fn set_idents(&mut self, v: ::protobuf::RepeatedField<OriginPackageIdent>) {
        self.idents = v;
    }

    // Mutable pointer to the field.
    pub fn mut_idents(&mut self) -> &mut ::protobuf::RepeatedField<OriginPackageIdent> {
        &mut self.idents
    }

    // Take field
    pub fn take_idents(&mut self) -> ::protobuf::RepeatedField<OriginPackageIdent> {
        ::std::mem::replace(&mut self.idents, ::protobuf::RepeatedField::new())
    }

    pub fn get_idents(&self) -> &[OriginPackageIdent] {
        &self.idents
    }

    fn get_idents_for_reflect(&self) -> &::protobuf::RepeatedField<OriginPackageIdent> {
        &self.idents
    }

    fn mut_idents_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<OriginPackageIdent> {
        &mut self.idents
    }
}

impl ::protobuf::Message for OriginPackageListResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.idents {
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.start = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.stop = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.count = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.idents)?;
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
        if let Some(v) = self.start {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.stop {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.count {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.idents {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.start {
            os.write_uint64(1, v)?;
        }
        if let Some(v) = self.stop {
            os.write_uint64(2, v)?;
        }
        if let Some(v) = self.count {
            os.write_uint64(3, v)?;
        }
        for v in &self.idents {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for OriginPackageListResponse {
    fn new() -> OriginPackageListResponse {
        OriginPackageListResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<OriginPackageListResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "start",
                    OriginPackageListResponse::get_start_for_reflect,
                    OriginPackageListResponse::mut_start_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "stop",
                    OriginPackageListResponse::get_stop_for_reflect,
                    OriginPackageListResponse::mut_stop_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "count",
                    OriginPackageListResponse::get_count_for_reflect,
                    OriginPackageListResponse::mut_count_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<OriginPackageIdent>>(
                    "idents",
                    OriginPackageListResponse::get_idents_for_reflect,
                    OriginPackageListResponse::mut_idents_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OriginPackageListResponse>(
                    "OriginPackageListResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OriginPackageListResponse {
    fn clear(&mut self) {
        self.clear_start();
        self.clear_stop();
        self.clear_count();
        self.clear_idents();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OriginPackageListResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OriginPackageListResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OriginPackagePromote {
    // message fields
    channel_id: ::std::option::Option<u64>,
    package_id: ::std::option::Option<u64>,
    ident: ::protobuf::SingularPtrField<OriginPackageIdent>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OriginPackagePromote {}

impl OriginPackagePromote {
    pub fn new() -> OriginPackagePromote {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OriginPackagePromote {
        static mut instance: ::protobuf::lazy::Lazy<OriginPackagePromote> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OriginPackagePromote,
        };
        unsafe {
            instance.get(OriginPackagePromote::new)
        }
    }

    // optional uint64 channel_id = 1;

    pub fn clear_channel_id(&mut self) {
        self.channel_id = ::std::option::Option::None;
    }

    pub fn has_channel_id(&self) -> bool {
        self.channel_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_channel_id(&mut self, v: u64) {
        self.channel_id = ::std::option::Option::Some(v);
    }

    pub fn get_channel_id(&self) -> u64 {
        self.channel_id.unwrap_or(0)
    }

    fn get_channel_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.channel_id
    }

    fn mut_channel_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.channel_id
    }

    // optional uint64 package_id = 2;

    pub fn clear_package_id(&mut self) {
        self.package_id = ::std::option::Option::None;
    }

    pub fn has_package_id(&self) -> bool {
        self.package_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_package_id(&mut self, v: u64) {
        self.package_id = ::std::option::Option::Some(v);
    }

    pub fn get_package_id(&self) -> u64 {
        self.package_id.unwrap_or(0)
    }

    fn get_package_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.package_id
    }

    fn mut_package_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.package_id
    }

    // optional .originsrv.OriginPackageIdent ident = 3;

    pub fn clear_ident(&mut self) {
        self.ident.clear();
    }

    pub fn has_ident(&self) -> bool {
        self.ident.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ident(&mut self, v: OriginPackageIdent) {
        self.ident = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ident(&mut self) -> &mut OriginPackageIdent {
        if self.ident.is_none() {
            self.ident.set_default();
        }
        self.ident.as_mut().unwrap()
    }

    // Take field
    pub fn take_ident(&mut self) -> OriginPackageIdent {
        self.ident.take().unwrap_or_else(|| OriginPackageIdent::new())
    }

    pub fn get_ident(&self) -> &OriginPackageIdent {
        self.ident.as_ref().unwrap_or_else(|| OriginPackageIdent::default_instance())
    }

    fn get_ident_for_reflect(&self) -> &::protobuf::SingularPtrField<OriginPackageIdent> {
        &self.ident
    }

    fn mut_ident_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<OriginPackageIdent> {
        &mut self.ident
    }
}

impl ::protobuf::Message for OriginPackagePromote {
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.channel_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.package_id = ::std::option::Option::Some(tmp);
                },
                3 => {
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
        if let Some(v) = self.channel_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.package_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.ident.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.channel_id {
            os.write_uint64(1, v)?;
        }
        if let Some(v) = self.package_id {
            os.write_uint64(2, v)?;
        }
        if let Some(ref v) = self.ident.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for OriginPackagePromote {
    fn new() -> OriginPackagePromote {
        OriginPackagePromote::new()
    }

    fn descriptor_static(_: ::std::option::Option<OriginPackagePromote>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "channel_id",
                    OriginPackagePromote::get_channel_id_for_reflect,
                    OriginPackagePromote::mut_channel_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "package_id",
                    OriginPackagePromote::get_package_id_for_reflect,
                    OriginPackagePromote::mut_package_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<OriginPackageIdent>>(
                    "ident",
                    OriginPackagePromote::get_ident_for_reflect,
                    OriginPackagePromote::mut_ident_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OriginPackagePromote>(
                    "OriginPackagePromote",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OriginPackagePromote {
    fn clear(&mut self) {
        self.clear_channel_id();
        self.clear_package_id();
        self.clear_ident();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OriginPackagePromote {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OriginPackagePromote {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OriginPackageDemote {
    // message fields
    channel_id: ::std::option::Option<u64>,
    package_id: ::std::option::Option<u64>,
    ident: ::protobuf::SingularPtrField<OriginPackageIdent>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OriginPackageDemote {}

impl OriginPackageDemote {
    pub fn new() -> OriginPackageDemote {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OriginPackageDemote {
        static mut instance: ::protobuf::lazy::Lazy<OriginPackageDemote> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OriginPackageDemote,
        };
        unsafe {
            instance.get(OriginPackageDemote::new)
        }
    }

    // optional uint64 channel_id = 1;

    pub fn clear_channel_id(&mut self) {
        self.channel_id = ::std::option::Option::None;
    }

    pub fn has_channel_id(&self) -> bool {
        self.channel_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_channel_id(&mut self, v: u64) {
        self.channel_id = ::std::option::Option::Some(v);
    }

    pub fn get_channel_id(&self) -> u64 {
        self.channel_id.unwrap_or(0)
    }

    fn get_channel_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.channel_id
    }

    fn mut_channel_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.channel_id
    }

    // optional uint64 package_id = 2;

    pub fn clear_package_id(&mut self) {
        self.package_id = ::std::option::Option::None;
    }

    pub fn has_package_id(&self) -> bool {
        self.package_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_package_id(&mut self, v: u64) {
        self.package_id = ::std::option::Option::Some(v);
    }

    pub fn get_package_id(&self) -> u64 {
        self.package_id.unwrap_or(0)
    }

    fn get_package_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.package_id
    }

    fn mut_package_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.package_id
    }

    // optional .originsrv.OriginPackageIdent ident = 3;

    pub fn clear_ident(&mut self) {
        self.ident.clear();
    }

    pub fn has_ident(&self) -> bool {
        self.ident.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ident(&mut self, v: OriginPackageIdent) {
        self.ident = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ident(&mut self) -> &mut OriginPackageIdent {
        if self.ident.is_none() {
            self.ident.set_default();
        }
        self.ident.as_mut().unwrap()
    }

    // Take field
    pub fn take_ident(&mut self) -> OriginPackageIdent {
        self.ident.take().unwrap_or_else(|| OriginPackageIdent::new())
    }

    pub fn get_ident(&self) -> &OriginPackageIdent {
        self.ident.as_ref().unwrap_or_else(|| OriginPackageIdent::default_instance())
    }

    fn get_ident_for_reflect(&self) -> &::protobuf::SingularPtrField<OriginPackageIdent> {
        &self.ident
    }

    fn mut_ident_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<OriginPackageIdent> {
        &mut self.ident
    }
}

impl ::protobuf::Message for OriginPackageDemote {
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.channel_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.package_id = ::std::option::Option::Some(tmp);
                },
                3 => {
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
        if let Some(v) = self.channel_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.package_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.ident.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.channel_id {
            os.write_uint64(1, v)?;
        }
        if let Some(v) = self.package_id {
            os.write_uint64(2, v)?;
        }
        if let Some(ref v) = self.ident.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for OriginPackageDemote {
    fn new() -> OriginPackageDemote {
        OriginPackageDemote::new()
    }

    fn descriptor_static(_: ::std::option::Option<OriginPackageDemote>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "channel_id",
                    OriginPackageDemote::get_channel_id_for_reflect,
                    OriginPackageDemote::mut_channel_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "package_id",
                    OriginPackageDemote::get_package_id_for_reflect,
                    OriginPackageDemote::mut_package_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<OriginPackageIdent>>(
                    "ident",
                    OriginPackageDemote::get_ident_for_reflect,
                    OriginPackageDemote::mut_ident_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OriginPackageDemote>(
                    "OriginPackageDemote",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OriginPackageDemote {
    fn clear(&mut self) {
        self.clear_channel_id();
        self.clear_package_id();
        self.clear_ident();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OriginPackageDemote {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OriginPackageDemote {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OriginPackageSearchRequest {
    // message fields
    origin: ::protobuf::SingularField<::std::string::String>,
    query: ::protobuf::SingularField<::std::string::String>,
    start: ::std::option::Option<u64>,
    stop: ::std::option::Option<u64>,
    distinct: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OriginPackageSearchRequest {}

impl OriginPackageSearchRequest {
    pub fn new() -> OriginPackageSearchRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OriginPackageSearchRequest {
        static mut instance: ::protobuf::lazy::Lazy<OriginPackageSearchRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OriginPackageSearchRequest,
        };
        unsafe {
            instance.get(OriginPackageSearchRequest::new)
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

    // optional string query = 2;

    pub fn clear_query(&mut self) {
        self.query.clear();
    }

    pub fn has_query(&self) -> bool {
        self.query.is_some()
    }

    // Param is passed by value, moved
    pub fn set_query(&mut self, v: ::std::string::String) {
        self.query = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_query(&mut self) -> &mut ::std::string::String {
        if self.query.is_none() {
            self.query.set_default();
        }
        self.query.as_mut().unwrap()
    }

    // Take field
    pub fn take_query(&mut self) -> ::std::string::String {
        self.query.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_query(&self) -> &str {
        match self.query.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_query_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.query
    }

    fn mut_query_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.query
    }

    // optional uint64 start = 3;

    pub fn clear_start(&mut self) {
        self.start = ::std::option::Option::None;
    }

    pub fn has_start(&self) -> bool {
        self.start.is_some()
    }

    // Param is passed by value, moved
    pub fn set_start(&mut self, v: u64) {
        self.start = ::std::option::Option::Some(v);
    }

    pub fn get_start(&self) -> u64 {
        self.start.unwrap_or(0)
    }

    fn get_start_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.start
    }

    fn mut_start_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.start
    }

    // optional uint64 stop = 4;

    pub fn clear_stop(&mut self) {
        self.stop = ::std::option::Option::None;
    }

    pub fn has_stop(&self) -> bool {
        self.stop.is_some()
    }

    // Param is passed by value, moved
    pub fn set_stop(&mut self, v: u64) {
        self.stop = ::std::option::Option::Some(v);
    }

    pub fn get_stop(&self) -> u64 {
        self.stop.unwrap_or(0)
    }

    fn get_stop_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.stop
    }

    fn mut_stop_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.stop
    }

    // optional bool distinct = 5;

    pub fn clear_distinct(&mut self) {
        self.distinct = ::std::option::Option::None;
    }

    pub fn has_distinct(&self) -> bool {
        self.distinct.is_some()
    }

    // Param is passed by value, moved
    pub fn set_distinct(&mut self, v: bool) {
        self.distinct = ::std::option::Option::Some(v);
    }

    pub fn get_distinct(&self) -> bool {
        self.distinct.unwrap_or(false)
    }

    fn get_distinct_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.distinct
    }

    fn mut_distinct_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.distinct
    }
}

impl ::protobuf::Message for OriginPackageSearchRequest {
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.query)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.start = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.stop = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.distinct = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.query.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(v) = self.start {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.stop {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.distinct {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.origin.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.query.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(v) = self.start {
            os.write_uint64(3, v)?;
        }
        if let Some(v) = self.stop {
            os.write_uint64(4, v)?;
        }
        if let Some(v) = self.distinct {
            os.write_bool(5, v)?;
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

impl ::protobuf::MessageStatic for OriginPackageSearchRequest {
    fn new() -> OriginPackageSearchRequest {
        OriginPackageSearchRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<OriginPackageSearchRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "origin",
                    OriginPackageSearchRequest::get_origin_for_reflect,
                    OriginPackageSearchRequest::mut_origin_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "query",
                    OriginPackageSearchRequest::get_query_for_reflect,
                    OriginPackageSearchRequest::mut_query_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "start",
                    OriginPackageSearchRequest::get_start_for_reflect,
                    OriginPackageSearchRequest::mut_start_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "stop",
                    OriginPackageSearchRequest::get_stop_for_reflect,
                    OriginPackageSearchRequest::mut_stop_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "distinct",
                    OriginPackageSearchRequest::get_distinct_for_reflect,
                    OriginPackageSearchRequest::mut_distinct_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OriginPackageSearchRequest>(
                    "OriginPackageSearchRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OriginPackageSearchRequest {
    fn clear(&mut self) {
        self.clear_origin();
        self.clear_query();
        self.clear_start();
        self.clear_stop();
        self.clear_distinct();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OriginPackageSearchRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OriginPackageSearchRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OriginPackageUniqueListRequest {
    // message fields
    origin: ::protobuf::SingularField<::std::string::String>,
    start: ::std::option::Option<u64>,
    stop: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OriginPackageUniqueListRequest {}

impl OriginPackageUniqueListRequest {
    pub fn new() -> OriginPackageUniqueListRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OriginPackageUniqueListRequest {
        static mut instance: ::protobuf::lazy::Lazy<OriginPackageUniqueListRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OriginPackageUniqueListRequest,
        };
        unsafe {
            instance.get(OriginPackageUniqueListRequest::new)
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

    // optional uint64 start = 2;

    pub fn clear_start(&mut self) {
        self.start = ::std::option::Option::None;
    }

    pub fn has_start(&self) -> bool {
        self.start.is_some()
    }

    // Param is passed by value, moved
    pub fn set_start(&mut self, v: u64) {
        self.start = ::std::option::Option::Some(v);
    }

    pub fn get_start(&self) -> u64 {
        self.start.unwrap_or(0)
    }

    fn get_start_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.start
    }

    fn mut_start_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.start
    }

    // optional uint64 stop = 3;

    pub fn clear_stop(&mut self) {
        self.stop = ::std::option::Option::None;
    }

    pub fn has_stop(&self) -> bool {
        self.stop.is_some()
    }

    // Param is passed by value, moved
    pub fn set_stop(&mut self, v: u64) {
        self.stop = ::std::option::Option::Some(v);
    }

    pub fn get_stop(&self) -> u64 {
        self.stop.unwrap_or(0)
    }

    fn get_stop_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.stop
    }

    fn mut_stop_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.stop
    }
}

impl ::protobuf::Message for OriginPackageUniqueListRequest {
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.start = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.stop = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.start {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.stop {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.origin.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(v) = self.start {
            os.write_uint64(2, v)?;
        }
        if let Some(v) = self.stop {
            os.write_uint64(3, v)?;
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

impl ::protobuf::MessageStatic for OriginPackageUniqueListRequest {
    fn new() -> OriginPackageUniqueListRequest {
        OriginPackageUniqueListRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<OriginPackageUniqueListRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "origin",
                    OriginPackageUniqueListRequest::get_origin_for_reflect,
                    OriginPackageUniqueListRequest::mut_origin_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "start",
                    OriginPackageUniqueListRequest::get_start_for_reflect,
                    OriginPackageUniqueListRequest::mut_start_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "stop",
                    OriginPackageUniqueListRequest::get_stop_for_reflect,
                    OriginPackageUniqueListRequest::mut_stop_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OriginPackageUniqueListRequest>(
                    "OriginPackageUniqueListRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OriginPackageUniqueListRequest {
    fn clear(&mut self) {
        self.clear_origin();
        self.clear_start();
        self.clear_stop();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OriginPackageUniqueListRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OriginPackageUniqueListRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OriginPackageUniqueListResponse {
    // message fields
    start: ::std::option::Option<u64>,
    stop: ::std::option::Option<u64>,
    count: ::std::option::Option<u64>,
    idents: ::protobuf::RepeatedField<OriginPackageIdent>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OriginPackageUniqueListResponse {}

impl OriginPackageUniqueListResponse {
    pub fn new() -> OriginPackageUniqueListResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OriginPackageUniqueListResponse {
        static mut instance: ::protobuf::lazy::Lazy<OriginPackageUniqueListResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OriginPackageUniqueListResponse,
        };
        unsafe {
            instance.get(OriginPackageUniqueListResponse::new)
        }
    }

    // optional uint64 start = 1;

    pub fn clear_start(&mut self) {
        self.start = ::std::option::Option::None;
    }

    pub fn has_start(&self) -> bool {
        self.start.is_some()
    }

    // Param is passed by value, moved
    pub fn set_start(&mut self, v: u64) {
        self.start = ::std::option::Option::Some(v);
    }

    pub fn get_start(&self) -> u64 {
        self.start.unwrap_or(0)
    }

    fn get_start_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.start
    }

    fn mut_start_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.start
    }

    // optional uint64 stop = 2;

    pub fn clear_stop(&mut self) {
        self.stop = ::std::option::Option::None;
    }

    pub fn has_stop(&self) -> bool {
        self.stop.is_some()
    }

    // Param is passed by value, moved
    pub fn set_stop(&mut self, v: u64) {
        self.stop = ::std::option::Option::Some(v);
    }

    pub fn get_stop(&self) -> u64 {
        self.stop.unwrap_or(0)
    }

    fn get_stop_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.stop
    }

    fn mut_stop_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.stop
    }

    // optional uint64 count = 3;

    pub fn clear_count(&mut self) {
        self.count = ::std::option::Option::None;
    }

    pub fn has_count(&self) -> bool {
        self.count.is_some()
    }

    // Param is passed by value, moved
    pub fn set_count(&mut self, v: u64) {
        self.count = ::std::option::Option::Some(v);
    }

    pub fn get_count(&self) -> u64 {
        self.count.unwrap_or(0)
    }

    fn get_count_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.count
    }

    fn mut_count_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.count
    }

    // repeated .originsrv.OriginPackageIdent idents = 4;

    pub fn clear_idents(&mut self) {
        self.idents.clear();
    }

    // Param is passed by value, moved
    pub fn set_idents(&mut self, v: ::protobuf::RepeatedField<OriginPackageIdent>) {
        self.idents = v;
    }

    // Mutable pointer to the field.
    pub fn mut_idents(&mut self) -> &mut ::protobuf::RepeatedField<OriginPackageIdent> {
        &mut self.idents
    }

    // Take field
    pub fn take_idents(&mut self) -> ::protobuf::RepeatedField<OriginPackageIdent> {
        ::std::mem::replace(&mut self.idents, ::protobuf::RepeatedField::new())
    }

    pub fn get_idents(&self) -> &[OriginPackageIdent] {
        &self.idents
    }

    fn get_idents_for_reflect(&self) -> &::protobuf::RepeatedField<OriginPackageIdent> {
        &self.idents
    }

    fn mut_idents_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<OriginPackageIdent> {
        &mut self.idents
    }
}

impl ::protobuf::Message for OriginPackageUniqueListResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.idents {
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.start = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.stop = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.count = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.idents)?;
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
        if let Some(v) = self.start {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.stop {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.count {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.idents {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.start {
            os.write_uint64(1, v)?;
        }
        if let Some(v) = self.stop {
            os.write_uint64(2, v)?;
        }
        if let Some(v) = self.count {
            os.write_uint64(3, v)?;
        }
        for v in &self.idents {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for OriginPackageUniqueListResponse {
    fn new() -> OriginPackageUniqueListResponse {
        OriginPackageUniqueListResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<OriginPackageUniqueListResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "start",
                    OriginPackageUniqueListResponse::get_start_for_reflect,
                    OriginPackageUniqueListResponse::mut_start_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "stop",
                    OriginPackageUniqueListResponse::get_stop_for_reflect,
                    OriginPackageUniqueListResponse::mut_stop_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "count",
                    OriginPackageUniqueListResponse::get_count_for_reflect,
                    OriginPackageUniqueListResponse::mut_count_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<OriginPackageIdent>>(
                    "idents",
                    OriginPackageUniqueListResponse::get_idents_for_reflect,
                    OriginPackageUniqueListResponse::mut_idents_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OriginPackageUniqueListResponse>(
                    "OriginPackageUniqueListResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OriginPackageUniqueListResponse {
    fn clear(&mut self) {
        self.clear_start();
        self.clear_stop();
        self.clear_count();
        self.clear_idents();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OriginPackageUniqueListResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OriginPackageUniqueListResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OriginPackageVersionListRequest {
    // message fields
    origin: ::protobuf::SingularField<::std::string::String>,
    name: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OriginPackageVersionListRequest {}

impl OriginPackageVersionListRequest {
    pub fn new() -> OriginPackageVersionListRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OriginPackageVersionListRequest {
        static mut instance: ::protobuf::lazy::Lazy<OriginPackageVersionListRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OriginPackageVersionListRequest,
        };
        unsafe {
            instance.get(OriginPackageVersionListRequest::new)
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
}

impl ::protobuf::Message for OriginPackageVersionListRequest {
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

impl ::protobuf::MessageStatic for OriginPackageVersionListRequest {
    fn new() -> OriginPackageVersionListRequest {
        OriginPackageVersionListRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<OriginPackageVersionListRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "origin",
                    OriginPackageVersionListRequest::get_origin_for_reflect,
                    OriginPackageVersionListRequest::mut_origin_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    OriginPackageVersionListRequest::get_name_for_reflect,
                    OriginPackageVersionListRequest::mut_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OriginPackageVersionListRequest>(
                    "OriginPackageVersionListRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OriginPackageVersionListRequest {
    fn clear(&mut self) {
        self.clear_origin();
        self.clear_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OriginPackageVersionListRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OriginPackageVersionListRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OriginPackageVersionListResponse {
    // message fields
    versions: ::protobuf::RepeatedField<OriginPackageVersion>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OriginPackageVersionListResponse {}

impl OriginPackageVersionListResponse {
    pub fn new() -> OriginPackageVersionListResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OriginPackageVersionListResponse {
        static mut instance: ::protobuf::lazy::Lazy<OriginPackageVersionListResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OriginPackageVersionListResponse,
        };
        unsafe {
            instance.get(OriginPackageVersionListResponse::new)
        }
    }

    // repeated .originsrv.OriginPackageVersion versions = 1;

    pub fn clear_versions(&mut self) {
        self.versions.clear();
    }

    // Param is passed by value, moved
    pub fn set_versions(&mut self, v: ::protobuf::RepeatedField<OriginPackageVersion>) {
        self.versions = v;
    }

    // Mutable pointer to the field.
    pub fn mut_versions(&mut self) -> &mut ::protobuf::RepeatedField<OriginPackageVersion> {
        &mut self.versions
    }

    // Take field
    pub fn take_versions(&mut self) -> ::protobuf::RepeatedField<OriginPackageVersion> {
        ::std::mem::replace(&mut self.versions, ::protobuf::RepeatedField::new())
    }

    pub fn get_versions(&self) -> &[OriginPackageVersion] {
        &self.versions
    }

    fn get_versions_for_reflect(&self) -> &::protobuf::RepeatedField<OriginPackageVersion> {
        &self.versions
    }

    fn mut_versions_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<OriginPackageVersion> {
        &mut self.versions
    }
}

impl ::protobuf::Message for OriginPackageVersionListResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.versions {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.versions)?;
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
        for value in &self.versions {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.versions {
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

impl ::protobuf::MessageStatic for OriginPackageVersionListResponse {
    fn new() -> OriginPackageVersionListResponse {
        OriginPackageVersionListResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<OriginPackageVersionListResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<OriginPackageVersion>>(
                    "versions",
                    OriginPackageVersionListResponse::get_versions_for_reflect,
                    OriginPackageVersionListResponse::mut_versions_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OriginPackageVersionListResponse>(
                    "OriginPackageVersionListResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OriginPackageVersionListResponse {
    fn clear(&mut self) {
        self.clear_versions();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OriginPackageVersionListResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OriginPackageVersionListResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OriginProject {
    // message fields
    id: ::std::option::Option<u64>,
    origin_id: ::std::option::Option<u64>,
    origin_name: ::protobuf::SingularField<::std::string::String>,
    package_name: ::protobuf::SingularField<::std::string::String>,
    name: ::protobuf::SingularField<::std::string::String>,
    plan_path: ::protobuf::SingularField<::std::string::String>,
    owner_id: ::std::option::Option<u64>,
    vcs_type: ::protobuf::SingularField<::std::string::String>,
    vcs_data: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OriginProject {}

impl OriginProject {
    pub fn new() -> OriginProject {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OriginProject {
        static mut instance: ::protobuf::lazy::Lazy<OriginProject> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OriginProject,
        };
        unsafe {
            instance.get(OriginProject::new)
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

    // optional uint64 origin_id = 2;

    pub fn clear_origin_id(&mut self) {
        self.origin_id = ::std::option::Option::None;
    }

    pub fn has_origin_id(&self) -> bool {
        self.origin_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin_id(&mut self, v: u64) {
        self.origin_id = ::std::option::Option::Some(v);
    }

    pub fn get_origin_id(&self) -> u64 {
        self.origin_id.unwrap_or(0)
    }

    fn get_origin_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.origin_id
    }

    fn mut_origin_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.origin_id
    }

    // optional string origin_name = 3;

    pub fn clear_origin_name(&mut self) {
        self.origin_name.clear();
    }

    pub fn has_origin_name(&self) -> bool {
        self.origin_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin_name(&mut self, v: ::std::string::String) {
        self.origin_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_origin_name(&mut self) -> &mut ::std::string::String {
        if self.origin_name.is_none() {
            self.origin_name.set_default();
        }
        self.origin_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_origin_name(&mut self) -> ::std::string::String {
        self.origin_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_origin_name(&self) -> &str {
        match self.origin_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_origin_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.origin_name
    }

    fn mut_origin_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.origin_name
    }

    // optional string package_name = 4;

    pub fn clear_package_name(&mut self) {
        self.package_name.clear();
    }

    pub fn has_package_name(&self) -> bool {
        self.package_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_package_name(&mut self, v: ::std::string::String) {
        self.package_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_package_name(&mut self) -> &mut ::std::string::String {
        if self.package_name.is_none() {
            self.package_name.set_default();
        }
        self.package_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_package_name(&mut self) -> ::std::string::String {
        self.package_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_package_name(&self) -> &str {
        match self.package_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_package_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.package_name
    }

    fn mut_package_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.package_name
    }

    // optional string name = 5;

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

    // optional string plan_path = 6;

    pub fn clear_plan_path(&mut self) {
        self.plan_path.clear();
    }

    pub fn has_plan_path(&self) -> bool {
        self.plan_path.is_some()
    }

    // Param is passed by value, moved
    pub fn set_plan_path(&mut self, v: ::std::string::String) {
        self.plan_path = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_plan_path(&mut self) -> &mut ::std::string::String {
        if self.plan_path.is_none() {
            self.plan_path.set_default();
        }
        self.plan_path.as_mut().unwrap()
    }

    // Take field
    pub fn take_plan_path(&mut self) -> ::std::string::String {
        self.plan_path.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_plan_path(&self) -> &str {
        match self.plan_path.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_plan_path_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.plan_path
    }

    fn mut_plan_path_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.plan_path
    }

    // optional uint64 owner_id = 7;

    pub fn clear_owner_id(&mut self) {
        self.owner_id = ::std::option::Option::None;
    }

    pub fn has_owner_id(&self) -> bool {
        self.owner_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_owner_id(&mut self, v: u64) {
        self.owner_id = ::std::option::Option::Some(v);
    }

    pub fn get_owner_id(&self) -> u64 {
        self.owner_id.unwrap_or(0)
    }

    fn get_owner_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.owner_id
    }

    fn mut_owner_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.owner_id
    }

    // optional string vcs_type = 8;

    pub fn clear_vcs_type(&mut self) {
        self.vcs_type.clear();
    }

    pub fn has_vcs_type(&self) -> bool {
        self.vcs_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_vcs_type(&mut self, v: ::std::string::String) {
        self.vcs_type = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_vcs_type(&mut self) -> &mut ::std::string::String {
        if self.vcs_type.is_none() {
            self.vcs_type.set_default();
        }
        self.vcs_type.as_mut().unwrap()
    }

    // Take field
    pub fn take_vcs_type(&mut self) -> ::std::string::String {
        self.vcs_type.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_vcs_type(&self) -> &str {
        match self.vcs_type.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_vcs_type_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.vcs_type
    }

    fn mut_vcs_type_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.vcs_type
    }

    // optional string vcs_data = 9;

    pub fn clear_vcs_data(&mut self) {
        self.vcs_data.clear();
    }

    pub fn has_vcs_data(&self) -> bool {
        self.vcs_data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_vcs_data(&mut self, v: ::std::string::String) {
        self.vcs_data = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_vcs_data(&mut self) -> &mut ::std::string::String {
        if self.vcs_data.is_none() {
            self.vcs_data.set_default();
        }
        self.vcs_data.as_mut().unwrap()
    }

    // Take field
    pub fn take_vcs_data(&mut self) -> ::std::string::String {
        self.vcs_data.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_vcs_data(&self) -> &str {
        match self.vcs_data.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_vcs_data_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.vcs_data
    }

    fn mut_vcs_data_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.vcs_data
    }
}

impl ::protobuf::Message for OriginProject {
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
                    let tmp = is.read_uint64()?;
                    self.origin_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.origin_name)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.package_name)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.plan_path)?;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.owner_id = ::std::option::Option::Some(tmp);
                },
                8 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.vcs_type)?;
                },
                9 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.vcs_data)?;
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
        if let Some(v) = self.origin_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.origin_name.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        if let Some(ref v) = self.package_name.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        }
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(5, &v);
        }
        if let Some(ref v) = self.plan_path.as_ref() {
            my_size += ::protobuf::rt::string_size(6, &v);
        }
        if let Some(v) = self.owner_id {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.vcs_type.as_ref() {
            my_size += ::protobuf::rt::string_size(8, &v);
        }
        if let Some(ref v) = self.vcs_data.as_ref() {
            my_size += ::protobuf::rt::string_size(9, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id {
            os.write_uint64(1, v)?;
        }
        if let Some(v) = self.origin_id {
            os.write_uint64(2, v)?;
        }
        if let Some(ref v) = self.origin_name.as_ref() {
            os.write_string(3, &v)?;
        }
        if let Some(ref v) = self.package_name.as_ref() {
            os.write_string(4, &v)?;
        }
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(5, &v)?;
        }
        if let Some(ref v) = self.plan_path.as_ref() {
            os.write_string(6, &v)?;
        }
        if let Some(v) = self.owner_id {
            os.write_uint64(7, v)?;
        }
        if let Some(ref v) = self.vcs_type.as_ref() {
            os.write_string(8, &v)?;
        }
        if let Some(ref v) = self.vcs_data.as_ref() {
            os.write_string(9, &v)?;
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

impl ::protobuf::MessageStatic for OriginProject {
    fn new() -> OriginProject {
        OriginProject::new()
    }

    fn descriptor_static(_: ::std::option::Option<OriginProject>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "id",
                    OriginProject::get_id_for_reflect,
                    OriginProject::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "origin_id",
                    OriginProject::get_origin_id_for_reflect,
                    OriginProject::mut_origin_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "origin_name",
                    OriginProject::get_origin_name_for_reflect,
                    OriginProject::mut_origin_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "package_name",
                    OriginProject::get_package_name_for_reflect,
                    OriginProject::mut_package_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    OriginProject::get_name_for_reflect,
                    OriginProject::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "plan_path",
                    OriginProject::get_plan_path_for_reflect,
                    OriginProject::mut_plan_path_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "owner_id",
                    OriginProject::get_owner_id_for_reflect,
                    OriginProject::mut_owner_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "vcs_type",
                    OriginProject::get_vcs_type_for_reflect,
                    OriginProject::mut_vcs_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "vcs_data",
                    OriginProject::get_vcs_data_for_reflect,
                    OriginProject::mut_vcs_data_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OriginProject>(
                    "OriginProject",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OriginProject {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_origin_id();
        self.clear_origin_name();
        self.clear_package_name();
        self.clear_name();
        self.clear_plan_path();
        self.clear_owner_id();
        self.clear_vcs_type();
        self.clear_vcs_data();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OriginProject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OriginProject {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OriginProjectCreate {
    // message fields
    project: ::protobuf::SingularPtrField<OriginProject>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OriginProjectCreate {}

impl OriginProjectCreate {
    pub fn new() -> OriginProjectCreate {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OriginProjectCreate {
        static mut instance: ::protobuf::lazy::Lazy<OriginProjectCreate> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OriginProjectCreate,
        };
        unsafe {
            instance.get(OriginProjectCreate::new)
        }
    }

    // optional .originsrv.OriginProject project = 1;

    pub fn clear_project(&mut self) {
        self.project.clear();
    }

    pub fn has_project(&self) -> bool {
        self.project.is_some()
    }

    // Param is passed by value, moved
    pub fn set_project(&mut self, v: OriginProject) {
        self.project = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_project(&mut self) -> &mut OriginProject {
        if self.project.is_none() {
            self.project.set_default();
        }
        self.project.as_mut().unwrap()
    }

    // Take field
    pub fn take_project(&mut self) -> OriginProject {
        self.project.take().unwrap_or_else(|| OriginProject::new())
    }

    pub fn get_project(&self) -> &OriginProject {
        self.project.as_ref().unwrap_or_else(|| OriginProject::default_instance())
    }

    fn get_project_for_reflect(&self) -> &::protobuf::SingularPtrField<OriginProject> {
        &self.project
    }

    fn mut_project_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<OriginProject> {
        &mut self.project
    }
}

impl ::protobuf::Message for OriginProjectCreate {
    fn is_initialized(&self) -> bool {
        for v in &self.project {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.project)?;
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
        if let Some(ref v) = self.project.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.project.as_ref() {
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

impl ::protobuf::MessageStatic for OriginProjectCreate {
    fn new() -> OriginProjectCreate {
        OriginProjectCreate::new()
    }

    fn descriptor_static(_: ::std::option::Option<OriginProjectCreate>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<OriginProject>>(
                    "project",
                    OriginProjectCreate::get_project_for_reflect,
                    OriginProjectCreate::mut_project_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OriginProjectCreate>(
                    "OriginProjectCreate",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OriginProjectCreate {
    fn clear(&mut self) {
        self.clear_project();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OriginProjectCreate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OriginProjectCreate {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OriginProjectDelete {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    requestor_id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OriginProjectDelete {}

impl OriginProjectDelete {
    pub fn new() -> OriginProjectDelete {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OriginProjectDelete {
        static mut instance: ::protobuf::lazy::Lazy<OriginProjectDelete> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OriginProjectDelete,
        };
        unsafe {
            instance.get(OriginProjectDelete::new)
        }
    }

    // optional string name = 1;

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

    // optional uint64 requestor_id = 2;

    pub fn clear_requestor_id(&mut self) {
        self.requestor_id = ::std::option::Option::None;
    }

    pub fn has_requestor_id(&self) -> bool {
        self.requestor_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_requestor_id(&mut self, v: u64) {
        self.requestor_id = ::std::option::Option::Some(v);
    }

    pub fn get_requestor_id(&self) -> u64 {
        self.requestor_id.unwrap_or(0)
    }

    fn get_requestor_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.requestor_id
    }

    fn mut_requestor_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.requestor_id
    }
}

impl ::protobuf::Message for OriginProjectDelete {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.requestor_id = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(v) = self.requestor_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(v) = self.requestor_id {
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

impl ::protobuf::MessageStatic for OriginProjectDelete {
    fn new() -> OriginProjectDelete {
        OriginProjectDelete::new()
    }

    fn descriptor_static(_: ::std::option::Option<OriginProjectDelete>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    OriginProjectDelete::get_name_for_reflect,
                    OriginProjectDelete::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "requestor_id",
                    OriginProjectDelete::get_requestor_id_for_reflect,
                    OriginProjectDelete::mut_requestor_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OriginProjectDelete>(
                    "OriginProjectDelete",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OriginProjectDelete {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_requestor_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OriginProjectDelete {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OriginProjectDelete {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OriginProjectGet {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OriginProjectGet {}

impl OriginProjectGet {
    pub fn new() -> OriginProjectGet {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OriginProjectGet {
        static mut instance: ::protobuf::lazy::Lazy<OriginProjectGet> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OriginProjectGet,
        };
        unsafe {
            instance.get(OriginProjectGet::new)
        }
    }

    // optional string name = 1;

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
}

impl ::protobuf::Message for OriginProjectGet {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
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
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.name.as_ref() {
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

impl ::protobuf::MessageStatic for OriginProjectGet {
    fn new() -> OriginProjectGet {
        OriginProjectGet::new()
    }

    fn descriptor_static(_: ::std::option::Option<OriginProjectGet>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    OriginProjectGet::get_name_for_reflect,
                    OriginProjectGet::mut_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OriginProjectGet>(
                    "OriginProjectGet",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OriginProjectGet {
    fn clear(&mut self) {
        self.clear_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OriginProjectGet {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OriginProjectGet {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OriginProjectUpdate {
    // message fields
    requestor_id: ::std::option::Option<u64>,
    project: ::protobuf::SingularPtrField<OriginProject>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OriginProjectUpdate {}

impl OriginProjectUpdate {
    pub fn new() -> OriginProjectUpdate {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OriginProjectUpdate {
        static mut instance: ::protobuf::lazy::Lazy<OriginProjectUpdate> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OriginProjectUpdate,
        };
        unsafe {
            instance.get(OriginProjectUpdate::new)
        }
    }

    // optional uint64 requestor_id = 1;

    pub fn clear_requestor_id(&mut self) {
        self.requestor_id = ::std::option::Option::None;
    }

    pub fn has_requestor_id(&self) -> bool {
        self.requestor_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_requestor_id(&mut self, v: u64) {
        self.requestor_id = ::std::option::Option::Some(v);
    }

    pub fn get_requestor_id(&self) -> u64 {
        self.requestor_id.unwrap_or(0)
    }

    fn get_requestor_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.requestor_id
    }

    fn mut_requestor_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.requestor_id
    }

    // optional .originsrv.OriginProject project = 2;

    pub fn clear_project(&mut self) {
        self.project.clear();
    }

    pub fn has_project(&self) -> bool {
        self.project.is_some()
    }

    // Param is passed by value, moved
    pub fn set_project(&mut self, v: OriginProject) {
        self.project = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_project(&mut self) -> &mut OriginProject {
        if self.project.is_none() {
            self.project.set_default();
        }
        self.project.as_mut().unwrap()
    }

    // Take field
    pub fn take_project(&mut self) -> OriginProject {
        self.project.take().unwrap_or_else(|| OriginProject::new())
    }

    pub fn get_project(&self) -> &OriginProject {
        self.project.as_ref().unwrap_or_else(|| OriginProject::default_instance())
    }

    fn get_project_for_reflect(&self) -> &::protobuf::SingularPtrField<OriginProject> {
        &self.project
    }

    fn mut_project_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<OriginProject> {
        &mut self.project
    }
}

impl ::protobuf::Message for OriginProjectUpdate {
    fn is_initialized(&self) -> bool {
        for v in &self.project {
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.requestor_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.project)?;
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
        if let Some(v) = self.requestor_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.project.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.requestor_id {
            os.write_uint64(1, v)?;
        }
        if let Some(ref v) = self.project.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for OriginProjectUpdate {
    fn new() -> OriginProjectUpdate {
        OriginProjectUpdate::new()
    }

    fn descriptor_static(_: ::std::option::Option<OriginProjectUpdate>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "requestor_id",
                    OriginProjectUpdate::get_requestor_id_for_reflect,
                    OriginProjectUpdate::mut_requestor_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<OriginProject>>(
                    "project",
                    OriginProjectUpdate::get_project_for_reflect,
                    OriginProjectUpdate::mut_project_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OriginProjectUpdate>(
                    "OriginProjectUpdate",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OriginProjectUpdate {
    fn clear(&mut self) {
        self.clear_requestor_id();
        self.clear_project();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OriginProjectUpdate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OriginProjectUpdate {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OriginPublicKey {
    // message fields
    id: ::std::option::Option<u64>,
    origin_id: ::std::option::Option<u64>,
    name: ::protobuf::SingularField<::std::string::String>,
    revision: ::protobuf::SingularField<::std::string::String>,
    body: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    owner_id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OriginPublicKey {}

impl OriginPublicKey {
    pub fn new() -> OriginPublicKey {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OriginPublicKey {
        static mut instance: ::protobuf::lazy::Lazy<OriginPublicKey> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OriginPublicKey,
        };
        unsafe {
            instance.get(OriginPublicKey::new)
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

    // optional uint64 origin_id = 2;

    pub fn clear_origin_id(&mut self) {
        self.origin_id = ::std::option::Option::None;
    }

    pub fn has_origin_id(&self) -> bool {
        self.origin_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin_id(&mut self, v: u64) {
        self.origin_id = ::std::option::Option::Some(v);
    }

    pub fn get_origin_id(&self) -> u64 {
        self.origin_id.unwrap_or(0)
    }

    fn get_origin_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.origin_id
    }

    fn mut_origin_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.origin_id
    }

    // optional string name = 3;

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

    // optional string revision = 4;

    pub fn clear_revision(&mut self) {
        self.revision.clear();
    }

    pub fn has_revision(&self) -> bool {
        self.revision.is_some()
    }

    // Param is passed by value, moved
    pub fn set_revision(&mut self, v: ::std::string::String) {
        self.revision = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_revision(&mut self) -> &mut ::std::string::String {
        if self.revision.is_none() {
            self.revision.set_default();
        }
        self.revision.as_mut().unwrap()
    }

    // Take field
    pub fn take_revision(&mut self) -> ::std::string::String {
        self.revision.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_revision(&self) -> &str {
        match self.revision.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_revision_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.revision
    }

    fn mut_revision_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.revision
    }

    // optional bytes body = 5;

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
        }
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

    // optional uint64 owner_id = 6;

    pub fn clear_owner_id(&mut self) {
        self.owner_id = ::std::option::Option::None;
    }

    pub fn has_owner_id(&self) -> bool {
        self.owner_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_owner_id(&mut self, v: u64) {
        self.owner_id = ::std::option::Option::Some(v);
    }

    pub fn get_owner_id(&self) -> u64 {
        self.owner_id.unwrap_or(0)
    }

    fn get_owner_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.owner_id
    }

    fn mut_owner_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.owner_id
    }
}

impl ::protobuf::Message for OriginPublicKey {
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
                    let tmp = is.read_uint64()?;
                    self.origin_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.revision)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.body)?;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.owner_id = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.origin_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        if let Some(ref v) = self.revision.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        }
        if let Some(ref v) = self.body.as_ref() {
            my_size += ::protobuf::rt::bytes_size(5, &v);
        }
        if let Some(v) = self.owner_id {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id {
            os.write_uint64(1, v)?;
        }
        if let Some(v) = self.origin_id {
            os.write_uint64(2, v)?;
        }
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(3, &v)?;
        }
        if let Some(ref v) = self.revision.as_ref() {
            os.write_string(4, &v)?;
        }
        if let Some(ref v) = self.body.as_ref() {
            os.write_bytes(5, &v)?;
        }
        if let Some(v) = self.owner_id {
            os.write_uint64(6, v)?;
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

impl ::protobuf::MessageStatic for OriginPublicKey {
    fn new() -> OriginPublicKey {
        OriginPublicKey::new()
    }

    fn descriptor_static(_: ::std::option::Option<OriginPublicKey>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "id",
                    OriginPublicKey::get_id_for_reflect,
                    OriginPublicKey::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "origin_id",
                    OriginPublicKey::get_origin_id_for_reflect,
                    OriginPublicKey::mut_origin_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    OriginPublicKey::get_name_for_reflect,
                    OriginPublicKey::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "revision",
                    OriginPublicKey::get_revision_for_reflect,
                    OriginPublicKey::mut_revision_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "body",
                    OriginPublicKey::get_body_for_reflect,
                    OriginPublicKey::mut_body_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "owner_id",
                    OriginPublicKey::get_owner_id_for_reflect,
                    OriginPublicKey::mut_owner_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OriginPublicKey>(
                    "OriginPublicKey",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OriginPublicKey {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_origin_id();
        self.clear_name();
        self.clear_revision();
        self.clear_body();
        self.clear_owner_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OriginPublicKey {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OriginPublicKey {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OriginPublicKeyCreate {
    // message fields
    origin_id: ::std::option::Option<u64>,
    name: ::protobuf::SingularField<::std::string::String>,
    revision: ::protobuf::SingularField<::std::string::String>,
    body: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    owner_id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OriginPublicKeyCreate {}

impl OriginPublicKeyCreate {
    pub fn new() -> OriginPublicKeyCreate {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OriginPublicKeyCreate {
        static mut instance: ::protobuf::lazy::Lazy<OriginPublicKeyCreate> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OriginPublicKeyCreate,
        };
        unsafe {
            instance.get(OriginPublicKeyCreate::new)
        }
    }

    // optional uint64 origin_id = 1;

    pub fn clear_origin_id(&mut self) {
        self.origin_id = ::std::option::Option::None;
    }

    pub fn has_origin_id(&self) -> bool {
        self.origin_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin_id(&mut self, v: u64) {
        self.origin_id = ::std::option::Option::Some(v);
    }

    pub fn get_origin_id(&self) -> u64 {
        self.origin_id.unwrap_or(0)
    }

    fn get_origin_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.origin_id
    }

    fn mut_origin_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.origin_id
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

    // optional string revision = 3;

    pub fn clear_revision(&mut self) {
        self.revision.clear();
    }

    pub fn has_revision(&self) -> bool {
        self.revision.is_some()
    }

    // Param is passed by value, moved
    pub fn set_revision(&mut self, v: ::std::string::String) {
        self.revision = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_revision(&mut self) -> &mut ::std::string::String {
        if self.revision.is_none() {
            self.revision.set_default();
        }
        self.revision.as_mut().unwrap()
    }

    // Take field
    pub fn take_revision(&mut self) -> ::std::string::String {
        self.revision.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_revision(&self) -> &str {
        match self.revision.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_revision_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.revision
    }

    fn mut_revision_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.revision
    }

    // optional bytes body = 4;

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
        }
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

    // optional uint64 owner_id = 5;

    pub fn clear_owner_id(&mut self) {
        self.owner_id = ::std::option::Option::None;
    }

    pub fn has_owner_id(&self) -> bool {
        self.owner_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_owner_id(&mut self, v: u64) {
        self.owner_id = ::std::option::Option::Some(v);
    }

    pub fn get_owner_id(&self) -> u64 {
        self.owner_id.unwrap_or(0)
    }

    fn get_owner_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.owner_id
    }

    fn mut_owner_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.owner_id
    }
}

impl ::protobuf::Message for OriginPublicKeyCreate {
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
                    self.origin_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.revision)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.body)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.owner_id = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.origin_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(ref v) = self.revision.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        if let Some(ref v) = self.body.as_ref() {
            my_size += ::protobuf::rt::bytes_size(4, &v);
        }
        if let Some(v) = self.owner_id {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.origin_id {
            os.write_uint64(1, v)?;
        }
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(ref v) = self.revision.as_ref() {
            os.write_string(3, &v)?;
        }
        if let Some(ref v) = self.body.as_ref() {
            os.write_bytes(4, &v)?;
        }
        if let Some(v) = self.owner_id {
            os.write_uint64(5, v)?;
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

impl ::protobuf::MessageStatic for OriginPublicKeyCreate {
    fn new() -> OriginPublicKeyCreate {
        OriginPublicKeyCreate::new()
    }

    fn descriptor_static(_: ::std::option::Option<OriginPublicKeyCreate>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "origin_id",
                    OriginPublicKeyCreate::get_origin_id_for_reflect,
                    OriginPublicKeyCreate::mut_origin_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    OriginPublicKeyCreate::get_name_for_reflect,
                    OriginPublicKeyCreate::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "revision",
                    OriginPublicKeyCreate::get_revision_for_reflect,
                    OriginPublicKeyCreate::mut_revision_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "body",
                    OriginPublicKeyCreate::get_body_for_reflect,
                    OriginPublicKeyCreate::mut_body_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "owner_id",
                    OriginPublicKeyCreate::get_owner_id_for_reflect,
                    OriginPublicKeyCreate::mut_owner_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OriginPublicKeyCreate>(
                    "OriginPublicKeyCreate",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OriginPublicKeyCreate {
    fn clear(&mut self) {
        self.clear_origin_id();
        self.clear_name();
        self.clear_revision();
        self.clear_body();
        self.clear_owner_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OriginPublicKeyCreate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OriginPublicKeyCreate {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OriginPublicKeyGet {
    // message fields
    owner_id: ::std::option::Option<u64>,
    origin: ::protobuf::SingularField<::std::string::String>,
    revision: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OriginPublicKeyGet {}

impl OriginPublicKeyGet {
    pub fn new() -> OriginPublicKeyGet {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OriginPublicKeyGet {
        static mut instance: ::protobuf::lazy::Lazy<OriginPublicKeyGet> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OriginPublicKeyGet,
        };
        unsafe {
            instance.get(OriginPublicKeyGet::new)
        }
    }

    // optional uint64 owner_id = 1;

    pub fn clear_owner_id(&mut self) {
        self.owner_id = ::std::option::Option::None;
    }

    pub fn has_owner_id(&self) -> bool {
        self.owner_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_owner_id(&mut self, v: u64) {
        self.owner_id = ::std::option::Option::Some(v);
    }

    pub fn get_owner_id(&self) -> u64 {
        self.owner_id.unwrap_or(0)
    }

    fn get_owner_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.owner_id
    }

    fn mut_owner_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.owner_id
    }

    // optional string origin = 2;

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

    // optional string revision = 3;

    pub fn clear_revision(&mut self) {
        self.revision.clear();
    }

    pub fn has_revision(&self) -> bool {
        self.revision.is_some()
    }

    // Param is passed by value, moved
    pub fn set_revision(&mut self, v: ::std::string::String) {
        self.revision = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_revision(&mut self) -> &mut ::std::string::String {
        if self.revision.is_none() {
            self.revision.set_default();
        }
        self.revision.as_mut().unwrap()
    }

    // Take field
    pub fn take_revision(&mut self) -> ::std::string::String {
        self.revision.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_revision(&self) -> &str {
        match self.revision.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_revision_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.revision
    }

    fn mut_revision_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.revision
    }
}

impl ::protobuf::Message for OriginPublicKeyGet {
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
                    self.owner_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.origin)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.revision)?;
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
        if let Some(v) = self.owner_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.origin.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(ref v) = self.revision.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.owner_id {
            os.write_uint64(1, v)?;
        }
        if let Some(ref v) = self.origin.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(ref v) = self.revision.as_ref() {
            os.write_string(3, &v)?;
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

impl ::protobuf::MessageStatic for OriginPublicKeyGet {
    fn new() -> OriginPublicKeyGet {
        OriginPublicKeyGet::new()
    }

    fn descriptor_static(_: ::std::option::Option<OriginPublicKeyGet>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "owner_id",
                    OriginPublicKeyGet::get_owner_id_for_reflect,
                    OriginPublicKeyGet::mut_owner_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "origin",
                    OriginPublicKeyGet::get_origin_for_reflect,
                    OriginPublicKeyGet::mut_origin_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "revision",
                    OriginPublicKeyGet::get_revision_for_reflect,
                    OriginPublicKeyGet::mut_revision_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OriginPublicKeyGet>(
                    "OriginPublicKeyGet",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OriginPublicKeyGet {
    fn clear(&mut self) {
        self.clear_owner_id();
        self.clear_origin();
        self.clear_revision();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OriginPublicKeyGet {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OriginPublicKeyGet {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OriginPublicKeyLatestGet {
    // message fields
    owner_id: ::std::option::Option<u64>,
    origin: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OriginPublicKeyLatestGet {}

impl OriginPublicKeyLatestGet {
    pub fn new() -> OriginPublicKeyLatestGet {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OriginPublicKeyLatestGet {
        static mut instance: ::protobuf::lazy::Lazy<OriginPublicKeyLatestGet> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OriginPublicKeyLatestGet,
        };
        unsafe {
            instance.get(OriginPublicKeyLatestGet::new)
        }
    }

    // optional uint64 owner_id = 1;

    pub fn clear_owner_id(&mut self) {
        self.owner_id = ::std::option::Option::None;
    }

    pub fn has_owner_id(&self) -> bool {
        self.owner_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_owner_id(&mut self, v: u64) {
        self.owner_id = ::std::option::Option::Some(v);
    }

    pub fn get_owner_id(&self) -> u64 {
        self.owner_id.unwrap_or(0)
    }

    fn get_owner_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.owner_id
    }

    fn mut_owner_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.owner_id
    }

    // optional string origin = 2;

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
}

impl ::protobuf::Message for OriginPublicKeyLatestGet {
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
                    self.owner_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.origin)?;
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
        if let Some(v) = self.owner_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.origin.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.owner_id {
            os.write_uint64(1, v)?;
        }
        if let Some(ref v) = self.origin.as_ref() {
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

impl ::protobuf::MessageStatic for OriginPublicKeyLatestGet {
    fn new() -> OriginPublicKeyLatestGet {
        OriginPublicKeyLatestGet::new()
    }

    fn descriptor_static(_: ::std::option::Option<OriginPublicKeyLatestGet>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "owner_id",
                    OriginPublicKeyLatestGet::get_owner_id_for_reflect,
                    OriginPublicKeyLatestGet::mut_owner_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "origin",
                    OriginPublicKeyLatestGet::get_origin_for_reflect,
                    OriginPublicKeyLatestGet::mut_origin_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OriginPublicKeyLatestGet>(
                    "OriginPublicKeyLatestGet",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OriginPublicKeyLatestGet {
    fn clear(&mut self) {
        self.clear_owner_id();
        self.clear_origin();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OriginPublicKeyLatestGet {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OriginPublicKeyLatestGet {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OriginPublicKeyListRequest {
    // message fields
    owner_id: ::std::option::Option<u64>,
    origin_id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OriginPublicKeyListRequest {}

impl OriginPublicKeyListRequest {
    pub fn new() -> OriginPublicKeyListRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OriginPublicKeyListRequest {
        static mut instance: ::protobuf::lazy::Lazy<OriginPublicKeyListRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OriginPublicKeyListRequest,
        };
        unsafe {
            instance.get(OriginPublicKeyListRequest::new)
        }
    }

    // optional uint64 owner_id = 1;

    pub fn clear_owner_id(&mut self) {
        self.owner_id = ::std::option::Option::None;
    }

    pub fn has_owner_id(&self) -> bool {
        self.owner_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_owner_id(&mut self, v: u64) {
        self.owner_id = ::std::option::Option::Some(v);
    }

    pub fn get_owner_id(&self) -> u64 {
        self.owner_id.unwrap_or(0)
    }

    fn get_owner_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.owner_id
    }

    fn mut_owner_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.owner_id
    }

    // optional uint64 origin_id = 2;

    pub fn clear_origin_id(&mut self) {
        self.origin_id = ::std::option::Option::None;
    }

    pub fn has_origin_id(&self) -> bool {
        self.origin_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin_id(&mut self, v: u64) {
        self.origin_id = ::std::option::Option::Some(v);
    }

    pub fn get_origin_id(&self) -> u64 {
        self.origin_id.unwrap_or(0)
    }

    fn get_origin_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.origin_id
    }

    fn mut_origin_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.origin_id
    }
}

impl ::protobuf::Message for OriginPublicKeyListRequest {
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
                    self.owner_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.origin_id = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.owner_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.origin_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.owner_id {
            os.write_uint64(1, v)?;
        }
        if let Some(v) = self.origin_id {
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

impl ::protobuf::MessageStatic for OriginPublicKeyListRequest {
    fn new() -> OriginPublicKeyListRequest {
        OriginPublicKeyListRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<OriginPublicKeyListRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "owner_id",
                    OriginPublicKeyListRequest::get_owner_id_for_reflect,
                    OriginPublicKeyListRequest::mut_owner_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "origin_id",
                    OriginPublicKeyListRequest::get_origin_id_for_reflect,
                    OriginPublicKeyListRequest::mut_origin_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OriginPublicKeyListRequest>(
                    "OriginPublicKeyListRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OriginPublicKeyListRequest {
    fn clear(&mut self) {
        self.clear_owner_id();
        self.clear_origin_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OriginPublicKeyListRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OriginPublicKeyListRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OriginPublicKeyListResponse {
    // message fields
    origin_id: ::std::option::Option<u64>,
    keys: ::protobuf::RepeatedField<OriginPublicKey>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OriginPublicKeyListResponse {}

impl OriginPublicKeyListResponse {
    pub fn new() -> OriginPublicKeyListResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OriginPublicKeyListResponse {
        static mut instance: ::protobuf::lazy::Lazy<OriginPublicKeyListResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OriginPublicKeyListResponse,
        };
        unsafe {
            instance.get(OriginPublicKeyListResponse::new)
        }
    }

    // optional uint64 origin_id = 1;

    pub fn clear_origin_id(&mut self) {
        self.origin_id = ::std::option::Option::None;
    }

    pub fn has_origin_id(&self) -> bool {
        self.origin_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin_id(&mut self, v: u64) {
        self.origin_id = ::std::option::Option::Some(v);
    }

    pub fn get_origin_id(&self) -> u64 {
        self.origin_id.unwrap_or(0)
    }

    fn get_origin_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.origin_id
    }

    fn mut_origin_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.origin_id
    }

    // repeated .originsrv.OriginPublicKey keys = 2;

    pub fn clear_keys(&mut self) {
        self.keys.clear();
    }

    // Param is passed by value, moved
    pub fn set_keys(&mut self, v: ::protobuf::RepeatedField<OriginPublicKey>) {
        self.keys = v;
    }

    // Mutable pointer to the field.
    pub fn mut_keys(&mut self) -> &mut ::protobuf::RepeatedField<OriginPublicKey> {
        &mut self.keys
    }

    // Take field
    pub fn take_keys(&mut self) -> ::protobuf::RepeatedField<OriginPublicKey> {
        ::std::mem::replace(&mut self.keys, ::protobuf::RepeatedField::new())
    }

    pub fn get_keys(&self) -> &[OriginPublicKey] {
        &self.keys
    }

    fn get_keys_for_reflect(&self) -> &::protobuf::RepeatedField<OriginPublicKey> {
        &self.keys
    }

    fn mut_keys_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<OriginPublicKey> {
        &mut self.keys
    }
}

impl ::protobuf::Message for OriginPublicKeyListResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.keys {
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.origin_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.keys)?;
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
        if let Some(v) = self.origin_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.keys {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.origin_id {
            os.write_uint64(1, v)?;
        }
        for v in &self.keys {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for OriginPublicKeyListResponse {
    fn new() -> OriginPublicKeyListResponse {
        OriginPublicKeyListResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<OriginPublicKeyListResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "origin_id",
                    OriginPublicKeyListResponse::get_origin_id_for_reflect,
                    OriginPublicKeyListResponse::mut_origin_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<OriginPublicKey>>(
                    "keys",
                    OriginPublicKeyListResponse::get_keys_for_reflect,
                    OriginPublicKeyListResponse::mut_keys_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OriginPublicKeyListResponse>(
                    "OriginPublicKeyListResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OriginPublicKeyListResponse {
    fn clear(&mut self) {
        self.clear_origin_id();
        self.clear_keys();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OriginPublicKeyListResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OriginPublicKeyListResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OriginSecretKey {
    // message fields
    id: ::std::option::Option<u64>,
    origin_id: ::std::option::Option<u64>,
    name: ::protobuf::SingularField<::std::string::String>,
    revision: ::protobuf::SingularField<::std::string::String>,
    body: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    owner_id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OriginSecretKey {}

impl OriginSecretKey {
    pub fn new() -> OriginSecretKey {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OriginSecretKey {
        static mut instance: ::protobuf::lazy::Lazy<OriginSecretKey> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OriginSecretKey,
        };
        unsafe {
            instance.get(OriginSecretKey::new)
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

    // optional uint64 origin_id = 2;

    pub fn clear_origin_id(&mut self) {
        self.origin_id = ::std::option::Option::None;
    }

    pub fn has_origin_id(&self) -> bool {
        self.origin_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin_id(&mut self, v: u64) {
        self.origin_id = ::std::option::Option::Some(v);
    }

    pub fn get_origin_id(&self) -> u64 {
        self.origin_id.unwrap_or(0)
    }

    fn get_origin_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.origin_id
    }

    fn mut_origin_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.origin_id
    }

    // optional string name = 3;

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

    // optional string revision = 4;

    pub fn clear_revision(&mut self) {
        self.revision.clear();
    }

    pub fn has_revision(&self) -> bool {
        self.revision.is_some()
    }

    // Param is passed by value, moved
    pub fn set_revision(&mut self, v: ::std::string::String) {
        self.revision = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_revision(&mut self) -> &mut ::std::string::String {
        if self.revision.is_none() {
            self.revision.set_default();
        }
        self.revision.as_mut().unwrap()
    }

    // Take field
    pub fn take_revision(&mut self) -> ::std::string::String {
        self.revision.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_revision(&self) -> &str {
        match self.revision.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_revision_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.revision
    }

    fn mut_revision_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.revision
    }

    // optional bytes body = 5;

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
        }
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

    // optional uint64 owner_id = 6;

    pub fn clear_owner_id(&mut self) {
        self.owner_id = ::std::option::Option::None;
    }

    pub fn has_owner_id(&self) -> bool {
        self.owner_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_owner_id(&mut self, v: u64) {
        self.owner_id = ::std::option::Option::Some(v);
    }

    pub fn get_owner_id(&self) -> u64 {
        self.owner_id.unwrap_or(0)
    }

    fn get_owner_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.owner_id
    }

    fn mut_owner_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.owner_id
    }
}

impl ::protobuf::Message for OriginSecretKey {
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
                    let tmp = is.read_uint64()?;
                    self.origin_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.revision)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.body)?;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.owner_id = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.origin_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        if let Some(ref v) = self.revision.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        }
        if let Some(ref v) = self.body.as_ref() {
            my_size += ::protobuf::rt::bytes_size(5, &v);
        }
        if let Some(v) = self.owner_id {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id {
            os.write_uint64(1, v)?;
        }
        if let Some(v) = self.origin_id {
            os.write_uint64(2, v)?;
        }
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(3, &v)?;
        }
        if let Some(ref v) = self.revision.as_ref() {
            os.write_string(4, &v)?;
        }
        if let Some(ref v) = self.body.as_ref() {
            os.write_bytes(5, &v)?;
        }
        if let Some(v) = self.owner_id {
            os.write_uint64(6, v)?;
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

impl ::protobuf::MessageStatic for OriginSecretKey {
    fn new() -> OriginSecretKey {
        OriginSecretKey::new()
    }

    fn descriptor_static(_: ::std::option::Option<OriginSecretKey>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "id",
                    OriginSecretKey::get_id_for_reflect,
                    OriginSecretKey::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "origin_id",
                    OriginSecretKey::get_origin_id_for_reflect,
                    OriginSecretKey::mut_origin_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    OriginSecretKey::get_name_for_reflect,
                    OriginSecretKey::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "revision",
                    OriginSecretKey::get_revision_for_reflect,
                    OriginSecretKey::mut_revision_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "body",
                    OriginSecretKey::get_body_for_reflect,
                    OriginSecretKey::mut_body_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "owner_id",
                    OriginSecretKey::get_owner_id_for_reflect,
                    OriginSecretKey::mut_owner_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OriginSecretKey>(
                    "OriginSecretKey",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OriginSecretKey {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_origin_id();
        self.clear_name();
        self.clear_revision();
        self.clear_body();
        self.clear_owner_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OriginSecretKey {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OriginSecretKey {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OriginSecretKeyCreate {
    // message fields
    origin_id: ::std::option::Option<u64>,
    name: ::protobuf::SingularField<::std::string::String>,
    revision: ::protobuf::SingularField<::std::string::String>,
    body: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    owner_id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OriginSecretKeyCreate {}

impl OriginSecretKeyCreate {
    pub fn new() -> OriginSecretKeyCreate {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OriginSecretKeyCreate {
        static mut instance: ::protobuf::lazy::Lazy<OriginSecretKeyCreate> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OriginSecretKeyCreate,
        };
        unsafe {
            instance.get(OriginSecretKeyCreate::new)
        }
    }

    // optional uint64 origin_id = 1;

    pub fn clear_origin_id(&mut self) {
        self.origin_id = ::std::option::Option::None;
    }

    pub fn has_origin_id(&self) -> bool {
        self.origin_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin_id(&mut self, v: u64) {
        self.origin_id = ::std::option::Option::Some(v);
    }

    pub fn get_origin_id(&self) -> u64 {
        self.origin_id.unwrap_or(0)
    }

    fn get_origin_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.origin_id
    }

    fn mut_origin_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.origin_id
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

    // optional string revision = 3;

    pub fn clear_revision(&mut self) {
        self.revision.clear();
    }

    pub fn has_revision(&self) -> bool {
        self.revision.is_some()
    }

    // Param is passed by value, moved
    pub fn set_revision(&mut self, v: ::std::string::String) {
        self.revision = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_revision(&mut self) -> &mut ::std::string::String {
        if self.revision.is_none() {
            self.revision.set_default();
        }
        self.revision.as_mut().unwrap()
    }

    // Take field
    pub fn take_revision(&mut self) -> ::std::string::String {
        self.revision.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_revision(&self) -> &str {
        match self.revision.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_revision_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.revision
    }

    fn mut_revision_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.revision
    }

    // optional bytes body = 4;

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
        }
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

    // optional uint64 owner_id = 5;

    pub fn clear_owner_id(&mut self) {
        self.owner_id = ::std::option::Option::None;
    }

    pub fn has_owner_id(&self) -> bool {
        self.owner_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_owner_id(&mut self, v: u64) {
        self.owner_id = ::std::option::Option::Some(v);
    }

    pub fn get_owner_id(&self) -> u64 {
        self.owner_id.unwrap_or(0)
    }

    fn get_owner_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.owner_id
    }

    fn mut_owner_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.owner_id
    }
}

impl ::protobuf::Message for OriginSecretKeyCreate {
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
                    self.origin_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.revision)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.body)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.owner_id = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.origin_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(ref v) = self.revision.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        if let Some(ref v) = self.body.as_ref() {
            my_size += ::protobuf::rt::bytes_size(4, &v);
        }
        if let Some(v) = self.owner_id {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.origin_id {
            os.write_uint64(1, v)?;
        }
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(ref v) = self.revision.as_ref() {
            os.write_string(3, &v)?;
        }
        if let Some(ref v) = self.body.as_ref() {
            os.write_bytes(4, &v)?;
        }
        if let Some(v) = self.owner_id {
            os.write_uint64(5, v)?;
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

impl ::protobuf::MessageStatic for OriginSecretKeyCreate {
    fn new() -> OriginSecretKeyCreate {
        OriginSecretKeyCreate::new()
    }

    fn descriptor_static(_: ::std::option::Option<OriginSecretKeyCreate>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "origin_id",
                    OriginSecretKeyCreate::get_origin_id_for_reflect,
                    OriginSecretKeyCreate::mut_origin_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    OriginSecretKeyCreate::get_name_for_reflect,
                    OriginSecretKeyCreate::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "revision",
                    OriginSecretKeyCreate::get_revision_for_reflect,
                    OriginSecretKeyCreate::mut_revision_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "body",
                    OriginSecretKeyCreate::get_body_for_reflect,
                    OriginSecretKeyCreate::mut_body_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "owner_id",
                    OriginSecretKeyCreate::get_owner_id_for_reflect,
                    OriginSecretKeyCreate::mut_owner_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OriginSecretKeyCreate>(
                    "OriginSecretKeyCreate",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OriginSecretKeyCreate {
    fn clear(&mut self) {
        self.clear_origin_id();
        self.clear_name();
        self.clear_revision();
        self.clear_body();
        self.clear_owner_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OriginSecretKeyCreate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OriginSecretKeyCreate {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OriginSecretKeyGet {
    // message fields
    owner_id: ::std::option::Option<u64>,
    origin: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OriginSecretKeyGet {}

impl OriginSecretKeyGet {
    pub fn new() -> OriginSecretKeyGet {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OriginSecretKeyGet {
        static mut instance: ::protobuf::lazy::Lazy<OriginSecretKeyGet> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OriginSecretKeyGet,
        };
        unsafe {
            instance.get(OriginSecretKeyGet::new)
        }
    }

    // optional uint64 owner_id = 1;

    pub fn clear_owner_id(&mut self) {
        self.owner_id = ::std::option::Option::None;
    }

    pub fn has_owner_id(&self) -> bool {
        self.owner_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_owner_id(&mut self, v: u64) {
        self.owner_id = ::std::option::Option::Some(v);
    }

    pub fn get_owner_id(&self) -> u64 {
        self.owner_id.unwrap_or(0)
    }

    fn get_owner_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.owner_id
    }

    fn mut_owner_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.owner_id
    }

    // optional string origin = 2;

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
}

impl ::protobuf::Message for OriginSecretKeyGet {
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
                    self.owner_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.origin)?;
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
        if let Some(v) = self.owner_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.origin.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.owner_id {
            os.write_uint64(1, v)?;
        }
        if let Some(ref v) = self.origin.as_ref() {
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

impl ::protobuf::MessageStatic for OriginSecretKeyGet {
    fn new() -> OriginSecretKeyGet {
        OriginSecretKeyGet::new()
    }

    fn descriptor_static(_: ::std::option::Option<OriginSecretKeyGet>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "owner_id",
                    OriginSecretKeyGet::get_owner_id_for_reflect,
                    OriginSecretKeyGet::mut_owner_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "origin",
                    OriginSecretKeyGet::get_origin_for_reflect,
                    OriginSecretKeyGet::mut_origin_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OriginSecretKeyGet>(
                    "OriginSecretKeyGet",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OriginSecretKeyGet {
    fn clear(&mut self) {
        self.clear_owner_id();
        self.clear_origin();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OriginSecretKeyGet {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OriginSecretKeyGet {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x19protocols/originsrv.proto\x12\toriginsrv\"=\n\x1cAccountInvitation\
    ListRequest\x12\x1d\n\naccount_id\x18\x01\x20\x01(\x04R\taccountId\"}\n\
    \x1dAccountInvitationListResponse\x12\x1d\n\naccount_id\x18\x01\x20\x01(\
    \x04R\taccountId\x12=\n\x0binvitations\x18\x02\x20\x03(\x0b2\x1b.origins\
    rv.OriginInvitationR\x0binvitations\"\xc1\x01\n\x18CheckOriginAccessRequ\
    est\x12\x1f\n\naccount_id\x18\x01\x20\x01(\x04H\0R\taccountId\x12#\n\x0c\
    account_name\x18\x02\x20\x01(\tH\0R\x0baccountName\x12\x1d\n\torigin_id\
    \x18\x03\x20\x01(\x04H\x01R\x08originId\x12!\n\x0borigin_name\x18\x04\
    \x20\x01(\tH\x01R\noriginNameB\x0e\n\x0caccount_infoB\r\n\x0borigin_info\
    \":\n\x19CheckOriginAccessResponse\x12\x1d\n\nhas_access\x18\x01\x20\x01\
    (\x08R\thasAccess\"q\n\x06Origin\x12\x0e\n\x02id\x18\x01\x20\x01(\x04R\
    \x02id\x12\x12\n\x04name\x18\x02\x20\x01(\tR\x04name\x12\x19\n\x08owner_\
    id\x18\x03\x20\x01(\x04R\x07ownerId\x12(\n\x10private_key_name\x18\x04\
    \x20\x01(\tR\x0eprivateKeyName\"\\\n\x0cOriginCreate\x12\x12\n\x04name\
    \x18\x01\x20\x01(\tR\x04name\x12\x19\n\x08owner_id\x18\x02\x20\x01(\x04R\
    \x07ownerId\x12\x1d\n\nowner_name\x18\x03\x20\x01(\tR\townerName\"\"\n\
    \x0cOriginDelete\x12\x12\n\x04name\x18\x01\x20\x01(\tR\x04name\"\x1f\n\t\
    OriginGet\x12\x12\n\x04name\x18\x01\x20\x01(\tR\x04name\"k\n\rOriginChan\
    nel\x12\x0e\n\x02id\x18\x01\x20\x01(\x04R\x02id\x12\x1b\n\torigin_id\x18\
    \x02\x20\x01(\x04R\x08originId\x12\x12\n\x04name\x18\x03\x20\x01(\tR\x04\
    name\x12\x19\n\x08owner_id\x18\x04\x20\x01(\x04R\x07ownerId\"@\n\x12Orig\
    inChannelIdent\x12\x16\n\x06origin\x18\x01\x20\x01(\tR\x06origin\x12\x12\
    \n\x04name\x18\x02\x20\x01(\tR\x04name\"\x82\x01\n\x13OriginChannelCreat\
    e\x12\x1b\n\torigin_id\x18\x01\x20\x01(\x04R\x08originId\x12\x1f\n\x0bor\
    igin_name\x18\x02\x20\x01(\tR\noriginName\x12\x12\n\x04name\x18\x03\x20\
    \x01(\tR\x04name\x12\x19\n\x08owner_id\x18\x04\x20\x01(\x04R\x07ownerId\
    \"G\n\x10OriginChannelGet\x12\x1f\n\x0borigin_name\x18\x01\x20\x01(\tR\n\
    originName\x12\x12\n\x04name\x18\x02\x20\x01(\tR\x04name\"7\n\x18OriginC\
    hannelListRequest\x12\x1b\n\torigin_id\x18\x01\x20\x01(\x04R\x08originId\
    \"n\n\x19OriginChannelListResponse\x12\x1b\n\torigin_id\x18\x01\x20\x01(\
    \x04R\x08originId\x124\n\x08channels\x18\x02\x20\x03(\x0b2\x18.originsrv\
    .OriginChannelR\x08channels\"b\n\x17OriginChannelPackageGet\x12\x12\n\
    \x04name\x18\x01\x20\x01(\tR\x04name\x123\n\x05ident\x18\x02\x20\x01(\
    \x0b2\x1d.originsrv.OriginPackageIdentR\x05ident\"\x80\x01\n\x1dOriginCh\
    annelPackageLatestGet\x12\x12\n\x04name\x18\x01\x20\x01(\tR\x04name\x123\
    \n\x05ident\x18\x02\x20\x01(\x0b2\x1d.originsrv.OriginPackageIdentR\x05i\
    dent\x12\x16\n\x06target\x18\x03\x20\x01(\tR\x06target\"\x94\x01\n\x1fOr\
    iginChannelPackageListRequest\x12\x12\n\x04name\x18\x01\x20\x01(\tR\x04n\
    ame\x123\n\x05ident\x18\x02\x20\x01(\x0b2\x1d.originsrv.OriginPackageIde\
    ntR\x05ident\x12\x14\n\x05start\x18\x03\x20\x01(\x04R\x05start\x12\x12\n\
    \x04stop\x18\x04\x20\x01(\x04R\x04stop\"B\n\x13OriginChannelDelete\x12\
    \x0e\n\x02id\x18\x01\x20\x01(\x04R\x02id\x12\x1b\n\torigin_id\x18\x02\
    \x20\x01(\x04R\x08originId\"\xbd\x01\n\x10OriginInvitation\x12\x0e\n\x02\
    id\x18\x01\x20\x01(\x04R\x02id\x12\x1d\n\naccount_id\x18\x02\x20\x01(\
    \x04R\taccountId\x12!\n\x0caccount_name\x18\x03\x20\x01(\tR\x0baccountNa\
    me\x12\x1b\n\torigin_id\x18\x04\x20\x01(\x04R\x08originId\x12\x1f\n\x0bo\
    rigin_name\x18\x05\x20\x01(\tR\noriginName\x12\x19\n\x08owner_id\x18\x06\
    \x20\x01(\x04R\x07ownerId\"\x94\x01\n\x1dOriginInvitationAcceptRequest\
    \x12\x1d\n\naccount_id\x18\x01\x20\x01(\x04R\taccountId\x12\x1b\n\tinvit\
    e_id\x18\x02\x20\x01(\x04R\x08inviteId\x12\x1f\n\x0borigin_name\x18\x03\
    \x20\x01(\tR\noriginName\x12\x16\n\x06ignore\x18\x04\x20\x01(\x08R\x06ig\
    nore\"\xb3\x01\n\x16OriginInvitationCreate\x12\x1d\n\naccount_id\x18\x01\
    \x20\x01(\x04R\taccountId\x12!\n\x0caccount_name\x18\x02\x20\x01(\tR\x0b\
    accountName\x12\x1b\n\torigin_id\x18\x03\x20\x01(\x04R\x08originId\x12\
    \x1f\n\x0borigin_name\x18\x04\x20\x01(\tR\noriginName\x12\x19\n\x08owner\
    _id\x18\x05\x20\x01(\x04R\x07ownerId\":\n\x1bOriginInvitationListRequest\
    \x12\x1b\n\torigin_id\x18\x01\x20\x01(\x04R\x08originId\"z\n\x1cOriginIn\
    vitationListResponse\x12\x1b\n\torigin_id\x18\x01\x20\x01(\x04R\x08origi\
    nId\x12=\n\x0binvitations\x18\x02\x20\x03(\x0b2\x1b.originsrv.OriginInvi\
    tationR\x0binvitations\"`\n\x0eOriginKeyIdent\x12\x16\n\x06origin\x18\
    \x01\x20\x01(\tR\x06origin\x12\x1a\n\x08revision\x18\x02\x20\x01(\tR\x08\
    revision\x12\x1a\n\x08location\x18\x03\x20\x01(\tR\x08location\"6\n\x17O\
    riginMemberListRequest\x12\x1b\n\torigin_id\x18\x01\x20\x01(\x04R\x08ori\
    ginId\"Q\n\x18OriginMemberListResponse\x12\x1b\n\torigin_id\x18\x01\x20\
    \x01(\x04R\x08originId\x12\x18\n\x07members\x18\x02\x20\x03(\tR\x07membe\
    rs\"J\n\x12OriginMemberRemove\x12\x1b\n\torigin_id\x18\x01\x20\x01(\x04R\
    \x08originId\x12\x17\n\x07user_id\x18\x02\x20\x01(\x04R\x06userId\"\xfa\
    \x02\n\rOriginPackage\x12\x0e\n\x02id\x18\x01\x20\x01(\x04R\x02id\x12\
    \x19\n\x08owner_id\x18\x02\x20\x01(\x04R\x07ownerId\x12\x1b\n\torigin_id\
    \x18\x03\x20\x01(\x04R\x08originId\x123\n\x05ident\x18\x04\x20\x01(\x0b2\
    \x1d.originsrv.OriginPackageIdentR\x05ident\x12\x1a\n\x08checksum\x18\
    \x05\x20\x01(\tR\x08checksum\x12\x1a\n\x08manifest\x18\x06\x20\x01(\tR\
    \x08manifest\x121\n\x04deps\x18\x07\x20\x03(\x0b2\x1d.originsrv.OriginPa\
    ckageIdentR\x04deps\x123\n\x05tdeps\x18\x08\x20\x03(\x0b2\x1d.originsrv.\
    OriginPackageIdentR\x05tdeps\x12\x1c\n\x07exposes\x18\t\x20\x03(\rR\x07e\
    xposesB\x02\x10\x01\x12\x16\n\x06config\x18\n\x20\x01(\tR\x06config\x12\
    \x16\n\x06target\x18\x0b\x20\x01(\tR\x06target\"t\n\x12OriginPackageIden\
    t\x12\x16\n\x06origin\x18\x01\x20\x01(\tR\x06origin\x12\x12\n\x04name\
    \x18\x02\x20\x01(\tR\x04name\x12\x18\n\x07version\x18\x03\x20\x01(\tR\
    \x07version\x12\x18\n\x07release\x18\x04\x20\x01(\tR\x07release\"\x99\
    \x01\n\x14OriginPackageVersion\x12\x16\n\x06origin\x18\x01\x20\x01(\tR\
    \x06origin\x12\x12\n\x04name\x18\x02\x20\x01(\tR\x04name\x12\x18\n\x07ve\
    rsion\x18\x03\x20\x01(\tR\x07version\x12#\n\rrelease_count\x18\x04\x20\
    \x01(\x04R\x0creleaseCount\x12\x16\n\x06latest\x18\x05\x20\x01(\tR\x06la\
    test\"\xf0\x02\n\x13OriginPackageCreate\x12\x19\n\x08owner_id\x18\x01\
    \x20\x01(\x04R\x07ownerId\x12\x1b\n\torigin_id\x18\x02\x20\x01(\x04R\x08\
    originId\x123\n\x05ident\x18\x03\x20\x01(\x0b2\x1d.originsrv.OriginPacka\
    geIdentR\x05ident\x12\x1a\n\x08checksum\x18\x04\x20\x01(\tR\x08checksum\
    \x12\x1a\n\x08manifest\x18\x05\x20\x01(\tR\x08manifest\x121\n\x04deps\
    \x18\x06\x20\x03(\x0b2\x1d.originsrv.OriginPackageIdentR\x04deps\x123\n\
    \x05tdeps\x18\x07\x20\x03(\x0b2\x1d.originsrv.OriginPackageIdentR\x05tde\
    ps\x12\x1c\n\x07exposes\x18\x08\x20\x03(\rR\x07exposesB\x02\x10\x01\x12\
    \x16\n\x06config\x18\t\x20\x01(\tR\x06config\x12\x16\n\x06target\x18\n\
    \x20\x01(\tR\x06target\"G\n\x10OriginPackageGet\x123\n\x05ident\x18\x01\
    \x20\x01(\x0b2\x1d.originsrv.OriginPackageIdentR\x05ident\"e\n\x16Origin\
    PackageLatestGet\x123\n\x05ident\x18\x01\x20\x01(\x0b2\x1d.originsrv.Ori\
    ginPackageIdentR\x05ident\x12\x16\n\x06target\x18\x02\x20\x01(\tR\x06tar\
    get\"\x95\x01\n\x18OriginPackageListRequest\x123\n\x05ident\x18\x01\x20\
    \x01(\x0b2\x1d.originsrv.OriginPackageIdentR\x05ident\x12\x14\n\x05start\
    \x18\x02\x20\x01(\x04R\x05start\x12\x12\n\x04stop\x18\x03\x20\x01(\x04R\
    \x04stop\x12\x1a\n\x08distinct\x18\x04\x20\x01(\x08R\x08distinct\"\x92\
    \x01\n\x19OriginPackageListResponse\x12\x14\n\x05start\x18\x01\x20\x01(\
    \x04R\x05start\x12\x12\n\x04stop\x18\x02\x20\x01(\x04R\x04stop\x12\x14\n\
    \x05count\x18\x03\x20\x01(\x04R\x05count\x125\n\x06idents\x18\x04\x20\
    \x03(\x0b2\x1d.originsrv.OriginPackageIdentR\x06idents\"\x89\x01\n\x14Or\
    iginPackagePromote\x12\x1d\n\nchannel_id\x18\x01\x20\x01(\x04R\tchannelI\
    d\x12\x1d\n\npackage_id\x18\x02\x20\x01(\x04R\tpackageId\x123\n\x05ident\
    \x18\x03\x20\x01(\x0b2\x1d.originsrv.OriginPackageIdentR\x05ident\"\x88\
    \x01\n\x13OriginPackageDemote\x12\x1d\n\nchannel_id\x18\x01\x20\x01(\x04\
    R\tchannelId\x12\x1d\n\npackage_id\x18\x02\x20\x01(\x04R\tpackageId\x123\
    \n\x05ident\x18\x03\x20\x01(\x0b2\x1d.originsrv.OriginPackageIdentR\x05i\
    dent\"\x90\x01\n\x1aOriginPackageSearchRequest\x12\x16\n\x06origin\x18\
    \x01\x20\x01(\tR\x06origin\x12\x14\n\x05query\x18\x02\x20\x01(\tR\x05que\
    ry\x12\x14\n\x05start\x18\x03\x20\x01(\x04R\x05start\x12\x12\n\x04stop\
    \x18\x04\x20\x01(\x04R\x04stop\x12\x1a\n\x08distinct\x18\x05\x20\x01(\
    \x08R\x08distinct\"b\n\x1eOriginPackageUniqueListRequest\x12\x16\n\x06or\
    igin\x18\x01\x20\x01(\tR\x06origin\x12\x14\n\x05start\x18\x02\x20\x01(\
    \x04R\x05start\x12\x12\n\x04stop\x18\x03\x20\x01(\x04R\x04stop\"\x98\x01\
    \n\x1fOriginPackageUniqueListResponse\x12\x14\n\x05start\x18\x01\x20\x01\
    (\x04R\x05start\x12\x12\n\x04stop\x18\x02\x20\x01(\x04R\x04stop\x12\x14\
    \n\x05count\x18\x03\x20\x01(\x04R\x05count\x125\n\x06idents\x18\x04\x20\
    \x03(\x0b2\x1d.originsrv.OriginPackageIdentR\x06idents\"M\n\x1fOriginPac\
    kageVersionListRequest\x12\x16\n\x06origin\x18\x01\x20\x01(\tR\x06origin\
    \x12\x12\n\x04name\x18\x02\x20\x01(\tR\x04name\"_\n\x20OriginPackageVers\
    ionListResponse\x12;\n\x08versions\x18\x01\x20\x03(\x0b2\x1f.originsrv.O\
    riginPackageVersionR\x08versions\"\x82\x02\n\rOriginProject\x12\x0e\n\
    \x02id\x18\x01\x20\x01(\x04R\x02id\x12\x1b\n\torigin_id\x18\x02\x20\x01(\
    \x04R\x08originId\x12\x1f\n\x0borigin_name\x18\x03\x20\x01(\tR\noriginNa\
    me\x12!\n\x0cpackage_name\x18\x04\x20\x01(\tR\x0bpackageName\x12\x12\n\
    \x04name\x18\x05\x20\x01(\tR\x04name\x12\x1b\n\tplan_path\x18\x06\x20\
    \x01(\tR\x08planPath\x12\x19\n\x08owner_id\x18\x07\x20\x01(\x04R\x07owne\
    rId\x12\x19\n\x08vcs_type\x18\x08\x20\x01(\tR\x07vcsType\x12\x19\n\x08vc\
    s_data\x18\t\x20\x01(\tR\x07vcsData\"I\n\x13OriginProjectCreate\x122\n\
    \x07project\x18\x01\x20\x01(\x0b2\x18.originsrv.OriginProjectR\x07projec\
    t\"L\n\x13OriginProjectDelete\x12\x12\n\x04name\x18\x01\x20\x01(\tR\x04n\
    ame\x12!\n\x0crequestor_id\x18\x02\x20\x01(\x04R\x0brequestorId\"&\n\x10\
    OriginProjectGet\x12\x12\n\x04name\x18\x01\x20\x01(\tR\x04name\"l\n\x13O\
    riginProjectUpdate\x12!\n\x0crequestor_id\x18\x01\x20\x01(\x04R\x0breque\
    storId\x122\n\x07project\x18\x02\x20\x01(\x0b2\x18.originsrv.OriginProje\
    ctR\x07project\"\x9d\x01\n\x0fOriginPublicKey\x12\x0e\n\x02id\x18\x01\
    \x20\x01(\x04R\x02id\x12\x1b\n\torigin_id\x18\x02\x20\x01(\x04R\x08origi\
    nId\x12\x12\n\x04name\x18\x03\x20\x01(\tR\x04name\x12\x1a\n\x08revision\
    \x18\x04\x20\x01(\tR\x08revision\x12\x12\n\x04body\x18\x05\x20\x01(\x0cR\
    \x04body\x12\x19\n\x08owner_id\x18\x06\x20\x01(\x04R\x07ownerId\"\x93\
    \x01\n\x15OriginPublicKeyCreate\x12\x1b\n\torigin_id\x18\x01\x20\x01(\
    \x04R\x08originId\x12\x12\n\x04name\x18\x02\x20\x01(\tR\x04name\x12\x1a\
    \n\x08revision\x18\x03\x20\x01(\tR\x08revision\x12\x12\n\x04body\x18\x04\
    \x20\x01(\x0cR\x04body\x12\x19\n\x08owner_id\x18\x05\x20\x01(\x04R\x07ow\
    nerId\"c\n\x12OriginPublicKeyGet\x12\x19\n\x08owner_id\x18\x01\x20\x01(\
    \x04R\x07ownerId\x12\x16\n\x06origin\x18\x02\x20\x01(\tR\x06origin\x12\
    \x1a\n\x08revision\x18\x03\x20\x01(\tR\x08revision\"M\n\x18OriginPublicK\
    eyLatestGet\x12\x19\n\x08owner_id\x18\x01\x20\x01(\x04R\x07ownerId\x12\
    \x16\n\x06origin\x18\x02\x20\x01(\tR\x06origin\"T\n\x1aOriginPublicKeyLi\
    stRequest\x12\x19\n\x08owner_id\x18\x01\x20\x01(\x04R\x07ownerId\x12\x1b\
    \n\torigin_id\x18\x02\x20\x01(\x04R\x08originId\"j\n\x1bOriginPublicKeyL\
    istResponse\x12\x1b\n\torigin_id\x18\x01\x20\x01(\x04R\x08originId\x12.\
    \n\x04keys\x18\x02\x20\x03(\x0b2\x1a.originsrv.OriginPublicKeyR\x04keys\
    \"\x9d\x01\n\x0fOriginSecretKey\x12\x0e\n\x02id\x18\x01\x20\x01(\x04R\
    \x02id\x12\x1b\n\torigin_id\x18\x02\x20\x01(\x04R\x08originId\x12\x12\n\
    \x04name\x18\x03\x20\x01(\tR\x04name\x12\x1a\n\x08revision\x18\x04\x20\
    \x01(\tR\x08revision\x12\x12\n\x04body\x18\x05\x20\x01(\x0cR\x04body\x12\
    \x19\n\x08owner_id\x18\x06\x20\x01(\x04R\x07ownerId\"\x93\x01\n\x15Origi\
    nSecretKeyCreate\x12\x1b\n\torigin_id\x18\x01\x20\x01(\x04R\x08originId\
    \x12\x12\n\x04name\x18\x02\x20\x01(\tR\x04name\x12\x1a\n\x08revision\x18\
    \x03\x20\x01(\tR\x08revision\x12\x12\n\x04body\x18\x04\x20\x01(\x0cR\x04\
    body\x12\x19\n\x08owner_id\x18\x05\x20\x01(\x04R\x07ownerId\"G\n\x12Orig\
    inSecretKeyGet\x12\x19\n\x08owner_id\x18\x01\x20\x01(\x04R\x07ownerId\
    \x12\x16\n\x06origin\x18\x02\x20\x01(\tR\x06originJ\xd9s\n\x07\x12\x05\0\
    \0\xe8\x02\x01\n\x08\n\x01\x02\x12\x03\0\x08\x11\n\x15\n\x02\x04\0\x12\
    \x04\x03\0\x05\x01\x1a\t\x20Account\n\n\n\n\x03\x04\0\x01\x12\x03\x03\
    \x08$\n\x0b\n\x04\x04\0\x02\0\x12\x03\x04\x02!\n\x0c\n\x05\x04\0\x02\0\
    \x04\x12\x03\x04\x02\n\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\x04\x0b\x11\n\
    \x0c\n\x05\x04\0\x02\0\x01\x12\x03\x04\x12\x1c\n\x0c\n\x05\x04\0\x02\0\
    \x03\x12\x03\x04\x1f\x20\n\n\n\x02\x04\x01\x12\x04\x07\0\n\x01\n\n\n\x03\
    \x04\x01\x01\x12\x03\x07\x08%\n\x0b\n\x04\x04\x01\x02\0\x12\x03\x08\x02!\
    \n\x0c\n\x05\x04\x01\x02\0\x04\x12\x03\x08\x02\n\n\x0c\n\x05\x04\x01\x02\
    \0\x05\x12\x03\x08\x0b\x11\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\x08\x12\
    \x1c\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03\x08\x1f\x20\n\x0b\n\x04\x04\
    \x01\x02\x01\x12\x03\t\x02,\n\x0c\n\x05\x04\x01\x02\x01\x04\x12\x03\t\
    \x02\n\n\x0c\n\x05\x04\x01\x02\x01\x06\x12\x03\t\x0b\x1b\n\x0c\n\x05\x04\
    \x01\x02\x01\x01\x12\x03\t\x1c'\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x03\
    \t*+\n\n\n\x02\x04\x02\x12\x04\x0c\0\x15\x01\n\n\n\x03\x04\x02\x01\x12\
    \x03\x0c\x08\x20\n\x0c\n\x04\x04\x02\x08\0\x12\x04\r\x02\x10\x03\n\x0c\n\
    \x05\x04\x02\x08\0\x01\x12\x03\r\x08\x14\n\x0b\n\x04\x04\x02\x02\0\x12\
    \x03\x0e\x04\x1a\n\x0c\n\x05\x04\x02\x02\0\x05\x12\x03\x0e\x04\n\n\x0c\n\
    \x05\x04\x02\x02\0\x01\x12\x03\x0e\x0b\x15\n\x0c\n\x05\x04\x02\x02\0\x03\
    \x12\x03\x0e\x18\x19\n\x0b\n\x04\x04\x02\x02\x01\x12\x03\x0f\x04\x1c\n\
    \x0c\n\x05\x04\x02\x02\x01\x05\x12\x03\x0f\x04\n\n\x0c\n\x05\x04\x02\x02\
    \x01\x01\x12\x03\x0f\x0b\x17\n\x0c\n\x05\x04\x02\x02\x01\x03\x12\x03\x0f\
    \x1a\x1b\n\x0c\n\x04\x04\x02\x08\x01\x12\x04\x11\x02\x14\x03\n\x0c\n\x05\
    \x04\x02\x08\x01\x01\x12\x03\x11\x08\x13\n\x0b\n\x04\x04\x02\x02\x02\x12\
    \x03\x12\x04\x19\n\x0c\n\x05\x04\x02\x02\x02\x05\x12\x03\x12\x04\n\n\x0c\
    \n\x05\x04\x02\x02\x02\x01\x12\x03\x12\x0b\x14\n\x0c\n\x05\x04\x02\x02\
    \x02\x03\x12\x03\x12\x17\x18\n\x0b\n\x04\x04\x02\x02\x03\x12\x03\x13\x04\
    \x1b\n\x0c\n\x05\x04\x02\x02\x03\x05\x12\x03\x13\x04\n\n\x0c\n\x05\x04\
    \x02\x02\x03\x01\x12\x03\x13\x0b\x16\n\x0c\n\x05\x04\x02\x02\x03\x03\x12\
    \x03\x13\x19\x1a\n\n\n\x02\x04\x03\x12\x04\x17\0\x19\x01\n\n\n\x03\x04\
    \x03\x01\x12\x03\x17\x08!\n\x0b\n\x04\x04\x03\x02\0\x12\x03\x18\x02\x1f\
    \n\x0c\n\x05\x04\x03\x02\0\x04\x12\x03\x18\x02\n\n\x0c\n\x05\x04\x03\x02\
    \0\x05\x12\x03\x18\x0b\x0f\n\x0c\n\x05\x04\x03\x02\0\x01\x12\x03\x18\x10\
    \x1a\n\x0c\n\x05\x04\x03\x02\0\x03\x12\x03\x18\x1d\x1e\n\x14\n\x02\x04\
    \x04\x12\x04\x1c\0!\x01\x1a\x08\x20Origin\n\n\n\n\x03\x04\x04\x01\x12\
    \x03\x1c\x08\x0e\n\x0b\n\x04\x04\x04\x02\0\x12\x03\x1d\x02\x19\n\x0c\n\
    \x05\x04\x04\x02\0\x04\x12\x03\x1d\x02\n\n\x0c\n\x05\x04\x04\x02\0\x05\
    \x12\x03\x1d\x0b\x11\n\x0c\n\x05\x04\x04\x02\0\x01\x12\x03\x1d\x12\x14\n\
    \x0c\n\x05\x04\x04\x02\0\x03\x12\x03\x1d\x17\x18\n\x0b\n\x04\x04\x04\x02\
    \x01\x12\x03\x1e\x02\x1b\n\x0c\n\x05\x04\x04\x02\x01\x04\x12\x03\x1e\x02\
    \n\n\x0c\n\x05\x04\x04\x02\x01\x05\x12\x03\x1e\x0b\x11\n\x0c\n\x05\x04\
    \x04\x02\x01\x01\x12\x03\x1e\x12\x16\n\x0c\n\x05\x04\x04\x02\x01\x03\x12\
    \x03\x1e\x19\x1a\n\x0b\n\x04\x04\x04\x02\x02\x12\x03\x1f\x02\x1f\n\x0c\n\
    \x05\x04\x04\x02\x02\x04\x12\x03\x1f\x02\n\n\x0c\n\x05\x04\x04\x02\x02\
    \x05\x12\x03\x1f\x0b\x11\n\x0c\n\x05\x04\x04\x02\x02\x01\x12\x03\x1f\x12\
    \x1a\n\x0c\n\x05\x04\x04\x02\x02\x03\x12\x03\x1f\x1d\x1e\n\x0b\n\x04\x04\
    \x04\x02\x03\x12\x03\x20\x02'\n\x0c\n\x05\x04\x04\x02\x03\x04\x12\x03\
    \x20\x02\n\n\x0c\n\x05\x04\x04\x02\x03\x05\x12\x03\x20\x0b\x11\n\x0c\n\
    \x05\x04\x04\x02\x03\x01\x12\x03\x20\x12\"\n\x0c\n\x05\x04\x04\x02\x03\
    \x03\x12\x03\x20%&\n\n\n\x02\x04\x05\x12\x04#\0'\x01\n\n\n\x03\x04\x05\
    \x01\x12\x03#\x08\x14\n\x0b\n\x04\x04\x05\x02\0\x12\x03$\x02\x1b\n\x0c\n\
    \x05\x04\x05\x02\0\x04\x12\x03$\x02\n\n\x0c\n\x05\x04\x05\x02\0\x05\x12\
    \x03$\x0b\x11\n\x0c\n\x05\x04\x05\x02\0\x01\x12\x03$\x12\x16\n\x0c\n\x05\
    \x04\x05\x02\0\x03\x12\x03$\x19\x1a\n\x0b\n\x04\x04\x05\x02\x01\x12\x03%\
    \x02\x1f\n\x0c\n\x05\x04\x05\x02\x01\x04\x12\x03%\x02\n\n\x0c\n\x05\x04\
    \x05\x02\x01\x05\x12\x03%\x0b\x11\n\x0c\n\x05\x04\x05\x02\x01\x01\x12\
    \x03%\x12\x1a\n\x0c\n\x05\x04\x05\x02\x01\x03\x12\x03%\x1d\x1e\n\x0b\n\
    \x04\x04\x05\x02\x02\x12\x03&\x02!\n\x0c\n\x05\x04\x05\x02\x02\x04\x12\
    \x03&\x02\n\n\x0c\n\x05\x04\x05\x02\x02\x05\x12\x03&\x0b\x11\n\x0c\n\x05\
    \x04\x05\x02\x02\x01\x12\x03&\x12\x1c\n\x0c\n\x05\x04\x05\x02\x02\x03\
    \x12\x03&\x1f\x20\n\n\n\x02\x04\x06\x12\x04)\0+\x01\n\n\n\x03\x04\x06\
    \x01\x12\x03)\x08\x14\n\x0b\n\x04\x04\x06\x02\0\x12\x03*\x02\x1b\n\x0c\n\
    \x05\x04\x06\x02\0\x04\x12\x03*\x02\n\n\x0c\n\x05\x04\x06\x02\0\x05\x12\
    \x03*\x0b\x11\n\x0c\n\x05\x04\x06\x02\0\x01\x12\x03*\x12\x16\n\x0c\n\x05\
    \x04\x06\x02\0\x03\x12\x03*\x19\x1a\n\n\n\x02\x04\x07\x12\x04-\0/\x01\n\
    \n\n\x03\x04\x07\x01\x12\x03-\x08\x11\n\x0b\n\x04\x04\x07\x02\0\x12\x03.\
    \x02\x1b\n\x0c\n\x05\x04\x07\x02\0\x04\x12\x03.\x02\n\n\x0c\n\x05\x04\
    \x07\x02\0\x05\x12\x03.\x0b\x11\n\x0c\n\x05\x04\x07\x02\0\x01\x12\x03.\
    \x12\x16\n\x0c\n\x05\x04\x07\x02\0\x03\x12\x03.\x19\x1a\n\x1c\n\x02\x04\
    \x08\x12\x042\07\x01\x1a\x10\x20Origin\x20Channel\n\n\n\n\x03\x04\x08\
    \x01\x12\x032\x08\x15\n\x0b\n\x04\x04\x08\x02\0\x12\x033\x02\x19\n\x0c\n\
    \x05\x04\x08\x02\0\x04\x12\x033\x02\n\n\x0c\n\x05\x04\x08\x02\0\x05\x12\
    \x033\x0b\x11\n\x0c\n\x05\x04\x08\x02\0\x01\x12\x033\x12\x14\n\x0c\n\x05\
    \x04\x08\x02\0\x03\x12\x033\x17\x18\n\x0b\n\x04\x04\x08\x02\x01\x12\x034\
    \x02\x20\n\x0c\n\x05\x04\x08\x02\x01\x04\x12\x034\x02\n\n\x0c\n\x05\x04\
    \x08\x02\x01\x05\x12\x034\x0b\x11\n\x0c\n\x05\x04\x08\x02\x01\x01\x12\
    \x034\x12\x1b\n\x0c\n\x05\x04\x08\x02\x01\x03\x12\x034\x1e\x1f\n\x0b\n\
    \x04\x04\x08\x02\x02\x12\x035\x02\x1b\n\x0c\n\x05\x04\x08\x02\x02\x04\
    \x12\x035\x02\n\n\x0c\n\x05\x04\x08\x02\x02\x05\x12\x035\x0b\x11\n\x0c\n\
    \x05\x04\x08\x02\x02\x01\x12\x035\x12\x16\n\x0c\n\x05\x04\x08\x02\x02\
    \x03\x12\x035\x19\x1a\n\x0b\n\x04\x04\x08\x02\x03\x12\x036\x02\x1f\n\x0c\
    \n\x05\x04\x08\x02\x03\x04\x12\x036\x02\n\n\x0c\n\x05\x04\x08\x02\x03\
    \x05\x12\x036\x0b\x11\n\x0c\n\x05\x04\x08\x02\x03\x01\x12\x036\x12\x1a\n\
    \x0c\n\x05\x04\x08\x02\x03\x03\x12\x036\x1d\x1e\n\n\n\x02\x04\t\x12\x049\
    \0<\x01\n\n\n\x03\x04\t\x01\x12\x039\x08\x1a\n\x0b\n\x04\x04\t\x02\0\x12\
    \x03:\x02\x1d\n\x0c\n\x05\x04\t\x02\0\x04\x12\x03:\x02\n\n\x0c\n\x05\x04\
    \t\x02\0\x05\x12\x03:\x0b\x11\n\x0c\n\x05\x04\t\x02\0\x01\x12\x03:\x12\
    \x18\n\x0c\n\x05\x04\t\x02\0\x03\x12\x03:\x1b\x1c\n\x0b\n\x04\x04\t\x02\
    \x01\x12\x03;\x02\x1b\n\x0c\n\x05\x04\t\x02\x01\x04\x12\x03;\x02\n\n\x0c\
    \n\x05\x04\t\x02\x01\x05\x12\x03;\x0b\x11\n\x0c\n\x05\x04\t\x02\x01\x01\
    \x12\x03;\x12\x16\n\x0c\n\x05\x04\t\x02\x01\x03\x12\x03;\x19\x1a\n\n\n\
    \x02\x04\n\x12\x04>\0C\x01\n\n\n\x03\x04\n\x01\x12\x03>\x08\x1b\n\x0b\n\
    \x04\x04\n\x02\0\x12\x03?\x02\x20\n\x0c\n\x05\x04\n\x02\0\x04\x12\x03?\
    \x02\n\n\x0c\n\x05\x04\n\x02\0\x05\x12\x03?\x0b\x11\n\x0c\n\x05\x04\n\
    \x02\0\x01\x12\x03?\x12\x1b\n\x0c\n\x05\x04\n\x02\0\x03\x12\x03?\x1e\x1f\
    \n\x0b\n\x04\x04\n\x02\x01\x12\x03@\x02\"\n\x0c\n\x05\x04\n\x02\x01\x04\
    \x12\x03@\x02\n\n\x0c\n\x05\x04\n\x02\x01\x05\x12\x03@\x0b\x11\n\x0c\n\
    \x05\x04\n\x02\x01\x01\x12\x03@\x12\x1d\n\x0c\n\x05\x04\n\x02\x01\x03\
    \x12\x03@\x20!\n\x0b\n\x04\x04\n\x02\x02\x12\x03A\x02\x1b\n\x0c\n\x05\
    \x04\n\x02\x02\x04\x12\x03A\x02\n\n\x0c\n\x05\x04\n\x02\x02\x05\x12\x03A\
    \x0b\x11\n\x0c\n\x05\x04\n\x02\x02\x01\x12\x03A\x12\x16\n\x0c\n\x05\x04\
    \n\x02\x02\x03\x12\x03A\x19\x1a\n\x0b\n\x04\x04\n\x02\x03\x12\x03B\x02\
    \x1f\n\x0c\n\x05\x04\n\x02\x03\x04\x12\x03B\x02\n\n\x0c\n\x05\x04\n\x02\
    \x03\x05\x12\x03B\x0b\x11\n\x0c\n\x05\x04\n\x02\x03\x01\x12\x03B\x12\x1a\
    \n\x0c\n\x05\x04\n\x02\x03\x03\x12\x03B\x1d\x1e\n\n\n\x02\x04\x0b\x12\
    \x04E\0H\x01\n\n\n\x03\x04\x0b\x01\x12\x03E\x08\x18\n\x0b\n\x04\x04\x0b\
    \x02\0\x12\x03F\x02\"\n\x0c\n\x05\x04\x0b\x02\0\x04\x12\x03F\x02\n\n\x0c\
    \n\x05\x04\x0b\x02\0\x05\x12\x03F\x0b\x11\n\x0c\n\x05\x04\x0b\x02\0\x01\
    \x12\x03F\x12\x1d\n\x0c\n\x05\x04\x0b\x02\0\x03\x12\x03F\x20!\n\x0b\n\
    \x04\x04\x0b\x02\x01\x12\x03G\x02\x1b\n\x0c\n\x05\x04\x0b\x02\x01\x04\
    \x12\x03G\x02\n\n\x0c\n\x05\x04\x0b\x02\x01\x05\x12\x03G\x0b\x11\n\x0c\n\
    \x05\x04\x0b\x02\x01\x01\x12\x03G\x12\x16\n\x0c\n\x05\x04\x0b\x02\x01\
    \x03\x12\x03G\x19\x1a\n\n\n\x02\x04\x0c\x12\x04J\0L\x01\n\n\n\x03\x04\
    \x0c\x01\x12\x03J\x08\x20\n\x0b\n\x04\x04\x0c\x02\0\x12\x03K\x02\x20\n\
    \x0c\n\x05\x04\x0c\x02\0\x04\x12\x03K\x02\n\n\x0c\n\x05\x04\x0c\x02\0\
    \x05\x12\x03K\x0b\x11\n\x0c\n\x05\x04\x0c\x02\0\x01\x12\x03K\x12\x1b\n\
    \x0c\n\x05\x04\x0c\x02\0\x03\x12\x03K\x1e\x1f\n\n\n\x02\x04\r\x12\x04N\0\
    Q\x01\n\n\n\x03\x04\r\x01\x12\x03N\x08!\n\x0b\n\x04\x04\r\x02\0\x12\x03O\
    \x02\x20\n\x0c\n\x05\x04\r\x02\0\x04\x12\x03O\x02\n\n\x0c\n\x05\x04\r\
    \x02\0\x05\x12\x03O\x0b\x11\n\x0c\n\x05\x04\r\x02\0\x01\x12\x03O\x12\x1b\
    \n\x0c\n\x05\x04\r\x02\0\x03\x12\x03O\x1e\x1f\n\x0b\n\x04\x04\r\x02\x01\
    \x12\x03P\x02&\n\x0c\n\x05\x04\r\x02\x01\x04\x12\x03P\x02\n\n\x0c\n\x05\
    \x04\r\x02\x01\x06\x12\x03P\x0b\x18\n\x0c\n\x05\x04\r\x02\x01\x01\x12\
    \x03P\x19!\n\x0c\n\x05\x04\r\x02\x01\x03\x12\x03P$%\n\n\n\x02\x04\x0e\
    \x12\x04S\0V\x01\n\n\n\x03\x04\x0e\x01\x12\x03S\x08\x1f\n\x0b\n\x04\x04\
    \x0e\x02\0\x12\x03T\x02\x1b\n\x0c\n\x05\x04\x0e\x02\0\x04\x12\x03T\x02\n\
    \n\x0c\n\x05\x04\x0e\x02\0\x05\x12\x03T\x0b\x11\n\x0c\n\x05\x04\x0e\x02\
    \0\x01\x12\x03T\x12\x16\n\x0c\n\x05\x04\x0e\x02\0\x03\x12\x03T\x19\x1a\n\
    \x0b\n\x04\x04\x0e\x02\x01\x12\x03U\x02(\n\x0c\n\x05\x04\x0e\x02\x01\x04\
    \x12\x03U\x02\n\n\x0c\n\x05\x04\x0e\x02\x01\x06\x12\x03U\x0b\x1d\n\x0c\n\
    \x05\x04\x0e\x02\x01\x01\x12\x03U\x1e#\n\x0c\n\x05\x04\x0e\x02\x01\x03\
    \x12\x03U&'\n\n\n\x02\x04\x0f\x12\x04X\0\\\x01\n\n\n\x03\x04\x0f\x01\x12\
    \x03X\x08%\n\x0b\n\x04\x04\x0f\x02\0\x12\x03Y\x02\x1b\n\x0c\n\x05\x04\
    \x0f\x02\0\x04\x12\x03Y\x02\n\n\x0c\n\x05\x04\x0f\x02\0\x05\x12\x03Y\x0b\
    \x11\n\x0c\n\x05\x04\x0f\x02\0\x01\x12\x03Y\x12\x16\n\x0c\n\x05\x04\x0f\
    \x02\0\x03\x12\x03Y\x19\x1a\n\x0b\n\x04\x04\x0f\x02\x01\x12\x03Z\x02(\n\
    \x0c\n\x05\x04\x0f\x02\x01\x04\x12\x03Z\x02\n\n\x0c\n\x05\x04\x0f\x02\
    \x01\x06\x12\x03Z\x0b\x1d\n\x0c\n\x05\x04\x0f\x02\x01\x01\x12\x03Z\x1e#\
    \n\x0c\n\x05\x04\x0f\x02\x01\x03\x12\x03Z&'\n\x0b\n\x04\x04\x0f\x02\x02\
    \x12\x03[\x02\x1d\n\x0c\n\x05\x04\x0f\x02\x02\x04\x12\x03[\x02\n\n\x0c\n\
    \x05\x04\x0f\x02\x02\x05\x12\x03[\x0b\x11\n\x0c\n\x05\x04\x0f\x02\x02\
    \x01\x12\x03[\x12\x18\n\x0c\n\x05\x04\x0f\x02\x02\x03\x12\x03[\x1b\x1c\n\
    \n\n\x02\x04\x10\x12\x04^\0c\x01\n\n\n\x03\x04\x10\x01\x12\x03^\x08'\n\
    \x0b\n\x04\x04\x10\x02\0\x12\x03_\x02\x1b\n\x0c\n\x05\x04\x10\x02\0\x04\
    \x12\x03_\x02\n\n\x0c\n\x05\x04\x10\x02\0\x05\x12\x03_\x0b\x11\n\x0c\n\
    \x05\x04\x10\x02\0\x01\x12\x03_\x12\x16\n\x0c\n\x05\x04\x10\x02\0\x03\
    \x12\x03_\x19\x1a\n\x0b\n\x04\x04\x10\x02\x01\x12\x03`\x02(\n\x0c\n\x05\
    \x04\x10\x02\x01\x04\x12\x03`\x02\n\n\x0c\n\x05\x04\x10\x02\x01\x06\x12\
    \x03`\x0b\x1d\n\x0c\n\x05\x04\x10\x02\x01\x01\x12\x03`\x1e#\n\x0c\n\x05\
    \x04\x10\x02\x01\x03\x12\x03`&'\n\x0b\n\x04\x04\x10\x02\x02\x12\x03a\x02\
    \x1c\n\x0c\n\x05\x04\x10\x02\x02\x04\x12\x03a\x02\n\n\x0c\n\x05\x04\x10\
    \x02\x02\x05\x12\x03a\x0b\x11\n\x0c\n\x05\x04\x10\x02\x02\x01\x12\x03a\
    \x12\x17\n\x0c\n\x05\x04\x10\x02\x02\x03\x12\x03a\x1a\x1b\n\x0b\n\x04\
    \x04\x10\x02\x03\x12\x03b\x02\x1b\n\x0c\n\x05\x04\x10\x02\x03\x04\x12\
    \x03b\x02\n\n\x0c\n\x05\x04\x10\x02\x03\x05\x12\x03b\x0b\x11\n\x0c\n\x05\
    \x04\x10\x02\x03\x01\x12\x03b\x12\x16\n\x0c\n\x05\x04\x10\x02\x03\x03\
    \x12\x03b\x19\x1a\n\n\n\x02\x04\x11\x12\x04e\0h\x01\n\n\n\x03\x04\x11\
    \x01\x12\x03e\x08\x1b\n\x0b\n\x04\x04\x11\x02\0\x12\x03f\x02\x19\n\x0c\n\
    \x05\x04\x11\x02\0\x04\x12\x03f\x02\n\n\x0c\n\x05\x04\x11\x02\0\x05\x12\
    \x03f\x0b\x11\n\x0c\n\x05\x04\x11\x02\0\x01\x12\x03f\x12\x14\n\x0c\n\x05\
    \x04\x11\x02\0\x03\x12\x03f\x17\x18\n\x0b\n\x04\x04\x11\x02\x01\x12\x03g\
    \x02\x20\n\x0c\n\x05\x04\x11\x02\x01\x04\x12\x03g\x02\n\n\x0c\n\x05\x04\
    \x11\x02\x01\x05\x12\x03g\x0b\x11\n\x0c\n\x05\x04\x11\x02\x01\x01\x12\
    \x03g\x12\x1b\n\x0c\n\x05\x04\x11\x02\x01\x03\x12\x03g\x1e\x1f\n\x1f\n\
    \x02\x04\x12\x12\x04k\0r\x01\x1a\x13\x20Origin\x20Invitation\n\n\n\n\x03\
    \x04\x12\x01\x12\x03k\x08\x18\n\x0b\n\x04\x04\x12\x02\0\x12\x03l\x02\x19\
    \n\x0c\n\x05\x04\x12\x02\0\x04\x12\x03l\x02\n\n\x0c\n\x05\x04\x12\x02\0\
    \x05\x12\x03l\x0b\x11\n\x0c\n\x05\x04\x12\x02\0\x01\x12\x03l\x12\x14\n\
    \x0c\n\x05\x04\x12\x02\0\x03\x12\x03l\x17\x18\n\x0b\n\x04\x04\x12\x02\
    \x01\x12\x03m\x02!\n\x0c\n\x05\x04\x12\x02\x01\x04\x12\x03m\x02\n\n\x0c\
    \n\x05\x04\x12\x02\x01\x05\x12\x03m\x0b\x11\n\x0c\n\x05\x04\x12\x02\x01\
    \x01\x12\x03m\x12\x1c\n\x0c\n\x05\x04\x12\x02\x01\x03\x12\x03m\x1f\x20\n\
    \x0b\n\x04\x04\x12\x02\x02\x12\x03n\x02#\n\x0c\n\x05\x04\x12\x02\x02\x04\
    \x12\x03n\x02\n\n\x0c\n\x05\x04\x12\x02\x02\x05\x12\x03n\x0b\x11\n\x0c\n\
    \x05\x04\x12\x02\x02\x01\x12\x03n\x12\x1e\n\x0c\n\x05\x04\x12\x02\x02\
    \x03\x12\x03n!\"\n\x0b\n\x04\x04\x12\x02\x03\x12\x03o\x02\x20\n\x0c\n\
    \x05\x04\x12\x02\x03\x04\x12\x03o\x02\n\n\x0c\n\x05\x04\x12\x02\x03\x05\
    \x12\x03o\x0b\x11\n\x0c\n\x05\x04\x12\x02\x03\x01\x12\x03o\x12\x1b\n\x0c\
    \n\x05\x04\x12\x02\x03\x03\x12\x03o\x1e\x1f\n\x0b\n\x04\x04\x12\x02\x04\
    \x12\x03p\x02\"\n\x0c\n\x05\x04\x12\x02\x04\x04\x12\x03p\x02\n\n\x0c\n\
    \x05\x04\x12\x02\x04\x05\x12\x03p\x0b\x11\n\x0c\n\x05\x04\x12\x02\x04\
    \x01\x12\x03p\x12\x1d\n\x0c\n\x05\x04\x12\x02\x04\x03\x12\x03p\x20!\n\
    \x0b\n\x04\x04\x12\x02\x05\x12\x03q\x02\x1f\n\x0c\n\x05\x04\x12\x02\x05\
    \x04\x12\x03q\x02\n\n\x0c\n\x05\x04\x12\x02\x05\x05\x12\x03q\x0b\x11\n\
    \x0c\n\x05\x04\x12\x02\x05\x01\x12\x03q\x12\x1a\n\x0c\n\x05\x04\x12\x02\
    \x05\x03\x12\x03q\x1d\x1e\n\n\n\x02\x04\x13\x12\x04t\0y\x01\n\n\n\x03\
    \x04\x13\x01\x12\x03t\x08%\n\x0b\n\x04\x04\x13\x02\0\x12\x03u\x02!\n\x0c\
    \n\x05\x04\x13\x02\0\x04\x12\x03u\x02\n\n\x0c\n\x05\x04\x13\x02\0\x05\
    \x12\x03u\x0b\x11\n\x0c\n\x05\x04\x13\x02\0\x01\x12\x03u\x12\x1c\n\x0c\n\
    \x05\x04\x13\x02\0\x03\x12\x03u\x1f\x20\n\x0b\n\x04\x04\x13\x02\x01\x12\
    \x03v\x02\x20\n\x0c\n\x05\x04\x13\x02\x01\x04\x12\x03v\x02\n\n\x0c\n\x05\
    \x04\x13\x02\x01\x05\x12\x03v\x0b\x11\n\x0c\n\x05\x04\x13\x02\x01\x01\
    \x12\x03v\x12\x1b\n\x0c\n\x05\x04\x13\x02\x01\x03\x12\x03v\x1e\x1f\n\x0b\
    \n\x04\x04\x13\x02\x02\x12\x03w\x02\"\n\x0c\n\x05\x04\x13\x02\x02\x04\
    \x12\x03w\x02\n\n\x0c\n\x05\x04\x13\x02\x02\x05\x12\x03w\x0b\x11\n\x0c\n\
    \x05\x04\x13\x02\x02\x01\x12\x03w\x12\x1d\n\x0c\n\x05\x04\x13\x02\x02\
    \x03\x12\x03w\x20!\n\x0b\n\x04\x04\x13\x02\x03\x12\x03x\x02\x1b\n\x0c\n\
    \x05\x04\x13\x02\x03\x04\x12\x03x\x02\n\n\x0c\n\x05\x04\x13\x02\x03\x05\
    \x12\x03x\x0b\x0f\n\x0c\n\x05\x04\x13\x02\x03\x01\x12\x03x\x10\x16\n\x0c\
    \n\x05\x04\x13\x02\x03\x03\x12\x03x\x19\x1a\n\x0b\n\x02\x04\x14\x12\x05{\
    \0\x81\x01\x01\n\n\n\x03\x04\x14\x01\x12\x03{\x08\x1e\n\x0b\n\x04\x04\
    \x14\x02\0\x12\x03|\x02!\n\x0c\n\x05\x04\x14\x02\0\x04\x12\x03|\x02\n\n\
    \x0c\n\x05\x04\x14\x02\0\x05\x12\x03|\x0b\x11\n\x0c\n\x05\x04\x14\x02\0\
    \x01\x12\x03|\x12\x1c\n\x0c\n\x05\x04\x14\x02\0\x03\x12\x03|\x1f\x20\n\
    \x0b\n\x04\x04\x14\x02\x01\x12\x03}\x02#\n\x0c\n\x05\x04\x14\x02\x01\x04\
    \x12\x03}\x02\n\n\x0c\n\x05\x04\x14\x02\x01\x05\x12\x03}\x0b\x11\n\x0c\n\
    \x05\x04\x14\x02\x01\x01\x12\x03}\x12\x1e\n\x0c\n\x05\x04\x14\x02\x01\
    \x03\x12\x03}!\"\n\x0b\n\x04\x04\x14\x02\x02\x12\x03~\x02\x20\n\x0c\n\
    \x05\x04\x14\x02\x02\x04\x12\x03~\x02\n\n\x0c\n\x05\x04\x14\x02\x02\x05\
    \x12\x03~\x0b\x11\n\x0c\n\x05\x04\x14\x02\x02\x01\x12\x03~\x12\x1b\n\x0c\
    \n\x05\x04\x14\x02\x02\x03\x12\x03~\x1e\x1f\n\x0b\n\x04\x04\x14\x02\x03\
    \x12\x03\x7f\x02\"\n\x0c\n\x05\x04\x14\x02\x03\x04\x12\x03\x7f\x02\n\n\
    \x0c\n\x05\x04\x14\x02\x03\x05\x12\x03\x7f\x0b\x11\n\x0c\n\x05\x04\x14\
    \x02\x03\x01\x12\x03\x7f\x12\x1d\n\x0c\n\x05\x04\x14\x02\x03\x03\x12\x03\
    \x7f\x20!\n\x0c\n\x04\x04\x14\x02\x04\x12\x04\x80\x01\x02\x1f\n\r\n\x05\
    \x04\x14\x02\x04\x04\x12\x04\x80\x01\x02\n\n\r\n\x05\x04\x14\x02\x04\x05\
    \x12\x04\x80\x01\x0b\x11\n\r\n\x05\x04\x14\x02\x04\x01\x12\x04\x80\x01\
    \x12\x1a\n\r\n\x05\x04\x14\x02\x04\x03\x12\x04\x80\x01\x1d\x1e\n\x0c\n\
    \x02\x04\x15\x12\x06\x83\x01\0\x85\x01\x01\n\x0b\n\x03\x04\x15\x01\x12\
    \x04\x83\x01\x08#\n\x0c\n\x04\x04\x15\x02\0\x12\x04\x84\x01\x02\x20\n\r\
    \n\x05\x04\x15\x02\0\x04\x12\x04\x84\x01\x02\n\n\r\n\x05\x04\x15\x02\0\
    \x05\x12\x04\x84\x01\x0b\x11\n\r\n\x05\x04\x15\x02\0\x01\x12\x04\x84\x01\
    \x12\x1b\n\r\n\x05\x04\x15\x02\0\x03\x12\x04\x84\x01\x1e\x1f\n\x0c\n\x02\
    \x04\x16\x12\x06\x87\x01\0\x8a\x01\x01\n\x0b\n\x03\x04\x16\x01\x12\x04\
    \x87\x01\x08$\n\x0c\n\x04\x04\x16\x02\0\x12\x04\x88\x01\x02\x20\n\r\n\
    \x05\x04\x16\x02\0\x04\x12\x04\x88\x01\x02\n\n\r\n\x05\x04\x16\x02\0\x05\
    \x12\x04\x88\x01\x0b\x11\n\r\n\x05\x04\x16\x02\0\x01\x12\x04\x88\x01\x12\
    \x1b\n\r\n\x05\x04\x16\x02\0\x03\x12\x04\x88\x01\x1e\x1f\n\x0c\n\x04\x04\
    \x16\x02\x01\x12\x04\x89\x01\x02,\n\r\n\x05\x04\x16\x02\x01\x04\x12\x04\
    \x89\x01\x02\n\n\r\n\x05\x04\x16\x02\x01\x06\x12\x04\x89\x01\x0b\x1b\n\r\
    \n\x05\x04\x16\x02\x01\x01\x12\x04\x89\x01\x1c'\n\r\n\x05\x04\x16\x02\
    \x01\x03\x12\x04\x89\x01*+\n\x0c\n\x02\x04\x17\x12\x06\x8c\x01\0\x90\x01\
    \x01\n\x0b\n\x03\x04\x17\x01\x12\x04\x8c\x01\x08\x16\n\x0c\n\x04\x04\x17\
    \x02\0\x12\x04\x8d\x01\x02\x1d\n\r\n\x05\x04\x17\x02\0\x04\x12\x04\x8d\
    \x01\x02\n\n\r\n\x05\x04\x17\x02\0\x05\x12\x04\x8d\x01\x0b\x11\n\r\n\x05\
    \x04\x17\x02\0\x01\x12\x04\x8d\x01\x12\x18\n\r\n\x05\x04\x17\x02\0\x03\
    \x12\x04\x8d\x01\x1b\x1c\n\x0c\n\x04\x04\x17\x02\x01\x12\x04\x8e\x01\x02\
    \x1f\n\r\n\x05\x04\x17\x02\x01\x04\x12\x04\x8e\x01\x02\n\n\r\n\x05\x04\
    \x17\x02\x01\x05\x12\x04\x8e\x01\x0b\x11\n\r\n\x05\x04\x17\x02\x01\x01\
    \x12\x04\x8e\x01\x12\x1a\n\r\n\x05\x04\x17\x02\x01\x03\x12\x04\x8e\x01\
    \x1d\x1e\n\x0c\n\x04\x04\x17\x02\x02\x12\x04\x8f\x01\x02\x1f\n\r\n\x05\
    \x04\x17\x02\x02\x04\x12\x04\x8f\x01\x02\n\n\r\n\x05\x04\x17\x02\x02\x05\
    \x12\x04\x8f\x01\x0b\x11\n\r\n\x05\x04\x17\x02\x02\x01\x12\x04\x8f\x01\
    \x12\x1a\n\r\n\x05\x04\x17\x02\x02\x03\x12\x04\x8f\x01\x1d\x1e\n\x1d\n\
    \x02\x04\x18\x12\x06\x93\x01\0\x95\x01\x01\x1a\x0f\x20Origin\x20Member\n\
    \n\x0b\n\x03\x04\x18\x01\x12\x04\x93\x01\x08\x1f\n\x0c\n\x04\x04\x18\x02\
    \0\x12\x04\x94\x01\x02\x20\n\r\n\x05\x04\x18\x02\0\x04\x12\x04\x94\x01\
    \x02\n\n\r\n\x05\x04\x18\x02\0\x05\x12\x04\x94\x01\x0b\x11\n\r\n\x05\x04\
    \x18\x02\0\x01\x12\x04\x94\x01\x12\x1b\n\r\n\x05\x04\x18\x02\0\x03\x12\
    \x04\x94\x01\x1e\x1f\n\x0c\n\x02\x04\x19\x12\x06\x97\x01\0\x9a\x01\x01\n\
    \x0b\n\x03\x04\x19\x01\x12\x04\x97\x01\x08\x20\n\x0c\n\x04\x04\x19\x02\0\
    \x12\x04\x98\x01\x02\x20\n\r\n\x05\x04\x19\x02\0\x04\x12\x04\x98\x01\x02\
    \n\n\r\n\x05\x04\x19\x02\0\x05\x12\x04\x98\x01\x0b\x11\n\r\n\x05\x04\x19\
    \x02\0\x01\x12\x04\x98\x01\x12\x1b\n\r\n\x05\x04\x19\x02\0\x03\x12\x04\
    \x98\x01\x1e\x1f\n\x0c\n\x04\x04\x19\x02\x01\x12\x04\x99\x01\x02\x1e\n\r\
    \n\x05\x04\x19\x02\x01\x04\x12\x04\x99\x01\x02\n\n\r\n\x05\x04\x19\x02\
    \x01\x05\x12\x04\x99\x01\x0b\x11\n\r\n\x05\x04\x19\x02\x01\x01\x12\x04\
    \x99\x01\x12\x19\n\r\n\x05\x04\x19\x02\x01\x03\x12\x04\x99\x01\x1c\x1d\n\
    \x0c\n\x02\x04\x1a\x12\x06\x9c\x01\0\x9f\x01\x01\n\x0b\n\x03\x04\x1a\x01\
    \x12\x04\x9c\x01\x08\x1a\n\x0c\n\x04\x04\x1a\x02\0\x12\x04\x9d\x01\x02\
    \x20\n\r\n\x05\x04\x1a\x02\0\x04\x12\x04\x9d\x01\x02\n\n\r\n\x05\x04\x1a\
    \x02\0\x05\x12\x04\x9d\x01\x0b\x11\n\r\n\x05\x04\x1a\x02\0\x01\x12\x04\
    \x9d\x01\x12\x1b\n\r\n\x05\x04\x1a\x02\0\x03\x12\x04\x9d\x01\x1e\x1f\n\
    \x0c\n\x04\x04\x1a\x02\x01\x12\x04\x9e\x01\x02\x1e\n\r\n\x05\x04\x1a\x02\
    \x01\x04\x12\x04\x9e\x01\x02\n\n\r\n\x05\x04\x1a\x02\x01\x05\x12\x04\x9e\
    \x01\x0b\x11\n\r\n\x05\x04\x1a\x02\x01\x01\x12\x04\x9e\x01\x12\x19\n\r\n\
    \x05\x04\x1a\x02\x01\x03\x12\x04\x9e\x01\x1c\x1d\n\x1e\n\x02\x04\x1b\x12\
    \x06\xa2\x01\0\xae\x01\x01\x1a\x10\x20Origin\x20Package\n\n\x0b\n\x03\
    \x04\x1b\x01\x12\x04\xa2\x01\x08\x15\n\x0c\n\x04\x04\x1b\x02\0\x12\x04\
    \xa3\x01\x02\x19\n\r\n\x05\x04\x1b\x02\0\x04\x12\x04\xa3\x01\x02\n\n\r\n\
    \x05\x04\x1b\x02\0\x05\x12\x04\xa3\x01\x0b\x11\n\r\n\x05\x04\x1b\x02\0\
    \x01\x12\x04\xa3\x01\x12\x14\n\r\n\x05\x04\x1b\x02\0\x03\x12\x04\xa3\x01\
    \x17\x18\n\x0c\n\x04\x04\x1b\x02\x01\x12\x04\xa4\x01\x02\x1f\n\r\n\x05\
    \x04\x1b\x02\x01\x04\x12\x04\xa4\x01\x02\n\n\r\n\x05\x04\x1b\x02\x01\x05\
    \x12\x04\xa4\x01\x0b\x11\n\r\n\x05\x04\x1b\x02\x01\x01\x12\x04\xa4\x01\
    \x12\x1a\n\r\n\x05\x04\x1b\x02\x01\x03\x12\x04\xa4\x01\x1d\x1e\n\x0c\n\
    \x04\x04\x1b\x02\x02\x12\x04\xa5\x01\x02\x20\n\r\n\x05\x04\x1b\x02\x02\
    \x04\x12\x04\xa5\x01\x02\n\n\r\n\x05\x04\x1b\x02\x02\x05\x12\x04\xa5\x01\
    \x0b\x11\n\r\n\x05\x04\x1b\x02\x02\x01\x12\x04\xa5\x01\x12\x1b\n\r\n\x05\
    \x04\x1b\x02\x02\x03\x12\x04\xa5\x01\x1e\x1f\n\x0c\n\x04\x04\x1b\x02\x03\
    \x12\x04\xa6\x01\x02(\n\r\n\x05\x04\x1b\x02\x03\x04\x12\x04\xa6\x01\x02\
    \n\n\r\n\x05\x04\x1b\x02\x03\x06\x12\x04\xa6\x01\x0b\x1d\n\r\n\x05\x04\
    \x1b\x02\x03\x01\x12\x04\xa6\x01\x1e#\n\r\n\x05\x04\x1b\x02\x03\x03\x12\
    \x04\xa6\x01&'\n\x0c\n\x04\x04\x1b\x02\x04\x12\x04\xa7\x01\x02\x1f\n\r\n\
    \x05\x04\x1b\x02\x04\x04\x12\x04\xa7\x01\x02\n\n\r\n\x05\x04\x1b\x02\x04\
    \x05\x12\x04\xa7\x01\x0b\x11\n\r\n\x05\x04\x1b\x02\x04\x01\x12\x04\xa7\
    \x01\x12\x1a\n\r\n\x05\x04\x1b\x02\x04\x03\x12\x04\xa7\x01\x1d\x1e\n\x0c\
    \n\x04\x04\x1b\x02\x05\x12\x04\xa8\x01\x02\x1f\n\r\n\x05\x04\x1b\x02\x05\
    \x04\x12\x04\xa8\x01\x02\n\n\r\n\x05\x04\x1b\x02\x05\x05\x12\x04\xa8\x01\
    \x0b\x11\n\r\n\x05\x04\x1b\x02\x05\x01\x12\x04\xa8\x01\x12\x1a\n\r\n\x05\
    \x04\x1b\x02\x05\x03\x12\x04\xa8\x01\x1d\x1e\n\x0c\n\x04\x04\x1b\x02\x06\
    \x12\x04\xa9\x01\x02'\n\r\n\x05\x04\x1b\x02\x06\x04\x12\x04\xa9\x01\x02\
    \n\n\r\n\x05\x04\x1b\x02\x06\x06\x12\x04\xa9\x01\x0b\x1d\n\r\n\x05\x04\
    \x1b\x02\x06\x01\x12\x04\xa9\x01\x1e\"\n\r\n\x05\x04\x1b\x02\x06\x03\x12\
    \x04\xa9\x01%&\n\x0c\n\x04\x04\x1b\x02\x07\x12\x04\xaa\x01\x02(\n\r\n\
    \x05\x04\x1b\x02\x07\x04\x12\x04\xaa\x01\x02\n\n\r\n\x05\x04\x1b\x02\x07\
    \x06\x12\x04\xaa\x01\x0b\x1d\n\r\n\x05\x04\x1b\x02\x07\x01\x12\x04\xaa\
    \x01\x1e#\n\r\n\x05\x04\x1b\x02\x07\x03\x12\x04\xaa\x01&'\n\x0c\n\x04\
    \x04\x1b\x02\x08\x12\x04\xab\x01\x02,\n\r\n\x05\x04\x1b\x02\x08\x04\x12\
    \x04\xab\x01\x02\n\n\r\n\x05\x04\x1b\x02\x08\x05\x12\x04\xab\x01\x0b\x11\
    \n\r\n\x05\x04\x1b\x02\x08\x01\x12\x04\xab\x01\x12\x19\n\r\n\x05\x04\x1b\
    \x02\x08\x03\x12\x04\xab\x01\x1c\x1d\n\r\n\x05\x04\x1b\x02\x08\x08\x12\
    \x04\xab\x01\x1e+\n\x10\n\x08\x04\x1b\x02\x08\x08\xe7\x07\0\x12\x04\xab\
    \x01\x1f*\n\x11\n\t\x04\x1b\x02\x08\x08\xe7\x07\0\x02\x12\x04\xab\x01\
    \x1f%\n\x12\n\n\x04\x1b\x02\x08\x08\xe7\x07\0\x02\0\x12\x04\xab\x01\x1f%\
    \n\x13\n\x0b\x04\x1b\x02\x08\x08\xe7\x07\0\x02\0\x01\x12\x04\xab\x01\x1f\
    %\n\x11\n\t\x04\x1b\x02\x08\x08\xe7\x07\0\x03\x12\x04\xab\x01&*\n\x0c\n\
    \x04\x04\x1b\x02\t\x12\x04\xac\x01\x02\x1e\n\r\n\x05\x04\x1b\x02\t\x04\
    \x12\x04\xac\x01\x02\n\n\r\n\x05\x04\x1b\x02\t\x05\x12\x04\xac\x01\x0b\
    \x11\n\r\n\x05\x04\x1b\x02\t\x01\x12\x04\xac\x01\x12\x18\n\r\n\x05\x04\
    \x1b\x02\t\x03\x12\x04\xac\x01\x1b\x1d\n\x0c\n\x04\x04\x1b\x02\n\x12\x04\
    \xad\x01\x02\x1e\n\r\n\x05\x04\x1b\x02\n\x04\x12\x04\xad\x01\x02\n\n\r\n\
    \x05\x04\x1b\x02\n\x05\x12\x04\xad\x01\x0b\x11\n\r\n\x05\x04\x1b\x02\n\
    \x01\x12\x04\xad\x01\x12\x18\n\r\n\x05\x04\x1b\x02\n\x03\x12\x04\xad\x01\
    \x1b\x1d\n\x0c\n\x02\x04\x1c\x12\x06\xb0\x01\0\xb5\x01\x01\n\x0b\n\x03\
    \x04\x1c\x01\x12\x04\xb0\x01\x08\x1a\n\x0c\n\x04\x04\x1c\x02\0\x12\x04\
    \xb1\x01\x02\x1d\n\r\n\x05\x04\x1c\x02\0\x04\x12\x04\xb1\x01\x02\n\n\r\n\
    \x05\x04\x1c\x02\0\x05\x12\x04\xb1\x01\x0b\x11\n\r\n\x05\x04\x1c\x02\0\
    \x01\x12\x04\xb1\x01\x12\x18\n\r\n\x05\x04\x1c\x02\0\x03\x12\x04\xb1\x01\
    \x1b\x1c\n\x0c\n\x04\x04\x1c\x02\x01\x12\x04\xb2\x01\x02\x1b\n\r\n\x05\
    \x04\x1c\x02\x01\x04\x12\x04\xb2\x01\x02\n\n\r\n\x05\x04\x1c\x02\x01\x05\
    \x12\x04\xb2\x01\x0b\x11\n\r\n\x05\x04\x1c\x02\x01\x01\x12\x04\xb2\x01\
    \x12\x16\n\r\n\x05\x04\x1c\x02\x01\x03\x12\x04\xb2\x01\x19\x1a\n\x0c\n\
    \x04\x04\x1c\x02\x02\x12\x04\xb3\x01\x02\x1e\n\r\n\x05\x04\x1c\x02\x02\
    \x04\x12\x04\xb3\x01\x02\n\n\r\n\x05\x04\x1c\x02\x02\x05\x12\x04\xb3\x01\
    \x0b\x11\n\r\n\x05\x04\x1c\x02\x02\x01\x12\x04\xb3\x01\x12\x19\n\r\n\x05\
    \x04\x1c\x02\x02\x03\x12\x04\xb3\x01\x1c\x1d\n\x0c\n\x04\x04\x1c\x02\x03\
    \x12\x04\xb4\x01\x02\x1e\n\r\n\x05\x04\x1c\x02\x03\x04\x12\x04\xb4\x01\
    \x02\n\n\r\n\x05\x04\x1c\x02\x03\x05\x12\x04\xb4\x01\x0b\x11\n\r\n\x05\
    \x04\x1c\x02\x03\x01\x12\x04\xb4\x01\x12\x19\n\r\n\x05\x04\x1c\x02\x03\
    \x03\x12\x04\xb4\x01\x1c\x1d\n\x0c\n\x02\x04\x1d\x12\x06\xb7\x01\0\xbd\
    \x01\x01\n\x0b\n\x03\x04\x1d\x01\x12\x04\xb7\x01\x08\x1c\n\x0c\n\x04\x04\
    \x1d\x02\0\x12\x04\xb8\x01\x02\x1d\n\r\n\x05\x04\x1d\x02\0\x04\x12\x04\
    \xb8\x01\x02\n\n\r\n\x05\x04\x1d\x02\0\x05\x12\x04\xb8\x01\x0b\x11\n\r\n\
    \x05\x04\x1d\x02\0\x01\x12\x04\xb8\x01\x12\x18\n\r\n\x05\x04\x1d\x02\0\
    \x03\x12\x04\xb8\x01\x1b\x1c\n\x0c\n\x04\x04\x1d\x02\x01\x12\x04\xb9\x01\
    \x02\x1b\n\r\n\x05\x04\x1d\x02\x01\x04\x12\x04\xb9\x01\x02\n\n\r\n\x05\
    \x04\x1d\x02\x01\x05\x12\x04\xb9\x01\x0b\x11\n\r\n\x05\x04\x1d\x02\x01\
    \x01\x12\x04\xb9\x01\x12\x16\n\r\n\x05\x04\x1d\x02\x01\x03\x12\x04\xb9\
    \x01\x19\x1a\n\x0c\n\x04\x04\x1d\x02\x02\x12\x04\xba\x01\x02\x1e\n\r\n\
    \x05\x04\x1d\x02\x02\x04\x12\x04\xba\x01\x02\n\n\r\n\x05\x04\x1d\x02\x02\
    \x05\x12\x04\xba\x01\x0b\x11\n\r\n\x05\x04\x1d\x02\x02\x01\x12\x04\xba\
    \x01\x12\x19\n\r\n\x05\x04\x1d\x02\x02\x03\x12\x04\xba\x01\x1c\x1d\n\x0c\
    \n\x04\x04\x1d\x02\x03\x12\x04\xbb\x01\x02$\n\r\n\x05\x04\x1d\x02\x03\
    \x04\x12\x04\xbb\x01\x02\n\n\r\n\x05\x04\x1d\x02\x03\x05\x12\x04\xbb\x01\
    \x0b\x11\n\r\n\x05\x04\x1d\x02\x03\x01\x12\x04\xbb\x01\x12\x1f\n\r\n\x05\
    \x04\x1d\x02\x03\x03\x12\x04\xbb\x01\"#\n\x0c\n\x04\x04\x1d\x02\x04\x12\
    \x04\xbc\x01\x02\x1d\n\r\n\x05\x04\x1d\x02\x04\x04\x12\x04\xbc\x01\x02\n\
    \n\r\n\x05\x04\x1d\x02\x04\x05\x12\x04\xbc\x01\x0b\x11\n\r\n\x05\x04\x1d\
    \x02\x04\x01\x12\x04\xbc\x01\x12\x18\n\r\n\x05\x04\x1d\x02\x04\x03\x12\
    \x04\xbc\x01\x1b\x1c\n\x0c\n\x02\x04\x1e\x12\x06\xbf\x01\0\xca\x01\x01\n\
    \x0b\n\x03\x04\x1e\x01\x12\x04\xbf\x01\x08\x1b\n\x0c\n\x04\x04\x1e\x02\0\
    \x12\x04\xc0\x01\x02\x1f\n\r\n\x05\x04\x1e\x02\0\x04\x12\x04\xc0\x01\x02\
    \n\n\r\n\x05\x04\x1e\x02\0\x05\x12\x04\xc0\x01\x0b\x11\n\r\n\x05\x04\x1e\
    \x02\0\x01\x12\x04\xc0\x01\x12\x1a\n\r\n\x05\x04\x1e\x02\0\x03\x12\x04\
    \xc0\x01\x1d\x1e\n\x0c\n\x04\x04\x1e\x02\x01\x12\x04\xc1\x01\x02\x20\n\r\
    \n\x05\x04\x1e\x02\x01\x04\x12\x04\xc1\x01\x02\n\n\r\n\x05\x04\x1e\x02\
    \x01\x05\x12\x04\xc1\x01\x0b\x11\n\r\n\x05\x04\x1e\x02\x01\x01\x12\x04\
    \xc1\x01\x12\x1b\n\r\n\x05\x04\x1e\x02\x01\x03\x12\x04\xc1\x01\x1e\x1f\n\
    \x0c\n\x04\x04\x1e\x02\x02\x12\x04\xc2\x01\x02(\n\r\n\x05\x04\x1e\x02\
    \x02\x04\x12\x04\xc2\x01\x02\n\n\r\n\x05\x04\x1e\x02\x02\x06\x12\x04\xc2\
    \x01\x0b\x1d\n\r\n\x05\x04\x1e\x02\x02\x01\x12\x04\xc2\x01\x1e#\n\r\n\
    \x05\x04\x1e\x02\x02\x03\x12\x04\xc2\x01&'\n\x0c\n\x04\x04\x1e\x02\x03\
    \x12\x04\xc3\x01\x02\x1f\n\r\n\x05\x04\x1e\x02\x03\x04\x12\x04\xc3\x01\
    \x02\n\n\r\n\x05\x04\x1e\x02\x03\x05\x12\x04\xc3\x01\x0b\x11\n\r\n\x05\
    \x04\x1e\x02\x03\x01\x12\x04\xc3\x01\x12\x1a\n\r\n\x05\x04\x1e\x02\x03\
    \x03\x12\x04\xc3\x01\x1d\x1e\n\x0c\n\x04\x04\x1e\x02\x04\x12\x04\xc4\x01\
    \x02\x1f\n\r\n\x05\x04\x1e\x02\x04\x04\x12\x04\xc4\x01\x02\n\n\r\n\x05\
    \x04\x1e\x02\x04\x05\x12\x04\xc4\x01\x0b\x11\n\r\n\x05\x04\x1e\x02\x04\
    \x01\x12\x04\xc4\x01\x12\x1a\n\r\n\x05\x04\x1e\x02\x04\x03\x12\x04\xc4\
    \x01\x1d\x1e\n\x0c\n\x04\x04\x1e\x02\x05\x12\x04\xc5\x01\x02'\n\r\n\x05\
    \x04\x1e\x02\x05\x04\x12\x04\xc5\x01\x02\n\n\r\n\x05\x04\x1e\x02\x05\x06\
    \x12\x04\xc5\x01\x0b\x1d\n\r\n\x05\x04\x1e\x02\x05\x01\x12\x04\xc5\x01\
    \x1e\"\n\r\n\x05\x04\x1e\x02\x05\x03\x12\x04\xc5\x01%&\n\x0c\n\x04\x04\
    \x1e\x02\x06\x12\x04\xc6\x01\x02(\n\r\n\x05\x04\x1e\x02\x06\x04\x12\x04\
    \xc6\x01\x02\n\n\r\n\x05\x04\x1e\x02\x06\x06\x12\x04\xc6\x01\x0b\x1d\n\r\
    \n\x05\x04\x1e\x02\x06\x01\x12\x04\xc6\x01\x1e#\n\r\n\x05\x04\x1e\x02\
    \x06\x03\x12\x04\xc6\x01&'\n\x0c\n\x04\x04\x1e\x02\x07\x12\x04\xc7\x01\
    \x02,\n\r\n\x05\x04\x1e\x02\x07\x04\x12\x04\xc7\x01\x02\n\n\r\n\x05\x04\
    \x1e\x02\x07\x05\x12\x04\xc7\x01\x0b\x11\n\r\n\x05\x04\x1e\x02\x07\x01\
    \x12\x04\xc7\x01\x12\x19\n\r\n\x05\x04\x1e\x02\x07\x03\x12\x04\xc7\x01\
    \x1c\x1d\n\r\n\x05\x04\x1e\x02\x07\x08\x12\x04\xc7\x01\x1e+\n\x10\n\x08\
    \x04\x1e\x02\x07\x08\xe7\x07\0\x12\x04\xc7\x01\x1f*\n\x11\n\t\x04\x1e\
    \x02\x07\x08\xe7\x07\0\x02\x12\x04\xc7\x01\x1f%\n\x12\n\n\x04\x1e\x02\
    \x07\x08\xe7\x07\0\x02\0\x12\x04\xc7\x01\x1f%\n\x13\n\x0b\x04\x1e\x02\
    \x07\x08\xe7\x07\0\x02\0\x01\x12\x04\xc7\x01\x1f%\n\x11\n\t\x04\x1e\x02\
    \x07\x08\xe7\x07\0\x03\x12\x04\xc7\x01&*\n\x0c\n\x04\x04\x1e\x02\x08\x12\
    \x04\xc8\x01\x02\x1d\n\r\n\x05\x04\x1e\x02\x08\x04\x12\x04\xc8\x01\x02\n\
    \n\r\n\x05\x04\x1e\x02\x08\x05\x12\x04\xc8\x01\x0b\x11\n\r\n\x05\x04\x1e\
    \x02\x08\x01\x12\x04\xc8\x01\x12\x18\n\r\n\x05\x04\x1e\x02\x08\x03\x12\
    \x04\xc8\x01\x1b\x1c\n\x0c\n\x04\x04\x1e\x02\t\x12\x04\xc9\x01\x02\x1e\n\
    \r\n\x05\x04\x1e\x02\t\x04\x12\x04\xc9\x01\x02\n\n\r\n\x05\x04\x1e\x02\t\
    \x05\x12\x04\xc9\x01\x0b\x11\n\r\n\x05\x04\x1e\x02\t\x01\x12\x04\xc9\x01\
    \x12\x18\n\r\n\x05\x04\x1e\x02\t\x03\x12\x04\xc9\x01\x1b\x1d\n\x0c\n\x02\
    \x04\x1f\x12\x06\xcc\x01\0\xce\x01\x01\n\x0b\n\x03\x04\x1f\x01\x12\x04\
    \xcc\x01\x08\x18\n\x0c\n\x04\x04\x1f\x02\0\x12\x04\xcd\x01\x02(\n\r\n\
    \x05\x04\x1f\x02\0\x04\x12\x04\xcd\x01\x02\n\n\r\n\x05\x04\x1f\x02\0\x06\
    \x12\x04\xcd\x01\x0b\x1d\n\r\n\x05\x04\x1f\x02\0\x01\x12\x04\xcd\x01\x1e\
    #\n\r\n\x05\x04\x1f\x02\0\x03\x12\x04\xcd\x01&'\n\x0c\n\x02\x04\x20\x12\
    \x06\xd0\x01\0\xd3\x01\x01\n\x0b\n\x03\x04\x20\x01\x12\x04\xd0\x01\x08\
    \x1e\n\x0c\n\x04\x04\x20\x02\0\x12\x04\xd1\x01\x02(\n\r\n\x05\x04\x20\
    \x02\0\x04\x12\x04\xd1\x01\x02\n\n\r\n\x05\x04\x20\x02\0\x06\x12\x04\xd1\
    \x01\x0b\x1d\n\r\n\x05\x04\x20\x02\0\x01\x12\x04\xd1\x01\x1e#\n\r\n\x05\
    \x04\x20\x02\0\x03\x12\x04\xd1\x01&'\n\x0c\n\x04\x04\x20\x02\x01\x12\x04\
    \xd2\x01\x02\x1d\n\r\n\x05\x04\x20\x02\x01\x04\x12\x04\xd2\x01\x02\n\n\r\
    \n\x05\x04\x20\x02\x01\x05\x12\x04\xd2\x01\x0b\x11\n\r\n\x05\x04\x20\x02\
    \x01\x01\x12\x04\xd2\x01\x12\x18\n\r\n\x05\x04\x20\x02\x01\x03\x12\x04\
    \xd2\x01\x1b\x1c\n\x0c\n\x02\x04!\x12\x06\xd5\x01\0\xda\x01\x01\n\x0b\n\
    \x03\x04!\x01\x12\x04\xd5\x01\x08\x20\n\x0c\n\x04\x04!\x02\0\x12\x04\xd6\
    \x01\x02(\n\r\n\x05\x04!\x02\0\x04\x12\x04\xd6\x01\x02\n\n\r\n\x05\x04!\
    \x02\0\x06\x12\x04\xd6\x01\x0b\x1d\n\r\n\x05\x04!\x02\0\x01\x12\x04\xd6\
    \x01\x1e#\n\r\n\x05\x04!\x02\0\x03\x12\x04\xd6\x01&'\n\x0c\n\x04\x04!\
    \x02\x01\x12\x04\xd7\x01\x02\x1c\n\r\n\x05\x04!\x02\x01\x04\x12\x04\xd7\
    \x01\x02\n\n\r\n\x05\x04!\x02\x01\x05\x12\x04\xd7\x01\x0b\x11\n\r\n\x05\
    \x04!\x02\x01\x01\x12\x04\xd7\x01\x12\x17\n\r\n\x05\x04!\x02\x01\x03\x12\
    \x04\xd7\x01\x1a\x1b\n\x0c\n\x04\x04!\x02\x02\x12\x04\xd8\x01\x02\x1b\n\
    \r\n\x05\x04!\x02\x02\x04\x12\x04\xd8\x01\x02\n\n\r\n\x05\x04!\x02\x02\
    \x05\x12\x04\xd8\x01\x0b\x11\n\r\n\x05\x04!\x02\x02\x01\x12\x04\xd8\x01\
    \x12\x16\n\r\n\x05\x04!\x02\x02\x03\x12\x04\xd8\x01\x19\x1a\n\x0c\n\x04\
    \x04!\x02\x03\x12\x04\xd9\x01\x02\x1d\n\r\n\x05\x04!\x02\x03\x04\x12\x04\
    \xd9\x01\x02\n\n\r\n\x05\x04!\x02\x03\x05\x12\x04\xd9\x01\x0b\x0f\n\r\n\
    \x05\x04!\x02\x03\x01\x12\x04\xd9\x01\x10\x18\n\r\n\x05\x04!\x02\x03\x03\
    \x12\x04\xd9\x01\x1b\x1c\n\x0c\n\x02\x04\"\x12\x06\xdc\x01\0\xe1\x01\x01\
    \n\x0b\n\x03\x04\"\x01\x12\x04\xdc\x01\x08!\n\x0c\n\x04\x04\"\x02\0\x12\
    \x04\xdd\x01\x02\x1c\n\r\n\x05\x04\"\x02\0\x04\x12\x04\xdd\x01\x02\n\n\r\
    \n\x05\x04\"\x02\0\x05\x12\x04\xdd\x01\x0b\x11\n\r\n\x05\x04\"\x02\0\x01\
    \x12\x04\xdd\x01\x12\x17\n\r\n\x05\x04\"\x02\0\x03\x12\x04\xdd\x01\x1a\
    \x1b\n\x0c\n\x04\x04\"\x02\x01\x12\x04\xde\x01\x02\x1b\n\r\n\x05\x04\"\
    \x02\x01\x04\x12\x04\xde\x01\x02\n\n\r\n\x05\x04\"\x02\x01\x05\x12\x04\
    \xde\x01\x0b\x11\n\r\n\x05\x04\"\x02\x01\x01\x12\x04\xde\x01\x12\x16\n\r\
    \n\x05\x04\"\x02\x01\x03\x12\x04\xde\x01\x19\x1a\n\x0c\n\x04\x04\"\x02\
    \x02\x12\x04\xdf\x01\x02\x1c\n\r\n\x05\x04\"\x02\x02\x04\x12\x04\xdf\x01\
    \x02\n\n\r\n\x05\x04\"\x02\x02\x05\x12\x04\xdf\x01\x0b\x11\n\r\n\x05\x04\
    \"\x02\x02\x01\x12\x04\xdf\x01\x12\x17\n\r\n\x05\x04\"\x02\x02\x03\x12\
    \x04\xdf\x01\x1a\x1b\n\x0c\n\x04\x04\"\x02\x03\x12\x04\xe0\x01\x02)\n\r\
    \n\x05\x04\"\x02\x03\x04\x12\x04\xe0\x01\x02\n\n\r\n\x05\x04\"\x02\x03\
    \x06\x12\x04\xe0\x01\x0b\x1d\n\r\n\x05\x04\"\x02\x03\x01\x12\x04\xe0\x01\
    \x1e$\n\r\n\x05\x04\"\x02\x03\x03\x12\x04\xe0\x01'(\n\x0c\n\x02\x04#\x12\
    \x06\xe3\x01\0\xe7\x01\x01\n\x0b\n\x03\x04#\x01\x12\x04\xe3\x01\x08\x1c\
    \n\x0c\n\x04\x04#\x02\0\x12\x04\xe4\x01\x02!\n\r\n\x05\x04#\x02\0\x04\
    \x12\x04\xe4\x01\x02\n\n\r\n\x05\x04#\x02\0\x05\x12\x04\xe4\x01\x0b\x11\
    \n\r\n\x05\x04#\x02\0\x01\x12\x04\xe4\x01\x12\x1c\n\r\n\x05\x04#\x02\0\
    \x03\x12\x04\xe4\x01\x1f\x20\n\x0c\n\x04\x04#\x02\x01\x12\x04\xe5\x01\
    \x02!\n\r\n\x05\x04#\x02\x01\x04\x12\x04\xe5\x01\x02\n\n\r\n\x05\x04#\
    \x02\x01\x05\x12\x04\xe5\x01\x0b\x11\n\r\n\x05\x04#\x02\x01\x01\x12\x04\
    \xe5\x01\x12\x1c\n\r\n\x05\x04#\x02\x01\x03\x12\x04\xe5\x01\x1f\x20\n\
    \x0c\n\x04\x04#\x02\x02\x12\x04\xe6\x01\x02(\n\r\n\x05\x04#\x02\x02\x04\
    \x12\x04\xe6\x01\x02\n\n\r\n\x05\x04#\x02\x02\x06\x12\x04\xe6\x01\x0b\
    \x1d\n\r\n\x05\x04#\x02\x02\x01\x12\x04\xe6\x01\x1e#\n\r\n\x05\x04#\x02\
    \x02\x03\x12\x04\xe6\x01&'\n\x0c\n\x02\x04$\x12\x06\xe9\x01\0\xed\x01\
    \x01\n\x0b\n\x03\x04$\x01\x12\x04\xe9\x01\x08\x1b\n\x0c\n\x04\x04$\x02\0\
    \x12\x04\xea\x01\x02!\n\r\n\x05\x04$\x02\0\x04\x12\x04\xea\x01\x02\n\n\r\
    \n\x05\x04$\x02\0\x05\x12\x04\xea\x01\x0b\x11\n\r\n\x05\x04$\x02\0\x01\
    \x12\x04\xea\x01\x12\x1c\n\r\n\x05\x04$\x02\0\x03\x12\x04\xea\x01\x1f\
    \x20\n\x0c\n\x04\x04$\x02\x01\x12\x04\xeb\x01\x02!\n\r\n\x05\x04$\x02\
    \x01\x04\x12\x04\xeb\x01\x02\n\n\r\n\x05\x04$\x02\x01\x05\x12\x04\xeb\
    \x01\x0b\x11\n\r\n\x05\x04$\x02\x01\x01\x12\x04\xeb\x01\x12\x1c\n\r\n\
    \x05\x04$\x02\x01\x03\x12\x04\xeb\x01\x1f\x20\n\x0c\n\x04\x04$\x02\x02\
    \x12\x04\xec\x01\x02(\n\r\n\x05\x04$\x02\x02\x04\x12\x04\xec\x01\x02\n\n\
    \r\n\x05\x04$\x02\x02\x06\x12\x04\xec\x01\x0b\x1d\n\r\n\x05\x04$\x02\x02\
    \x01\x12\x04\xec\x01\x1e#\n\r\n\x05\x04$\x02\x02\x03\x12\x04\xec\x01&'\n\
    \x0c\n\x02\x04%\x12\x06\xef\x01\0\xf5\x01\x01\n\x0b\n\x03\x04%\x01\x12\
    \x04\xef\x01\x08\"\n\x0c\n\x04\x04%\x02\0\x12\x04\xf0\x01\x02\x1d\n\r\n\
    \x05\x04%\x02\0\x04\x12\x04\xf0\x01\x02\n\n\r\n\x05\x04%\x02\0\x05\x12\
    \x04\xf0\x01\x0b\x11\n\r\n\x05\x04%\x02\0\x01\x12\x04\xf0\x01\x12\x18\n\
    \r\n\x05\x04%\x02\0\x03\x12\x04\xf0\x01\x1b\x1c\n\x0c\n\x04\x04%\x02\x01\
    \x12\x04\xf1\x01\x02\x1c\n\r\n\x05\x04%\x02\x01\x04\x12\x04\xf1\x01\x02\
    \n\n\r\n\x05\x04%\x02\x01\x05\x12\x04\xf1\x01\x0b\x11\n\r\n\x05\x04%\x02\
    \x01\x01\x12\x04\xf1\x01\x12\x17\n\r\n\x05\x04%\x02\x01\x03\x12\x04\xf1\
    \x01\x1a\x1b\n\x0c\n\x04\x04%\x02\x02\x12\x04\xf2\x01\x02\x1c\n\r\n\x05\
    \x04%\x02\x02\x04\x12\x04\xf2\x01\x02\n\n\r\n\x05\x04%\x02\x02\x05\x12\
    \x04\xf2\x01\x0b\x11\n\r\n\x05\x04%\x02\x02\x01\x12\x04\xf2\x01\x12\x17\
    \n\r\n\x05\x04%\x02\x02\x03\x12\x04\xf2\x01\x1a\x1b\n\x0c\n\x04\x04%\x02\
    \x03\x12\x04\xf3\x01\x02\x1b\n\r\n\x05\x04%\x02\x03\x04\x12\x04\xf3\x01\
    \x02\n\n\r\n\x05\x04%\x02\x03\x05\x12\x04\xf3\x01\x0b\x11\n\r\n\x05\x04%\
    \x02\x03\x01\x12\x04\xf3\x01\x12\x16\n\r\n\x05\x04%\x02\x03\x03\x12\x04\
    \xf3\x01\x19\x1a\n\x0c\n\x04\x04%\x02\x04\x12\x04\xf4\x01\x02\x1d\n\r\n\
    \x05\x04%\x02\x04\x04\x12\x04\xf4\x01\x02\n\n\r\n\x05\x04%\x02\x04\x05\
    \x12\x04\xf4\x01\x0b\x0f\n\r\n\x05\x04%\x02\x04\x01\x12\x04\xf4\x01\x10\
    \x18\n\r\n\x05\x04%\x02\x04\x03\x12\x04\xf4\x01\x1b\x1c\n\x0c\n\x02\x04&\
    \x12\x06\xf7\x01\0\xfb\x01\x01\n\x0b\n\x03\x04&\x01\x12\x04\xf7\x01\x08&\
    \n\x0c\n\x04\x04&\x02\0\x12\x04\xf8\x01\x02\x1d\n\r\n\x05\x04&\x02\0\x04\
    \x12\x04\xf8\x01\x02\n\n\r\n\x05\x04&\x02\0\x05\x12\x04\xf8\x01\x0b\x11\
    \n\r\n\x05\x04&\x02\0\x01\x12\x04\xf8\x01\x12\x18\n\r\n\x05\x04&\x02\0\
    \x03\x12\x04\xf8\x01\x1b\x1c\n\x0c\n\x04\x04&\x02\x01\x12\x04\xf9\x01\
    \x02\x1c\n\r\n\x05\x04&\x02\x01\x04\x12\x04\xf9\x01\x02\n\n\r\n\x05\x04&\
    \x02\x01\x05\x12\x04\xf9\x01\x0b\x11\n\r\n\x05\x04&\x02\x01\x01\x12\x04\
    \xf9\x01\x12\x17\n\r\n\x05\x04&\x02\x01\x03\x12\x04\xf9\x01\x1a\x1b\n\
    \x0c\n\x04\x04&\x02\x02\x12\x04\xfa\x01\x02\x1b\n\r\n\x05\x04&\x02\x02\
    \x04\x12\x04\xfa\x01\x02\n\n\r\n\x05\x04&\x02\x02\x05\x12\x04\xfa\x01\
    \x0b\x11\n\r\n\x05\x04&\x02\x02\x01\x12\x04\xfa\x01\x12\x16\n\r\n\x05\
    \x04&\x02\x02\x03\x12\x04\xfa\x01\x19\x1a\n\x0c\n\x02\x04'\x12\x06\xfd\
    \x01\0\x82\x02\x01\n\x0b\n\x03\x04'\x01\x12\x04\xfd\x01\x08'\n\x0c\n\x04\
    \x04'\x02\0\x12\x04\xfe\x01\x02\x1c\n\r\n\x05\x04'\x02\0\x04\x12\x04\xfe\
    \x01\x02\n\n\r\n\x05\x04'\x02\0\x05\x12\x04\xfe\x01\x0b\x11\n\r\n\x05\
    \x04'\x02\0\x01\x12\x04\xfe\x01\x12\x17\n\r\n\x05\x04'\x02\0\x03\x12\x04\
    \xfe\x01\x1a\x1b\n\x0c\n\x04\x04'\x02\x01\x12\x04\xff\x01\x02\x1b\n\r\n\
    \x05\x04'\x02\x01\x04\x12\x04\xff\x01\x02\n\n\r\n\x05\x04'\x02\x01\x05\
    \x12\x04\xff\x01\x0b\x11\n\r\n\x05\x04'\x02\x01\x01\x12\x04\xff\x01\x12\
    \x16\n\r\n\x05\x04'\x02\x01\x03\x12\x04\xff\x01\x19\x1a\n\x0c\n\x04\x04'\
    \x02\x02\x12\x04\x80\x02\x02\x1c\n\r\n\x05\x04'\x02\x02\x04\x12\x04\x80\
    \x02\x02\n\n\r\n\x05\x04'\x02\x02\x05\x12\x04\x80\x02\x0b\x11\n\r\n\x05\
    \x04'\x02\x02\x01\x12\x04\x80\x02\x12\x17\n\r\n\x05\x04'\x02\x02\x03\x12\
    \x04\x80\x02\x1a\x1b\n\x0c\n\x04\x04'\x02\x03\x12\x04\x81\x02\x02)\n\r\n\
    \x05\x04'\x02\x03\x04\x12\x04\x81\x02\x02\n\n\r\n\x05\x04'\x02\x03\x06\
    \x12\x04\x81\x02\x0b\x1d\n\r\n\x05\x04'\x02\x03\x01\x12\x04\x81\x02\x1e$\
    \n\r\n\x05\x04'\x02\x03\x03\x12\x04\x81\x02'(\n\x0c\n\x02\x04(\x12\x06\
    \x84\x02\0\x87\x02\x01\n\x0b\n\x03\x04(\x01\x12\x04\x84\x02\x08'\n\x0c\n\
    \x04\x04(\x02\0\x12\x04\x85\x02\x02\x1d\n\r\n\x05\x04(\x02\0\x04\x12\x04\
    \x85\x02\x02\n\n\r\n\x05\x04(\x02\0\x05\x12\x04\x85\x02\x0b\x11\n\r\n\
    \x05\x04(\x02\0\x01\x12\x04\x85\x02\x12\x18\n\r\n\x05\x04(\x02\0\x03\x12\
    \x04\x85\x02\x1b\x1c\n\x0c\n\x04\x04(\x02\x01\x12\x04\x86\x02\x02\x1b\n\
    \r\n\x05\x04(\x02\x01\x04\x12\x04\x86\x02\x02\n\n\r\n\x05\x04(\x02\x01\
    \x05\x12\x04\x86\x02\x0b\x11\n\r\n\x05\x04(\x02\x01\x01\x12\x04\x86\x02\
    \x12\x16\n\r\n\x05\x04(\x02\x01\x03\x12\x04\x86\x02\x19\x1a\n\x0c\n\x02\
    \x04)\x12\x06\x89\x02\0\x8b\x02\x01\n\x0b\n\x03\x04)\x01\x12\x04\x89\x02\
    \x08(\n\x0c\n\x04\x04)\x02\0\x12\x04\x8a\x02\x02-\n\r\n\x05\x04)\x02\0\
    \x04\x12\x04\x8a\x02\x02\n\n\r\n\x05\x04)\x02\0\x06\x12\x04\x8a\x02\x0b\
    \x1f\n\r\n\x05\x04)\x02\0\x01\x12\x04\x8a\x02\x20(\n\r\n\x05\x04)\x02\0\
    \x03\x12\x04\x8a\x02+,\n\x1e\n\x02\x04*\x12\x06\x8e\x02\0\x98\x02\x01\
    \x1a\x10\x20Origin\x20Project\n\n\x0b\n\x03\x04*\x01\x12\x04\x8e\x02\x08\
    \x15\n\x0c\n\x04\x04*\x02\0\x12\x04\x8f\x02\x02\x19\n\r\n\x05\x04*\x02\0\
    \x04\x12\x04\x8f\x02\x02\n\n\r\n\x05\x04*\x02\0\x05\x12\x04\x8f\x02\x0b\
    \x11\n\r\n\x05\x04*\x02\0\x01\x12\x04\x8f\x02\x12\x14\n\r\n\x05\x04*\x02\
    \0\x03\x12\x04\x8f\x02\x17\x18\n\x0c\n\x04\x04*\x02\x01\x12\x04\x90\x02\
    \x02\x20\n\r\n\x05\x04*\x02\x01\x04\x12\x04\x90\x02\x02\n\n\r\n\x05\x04*\
    \x02\x01\x05\x12\x04\x90\x02\x0b\x11\n\r\n\x05\x04*\x02\x01\x01\x12\x04\
    \x90\x02\x12\x1b\n\r\n\x05\x04*\x02\x01\x03\x12\x04\x90\x02\x1e\x1f\n\
    \x0c\n\x04\x04*\x02\x02\x12\x04\x91\x02\x02\"\n\r\n\x05\x04*\x02\x02\x04\
    \x12\x04\x91\x02\x02\n\n\r\n\x05\x04*\x02\x02\x05\x12\x04\x91\x02\x0b\
    \x11\n\r\n\x05\x04*\x02\x02\x01\x12\x04\x91\x02\x12\x1d\n\r\n\x05\x04*\
    \x02\x02\x03\x12\x04\x91\x02\x20!\n\x0c\n\x04\x04*\x02\x03\x12\x04\x92\
    \x02\x02#\n\r\n\x05\x04*\x02\x03\x04\x12\x04\x92\x02\x02\n\n\r\n\x05\x04\
    *\x02\x03\x05\x12\x04\x92\x02\x0b\x11\n\r\n\x05\x04*\x02\x03\x01\x12\x04\
    \x92\x02\x12\x1e\n\r\n\x05\x04*\x02\x03\x03\x12\x04\x92\x02!\"\n\x0c\n\
    \x04\x04*\x02\x04\x12\x04\x93\x02\x02\x1b\n\r\n\x05\x04*\x02\x04\x04\x12\
    \x04\x93\x02\x02\n\n\r\n\x05\x04*\x02\x04\x05\x12\x04\x93\x02\x0b\x11\n\
    \r\n\x05\x04*\x02\x04\x01\x12\x04\x93\x02\x12\x16\n\r\n\x05\x04*\x02\x04\
    \x03\x12\x04\x93\x02\x19\x1a\n\x0c\n\x04\x04*\x02\x05\x12\x04\x94\x02\
    \x02\x20\n\r\n\x05\x04*\x02\x05\x04\x12\x04\x94\x02\x02\n\n\r\n\x05\x04*\
    \x02\x05\x05\x12\x04\x94\x02\x0b\x11\n\r\n\x05\x04*\x02\x05\x01\x12\x04\
    \x94\x02\x12\x1b\n\r\n\x05\x04*\x02\x05\x03\x12\x04\x94\x02\x1e\x1f\n\
    \x0c\n\x04\x04*\x02\x06\x12\x04\x95\x02\x02\x1f\n\r\n\x05\x04*\x02\x06\
    \x04\x12\x04\x95\x02\x02\n\n\r\n\x05\x04*\x02\x06\x05\x12\x04\x95\x02\
    \x0b\x11\n\r\n\x05\x04*\x02\x06\x01\x12\x04\x95\x02\x12\x1a\n\r\n\x05\
    \x04*\x02\x06\x03\x12\x04\x95\x02\x1d\x1e\n\x0c\n\x04\x04*\x02\x07\x12\
    \x04\x96\x02\x02\x1f\n\r\n\x05\x04*\x02\x07\x04\x12\x04\x96\x02\x02\n\n\
    \r\n\x05\x04*\x02\x07\x05\x12\x04\x96\x02\x0b\x11\n\r\n\x05\x04*\x02\x07\
    \x01\x12\x04\x96\x02\x12\x1a\n\r\n\x05\x04*\x02\x07\x03\x12\x04\x96\x02\
    \x1d\x1e\n\x0c\n\x04\x04*\x02\x08\x12\x04\x97\x02\x02\x1f\n\r\n\x05\x04*\
    \x02\x08\x04\x12\x04\x97\x02\x02\n\n\r\n\x05\x04*\x02\x08\x05\x12\x04\
    \x97\x02\x0b\x11\n\r\n\x05\x04*\x02\x08\x01\x12\x04\x97\x02\x12\x1a\n\r\
    \n\x05\x04*\x02\x08\x03\x12\x04\x97\x02\x1d\x1e\n\x0c\n\x02\x04+\x12\x06\
    \x9a\x02\0\x9c\x02\x01\n\x0b\n\x03\x04+\x01\x12\x04\x9a\x02\x08\x1b\n\
    \x0c\n\x04\x04+\x02\0\x12\x04\x9b\x02\x02%\n\r\n\x05\x04+\x02\0\x04\x12\
    \x04\x9b\x02\x02\n\n\r\n\x05\x04+\x02\0\x06\x12\x04\x9b\x02\x0b\x18\n\r\
    \n\x05\x04+\x02\0\x01\x12\x04\x9b\x02\x19\x20\n\r\n\x05\x04+\x02\0\x03\
    \x12\x04\x9b\x02#$\n\x0c\n\x02\x04,\x12\x06\x9e\x02\0\xa1\x02\x01\n\x0b\
    \n\x03\x04,\x01\x12\x04\x9e\x02\x08\x1b\n\x0c\n\x04\x04,\x02\0\x12\x04\
    \x9f\x02\x02\x1b\n\r\n\x05\x04,\x02\0\x04\x12\x04\x9f\x02\x02\n\n\r\n\
    \x05\x04,\x02\0\x05\x12\x04\x9f\x02\x0b\x11\n\r\n\x05\x04,\x02\0\x01\x12\
    \x04\x9f\x02\x12\x16\n\r\n\x05\x04,\x02\0\x03\x12\x04\x9f\x02\x19\x1a\n\
    \x0c\n\x04\x04,\x02\x01\x12\x04\xa0\x02\x02#\n\r\n\x05\x04,\x02\x01\x04\
    \x12\x04\xa0\x02\x02\n\n\r\n\x05\x04,\x02\x01\x05\x12\x04\xa0\x02\x0b\
    \x11\n\r\n\x05\x04,\x02\x01\x01\x12\x04\xa0\x02\x12\x1e\n\r\n\x05\x04,\
    \x02\x01\x03\x12\x04\xa0\x02!\"\n\x0c\n\x02\x04-\x12\x06\xa3\x02\0\xa5\
    \x02\x01\n\x0b\n\x03\x04-\x01\x12\x04\xa3\x02\x08\x18\n\x0c\n\x04\x04-\
    \x02\0\x12\x04\xa4\x02\x02\x1b\n\r\n\x05\x04-\x02\0\x04\x12\x04\xa4\x02\
    \x02\n\n\r\n\x05\x04-\x02\0\x05\x12\x04\xa4\x02\x0b\x11\n\r\n\x05\x04-\
    \x02\0\x01\x12\x04\xa4\x02\x12\x16\n\r\n\x05\x04-\x02\0\x03\x12\x04\xa4\
    \x02\x19\x1a\n\x0c\n\x02\x04.\x12\x06\xa7\x02\0\xaa\x02\x01\n\x0b\n\x03\
    \x04.\x01\x12\x04\xa7\x02\x08\x1b\n\x0c\n\x04\x04.\x02\0\x12\x04\xa8\x02\
    \x02#\n\r\n\x05\x04.\x02\0\x04\x12\x04\xa8\x02\x02\n\n\r\n\x05\x04.\x02\
    \0\x05\x12\x04\xa8\x02\x0b\x11\n\r\n\x05\x04.\x02\0\x01\x12\x04\xa8\x02\
    \x12\x1e\n\r\n\x05\x04.\x02\0\x03\x12\x04\xa8\x02!\"\n\x0c\n\x04\x04.\
    \x02\x01\x12\x04\xa9\x02\x02%\n\r\n\x05\x04.\x02\x01\x04\x12\x04\xa9\x02\
    \x02\n\n\r\n\x05\x04.\x02\x01\x06\x12\x04\xa9\x02\x0b\x18\n\r\n\x05\x04.\
    \x02\x01\x01\x12\x04\xa9\x02\x19\x20\n\r\n\x05\x04.\x02\x01\x03\x12\x04\
    \xa9\x02#$\n!\n\x02\x04/\x12\x06\xad\x02\0\xb4\x02\x01\x1a\x13\x20Origin\
    \x20Public\x20Key\n\n\x0b\n\x03\x04/\x01\x12\x04\xad\x02\x08\x17\n\x0c\n\
    \x04\x04/\x02\0\x12\x04\xae\x02\x02\x19\n\r\n\x05\x04/\x02\0\x04\x12\x04\
    \xae\x02\x02\n\n\r\n\x05\x04/\x02\0\x05\x12\x04\xae\x02\x0b\x11\n\r\n\
    \x05\x04/\x02\0\x01\x12\x04\xae\x02\x12\x14\n\r\n\x05\x04/\x02\0\x03\x12\
    \x04\xae\x02\x17\x18\n\x0c\n\x04\x04/\x02\x01\x12\x04\xaf\x02\x02\x20\n\
    \r\n\x05\x04/\x02\x01\x04\x12\x04\xaf\x02\x02\n\n\r\n\x05\x04/\x02\x01\
    \x05\x12\x04\xaf\x02\x0b\x11\n\r\n\x05\x04/\x02\x01\x01\x12\x04\xaf\x02\
    \x12\x1b\n\r\n\x05\x04/\x02\x01\x03\x12\x04\xaf\x02\x1e\x1f\n\x0c\n\x04\
    \x04/\x02\x02\x12\x04\xb0\x02\x02\x1b\n\r\n\x05\x04/\x02\x02\x04\x12\x04\
    \xb0\x02\x02\n\n\r\n\x05\x04/\x02\x02\x05\x12\x04\xb0\x02\x0b\x11\n\r\n\
    \x05\x04/\x02\x02\x01\x12\x04\xb0\x02\x12\x16\n\r\n\x05\x04/\x02\x02\x03\
    \x12\x04\xb0\x02\x19\x1a\n\x0c\n\x04\x04/\x02\x03\x12\x04\xb1\x02\x02\
    \x1f\n\r\n\x05\x04/\x02\x03\x04\x12\x04\xb1\x02\x02\n\n\r\n\x05\x04/\x02\
    \x03\x05\x12\x04\xb1\x02\x0b\x11\n\r\n\x05\x04/\x02\x03\x01\x12\x04\xb1\
    \x02\x12\x1a\n\r\n\x05\x04/\x02\x03\x03\x12\x04\xb1\x02\x1d\x1e\n\x0c\n\
    \x04\x04/\x02\x04\x12\x04\xb2\x02\x02\x1a\n\r\n\x05\x04/\x02\x04\x04\x12\
    \x04\xb2\x02\x02\n\n\r\n\x05\x04/\x02\x04\x05\x12\x04\xb2\x02\x0b\x10\n\
    \r\n\x05\x04/\x02\x04\x01\x12\x04\xb2\x02\x11\x15\n\r\n\x05\x04/\x02\x04\
    \x03\x12\x04\xb2\x02\x18\x19\n\x0c\n\x04\x04/\x02\x05\x12\x04\xb3\x02\
    \x02\x1f\n\r\n\x05\x04/\x02\x05\x04\x12\x04\xb3\x02\x02\n\n\r\n\x05\x04/\
    \x02\x05\x05\x12\x04\xb3\x02\x0b\x11\n\r\n\x05\x04/\x02\x05\x01\x12\x04\
    \xb3\x02\x12\x1a\n\r\n\x05\x04/\x02\x05\x03\x12\x04\xb3\x02\x1d\x1e\n\
    \x0c\n\x02\x040\x12\x06\xb6\x02\0\xbc\x02\x01\n\x0b\n\x03\x040\x01\x12\
    \x04\xb6\x02\x08\x1d\n\x0c\n\x04\x040\x02\0\x12\x04\xb7\x02\x02\x20\n\r\
    \n\x05\x040\x02\0\x04\x12\x04\xb7\x02\x02\n\n\r\n\x05\x040\x02\0\x05\x12\
    \x04\xb7\x02\x0b\x11\n\r\n\x05\x040\x02\0\x01\x12\x04\xb7\x02\x12\x1b\n\
    \r\n\x05\x040\x02\0\x03\x12\x04\xb7\x02\x1e\x1f\n\x0c\n\x04\x040\x02\x01\
    \x12\x04\xb8\x02\x02\x1b\n\r\n\x05\x040\x02\x01\x04\x12\x04\xb8\x02\x02\
    \n\n\r\n\x05\x040\x02\x01\x05\x12\x04\xb8\x02\x0b\x11\n\r\n\x05\x040\x02\
    \x01\x01\x12\x04\xb8\x02\x12\x16\n\r\n\x05\x040\x02\x01\x03\x12\x04\xb8\
    \x02\x19\x1a\n\x0c\n\x04\x040\x02\x02\x12\x04\xb9\x02\x02\x1f\n\r\n\x05\
    \x040\x02\x02\x04\x12\x04\xb9\x02\x02\n\n\r\n\x05\x040\x02\x02\x05\x12\
    \x04\xb9\x02\x0b\x11\n\r\n\x05\x040\x02\x02\x01\x12\x04\xb9\x02\x12\x1a\
    \n\r\n\x05\x040\x02\x02\x03\x12\x04\xb9\x02\x1d\x1e\n\x0c\n\x04\x040\x02\
    \x03\x12\x04\xba\x02\x02\x1a\n\r\n\x05\x040\x02\x03\x04\x12\x04\xba\x02\
    \x02\n\n\r\n\x05\x040\x02\x03\x05\x12\x04\xba\x02\x0b\x10\n\r\n\x05\x040\
    \x02\x03\x01\x12\x04\xba\x02\x11\x15\n\r\n\x05\x040\x02\x03\x03\x12\x04\
    \xba\x02\x18\x19\n\x0c\n\x04\x040\x02\x04\x12\x04\xbb\x02\x02\x1f\n\r\n\
    \x05\x040\x02\x04\x04\x12\x04\xbb\x02\x02\n\n\r\n\x05\x040\x02\x04\x05\
    \x12\x04\xbb\x02\x0b\x11\n\r\n\x05\x040\x02\x04\x01\x12\x04\xbb\x02\x12\
    \x1a\n\r\n\x05\x040\x02\x04\x03\x12\x04\xbb\x02\x1d\x1e\n\x0c\n\x02\x041\
    \x12\x06\xbe\x02\0\xc2\x02\x01\n\x0b\n\x03\x041\x01\x12\x04\xbe\x02\x08\
    \x1a\n\x0c\n\x04\x041\x02\0\x12\x04\xbf\x02\x02\x1f\n\r\n\x05\x041\x02\0\
    \x04\x12\x04\xbf\x02\x02\n\n\r\n\x05\x041\x02\0\x05\x12\x04\xbf\x02\x0b\
    \x11\n\r\n\x05\x041\x02\0\x01\x12\x04\xbf\x02\x12\x1a\n\r\n\x05\x041\x02\
    \0\x03\x12\x04\xbf\x02\x1d\x1e\n\x0c\n\x04\x041\x02\x01\x12\x04\xc0\x02\
    \x02\x1d\n\r\n\x05\x041\x02\x01\x04\x12\x04\xc0\x02\x02\n\n\r\n\x05\x041\
    \x02\x01\x05\x12\x04\xc0\x02\x0b\x11\n\r\n\x05\x041\x02\x01\x01\x12\x04\
    \xc0\x02\x12\x18\n\r\n\x05\x041\x02\x01\x03\x12\x04\xc0\x02\x1b\x1c\n\
    \x0c\n\x04\x041\x02\x02\x12\x04\xc1\x02\x02\x1f\n\r\n\x05\x041\x02\x02\
    \x04\x12\x04\xc1\x02\x02\n\n\r\n\x05\x041\x02\x02\x05\x12\x04\xc1\x02\
    \x0b\x11\n\r\n\x05\x041\x02\x02\x01\x12\x04\xc1\x02\x12\x1a\n\r\n\x05\
    \x041\x02\x02\x03\x12\x04\xc1\x02\x1d\x1e\n\x0c\n\x02\x042\x12\x06\xc4\
    \x02\0\xc7\x02\x01\n\x0b\n\x03\x042\x01\x12\x04\xc4\x02\x08\x20\n\x0c\n\
    \x04\x042\x02\0\x12\x04\xc5\x02\x02\x1f\n\r\n\x05\x042\x02\0\x04\x12\x04\
    \xc5\x02\x02\n\n\r\n\x05\x042\x02\0\x05\x12\x04\xc5\x02\x0b\x11\n\r\n\
    \x05\x042\x02\0\x01\x12\x04\xc5\x02\x12\x1a\n\r\n\x05\x042\x02\0\x03\x12\
    \x04\xc5\x02\x1d\x1e\n\x0c\n\x04\x042\x02\x01\x12\x04\xc6\x02\x02\x1d\n\
    \r\n\x05\x042\x02\x01\x04\x12\x04\xc6\x02\x02\n\n\r\n\x05\x042\x02\x01\
    \x05\x12\x04\xc6\x02\x0b\x11\n\r\n\x05\x042\x02\x01\x01\x12\x04\xc6\x02\
    \x12\x18\n\r\n\x05\x042\x02\x01\x03\x12\x04\xc6\x02\x1b\x1c\n\x0c\n\x02\
    \x043\x12\x06\xc9\x02\0\xcc\x02\x01\n\x0b\n\x03\x043\x01\x12\x04\xc9\x02\
    \x08\"\n\x0c\n\x04\x043\x02\0\x12\x04\xca\x02\x02\x1f\n\r\n\x05\x043\x02\
    \0\x04\x12\x04\xca\x02\x02\n\n\r\n\x05\x043\x02\0\x05\x12\x04\xca\x02\
    \x0b\x11\n\r\n\x05\x043\x02\0\x01\x12\x04\xca\x02\x12\x1a\n\r\n\x05\x043\
    \x02\0\x03\x12\x04\xca\x02\x1d\x1e\n\x0c\n\x04\x043\x02\x01\x12\x04\xcb\
    \x02\x02\x20\n\r\n\x05\x043\x02\x01\x04\x12\x04\xcb\x02\x02\n\n\r\n\x05\
    \x043\x02\x01\x05\x12\x04\xcb\x02\x0b\x11\n\r\n\x05\x043\x02\x01\x01\x12\
    \x04\xcb\x02\x12\x1b\n\r\n\x05\x043\x02\x01\x03\x12\x04\xcb\x02\x1e\x1f\
    \n\x0c\n\x02\x044\x12\x06\xce\x02\0\xd1\x02\x01\n\x0b\n\x03\x044\x01\x12\
    \x04\xce\x02\x08#\n\x0c\n\x04\x044\x02\0\x12\x04\xcf\x02\x02\x20\n\r\n\
    \x05\x044\x02\0\x04\x12\x04\xcf\x02\x02\n\n\r\n\x05\x044\x02\0\x05\x12\
    \x04\xcf\x02\x0b\x11\n\r\n\x05\x044\x02\0\x01\x12\x04\xcf\x02\x12\x1b\n\
    \r\n\x05\x044\x02\0\x03\x12\x04\xcf\x02\x1e\x1f\n\x0c\n\x04\x044\x02\x01\
    \x12\x04\xd0\x02\x02$\n\r\n\x05\x044\x02\x01\x04\x12\x04\xd0\x02\x02\n\n\
    \r\n\x05\x044\x02\x01\x06\x12\x04\xd0\x02\x0b\x1a\n\r\n\x05\x044\x02\x01\
    \x01\x12\x04\xd0\x02\x1b\x1f\n\r\n\x05\x044\x02\x01\x03\x12\x04\xd0\x02\
    \"#\n!\n\x02\x045\x12\x06\xd4\x02\0\xdb\x02\x01\x1a\x13\x20Origin\x20Sec\
    ret\x20Key\n\n\x0b\n\x03\x045\x01\x12\x04\xd4\x02\x08\x17\n\x0c\n\x04\
    \x045\x02\0\x12\x04\xd5\x02\x02\x19\n\r\n\x05\x045\x02\0\x04\x12\x04\xd5\
    \x02\x02\n\n\r\n\x05\x045\x02\0\x05\x12\x04\xd5\x02\x0b\x11\n\r\n\x05\
    \x045\x02\0\x01\x12\x04\xd5\x02\x12\x14\n\r\n\x05\x045\x02\0\x03\x12\x04\
    \xd5\x02\x17\x18\n\x0c\n\x04\x045\x02\x01\x12\x04\xd6\x02\x02\x20\n\r\n\
    \x05\x045\x02\x01\x04\x12\x04\xd6\x02\x02\n\n\r\n\x05\x045\x02\x01\x05\
    \x12\x04\xd6\x02\x0b\x11\n\r\n\x05\x045\x02\x01\x01\x12\x04\xd6\x02\x12\
    \x1b\n\r\n\x05\x045\x02\x01\x03\x12\x04\xd6\x02\x1e\x1f\n\x0c\n\x04\x045\
    \x02\x02\x12\x04\xd7\x02\x02\x1b\n\r\n\x05\x045\x02\x02\x04\x12\x04\xd7\
    \x02\x02\n\n\r\n\x05\x045\x02\x02\x05\x12\x04\xd7\x02\x0b\x11\n\r\n\x05\
    \x045\x02\x02\x01\x12\x04\xd7\x02\x12\x16\n\r\n\x05\x045\x02\x02\x03\x12\
    \x04\xd7\x02\x19\x1a\n\x0c\n\x04\x045\x02\x03\x12\x04\xd8\x02\x02\x1f\n\
    \r\n\x05\x045\x02\x03\x04\x12\x04\xd8\x02\x02\n\n\r\n\x05\x045\x02\x03\
    \x05\x12\x04\xd8\x02\x0b\x11\n\r\n\x05\x045\x02\x03\x01\x12\x04\xd8\x02\
    \x12\x1a\n\r\n\x05\x045\x02\x03\x03\x12\x04\xd8\x02\x1d\x1e\n\x0c\n\x04\
    \x045\x02\x04\x12\x04\xd9\x02\x02\x1a\n\r\n\x05\x045\x02\x04\x04\x12\x04\
    \xd9\x02\x02\n\n\r\n\x05\x045\x02\x04\x05\x12\x04\xd9\x02\x0b\x10\n\r\n\
    \x05\x045\x02\x04\x01\x12\x04\xd9\x02\x11\x15\n\r\n\x05\x045\x02\x04\x03\
    \x12\x04\xd9\x02\x18\x19\n\x0c\n\x04\x045\x02\x05\x12\x04\xda\x02\x02\
    \x1f\n\r\n\x05\x045\x02\x05\x04\x12\x04\xda\x02\x02\n\n\r\n\x05\x045\x02\
    \x05\x05\x12\x04\xda\x02\x0b\x11\n\r\n\x05\x045\x02\x05\x01\x12\x04\xda\
    \x02\x12\x1a\n\r\n\x05\x045\x02\x05\x03\x12\x04\xda\x02\x1d\x1e\n\x0c\n\
    \x02\x046\x12\x06\xdd\x02\0\xe3\x02\x01\n\x0b\n\x03\x046\x01\x12\x04\xdd\
    \x02\x08\x1d\n\x0c\n\x04\x046\x02\0\x12\x04\xde\x02\x02\x20\n\r\n\x05\
    \x046\x02\0\x04\x12\x04\xde\x02\x02\n\n\r\n\x05\x046\x02\0\x05\x12\x04\
    \xde\x02\x0b\x11\n\r\n\x05\x046\x02\0\x01\x12\x04\xde\x02\x12\x1b\n\r\n\
    \x05\x046\x02\0\x03\x12\x04\xde\x02\x1e\x1f\n\x0c\n\x04\x046\x02\x01\x12\
    \x04\xdf\x02\x02\x1b\n\r\n\x05\x046\x02\x01\x04\x12\x04\xdf\x02\x02\n\n\
    \r\n\x05\x046\x02\x01\x05\x12\x04\xdf\x02\x0b\x11\n\r\n\x05\x046\x02\x01\
    \x01\x12\x04\xdf\x02\x12\x16\n\r\n\x05\x046\x02\x01\x03\x12\x04\xdf\x02\
    \x19\x1a\n\x0c\n\x04\x046\x02\x02\x12\x04\xe0\x02\x02\x1f\n\r\n\x05\x046\
    \x02\x02\x04\x12\x04\xe0\x02\x02\n\n\r\n\x05\x046\x02\x02\x05\x12\x04\
    \xe0\x02\x0b\x11\n\r\n\x05\x046\x02\x02\x01\x12\x04\xe0\x02\x12\x1a\n\r\
    \n\x05\x046\x02\x02\x03\x12\x04\xe0\x02\x1d\x1e\n\x0c\n\x04\x046\x02\x03\
    \x12\x04\xe1\x02\x02\x1a\n\r\n\x05\x046\x02\x03\x04\x12\x04\xe1\x02\x02\
    \n\n\r\n\x05\x046\x02\x03\x05\x12\x04\xe1\x02\x0b\x10\n\r\n\x05\x046\x02\
    \x03\x01\x12\x04\xe1\x02\x11\x15\n\r\n\x05\x046\x02\x03\x03\x12\x04\xe1\
    \x02\x18\x19\n\x0c\n\x04\x046\x02\x04\x12\x04\xe2\x02\x02\x1f\n\r\n\x05\
    \x046\x02\x04\x04\x12\x04\xe2\x02\x02\n\n\r\n\x05\x046\x02\x04\x05\x12\
    \x04\xe2\x02\x0b\x11\n\r\n\x05\x046\x02\x04\x01\x12\x04\xe2\x02\x12\x1a\
    \n\r\n\x05\x046\x02\x04\x03\x12\x04\xe2\x02\x1d\x1e\n\x0c\n\x02\x047\x12\
    \x06\xe5\x02\0\xe8\x02\x01\n\x0b\n\x03\x047\x01\x12\x04\xe5\x02\x08\x1a\
    \n\x0c\n\x04\x047\x02\0\x12\x04\xe6\x02\x02\x1f\n\r\n\x05\x047\x02\0\x04\
    \x12\x04\xe6\x02\x02\n\n\r\n\x05\x047\x02\0\x05\x12\x04\xe6\x02\x0b\x11\
    \n\r\n\x05\x047\x02\0\x01\x12\x04\xe6\x02\x12\x1a\n\r\n\x05\x047\x02\0\
    \x03\x12\x04\xe6\x02\x1d\x1e\n\x0c\n\x04\x047\x02\x01\x12\x04\xe7\x02\
    \x02\x1d\n\r\n\x05\x047\x02\x01\x04\x12\x04\xe7\x02\x02\n\n\r\n\x05\x047\
    \x02\x01\x05\x12\x04\xe7\x02\x0b\x11\n\r\n\x05\x047\x02\x01\x01\x12\x04\
    \xe7\x02\x12\x18\n\r\n\x05\x047\x02\x01\x03\x12\x04\xe7\x02\x1b\x1c\
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
