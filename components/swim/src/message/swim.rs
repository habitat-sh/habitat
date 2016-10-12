// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(Clone,Default)]
pub struct Member {
    // message fields
    id: ::protobuf::SingularField<::std::string::String>,
    incarnation: ::std::option::Option<u64>,
    address: ::protobuf::SingularField<::std::string::String>,
    persistent: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Member {}

impl Member {
    pub fn new() -> Member {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Member {
        static mut instance: ::protobuf::lazy::Lazy<Member> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Member,
        };
        unsafe {
            instance.get(|| {
                Member {
                    id: ::protobuf::SingularField::none(),
                    incarnation: ::std::option::Option::None,
                    address: ::protobuf::SingularField::none(),
                    persistent: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string id = 1;

    pub fn clear_id(&mut self) {
        self.id.clear();
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_id(&mut self) -> &mut ::std::string::String {
        if self.id.is_none() {
            self.id.set_default();
        };
        self.id.as_mut().unwrap()
    }

    // Take field
    pub fn take_id(&mut self) -> ::std::string::String {
        self.id.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_id(&self) -> &str {
        match self.id.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional uint64 incarnation = 2;

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

    // optional string address = 3;

    pub fn clear_address(&mut self) {
        self.address.clear();
    }

    pub fn has_address(&self) -> bool {
        self.address.is_some()
    }

    // Param is passed by value, moved
    pub fn set_address(&mut self, v: ::std::string::String) {
        self.address = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_address(&mut self) -> &mut ::std::string::String {
        if self.address.is_none() {
            self.address.set_default();
        };
        self.address.as_mut().unwrap()
    }

    // Take field
    pub fn take_address(&mut self) -> ::std::string::String {
        self.address.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_address(&self) -> &str {
        match self.address.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional bool persistent = 4;

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
}

impl ::protobuf::Message for Member {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.id));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.incarnation = ::std::option::Option::Some(tmp);
                },
                3 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.address));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.persistent = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.id {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.incarnation {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.address {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        if self.persistent.is_some() {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.incarnation {
            try!(os.write_uint64(2, v));
        };
        if let Some(v) = self.address.as_ref() {
            try!(os.write_string(3, &v));
        };
        if let Some(v) = self.persistent {
            try!(os.write_bool(4, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Member>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Member {
    fn new() -> Member {
        Member::new()
    }

    fn descriptor_static(_: ::std::option::Option<Member>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "id",
                    Member::has_id,
                    Member::get_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "incarnation",
                    Member::has_incarnation,
                    Member::get_incarnation,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "address",
                    Member::has_address,
                    Member::get_address,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "persistent",
                    Member::has_persistent,
                    Member::get_persistent,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Member>(
                    "Member",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Member {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_incarnation();
        self.clear_address();
        self.clear_persistent();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Member {
    fn eq(&self, other: &Member) -> bool {
        self.id == other.id &&
        self.incarnation == other.incarnation &&
        self.address == other.address &&
        self.persistent == other.persistent &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Member {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Ping {
    // message fields
    from: ::protobuf::SingularPtrField<Member>,
    forward_to: ::protobuf::SingularPtrField<Member>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
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
            instance.get(|| {
                Ping {
                    from: ::protobuf::SingularPtrField::none(),
                    forward_to: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .Member from = 1;

    pub fn clear_from(&mut self) {
        self.from.clear();
    }

    pub fn has_from(&self) -> bool {
        self.from.is_some()
    }

    // Param is passed by value, moved
    pub fn set_from(&mut self, v: Member) {
        self.from = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_from(&mut self) -> &mut Member {
        if self.from.is_none() {
            self.from.set_default();
        };
        self.from.as_mut().unwrap()
    }

    // Take field
    pub fn take_from(&mut self) -> Member {
        self.from.take().unwrap_or_else(|| Member::new())
    }

    pub fn get_from(&self) -> &Member {
        self.from.as_ref().unwrap_or_else(|| Member::default_instance())
    }

    // optional .Member forward_to = 2;

    pub fn clear_forward_to(&mut self) {
        self.forward_to.clear();
    }

    pub fn has_forward_to(&self) -> bool {
        self.forward_to.is_some()
    }

    // Param is passed by value, moved
    pub fn set_forward_to(&mut self, v: Member) {
        self.forward_to = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_forward_to(&mut self) -> &mut Member {
        if self.forward_to.is_none() {
            self.forward_to.set_default();
        };
        self.forward_to.as_mut().unwrap()
    }

    // Take field
    pub fn take_forward_to(&mut self) -> Member {
        self.forward_to.take().unwrap_or_else(|| Member::new())
    }

    pub fn get_forward_to(&self) -> &Member {
        self.forward_to.as_ref().unwrap_or_else(|| Member::default_instance())
    }
}

impl ::protobuf::Message for Ping {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.from));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.forward_to));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.from {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.forward_to {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.from.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.forward_to.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Ping>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
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
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "from",
                    Ping::has_from,
                    Ping::get_from,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "forward_to",
                    Ping::has_forward_to,
                    Ping::get_forward_to,
                ));
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
        self.clear_from();
        self.clear_forward_to();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Ping {
    fn eq(&self, other: &Ping) -> bool {
        self.from == other.from &&
        self.forward_to == other.forward_to &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Ping {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Ack {
    // message fields
    from: ::protobuf::SingularPtrField<Member>,
    forward_to: ::protobuf::SingularPtrField<Member>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Ack {}

impl Ack {
    pub fn new() -> Ack {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Ack {
        static mut instance: ::protobuf::lazy::Lazy<Ack> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Ack,
        };
        unsafe {
            instance.get(|| {
                Ack {
                    from: ::protobuf::SingularPtrField::none(),
                    forward_to: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .Member from = 1;

    pub fn clear_from(&mut self) {
        self.from.clear();
    }

    pub fn has_from(&self) -> bool {
        self.from.is_some()
    }

    // Param is passed by value, moved
    pub fn set_from(&mut self, v: Member) {
        self.from = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_from(&mut self) -> &mut Member {
        if self.from.is_none() {
            self.from.set_default();
        };
        self.from.as_mut().unwrap()
    }

    // Take field
    pub fn take_from(&mut self) -> Member {
        self.from.take().unwrap_or_else(|| Member::new())
    }

    pub fn get_from(&self) -> &Member {
        self.from.as_ref().unwrap_or_else(|| Member::default_instance())
    }

    // optional .Member forward_to = 2;

    pub fn clear_forward_to(&mut self) {
        self.forward_to.clear();
    }

    pub fn has_forward_to(&self) -> bool {
        self.forward_to.is_some()
    }

    // Param is passed by value, moved
    pub fn set_forward_to(&mut self, v: Member) {
        self.forward_to = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_forward_to(&mut self) -> &mut Member {
        if self.forward_to.is_none() {
            self.forward_to.set_default();
        };
        self.forward_to.as_mut().unwrap()
    }

    // Take field
    pub fn take_forward_to(&mut self) -> Member {
        self.forward_to.take().unwrap_or_else(|| Member::new())
    }

    pub fn get_forward_to(&self) -> &Member {
        self.forward_to.as_ref().unwrap_or_else(|| Member::default_instance())
    }
}

impl ::protobuf::Message for Ack {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.from));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.forward_to));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.from {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.forward_to {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.from.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.forward_to.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Ack>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Ack {
    fn new() -> Ack {
        Ack::new()
    }

    fn descriptor_static(_: ::std::option::Option<Ack>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "from",
                    Ack::has_from,
                    Ack::get_from,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "forward_to",
                    Ack::has_forward_to,
                    Ack::get_forward_to,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Ack>(
                    "Ack",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Ack {
    fn clear(&mut self) {
        self.clear_from();
        self.clear_forward_to();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Ack {
    fn eq(&self, other: &Ack) -> bool {
        self.from == other.from &&
        self.forward_to == other.forward_to &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Ack {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct PingReq {
    // message fields
    from: ::protobuf::SingularPtrField<Member>,
    target: ::protobuf::SingularPtrField<Member>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PingReq {}

impl PingReq {
    pub fn new() -> PingReq {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PingReq {
        static mut instance: ::protobuf::lazy::Lazy<PingReq> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PingReq,
        };
        unsafe {
            instance.get(|| {
                PingReq {
                    from: ::protobuf::SingularPtrField::none(),
                    target: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .Member from = 1;

    pub fn clear_from(&mut self) {
        self.from.clear();
    }

    pub fn has_from(&self) -> bool {
        self.from.is_some()
    }

    // Param is passed by value, moved
    pub fn set_from(&mut self, v: Member) {
        self.from = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_from(&mut self) -> &mut Member {
        if self.from.is_none() {
            self.from.set_default();
        };
        self.from.as_mut().unwrap()
    }

    // Take field
    pub fn take_from(&mut self) -> Member {
        self.from.take().unwrap_or_else(|| Member::new())
    }

    pub fn get_from(&self) -> &Member {
        self.from.as_ref().unwrap_or_else(|| Member::default_instance())
    }

    // optional .Member target = 2;

    pub fn clear_target(&mut self) {
        self.target.clear();
    }

    pub fn has_target(&self) -> bool {
        self.target.is_some()
    }

    // Param is passed by value, moved
    pub fn set_target(&mut self, v: Member) {
        self.target = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_target(&mut self) -> &mut Member {
        if self.target.is_none() {
            self.target.set_default();
        };
        self.target.as_mut().unwrap()
    }

    // Take field
    pub fn take_target(&mut self) -> Member {
        self.target.take().unwrap_or_else(|| Member::new())
    }

    pub fn get_target(&self) -> &Member {
        self.target.as_ref().unwrap_or_else(|| Member::default_instance())
    }
}

impl ::protobuf::Message for PingReq {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.from));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.target));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.from {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.target {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.from.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.target.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<PingReq>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PingReq {
    fn new() -> PingReq {
        PingReq::new()
    }

    fn descriptor_static(_: ::std::option::Option<PingReq>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "from",
                    PingReq::has_from,
                    PingReq::get_from,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "target",
                    PingReq::has_target,
                    PingReq::get_target,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PingReq>(
                    "PingReq",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PingReq {
    fn clear(&mut self) {
        self.clear_from();
        self.clear_target();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for PingReq {
    fn eq(&self, other: &PingReq) -> bool {
        self.from == other.from &&
        self.target == other.target &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for PingReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Membership {
    // message fields
    member: ::protobuf::SingularPtrField<Member>,
    health: ::std::option::Option<Membership_Health>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Membership {}

impl Membership {
    pub fn new() -> Membership {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Membership {
        static mut instance: ::protobuf::lazy::Lazy<Membership> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Membership,
        };
        unsafe {
            instance.get(|| {
                Membership {
                    member: ::protobuf::SingularPtrField::none(),
                    health: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .Member member = 1;

    pub fn clear_member(&mut self) {
        self.member.clear();
    }

    pub fn has_member(&self) -> bool {
        self.member.is_some()
    }

    // Param is passed by value, moved
    pub fn set_member(&mut self, v: Member) {
        self.member = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_member(&mut self) -> &mut Member {
        if self.member.is_none() {
            self.member.set_default();
        };
        self.member.as_mut().unwrap()
    }

    // Take field
    pub fn take_member(&mut self) -> Member {
        self.member.take().unwrap_or_else(|| Member::new())
    }

    pub fn get_member(&self) -> &Member {
        self.member.as_ref().unwrap_or_else(|| Member::default_instance())
    }

    // optional .Membership.Health health = 2;

    pub fn clear_health(&mut self) {
        self.health = ::std::option::Option::None;
    }

    pub fn has_health(&self) -> bool {
        self.health.is_some()
    }

    // Param is passed by value, moved
    pub fn set_health(&mut self, v: Membership_Health) {
        self.health = ::std::option::Option::Some(v);
    }

    pub fn get_health(&self) -> Membership_Health {
        self.health.unwrap_or(Membership_Health::ALIVE)
    }
}

impl ::protobuf::Message for Membership {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.member));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.health = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.member {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.health {
            my_size += ::protobuf::rt::enum_size(2, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.member.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.health {
            try!(os.write_enum(2, v.value()));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Membership>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Membership {
    fn new() -> Membership {
        Membership::new()
    }

    fn descriptor_static(_: ::std::option::Option<Membership>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "member",
                    Membership::has_member,
                    Membership::get_member,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "health",
                    Membership::has_health,
                    Membership::get_health,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Membership>(
                    "Membership",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Membership {
    fn clear(&mut self) {
        self.clear_member();
        self.clear_health();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Membership {
    fn eq(&self, other: &Membership) -> bool {
        self.member == other.member &&
        self.health == other.health &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Membership {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Membership_Health {
    ALIVE = 1,
    SUSPECT = 2,
    CONFIRMED = 3,
}

impl ::protobuf::ProtobufEnum for Membership_Health {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Membership_Health> {
        match value {
            1 => ::std::option::Option::Some(Membership_Health::ALIVE),
            2 => ::std::option::Option::Some(Membership_Health::SUSPECT),
            3 => ::std::option::Option::Some(Membership_Health::CONFIRMED),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Membership_Health] = &[
            Membership_Health::ALIVE,
            Membership_Health::SUSPECT,
            Membership_Health::CONFIRMED,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<Membership_Health>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Membership_Health", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Membership_Health {
}

#[derive(Clone,Default)]
pub struct Swim {
    // message fields
    field_type: ::std::option::Option<Swim_Type>,
    ping: ::protobuf::SingularPtrField<Ping>,
    ack: ::protobuf::SingularPtrField<Ack>,
    pingreq: ::protobuf::SingularPtrField<PingReq>,
    membership: ::protobuf::RepeatedField<Membership>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Swim {}

impl Swim {
    pub fn new() -> Swim {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Swim {
        static mut instance: ::protobuf::lazy::Lazy<Swim> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Swim,
        };
        unsafe {
            instance.get(|| {
                Swim {
                    field_type: ::std::option::Option::None,
                    ping: ::protobuf::SingularPtrField::none(),
                    ack: ::protobuf::SingularPtrField::none(),
                    pingreq: ::protobuf::SingularPtrField::none(),
                    membership: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .Swim.Type type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: Swim_Type) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type(&self) -> Swim_Type {
        self.field_type.unwrap_or(Swim_Type::PING)
    }

    // optional .Ping ping = 2;

    pub fn clear_ping(&mut self) {
        self.ping.clear();
    }

    pub fn has_ping(&self) -> bool {
        self.ping.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ping(&mut self, v: Ping) {
        self.ping = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ping(&mut self) -> &mut Ping {
        if self.ping.is_none() {
            self.ping.set_default();
        };
        self.ping.as_mut().unwrap()
    }

    // Take field
    pub fn take_ping(&mut self) -> Ping {
        self.ping.take().unwrap_or_else(|| Ping::new())
    }

    pub fn get_ping(&self) -> &Ping {
        self.ping.as_ref().unwrap_or_else(|| Ping::default_instance())
    }

    // optional .Ack ack = 3;

    pub fn clear_ack(&mut self) {
        self.ack.clear();
    }

    pub fn has_ack(&self) -> bool {
        self.ack.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ack(&mut self, v: Ack) {
        self.ack = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ack(&mut self) -> &mut Ack {
        if self.ack.is_none() {
            self.ack.set_default();
        };
        self.ack.as_mut().unwrap()
    }

    // Take field
    pub fn take_ack(&mut self) -> Ack {
        self.ack.take().unwrap_or_else(|| Ack::new())
    }

    pub fn get_ack(&self) -> &Ack {
        self.ack.as_ref().unwrap_or_else(|| Ack::default_instance())
    }

    // optional .PingReq pingreq = 4;

    pub fn clear_pingreq(&mut self) {
        self.pingreq.clear();
    }

    pub fn has_pingreq(&self) -> bool {
        self.pingreq.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pingreq(&mut self, v: PingReq) {
        self.pingreq = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_pingreq(&mut self) -> &mut PingReq {
        if self.pingreq.is_none() {
            self.pingreq.set_default();
        };
        self.pingreq.as_mut().unwrap()
    }

    // Take field
    pub fn take_pingreq(&mut self) -> PingReq {
        self.pingreq.take().unwrap_or_else(|| PingReq::new())
    }

    pub fn get_pingreq(&self) -> &PingReq {
        self.pingreq.as_ref().unwrap_or_else(|| PingReq::default_instance())
    }

    // repeated .Membership membership = 5;

    pub fn clear_membership(&mut self) {
        self.membership.clear();
    }

    // Param is passed by value, moved
    pub fn set_membership(&mut self, v: ::protobuf::RepeatedField<Membership>) {
        self.membership = v;
    }

    // Mutable pointer to the field.
    pub fn mut_membership(&mut self) -> &mut ::protobuf::RepeatedField<Membership> {
        &mut self.membership
    }

    // Take field
    pub fn take_membership(&mut self) -> ::protobuf::RepeatedField<Membership> {
        ::std::mem::replace(&mut self.membership, ::protobuf::RepeatedField::new())
    }

    pub fn get_membership(&self) -> &[Membership] {
        &self.membership
    }
}

impl ::protobuf::Message for Swim {
    fn is_initialized(&self) -> bool {
        if self.field_type.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.field_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.ping));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.ack));
                },
                4 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.pingreq));
                },
                5 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.membership));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.field_type {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in &self.ping {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.ack {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.pingreq {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.membership {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.field_type {
            try!(os.write_enum(1, v.value()));
        };
        if let Some(v) = self.ping.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.ack.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.pingreq.as_ref() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in &self.membership {
            try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Swim>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Swim {
    fn new() -> Swim {
        Swim::new()
    }

    fn descriptor_static(_: ::std::option::Option<Swim>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "type",
                    Swim::has_field_type,
                    Swim::get_field_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "ping",
                    Swim::has_ping,
                    Swim::get_ping,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "ack",
                    Swim::has_ack,
                    Swim::get_ack,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "pingreq",
                    Swim::has_pingreq,
                    Swim::get_pingreq,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "membership",
                    Swim::get_membership,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Swim>(
                    "Swim",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Swim {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_ping();
        self.clear_ack();
        self.clear_pingreq();
        self.clear_membership();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Swim {
    fn eq(&self, other: &Swim) -> bool {
        self.field_type == other.field_type &&
        self.ping == other.ping &&
        self.ack == other.ack &&
        self.pingreq == other.pingreq &&
        self.membership == other.membership &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Swim {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Swim_Type {
    PING = 1,
    ACK = 2,
    PINGREQ = 3,
}

impl ::protobuf::ProtobufEnum for Swim_Type {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Swim_Type> {
        match value {
            1 => ::std::option::Option::Some(Swim_Type::PING),
            2 => ::std::option::Option::Some(Swim_Type::ACK),
            3 => ::std::option::Option::Some(Swim_Type::PINGREQ),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Swim_Type] = &[
            Swim_Type::PING,
            Swim_Type::ACK,
            Swim_Type::PINGREQ,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<Swim_Type>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Swim_Type", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Swim_Type {
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x14, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x73, 0x2f, 0x73, 0x77, 0x69, 0x6d,
    0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x7b, 0x0a, 0x06, 0x4d, 0x65, 0x6d, 0x62, 0x65, 0x72,
    0x12, 0x0e, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x02, 0x69, 0x64,
    0x12, 0x20, 0x0a, 0x0b, 0x69, 0x6e, 0x63, 0x61, 0x72, 0x6e, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x04, 0x52, 0x0b, 0x69, 0x6e, 0x63, 0x61, 0x72, 0x6e, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x12, 0x18, 0x0a, 0x07, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x18, 0x03, 0x20,
    0x01, 0x28, 0x09, 0x52, 0x07, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x12, 0x25, 0x0a, 0x0a,
    0x70, 0x65, 0x72, 0x73, 0x69, 0x73, 0x74, 0x65, 0x6e, 0x74, 0x18, 0x04, 0x20, 0x01, 0x28, 0x08,
    0x3a, 0x05, 0x66, 0x61, 0x6c, 0x73, 0x65, 0x52, 0x0a, 0x70, 0x65, 0x72, 0x73, 0x69, 0x73, 0x74,
    0x65, 0x6e, 0x74, 0x22, 0x4b, 0x0a, 0x04, 0x50, 0x69, 0x6e, 0x67, 0x12, 0x1b, 0x0a, 0x04, 0x66,
    0x72, 0x6f, 0x6d, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x07, 0x2e, 0x4d, 0x65, 0x6d, 0x62,
    0x65, 0x72, 0x52, 0x04, 0x66, 0x72, 0x6f, 0x6d, 0x12, 0x26, 0x0a, 0x0a, 0x66, 0x6f, 0x72, 0x77,
    0x61, 0x72, 0x64, 0x5f, 0x74, 0x6f, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x07, 0x2e, 0x4d,
    0x65, 0x6d, 0x62, 0x65, 0x72, 0x52, 0x09, 0x66, 0x6f, 0x72, 0x77, 0x61, 0x72, 0x64, 0x54, 0x6f,
    0x22, 0x4a, 0x0a, 0x03, 0x41, 0x63, 0x6b, 0x12, 0x1b, 0x0a, 0x04, 0x66, 0x72, 0x6f, 0x6d, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x07, 0x2e, 0x4d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x52, 0x04,
    0x66, 0x72, 0x6f, 0x6d, 0x12, 0x26, 0x0a, 0x0a, 0x66, 0x6f, 0x72, 0x77, 0x61, 0x72, 0x64, 0x5f,
    0x74, 0x6f, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x07, 0x2e, 0x4d, 0x65, 0x6d, 0x62, 0x65,
    0x72, 0x52, 0x09, 0x66, 0x6f, 0x72, 0x77, 0x61, 0x72, 0x64, 0x54, 0x6f, 0x22, 0x47, 0x0a, 0x07,
    0x50, 0x69, 0x6e, 0x67, 0x52, 0x65, 0x71, 0x12, 0x1b, 0x0a, 0x04, 0x66, 0x72, 0x6f, 0x6d, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x07, 0x2e, 0x4d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x52, 0x04,
    0x66, 0x72, 0x6f, 0x6d, 0x12, 0x1f, 0x0a, 0x06, 0x74, 0x61, 0x72, 0x67, 0x65, 0x74, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x07, 0x2e, 0x4d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x52, 0x06, 0x74,
    0x61, 0x72, 0x67, 0x65, 0x74, 0x22, 0x8a, 0x01, 0x0a, 0x0a, 0x4d, 0x65, 0x6d, 0x62, 0x65, 0x72,
    0x73, 0x68, 0x69, 0x70, 0x12, 0x1f, 0x0a, 0x06, 0x6d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x07, 0x2e, 0x4d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x52, 0x06, 0x6d,
    0x65, 0x6d, 0x62, 0x65, 0x72, 0x12, 0x2a, 0x0a, 0x06, 0x68, 0x65, 0x61, 0x6c, 0x74, 0x68, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x12, 0x2e, 0x4d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x73, 0x68,
    0x69, 0x70, 0x2e, 0x48, 0x65, 0x61, 0x6c, 0x74, 0x68, 0x52, 0x06, 0x68, 0x65, 0x61, 0x6c, 0x74,
    0x68, 0x22, 0x2f, 0x0a, 0x06, 0x48, 0x65, 0x61, 0x6c, 0x74, 0x68, 0x12, 0x09, 0x0a, 0x05, 0x41,
    0x4c, 0x49, 0x56, 0x45, 0x10, 0x01, 0x12, 0x0b, 0x0a, 0x07, 0x53, 0x55, 0x53, 0x50, 0x45, 0x43,
    0x54, 0x10, 0x02, 0x12, 0x0d, 0x0a, 0x09, 0x43, 0x4f, 0x4e, 0x46, 0x49, 0x52, 0x4d, 0x45, 0x44,
    0x10, 0x03, 0x22, 0xd2, 0x01, 0x0a, 0x04, 0x53, 0x77, 0x69, 0x6d, 0x12, 0x1e, 0x0a, 0x04, 0x74,
    0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0e, 0x32, 0x0a, 0x2e, 0x53, 0x77, 0x69, 0x6d,
    0x2e, 0x54, 0x79, 0x70, 0x65, 0x52, 0x04, 0x74, 0x79, 0x70, 0x65, 0x12, 0x19, 0x0a, 0x04, 0x70,
    0x69, 0x6e, 0x67, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x05, 0x2e, 0x50, 0x69, 0x6e, 0x67,
    0x52, 0x04, 0x70, 0x69, 0x6e, 0x67, 0x12, 0x16, 0x0a, 0x03, 0x61, 0x63, 0x6b, 0x18, 0x03, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x04, 0x2e, 0x41, 0x63, 0x6b, 0x52, 0x03, 0x61, 0x63, 0x6b, 0x12, 0x22,
    0x0a, 0x07, 0x70, 0x69, 0x6e, 0x67, 0x72, 0x65, 0x71, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x08, 0x2e, 0x50, 0x69, 0x6e, 0x67, 0x52, 0x65, 0x71, 0x52, 0x07, 0x70, 0x69, 0x6e, 0x67, 0x72,
    0x65, 0x71, 0x12, 0x2b, 0x0a, 0x0a, 0x6d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x73, 0x68, 0x69, 0x70,
    0x18, 0x05, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x0b, 0x2e, 0x4d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x73,
    0x68, 0x69, 0x70, 0x52, 0x0a, 0x6d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x73, 0x68, 0x69, 0x70, 0x22,
    0x26, 0x0a, 0x04, 0x54, 0x79, 0x70, 0x65, 0x12, 0x08, 0x0a, 0x04, 0x50, 0x49, 0x4e, 0x47, 0x10,
    0x01, 0x12, 0x07, 0x0a, 0x03, 0x41, 0x43, 0x4b, 0x10, 0x02, 0x12, 0x0b, 0x0a, 0x07, 0x50, 0x49,
    0x4e, 0x47, 0x52, 0x45, 0x51, 0x10, 0x03, 0x4a, 0xd7, 0x0d, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00,
    0x2a, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x0a, 0x0a, 0x02,
    0x04, 0x00, 0x12, 0x04, 0x02, 0x00, 0x07, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12,
    0x03, 0x02, 0x08, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x03, 0x02,
    0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x03, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x03, 0x0b, 0x11, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x03, 0x12, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x03, 0x17, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x01, 0x12, 0x03, 0x04, 0x02, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12,
    0x03, 0x04, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x04,
    0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x04, 0x12, 0x1d,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x04, 0x20, 0x21, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x05, 0x02, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x05, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x02, 0x05, 0x12, 0x03, 0x05, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01,
    0x12, 0x03, 0x05, 0x12, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03,
    0x05, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x06, 0x02, 0x31,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x04, 0x12, 0x03, 0x06, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x05, 0x12, 0x03, 0x06, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x06, 0x10, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x03, 0x03, 0x12, 0x03, 0x06, 0x1d, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03,
    0x08, 0x12, 0x03, 0x06, 0x1f, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x07, 0x12,
    0x03, 0x06, 0x2a, 0x2f, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x09, 0x00, 0x0c, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x09, 0x08, 0x0c, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x0a, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x00, 0x04, 0x12, 0x03, 0x0a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x06,
    0x12, 0x03, 0x0a, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x0a, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0a, 0x19,
    0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x0b, 0x02, 0x21, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x04, 0x12, 0x03, 0x0b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x01, 0x06, 0x12, 0x03, 0x0b, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x0b, 0x12, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01,
    0x03, 0x12, 0x03, 0x0b, 0x1f, 0x20, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x0e, 0x00,
    0x11, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x0e, 0x08, 0x0b, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x0f, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x0f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x00, 0x06, 0x12, 0x03, 0x0f, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x0f, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x0f, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x10, 0x02, 0x21,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x04, 0x12, 0x03, 0x10, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x06, 0x12, 0x03, 0x10, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x10, 0x12, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x10, 0x1f, 0x20, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04,
    0x13, 0x00, 0x16, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x13, 0x08, 0x0f,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x14, 0x02, 0x1b, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x03, 0x14, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x00, 0x06, 0x12, 0x03, 0x14, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x14, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x14, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x01, 0x12, 0x03, 0x15,
    0x02, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x04, 0x12, 0x03, 0x15, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x06, 0x12, 0x03, 0x15, 0x0b, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x01, 0x12, 0x03, 0x15, 0x12, 0x18, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x01, 0x03, 0x12, 0x03, 0x15, 0x1b, 0x1c, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04,
    0x12, 0x04, 0x18, 0x00, 0x1d, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x18,
    0x08, 0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x04, 0x00, 0x12, 0x03, 0x19, 0x02, 0x38, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x04, 0x00, 0x01, 0x12, 0x03, 0x19, 0x07, 0x0d, 0x0a, 0x0d, 0x0a,
    0x06, 0x04, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x19, 0x10, 0x1a, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x04, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x19, 0x10, 0x15, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x04, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x19, 0x18, 0x19, 0x0a, 0x0d, 0x0a, 0x06,
    0x04, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x19, 0x1b, 0x27, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x04, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x19, 0x1b, 0x22, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x04, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x19, 0x25, 0x26, 0x0a, 0x0d, 0x0a, 0x06, 0x04,
    0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x19, 0x28, 0x36, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04,
    0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x19, 0x28, 0x31, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04,
    0x04, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x19, 0x34, 0x35, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04,
    0x02, 0x00, 0x12, 0x03, 0x1b, 0x02, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x04,
    0x12, 0x03, 0x1b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x06, 0x12, 0x03,
    0x1b, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1b, 0x12,
    0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x1b, 0x1b, 0x1c, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x01, 0x12, 0x03, 0x1c, 0x02, 0x1d, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x01, 0x04, 0x12, 0x03, 0x1c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x01, 0x06, 0x12, 0x03, 0x1c, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x1c, 0x12, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x1c, 0x1b, 0x1c, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x1f, 0x00, 0x2a, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x1f, 0x08, 0x0c, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x05, 0x04, 0x00, 0x12, 0x03, 0x20, 0x02, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x04,
    0x00, 0x01, 0x12, 0x03, 0x20, 0x07, 0x0b, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x12, 0x03, 0x20, 0x0e, 0x17, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x20, 0x0e, 0x12, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x02, 0x12, 0x03, 0x20, 0x15, 0x16, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x05, 0x04, 0x00, 0x02, 0x01,
    0x12, 0x03, 0x20, 0x18, 0x20, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x20, 0x18, 0x1b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05, 0x04, 0x00, 0x02, 0x01, 0x02,
    0x12, 0x03, 0x20, 0x1e, 0x1f, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x05, 0x04, 0x00, 0x02, 0x02, 0x12,
    0x03, 0x20, 0x21, 0x2d, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12,
    0x03, 0x20, 0x21, 0x28, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05, 0x04, 0x00, 0x02, 0x02, 0x02, 0x12,
    0x03, 0x20, 0x2b, 0x2c, 0x0a, 0x33, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x00, 0x12, 0x03, 0x23, 0x02,
    0x19, 0x1a, 0x26, 0x20, 0x49, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x66, 0x69, 0x65, 0x73, 0x20, 0x77,
    0x68, 0x69, 0x63, 0x68, 0x20, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x20, 0x69, 0x73, 0x20, 0x66, 0x69,
    0x6c, 0x6c, 0x65, 0x64, 0x20, 0x69, 0x6e, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x00, 0x04, 0x12, 0x03, 0x23, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x06,
    0x12, 0x03, 0x23, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x23, 0x10, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x23, 0x17,
    0x18, 0x0a, 0x17, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x01, 0x12, 0x03, 0x26, 0x02, 0x19, 0x1a, 0x0a,
    0x20, 0x4f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x01, 0x04, 0x12, 0x03, 0x26, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01,
    0x06, 0x12, 0x03, 0x26, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x26, 0x10, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x03, 0x12, 0x03, 0x26,
    0x17, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x02, 0x12, 0x03, 0x27, 0x02, 0x17, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x04, 0x12, 0x03, 0x27, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x02, 0x06, 0x12, 0x03, 0x27, 0x0b, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x02, 0x01, 0x12, 0x03, 0x27, 0x0f, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x02, 0x03, 0x12, 0x03, 0x27, 0x15, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x03, 0x12,
    0x03, 0x28, 0x02, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x04, 0x12, 0x03, 0x28,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x06, 0x12, 0x03, 0x28, 0x0b, 0x12,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x01, 0x12, 0x03, 0x28, 0x13, 0x1a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x03, 0x12, 0x03, 0x28, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x05, 0x02, 0x04, 0x12, 0x03, 0x29, 0x02, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x04, 0x04, 0x12, 0x03, 0x29, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x04, 0x06,
    0x12, 0x03, 0x29, 0x0b, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x04, 0x01, 0x12, 0x03,
    0x29, 0x16, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x04, 0x03, 0x12, 0x03, 0x29, 0x23,
    0x24,
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
