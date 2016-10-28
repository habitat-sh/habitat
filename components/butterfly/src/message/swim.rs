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
    swim_port: ::std::option::Option<i32>,
    gossip_port: ::std::option::Option<i32>,
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
                    swim_port: ::std::option::Option::None,
                    gossip_port: ::std::option::Option::None,
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

    // optional int32 swim_port = 4;

    pub fn clear_swim_port(&mut self) {
        self.swim_port = ::std::option::Option::None;
    }

    pub fn has_swim_port(&self) -> bool {
        self.swim_port.is_some()
    }

    // Param is passed by value, moved
    pub fn set_swim_port(&mut self, v: i32) {
        self.swim_port = ::std::option::Option::Some(v);
    }

    pub fn get_swim_port(&self) -> i32 {
        self.swim_port.unwrap_or(0)
    }

    // optional int32 gossip_port = 5;

    pub fn clear_gossip_port(&mut self) {
        self.gossip_port = ::std::option::Option::None;
    }

    pub fn has_gossip_port(&self) -> bool {
        self.gossip_port.is_some()
    }

    // Param is passed by value, moved
    pub fn set_gossip_port(&mut self, v: i32) {
        self.gossip_port = ::std::option::Option::Some(v);
    }

    pub fn get_gossip_port(&self) -> i32 {
        self.gossip_port.unwrap_or(0)
    }

    // optional bool persistent = 6;

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
                    let tmp = try!(is.read_int32());
                    self.swim_port = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.gossip_port = ::std::option::Option::Some(tmp);
                },
                6 => {
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
        for value in &self.swim_port {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.gossip_port {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
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
        if let Some(v) = self.swim_port {
            try!(os.write_int32(4, v));
        };
        if let Some(v) = self.gossip_port {
            try!(os.write_int32(5, v));
        };
        if let Some(v) = self.persistent {
            try!(os.write_bool(6, v));
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
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "swim_port",
                    Member::has_swim_port,
                    Member::get_swim_port,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "gossip_port",
                    Member::has_gossip_port,
                    Member::get_gossip_port,
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
        self.clear_swim_port();
        self.clear_gossip_port();
        self.clear_persistent();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Member {
    fn eq(&self, other: &Member) -> bool {
        self.id == other.id &&
        self.incarnation == other.incarnation &&
        self.address == other.address &&
        self.swim_port == other.swim_port &&
        self.gossip_port == other.gossip_port &&
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
pub struct Election {
    // message fields
    member_id: ::protobuf::SingularField<::std::string::String>,
    service_group: ::protobuf::SingularField<::std::string::String>,
    term: ::std::option::Option<u64>,
    suitability: ::std::option::Option<u64>,
    status: ::std::option::Option<Election_Status>,
    votes: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Election {}

impl Election {
    pub fn new() -> Election {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Election {
        static mut instance: ::protobuf::lazy::Lazy<Election> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Election,
        };
        unsafe {
            instance.get(|| {
                Election {
                    member_id: ::protobuf::SingularField::none(),
                    service_group: ::protobuf::SingularField::none(),
                    term: ::std::option::Option::None,
                    suitability: ::std::option::Option::None,
                    status: ::std::option::Option::None,
                    votes: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
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
        };
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

    // optional string service_group = 2;

    pub fn clear_service_group(&mut self) {
        self.service_group.clear();
    }

    pub fn has_service_group(&self) -> bool {
        self.service_group.is_some()
    }

    // Param is passed by value, moved
    pub fn set_service_group(&mut self, v: ::std::string::String) {
        self.service_group = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_service_group(&mut self) -> &mut ::std::string::String {
        if self.service_group.is_none() {
            self.service_group.set_default();
        };
        self.service_group.as_mut().unwrap()
    }

    // Take field
    pub fn take_service_group(&mut self) -> ::std::string::String {
        self.service_group.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_service_group(&self) -> &str {
        match self.service_group.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional uint64 term = 3;

    pub fn clear_term(&mut self) {
        self.term = ::std::option::Option::None;
    }

    pub fn has_term(&self) -> bool {
        self.term.is_some()
    }

    // Param is passed by value, moved
    pub fn set_term(&mut self, v: u64) {
        self.term = ::std::option::Option::Some(v);
    }

    pub fn get_term(&self) -> u64 {
        self.term.unwrap_or(0)
    }

    // optional uint64 suitability = 4;

    pub fn clear_suitability(&mut self) {
        self.suitability = ::std::option::Option::None;
    }

    pub fn has_suitability(&self) -> bool {
        self.suitability.is_some()
    }

    // Param is passed by value, moved
    pub fn set_suitability(&mut self, v: u64) {
        self.suitability = ::std::option::Option::Some(v);
    }

    pub fn get_suitability(&self) -> u64 {
        self.suitability.unwrap_or(0)
    }

    // optional .Election.Status status = 5;

    pub fn clear_status(&mut self) {
        self.status = ::std::option::Option::None;
    }

    pub fn has_status(&self) -> bool {
        self.status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: Election_Status) {
        self.status = ::std::option::Option::Some(v);
    }

    pub fn get_status(&self) -> Election_Status {
        self.status.unwrap_or(Election_Status::Running)
    }

    // repeated string votes = 6;

    pub fn clear_votes(&mut self) {
        self.votes.clear();
    }

    // Param is passed by value, moved
    pub fn set_votes(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.votes = v;
    }

    // Mutable pointer to the field.
    pub fn mut_votes(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.votes
    }

    // Take field
    pub fn take_votes(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.votes, ::protobuf::RepeatedField::new())
    }

    pub fn get_votes(&self) -> &[::std::string::String] {
        &self.votes
    }
}

impl ::protobuf::Message for Election {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.member_id));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.service_group));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.term = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.suitability = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.status = ::std::option::Option::Some(tmp);
                },
                6 => {
                    try!(::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.votes));
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
        for value in &self.member_id {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.service_group {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in &self.term {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.suitability {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.status {
            my_size += ::protobuf::rt::enum_size(5, *value);
        };
        for value in &self.votes {
            my_size += ::protobuf::rt::string_size(6, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.member_id.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.service_group.as_ref() {
            try!(os.write_string(2, &v));
        };
        if let Some(v) = self.term {
            try!(os.write_uint64(3, v));
        };
        if let Some(v) = self.suitability {
            try!(os.write_uint64(4, v));
        };
        if let Some(v) = self.status {
            try!(os.write_enum(5, v.value()));
        };
        for v in &self.votes {
            try!(os.write_string(6, &v));
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
        ::std::any::TypeId::of::<Election>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Election {
    fn new() -> Election {
        Election::new()
    }

    fn descriptor_static(_: ::std::option::Option<Election>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "member_id",
                    Election::has_member_id,
                    Election::get_member_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "service_group",
                    Election::has_service_group,
                    Election::get_service_group,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "term",
                    Election::has_term,
                    Election::get_term,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "suitability",
                    Election::has_suitability,
                    Election::get_suitability,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "status",
                    Election::has_status,
                    Election::get_status,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_string_accessor(
                    "votes",
                    Election::get_votes,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Election>(
                    "Election",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Election {
    fn clear(&mut self) {
        self.clear_member_id();
        self.clear_service_group();
        self.clear_term();
        self.clear_suitability();
        self.clear_status();
        self.clear_votes();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Election {
    fn eq(&self, other: &Election) -> bool {
        self.member_id == other.member_id &&
        self.service_group == other.service_group &&
        self.term == other.term &&
        self.suitability == other.suitability &&
        self.status == other.status &&
        self.votes == other.votes &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Election {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Election_Status {
    Running = 1,
    NoQuorum = 2,
    Finished = 3,
}

impl ::protobuf::ProtobufEnum for Election_Status {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Election_Status> {
        match value {
            1 => ::std::option::Option::Some(Election_Status::Running),
            2 => ::std::option::Option::Some(Election_Status::NoQuorum),
            3 => ::std::option::Option::Some(Election_Status::Finished),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Election_Status] = &[
            Election_Status::Running,
            Election_Status::NoQuorum,
            Election_Status::Finished,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<Election_Status>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Election_Status", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Election_Status {
}

#[derive(Clone,Default)]
pub struct Service {
    // message fields
    member_id: ::protobuf::SingularField<::std::string::String>,
    service_group: ::protobuf::SingularField<::std::string::String>,
    incarnation: ::std::option::Option<u64>,
    ip: ::protobuf::SingularField<::std::string::String>,
    hostname: ::protobuf::SingularField<::std::string::String>,
    port: ::std::option::Option<u32>,
    exposes: ::std::vec::Vec<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Service {}

impl Service {
    pub fn new() -> Service {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Service {
        static mut instance: ::protobuf::lazy::Lazy<Service> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Service,
        };
        unsafe {
            instance.get(|| {
                Service {
                    member_id: ::protobuf::SingularField::none(),
                    service_group: ::protobuf::SingularField::none(),
                    incarnation: ::std::option::Option::None,
                    ip: ::protobuf::SingularField::none(),
                    hostname: ::protobuf::SingularField::none(),
                    port: ::std::option::Option::None,
                    exposes: ::std::vec::Vec::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
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
        };
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

    // optional string service_group = 2;

    pub fn clear_service_group(&mut self) {
        self.service_group.clear();
    }

    pub fn has_service_group(&self) -> bool {
        self.service_group.is_some()
    }

    // Param is passed by value, moved
    pub fn set_service_group(&mut self, v: ::std::string::String) {
        self.service_group = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_service_group(&mut self) -> &mut ::std::string::String {
        if self.service_group.is_none() {
            self.service_group.set_default();
        };
        self.service_group.as_mut().unwrap()
    }

    // Take field
    pub fn take_service_group(&mut self) -> ::std::string::String {
        self.service_group.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_service_group(&self) -> &str {
        match self.service_group.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional uint64 incarnation = 3;

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

    // optional string ip = 4;

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
        };
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

    // optional string hostname = 5;

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
        };
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

    // optional uint32 port = 6;

    pub fn clear_port(&mut self) {
        self.port = ::std::option::Option::None;
    }

    pub fn has_port(&self) -> bool {
        self.port.is_some()
    }

    // Param is passed by value, moved
    pub fn set_port(&mut self, v: u32) {
        self.port = ::std::option::Option::Some(v);
    }

    pub fn get_port(&self) -> u32 {
        self.port.unwrap_or(0)
    }

    // repeated uint32 exposes = 7;

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
}

impl ::protobuf::Message for Service {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.member_id));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.service_group));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.incarnation = ::std::option::Option::Some(tmp);
                },
                4 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.ip));
                },
                5 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.hostname));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.port = ::std::option::Option::Some(tmp);
                },
                7 => {
                    try!(::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.exposes));
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
        for value in &self.member_id {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.service_group {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in &self.incarnation {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.ip {
            my_size += ::protobuf::rt::string_size(4, &value);
        };
        for value in &self.hostname {
            my_size += ::protobuf::rt::string_size(5, &value);
        };
        for value in &self.port {
            my_size += ::protobuf::rt::value_size(6, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.exposes {
            my_size += ::protobuf::rt::value_size(7, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.member_id.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.service_group.as_ref() {
            try!(os.write_string(2, &v));
        };
        if let Some(v) = self.incarnation {
            try!(os.write_uint64(3, v));
        };
        if let Some(v) = self.ip.as_ref() {
            try!(os.write_string(4, &v));
        };
        if let Some(v) = self.hostname.as_ref() {
            try!(os.write_string(5, &v));
        };
        if let Some(v) = self.port {
            try!(os.write_uint32(6, v));
        };
        for v in &self.exposes {
            try!(os.write_uint32(7, *v));
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
        ::std::any::TypeId::of::<Service>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Service {
    fn new() -> Service {
        Service::new()
    }

    fn descriptor_static(_: ::std::option::Option<Service>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "member_id",
                    Service::has_member_id,
                    Service::get_member_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "service_group",
                    Service::has_service_group,
                    Service::get_service_group,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "incarnation",
                    Service::has_incarnation,
                    Service::get_incarnation,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "ip",
                    Service::has_ip,
                    Service::get_ip,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "hostname",
                    Service::has_hostname,
                    Service::get_hostname,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "port",
                    Service::has_port,
                    Service::get_port,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_u32_accessor(
                    "exposes",
                    Service::get_exposes,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Service>(
                    "Service",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Service {
    fn clear(&mut self) {
        self.clear_member_id();
        self.clear_service_group();
        self.clear_incarnation();
        self.clear_ip();
        self.clear_hostname();
        self.clear_port();
        self.clear_exposes();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Service {
    fn eq(&self, other: &Service) -> bool {
        self.member_id == other.member_id &&
        self.service_group == other.service_group &&
        self.incarnation == other.incarnation &&
        self.ip == other.ip &&
        self.hostname == other.hostname &&
        self.port == other.port &&
        self.exposes == other.exposes &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Service {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
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

#[derive(Clone,Default)]
pub struct Rumor {
    // message fields
    field_type: ::std::option::Option<Rumor_Type>,
    tag: ::protobuf::RepeatedField<::std::string::String>,
    from_id: ::protobuf::SingularField<::std::string::String>,
    member: ::protobuf::SingularPtrField<Membership>,
    service: ::protobuf::SingularPtrField<Service>,
    election: ::protobuf::SingularPtrField<Election>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Rumor {}

impl Rumor {
    pub fn new() -> Rumor {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Rumor {
        static mut instance: ::protobuf::lazy::Lazy<Rumor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Rumor,
        };
        unsafe {
            instance.get(|| {
                Rumor {
                    field_type: ::std::option::Option::None,
                    tag: ::protobuf::RepeatedField::new(),
                    from_id: ::protobuf::SingularField::none(),
                    member: ::protobuf::SingularPtrField::none(),
                    service: ::protobuf::SingularPtrField::none(),
                    election: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .Rumor.Type type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: Rumor_Type) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type(&self) -> Rumor_Type {
        self.field_type.unwrap_or(Rumor_Type::Member)
    }

    // repeated string tag = 2;

    pub fn clear_tag(&mut self) {
        self.tag.clear();
    }

    // Param is passed by value, moved
    pub fn set_tag(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.tag = v;
    }

    // Mutable pointer to the field.
    pub fn mut_tag(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.tag
    }

    // Take field
    pub fn take_tag(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.tag, ::protobuf::RepeatedField::new())
    }

    pub fn get_tag(&self) -> &[::std::string::String] {
        &self.tag
    }

    // optional string from_id = 3;

    pub fn clear_from_id(&mut self) {
        self.from_id.clear();
    }

    pub fn has_from_id(&self) -> bool {
        self.from_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_from_id(&mut self, v: ::std::string::String) {
        self.from_id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_from_id(&mut self) -> &mut ::std::string::String {
        if self.from_id.is_none() {
            self.from_id.set_default();
        };
        self.from_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_from_id(&mut self) -> ::std::string::String {
        self.from_id.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_from_id(&self) -> &str {
        match self.from_id.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional .Membership member = 4;

    pub fn clear_member(&mut self) {
        self.member.clear();
    }

    pub fn has_member(&self) -> bool {
        self.member.is_some()
    }

    // Param is passed by value, moved
    pub fn set_member(&mut self, v: Membership) {
        self.member = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_member(&mut self) -> &mut Membership {
        if self.member.is_none() {
            self.member.set_default();
        };
        self.member.as_mut().unwrap()
    }

    // Take field
    pub fn take_member(&mut self) -> Membership {
        self.member.take().unwrap_or_else(|| Membership::new())
    }

    pub fn get_member(&self) -> &Membership {
        self.member.as_ref().unwrap_or_else(|| Membership::default_instance())
    }

    // optional .Service service = 5;

    pub fn clear_service(&mut self) {
        self.service.clear();
    }

    pub fn has_service(&self) -> bool {
        self.service.is_some()
    }

    // Param is passed by value, moved
    pub fn set_service(&mut self, v: Service) {
        self.service = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_service(&mut self) -> &mut Service {
        if self.service.is_none() {
            self.service.set_default();
        };
        self.service.as_mut().unwrap()
    }

    // Take field
    pub fn take_service(&mut self) -> Service {
        self.service.take().unwrap_or_else(|| Service::new())
    }

    pub fn get_service(&self) -> &Service {
        self.service.as_ref().unwrap_or_else(|| Service::default_instance())
    }

    // optional .Election election = 6;

    pub fn clear_election(&mut self) {
        self.election.clear();
    }

    pub fn has_election(&self) -> bool {
        self.election.is_some()
    }

    // Param is passed by value, moved
    pub fn set_election(&mut self, v: Election) {
        self.election = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_election(&mut self) -> &mut Election {
        if self.election.is_none() {
            self.election.set_default();
        };
        self.election.as_mut().unwrap()
    }

    // Take field
    pub fn take_election(&mut self) -> Election {
        self.election.take().unwrap_or_else(|| Election::new())
    }

    pub fn get_election(&self) -> &Election {
        self.election.as_ref().unwrap_or_else(|| Election::default_instance())
    }
}

impl ::protobuf::Message for Rumor {
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
                    try!(::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.tag));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.from_id));
                },
                4 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.member));
                },
                5 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.service));
                },
                6 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.election));
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
        for value in &self.tag {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in &self.from_id {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        for value in &self.member {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.service {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.election {
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
        for v in &self.tag {
            try!(os.write_string(2, &v));
        };
        if let Some(v) = self.from_id.as_ref() {
            try!(os.write_string(3, &v));
        };
        if let Some(v) = self.member.as_ref() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.service.as_ref() {
            try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.election.as_ref() {
            try!(os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited));
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
        ::std::any::TypeId::of::<Rumor>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Rumor {
    fn new() -> Rumor {
        Rumor::new()
    }

    fn descriptor_static(_: ::std::option::Option<Rumor>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "type",
                    Rumor::has_field_type,
                    Rumor::get_field_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_string_accessor(
                    "tag",
                    Rumor::get_tag,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "from_id",
                    Rumor::has_from_id,
                    Rumor::get_from_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "member",
                    Rumor::has_member,
                    Rumor::get_member,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "service",
                    Rumor::has_service,
                    Rumor::get_service,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "election",
                    Rumor::has_election,
                    Rumor::get_election,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Rumor>(
                    "Rumor",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Rumor {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_tag();
        self.clear_from_id();
        self.clear_member();
        self.clear_service();
        self.clear_election();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Rumor {
    fn eq(&self, other: &Rumor) -> bool {
        self.field_type == other.field_type &&
        self.tag == other.tag &&
        self.from_id == other.from_id &&
        self.member == other.member &&
        self.service == other.service &&
        self.election == other.election &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Rumor {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Rumor_Type {
    Member = 1,
    Service = 2,
    Election = 3,
    Fake = 4,
    Fake2 = 5,
}

impl ::protobuf::ProtobufEnum for Rumor_Type {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Rumor_Type> {
        match value {
            1 => ::std::option::Option::Some(Rumor_Type::Member),
            2 => ::std::option::Option::Some(Rumor_Type::Service),
            3 => ::std::option::Option::Some(Rumor_Type::Election),
            4 => ::std::option::Option::Some(Rumor_Type::Fake),
            5 => ::std::option::Option::Some(Rumor_Type::Fake2),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Rumor_Type] = &[
            Rumor_Type::Member,
            Rumor_Type::Service,
            Rumor_Type::Election,
            Rumor_Type::Fake,
            Rumor_Type::Fake2,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<Rumor_Type>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Rumor_Type", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Rumor_Type {
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x14, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x73, 0x2f, 0x73, 0x77, 0x69, 0x6d,
    0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0xb9, 0x01, 0x0a, 0x06, 0x4d, 0x65, 0x6d, 0x62, 0x65,
    0x72, 0x12, 0x0e, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x02, 0x69,
    0x64, 0x12, 0x20, 0x0a, 0x0b, 0x69, 0x6e, 0x63, 0x61, 0x72, 0x6e, 0x61, 0x74, 0x69, 0x6f, 0x6e,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x04, 0x52, 0x0b, 0x69, 0x6e, 0x63, 0x61, 0x72, 0x6e, 0x61, 0x74,
    0x69, 0x6f, 0x6e, 0x12, 0x18, 0x0a, 0x07, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x18, 0x03,
    0x20, 0x01, 0x28, 0x09, 0x52, 0x07, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x12, 0x1b, 0x0a,
    0x09, 0x73, 0x77, 0x69, 0x6d, 0x5f, 0x70, 0x6f, 0x72, 0x74, 0x18, 0x04, 0x20, 0x01, 0x28, 0x05,
    0x52, 0x08, 0x73, 0x77, 0x69, 0x6d, 0x50, 0x6f, 0x72, 0x74, 0x12, 0x1f, 0x0a, 0x0b, 0x67, 0x6f,
    0x73, 0x73, 0x69, 0x70, 0x5f, 0x70, 0x6f, 0x72, 0x74, 0x18, 0x05, 0x20, 0x01, 0x28, 0x05, 0x52,
    0x0a, 0x67, 0x6f, 0x73, 0x73, 0x69, 0x70, 0x50, 0x6f, 0x72, 0x74, 0x12, 0x25, 0x0a, 0x0a, 0x70,
    0x65, 0x72, 0x73, 0x69, 0x73, 0x74, 0x65, 0x6e, 0x74, 0x18, 0x06, 0x20, 0x01, 0x28, 0x08, 0x3a,
    0x05, 0x66, 0x61, 0x6c, 0x73, 0x65, 0x52, 0x0a, 0x70, 0x65, 0x72, 0x73, 0x69, 0x73, 0x74, 0x65,
    0x6e, 0x74, 0x22, 0x4b, 0x0a, 0x04, 0x50, 0x69, 0x6e, 0x67, 0x12, 0x1b, 0x0a, 0x04, 0x66, 0x72,
    0x6f, 0x6d, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x07, 0x2e, 0x4d, 0x65, 0x6d, 0x62, 0x65,
    0x72, 0x52, 0x04, 0x66, 0x72, 0x6f, 0x6d, 0x12, 0x26, 0x0a, 0x0a, 0x66, 0x6f, 0x72, 0x77, 0x61,
    0x72, 0x64, 0x5f, 0x74, 0x6f, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x07, 0x2e, 0x4d, 0x65,
    0x6d, 0x62, 0x65, 0x72, 0x52, 0x09, 0x66, 0x6f, 0x72, 0x77, 0x61, 0x72, 0x64, 0x54, 0x6f, 0x22,
    0x4a, 0x0a, 0x03, 0x41, 0x63, 0x6b, 0x12, 0x1b, 0x0a, 0x04, 0x66, 0x72, 0x6f, 0x6d, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x07, 0x2e, 0x4d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x52, 0x04, 0x66,
    0x72, 0x6f, 0x6d, 0x12, 0x26, 0x0a, 0x0a, 0x66, 0x6f, 0x72, 0x77, 0x61, 0x72, 0x64, 0x5f, 0x74,
    0x6f, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x07, 0x2e, 0x4d, 0x65, 0x6d, 0x62, 0x65, 0x72,
    0x52, 0x09, 0x66, 0x6f, 0x72, 0x77, 0x61, 0x72, 0x64, 0x54, 0x6f, 0x22, 0x47, 0x0a, 0x07, 0x50,
    0x69, 0x6e, 0x67, 0x52, 0x65, 0x71, 0x12, 0x1b, 0x0a, 0x04, 0x66, 0x72, 0x6f, 0x6d, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x07, 0x2e, 0x4d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x52, 0x04, 0x66,
    0x72, 0x6f, 0x6d, 0x12, 0x1f, 0x0a, 0x06, 0x74, 0x61, 0x72, 0x67, 0x65, 0x74, 0x18, 0x02, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x07, 0x2e, 0x4d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x52, 0x06, 0x74, 0x61,
    0x72, 0x67, 0x65, 0x74, 0x22, 0x8a, 0x01, 0x0a, 0x0a, 0x4d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x73,
    0x68, 0x69, 0x70, 0x12, 0x1f, 0x0a, 0x06, 0x6d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x07, 0x2e, 0x4d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x52, 0x06, 0x6d, 0x65,
    0x6d, 0x62, 0x65, 0x72, 0x12, 0x2a, 0x0a, 0x06, 0x68, 0x65, 0x61, 0x6c, 0x74, 0x68, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x0e, 0x32, 0x12, 0x2e, 0x4d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x73, 0x68, 0x69,
    0x70, 0x2e, 0x48, 0x65, 0x61, 0x6c, 0x74, 0x68, 0x52, 0x06, 0x68, 0x65, 0x61, 0x6c, 0x74, 0x68,
    0x22, 0x2f, 0x0a, 0x06, 0x48, 0x65, 0x61, 0x6c, 0x74, 0x68, 0x12, 0x09, 0x0a, 0x05, 0x41, 0x4c,
    0x49, 0x56, 0x45, 0x10, 0x01, 0x12, 0x0b, 0x0a, 0x07, 0x53, 0x55, 0x53, 0x50, 0x45, 0x43, 0x54,
    0x10, 0x02, 0x12, 0x0d, 0x0a, 0x09, 0x43, 0x4f, 0x4e, 0x46, 0x49, 0x52, 0x4d, 0x45, 0x44, 0x10,
    0x03, 0x22, 0xf5, 0x01, 0x0a, 0x08, 0x45, 0x6c, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x1b,
    0x0a, 0x09, 0x6d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x09, 0x52, 0x08, 0x6d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x49, 0x64, 0x12, 0x23, 0x0a, 0x0d, 0x73,
    0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x5f, 0x67, 0x72, 0x6f, 0x75, 0x70, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x09, 0x52, 0x0c, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x47, 0x72, 0x6f, 0x75, 0x70,
    0x12, 0x12, 0x0a, 0x04, 0x74, 0x65, 0x72, 0x6d, 0x18, 0x03, 0x20, 0x01, 0x28, 0x04, 0x52, 0x04,
    0x74, 0x65, 0x72, 0x6d, 0x12, 0x20, 0x0a, 0x0b, 0x73, 0x75, 0x69, 0x74, 0x61, 0x62, 0x69, 0x6c,
    0x69, 0x74, 0x79, 0x18, 0x04, 0x20, 0x01, 0x28, 0x04, 0x52, 0x0b, 0x73, 0x75, 0x69, 0x74, 0x61,
    0x62, 0x69, 0x6c, 0x69, 0x74, 0x79, 0x12, 0x28, 0x0a, 0x06, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73,
    0x18, 0x05, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x10, 0x2e, 0x45, 0x6c, 0x65, 0x63, 0x74, 0x69, 0x6f,
    0x6e, 0x2e, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x52, 0x06, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73,
    0x12, 0x14, 0x0a, 0x05, 0x76, 0x6f, 0x74, 0x65, 0x73, 0x18, 0x06, 0x20, 0x03, 0x28, 0x09, 0x52,
    0x05, 0x76, 0x6f, 0x74, 0x65, 0x73, 0x22, 0x31, 0x0a, 0x06, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73,
    0x12, 0x0b, 0x0a, 0x07, 0x52, 0x75, 0x6e, 0x6e, 0x69, 0x6e, 0x67, 0x10, 0x01, 0x12, 0x0c, 0x0a,
    0x08, 0x4e, 0x6f, 0x51, 0x75, 0x6f, 0x72, 0x75, 0x6d, 0x10, 0x02, 0x12, 0x0c, 0x0a, 0x08, 0x46,
    0x69, 0x6e, 0x69, 0x73, 0x68, 0x65, 0x64, 0x10, 0x03, 0x22, 0xc7, 0x01, 0x0a, 0x07, 0x53, 0x65,
    0x72, 0x76, 0x69, 0x63, 0x65, 0x12, 0x1b, 0x0a, 0x09, 0x6d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x5f,
    0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x08, 0x6d, 0x65, 0x6d, 0x62, 0x65, 0x72,
    0x49, 0x64, 0x12, 0x23, 0x0a, 0x0d, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x5f, 0x67, 0x72,
    0x6f, 0x75, 0x70, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0c, 0x73, 0x65, 0x72, 0x76, 0x69,
    0x63, 0x65, 0x47, 0x72, 0x6f, 0x75, 0x70, 0x12, 0x20, 0x0a, 0x0b, 0x69, 0x6e, 0x63, 0x61, 0x72,
    0x6e, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x03, 0x20, 0x01, 0x28, 0x04, 0x52, 0x0b, 0x69, 0x6e,
    0x63, 0x61, 0x72, 0x6e, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x0e, 0x0a, 0x02, 0x69, 0x70, 0x18,
    0x04, 0x20, 0x01, 0x28, 0x09, 0x52, 0x02, 0x69, 0x70, 0x12, 0x1a, 0x0a, 0x08, 0x68, 0x6f, 0x73,
    0x74, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x05, 0x20, 0x01, 0x28, 0x09, 0x52, 0x08, 0x68, 0x6f, 0x73,
    0x74, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x12, 0x0a, 0x04, 0x70, 0x6f, 0x72, 0x74, 0x18, 0x06, 0x20,
    0x01, 0x28, 0x0d, 0x52, 0x04, 0x70, 0x6f, 0x72, 0x74, 0x12, 0x18, 0x0a, 0x07, 0x65, 0x78, 0x70,
    0x6f, 0x73, 0x65, 0x73, 0x18, 0x07, 0x20, 0x03, 0x28, 0x0d, 0x52, 0x07, 0x65, 0x78, 0x70, 0x6f,
    0x73, 0x65, 0x73, 0x22, 0xd2, 0x01, 0x0a, 0x04, 0x53, 0x77, 0x69, 0x6d, 0x12, 0x1e, 0x0a, 0x04,
    0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0e, 0x32, 0x0a, 0x2e, 0x53, 0x77, 0x69,
    0x6d, 0x2e, 0x54, 0x79, 0x70, 0x65, 0x52, 0x04, 0x74, 0x79, 0x70, 0x65, 0x12, 0x19, 0x0a, 0x04,
    0x70, 0x69, 0x6e, 0x67, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x05, 0x2e, 0x50, 0x69, 0x6e,
    0x67, 0x52, 0x04, 0x70, 0x69, 0x6e, 0x67, 0x12, 0x16, 0x0a, 0x03, 0x61, 0x63, 0x6b, 0x18, 0x03,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x04, 0x2e, 0x41, 0x63, 0x6b, 0x52, 0x03, 0x61, 0x63, 0x6b, 0x12,
    0x22, 0x0a, 0x07, 0x70, 0x69, 0x6e, 0x67, 0x72, 0x65, 0x71, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x08, 0x2e, 0x50, 0x69, 0x6e, 0x67, 0x52, 0x65, 0x71, 0x52, 0x07, 0x70, 0x69, 0x6e, 0x67,
    0x72, 0x65, 0x71, 0x12, 0x2b, 0x0a, 0x0a, 0x6d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x73, 0x68, 0x69,
    0x70, 0x18, 0x05, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x0b, 0x2e, 0x4d, 0x65, 0x6d, 0x62, 0x65, 0x72,
    0x73, 0x68, 0x69, 0x70, 0x52, 0x0a, 0x6d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x73, 0x68, 0x69, 0x70,
    0x22, 0x26, 0x0a, 0x04, 0x54, 0x79, 0x70, 0x65, 0x12, 0x08, 0x0a, 0x04, 0x50, 0x49, 0x4e, 0x47,
    0x10, 0x01, 0x12, 0x07, 0x0a, 0x03, 0x41, 0x43, 0x4b, 0x10, 0x02, 0x12, 0x0b, 0x0a, 0x07, 0x50,
    0x49, 0x4e, 0x47, 0x52, 0x45, 0x51, 0x10, 0x03, 0x22, 0x87, 0x02, 0x0a, 0x05, 0x52, 0x75, 0x6d,
    0x6f, 0x72, 0x12, 0x1f, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0e,
    0x32, 0x0b, 0x2e, 0x52, 0x75, 0x6d, 0x6f, 0x72, 0x2e, 0x54, 0x79, 0x70, 0x65, 0x52, 0x04, 0x74,
    0x79, 0x70, 0x65, 0x12, 0x10, 0x0a, 0x03, 0x74, 0x61, 0x67, 0x18, 0x02, 0x20, 0x03, 0x28, 0x09,
    0x52, 0x03, 0x74, 0x61, 0x67, 0x12, 0x17, 0x0a, 0x07, 0x66, 0x72, 0x6f, 0x6d, 0x5f, 0x69, 0x64,
    0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x06, 0x66, 0x72, 0x6f, 0x6d, 0x49, 0x64, 0x12, 0x23,
    0x0a, 0x06, 0x6d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0b,
    0x2e, 0x4d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x73, 0x68, 0x69, 0x70, 0x52, 0x06, 0x6d, 0x65, 0x6d,
    0x62, 0x65, 0x72, 0x12, 0x22, 0x0a, 0x07, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x18, 0x05,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x08, 0x2e, 0x53, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x52, 0x07,
    0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x12, 0x25, 0x0a, 0x08, 0x65, 0x6c, 0x65, 0x63, 0x74,
    0x69, 0x6f, 0x6e, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x09, 0x2e, 0x45, 0x6c, 0x65, 0x63,
    0x74, 0x69, 0x6f, 0x6e, 0x52, 0x08, 0x65, 0x6c, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x22, 0x42,
    0x0a, 0x04, 0x54, 0x79, 0x70, 0x65, 0x12, 0x0a, 0x0a, 0x06, 0x4d, 0x65, 0x6d, 0x62, 0x65, 0x72,
    0x10, 0x01, 0x12, 0x0b, 0x0a, 0x07, 0x53, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x10, 0x02, 0x12,
    0x0c, 0x0a, 0x08, 0x45, 0x6c, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x10, 0x03, 0x12, 0x08, 0x0a,
    0x04, 0x46, 0x61, 0x6b, 0x65, 0x10, 0x04, 0x12, 0x09, 0x0a, 0x05, 0x46, 0x61, 0x6b, 0x65, 0x32,
    0x10, 0x05, 0x4a, 0x9b, 0x1d, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x54, 0x01, 0x0a, 0x08, 0x0a,
    0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x02,
    0x00, 0x09, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x02, 0x08, 0x0e, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x03, 0x02, 0x19, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x03, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x05, 0x12, 0x03, 0x03, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x03, 0x12, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x03, 0x17, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x04, 0x02,
    0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x04, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x04, 0x0b, 0x11, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x04, 0x12, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x04, 0x20, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x02, 0x12, 0x03, 0x05, 0x02, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12,
    0x03, 0x05, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x05,
    0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x05, 0x12, 0x19,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x05, 0x1c, 0x1d, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x06, 0x02, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x03, 0x04, 0x12, 0x03, 0x06, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x03, 0x05, 0x12, 0x03, 0x06, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01,
    0x12, 0x03, 0x06, 0x11, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03,
    0x06, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x07, 0x02, 0x21,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x04, 0x12, 0x03, 0x07, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x05, 0x12, 0x03, 0x07, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x07, 0x11, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x04, 0x03, 0x12, 0x03, 0x07, 0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x05,
    0x12, 0x03, 0x08, 0x02, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x04, 0x12, 0x03,
    0x08, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x05, 0x12, 0x03, 0x08, 0x0b,
    0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03, 0x08, 0x10, 0x1a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x03, 0x12, 0x03, 0x08, 0x1d, 0x1e, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x05, 0x08, 0x12, 0x03, 0x08, 0x1f, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x05, 0x07, 0x12, 0x03, 0x08, 0x2a, 0x2f, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12,
    0x04, 0x0b, 0x00, 0x0e, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x0b, 0x08,
    0x0c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x0c, 0x02, 0x1b, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x0c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x00, 0x06, 0x12, 0x03, 0x0c, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x0c, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x0c, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03,
    0x0d, 0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x04, 0x12, 0x03, 0x0d, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x06, 0x12, 0x03, 0x0d, 0x0b, 0x11, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0d, 0x12, 0x1c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0d, 0x1f, 0x20, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x02, 0x12, 0x04, 0x10, 0x00, 0x13, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03,
    0x10, 0x08, 0x0b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x11, 0x02, 0x1b,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x11, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x06, 0x12, 0x03, 0x11, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x11, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x11, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x01,
    0x12, 0x03, 0x12, 0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x04, 0x12, 0x03,
    0x12, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x06, 0x12, 0x03, 0x12, 0x0b,
    0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x12, 0x12, 0x1c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x12, 0x1f, 0x20, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x03, 0x12, 0x04, 0x15, 0x00, 0x18, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01,
    0x12, 0x03, 0x15, 0x08, 0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x16,
    0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x03, 0x16, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x06, 0x12, 0x03, 0x16, 0x0b, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x16, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x16, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03,
    0x02, 0x01, 0x12, 0x03, 0x17, 0x02, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x04,
    0x12, 0x03, 0x17, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x06, 0x12, 0x03,
    0x17, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x01, 0x12, 0x03, 0x17, 0x12,
    0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x03, 0x12, 0x03, 0x17, 0x1b, 0x1c, 0x0a,
    0x0a, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x1a, 0x00, 0x1f, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x04, 0x01, 0x12, 0x03, 0x1a, 0x08, 0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x04, 0x00, 0x12,
    0x03, 0x1b, 0x02, 0x38, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x04, 0x00, 0x01, 0x12, 0x03, 0x1b,
    0x07, 0x0d, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x1b, 0x10,
    0x1a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1b, 0x10,
    0x15, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x1b, 0x18,
    0x19, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x1b, 0x1b, 0x27,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x1b, 0x1b, 0x22,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x1b, 0x25, 0x26,
    0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x1b, 0x28, 0x36, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x04, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x1b, 0x28, 0x31, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x04, 0x04, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x1b, 0x34, 0x35, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x1d, 0x02, 0x1d, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x00, 0x04, 0x12, 0x03, 0x1d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x00, 0x06, 0x12, 0x03, 0x1d, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x1d, 0x12, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x1d, 0x1b, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x01, 0x12, 0x03, 0x1e, 0x02,
    0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x04, 0x12, 0x03, 0x1e, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x06, 0x12, 0x03, 0x1e, 0x0b, 0x11, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x01, 0x01, 0x12, 0x03, 0x1e, 0x12, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x01, 0x03, 0x12, 0x03, 0x1e, 0x1b, 0x1c, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x05, 0x12,
    0x04, 0x21, 0x00, 0x2a, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x21, 0x08,
    0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x04, 0x00, 0x12, 0x03, 0x22, 0x02, 0x3a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x05, 0x04, 0x00, 0x01, 0x12, 0x03, 0x22, 0x07, 0x0d, 0x0a, 0x0d, 0x0a, 0x06,
    0x04, 0x05, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x22, 0x10, 0x1c, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x22, 0x10, 0x17, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x22, 0x1a, 0x1b, 0x0a, 0x0d, 0x0a, 0x06, 0x04,
    0x05, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x22, 0x1d, 0x2a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05,
    0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x22, 0x1d, 0x25, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05,
    0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x22, 0x28, 0x29, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x05,
    0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x22, 0x2b, 0x38, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05, 0x04,
    0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x22, 0x2b, 0x33, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05, 0x04,
    0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x22, 0x36, 0x37, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02,
    0x00, 0x12, 0x03, 0x24, 0x02, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x04, 0x12,
    0x03, 0x24, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x05, 0x12, 0x03, 0x24,
    0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12, 0x03, 0x24, 0x12, 0x1b,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x24, 0x1e, 0x1f, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x05, 0x02, 0x01, 0x12, 0x03, 0x25, 0x02, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x01, 0x04, 0x12, 0x03, 0x25, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x01, 0x05, 0x12, 0x03, 0x25, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x25, 0x12, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x25, 0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x02, 0x12, 0x03, 0x26, 0x02, 0x1b,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x04, 0x12, 0x03, 0x26, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x05, 0x12, 0x03, 0x26, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x02, 0x01, 0x12, 0x03, 0x26, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x02, 0x03, 0x12, 0x03, 0x26, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x03,
    0x12, 0x03, 0x27, 0x02, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x04, 0x12, 0x03,
    0x27, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x05, 0x12, 0x03, 0x27, 0x0b,
    0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x01, 0x12, 0x03, 0x27, 0x12, 0x1d, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x03, 0x12, 0x03, 0x27, 0x20, 0x21, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x05, 0x02, 0x04, 0x12, 0x03, 0x28, 0x02, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x04, 0x04, 0x12, 0x03, 0x28, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x04,
    0x06, 0x12, 0x03, 0x28, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x04, 0x01, 0x12,
    0x03, 0x28, 0x12, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x04, 0x03, 0x12, 0x03, 0x28,
    0x1b, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x05, 0x12, 0x03, 0x29, 0x02, 0x1c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x05, 0x04, 0x12, 0x03, 0x29, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x05, 0x05, 0x12, 0x03, 0x29, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x05, 0x01, 0x12, 0x03, 0x29, 0x12, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x05, 0x03, 0x12, 0x03, 0x29, 0x1a, 0x1b, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x06, 0x12, 0x04, 0x2c,
    0x00, 0x34, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x06, 0x01, 0x12, 0x03, 0x2c, 0x08, 0x0f, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x00, 0x12, 0x03, 0x2d, 0x02, 0x20, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x00, 0x04, 0x12, 0x03, 0x2d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x00, 0x05, 0x12, 0x03, 0x2d, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x2d, 0x12, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x2d, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x01, 0x12, 0x03, 0x2e, 0x02,
    0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x04, 0x12, 0x03, 0x2e, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x05, 0x12, 0x03, 0x2e, 0x0b, 0x11, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x01, 0x01, 0x12, 0x03, 0x2e, 0x12, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x06, 0x02, 0x01, 0x03, 0x12, 0x03, 0x2e, 0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02,
    0x02, 0x12, 0x03, 0x2f, 0x02, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x04, 0x12,
    0x03, 0x2f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x05, 0x12, 0x03, 0x2f,
    0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x01, 0x12, 0x03, 0x2f, 0x12, 0x1d,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x03, 0x12, 0x03, 0x2f, 0x20, 0x21, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x06, 0x02, 0x03, 0x12, 0x03, 0x30, 0x02, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x06, 0x02, 0x03, 0x04, 0x12, 0x03, 0x30, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x03, 0x05, 0x12, 0x03, 0x30, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x01,
    0x12, 0x03, 0x30, 0x12, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x03, 0x12, 0x03,
    0x30, 0x17, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x04, 0x12, 0x03, 0x31, 0x02, 0x1f,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x04, 0x04, 0x12, 0x03, 0x31, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x04, 0x05, 0x12, 0x03, 0x31, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x04, 0x01, 0x12, 0x03, 0x31, 0x12, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x04, 0x03, 0x12, 0x03, 0x31, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x05,
    0x12, 0x03, 0x32, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x05, 0x04, 0x12, 0x03,
    0x32, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x05, 0x05, 0x12, 0x03, 0x32, 0x0b,
    0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x05, 0x01, 0x12, 0x03, 0x32, 0x12, 0x16, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x05, 0x03, 0x12, 0x03, 0x32, 0x19, 0x1a, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x06, 0x02, 0x06, 0x12, 0x03, 0x33, 0x02, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x06, 0x04, 0x12, 0x03, 0x33, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x06,
    0x05, 0x12, 0x03, 0x33, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x06, 0x01, 0x12,
    0x03, 0x33, 0x12, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x06, 0x03, 0x12, 0x03, 0x33,
    0x1c, 0x1d, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x07, 0x12, 0x04, 0x36, 0x00, 0x41, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x07, 0x01, 0x12, 0x03, 0x36, 0x08, 0x0c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07,
    0x04, 0x00, 0x12, 0x03, 0x37, 0x02, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x04, 0x00, 0x01,
    0x12, 0x03, 0x37, 0x07, 0x0b, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x07, 0x04, 0x00, 0x02, 0x00, 0x12,
    0x03, 0x37, 0x0e, 0x17, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x07, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x37, 0x0e, 0x12, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x07, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12,
    0x03, 0x37, 0x15, 0x16, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x07, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03,
    0x37, 0x18, 0x20, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x07, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x37, 0x18, 0x1b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x07, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03,
    0x37, 0x1e, 0x1f, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x07, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x37,
    0x21, 0x2d, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x07, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x37,
    0x21, 0x28, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x07, 0x04, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x37,
    0x2b, 0x2c, 0x0a, 0x33, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x00, 0x12, 0x03, 0x3a, 0x02, 0x19, 0x1a,
    0x26, 0x20, 0x49, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x66, 0x69, 0x65, 0x73, 0x20, 0x77, 0x68, 0x69,
    0x63, 0x68, 0x20, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x20, 0x69, 0x73, 0x20, 0x66, 0x69, 0x6c, 0x6c,
    0x65, 0x64, 0x20, 0x69, 0x6e, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x04,
    0x12, 0x03, 0x3a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x06, 0x12, 0x03,
    0x3a, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x01, 0x12, 0x03, 0x3a, 0x10,
    0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x03, 0x12, 0x03, 0x3a, 0x17, 0x18, 0x0a,
    0x17, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x01, 0x12, 0x03, 0x3d, 0x02, 0x19, 0x1a, 0x0a, 0x20, 0x4f,
    0x70, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01,
    0x04, 0x12, 0x03, 0x3d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x06, 0x12,
    0x03, 0x3d, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x01, 0x12, 0x03, 0x3d,
    0x10, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x03, 0x12, 0x03, 0x3d, 0x17, 0x18,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x02, 0x12, 0x03, 0x3e, 0x02, 0x17, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x07, 0x02, 0x02, 0x04, 0x12, 0x03, 0x3e, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x07, 0x02, 0x02, 0x06, 0x12, 0x03, 0x3e, 0x0b, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02,
    0x02, 0x01, 0x12, 0x03, 0x3e, 0x0f, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x03,
    0x12, 0x03, 0x3e, 0x15, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x03, 0x12, 0x03, 0x3f,
    0x02, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x03, 0x04, 0x12, 0x03, 0x3f, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x03, 0x06, 0x12, 0x03, 0x3f, 0x0b, 0x12, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x07, 0x02, 0x03, 0x01, 0x12, 0x03, 0x3f, 0x13, 0x1a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x07, 0x02, 0x03, 0x03, 0x12, 0x03, 0x3f, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07,
    0x02, 0x04, 0x12, 0x03, 0x40, 0x02, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x04, 0x04,
    0x12, 0x03, 0x40, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x04, 0x06, 0x12, 0x03,
    0x40, 0x0b, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x04, 0x01, 0x12, 0x03, 0x40, 0x16,
    0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x04, 0x03, 0x12, 0x03, 0x40, 0x23, 0x24, 0x0a,
    0x0a, 0x0a, 0x02, 0x04, 0x08, 0x12, 0x04, 0x43, 0x00, 0x54, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x08, 0x01, 0x12, 0x03, 0x43, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x08, 0x04, 0x00, 0x12,
    0x04, 0x44, 0x02, 0x4a, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x04, 0x00, 0x01, 0x12, 0x03,
    0x44, 0x07, 0x0b, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x08, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x45,
    0x04, 0x0f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x08, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x45,
    0x04, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x08, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x45,
    0x0d, 0x0e, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x08, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x46, 0x04,
    0x10, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x08, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x46, 0x04,
    0x0b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x08, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x46, 0x0e,
    0x0f, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x08, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x47, 0x04, 0x11,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x08, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x47, 0x04, 0x0c,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x08, 0x04, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x47, 0x0f, 0x10,
    0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x08, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x48, 0x04, 0x0d, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x08, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x48, 0x04, 0x08, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x08, 0x04, 0x00, 0x02, 0x03, 0x02, 0x12, 0x03, 0x48, 0x0b, 0x0c, 0x0a,
    0x0d, 0x0a, 0x06, 0x04, 0x08, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x49, 0x04, 0x0e, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x08, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x49, 0x04, 0x09, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x08, 0x04, 0x00, 0x02, 0x04, 0x02, 0x12, 0x03, 0x49, 0x0c, 0x0d, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x08, 0x02, 0x00, 0x12, 0x03, 0x4c, 0x02, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x08, 0x02, 0x00, 0x04, 0x12, 0x03, 0x4c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02,
    0x00, 0x06, 0x12, 0x03, 0x4c, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x4c, 0x10, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x4c, 0x17, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x01, 0x12, 0x03, 0x4d, 0x02, 0x1a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x04, 0x12, 0x03, 0x4d, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x05, 0x12, 0x03, 0x4d, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x08, 0x02, 0x01, 0x01, 0x12, 0x03, 0x4d, 0x12, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x4d, 0x18, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x02,
    0x12, 0x03, 0x4e, 0x02, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x04, 0x12, 0x03,
    0x4e, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x05, 0x12, 0x03, 0x4e, 0x0b,
    0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x01, 0x12, 0x03, 0x4e, 0x12, 0x19, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x03, 0x12, 0x03, 0x4e, 0x1c, 0x1d, 0x0a, 0x2f, 0x0a,
    0x04, 0x04, 0x08, 0x02, 0x03, 0x12, 0x03, 0x51, 0x02, 0x21, 0x1a, 0x22, 0x20, 0x45, 0x76, 0x65,
    0x72, 0x79, 0x74, 0x68, 0x69, 0x6e, 0x67, 0x20, 0x65, 0x6c, 0x73, 0x65, 0x20, 0x6d, 0x75, 0x73,
    0x74, 0x20, 0x62, 0x65, 0x20, 0x6f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x08, 0x02, 0x03, 0x04, 0x12, 0x03, 0x51, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x08, 0x02, 0x03, 0x06, 0x12, 0x03, 0x51, 0x0b, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08,
    0x02, 0x03, 0x01, 0x12, 0x03, 0x51, 0x16, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x03,
    0x03, 0x12, 0x03, 0x51, 0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x04, 0x12, 0x03,
    0x52, 0x02, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x04, 0x04, 0x12, 0x03, 0x52, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x04, 0x06, 0x12, 0x03, 0x52, 0x0b, 0x12, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x04, 0x01, 0x12, 0x03, 0x52, 0x13, 0x1a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x08, 0x02, 0x04, 0x03, 0x12, 0x03, 0x52, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x08, 0x02, 0x05, 0x12, 0x03, 0x53, 0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x05,
    0x04, 0x12, 0x03, 0x53, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x05, 0x06, 0x12,
    0x03, 0x53, 0x0b, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x05, 0x01, 0x12, 0x03, 0x53,
    0x14, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x05, 0x03, 0x12, 0x03, 0x53, 0x1f, 0x20,
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
