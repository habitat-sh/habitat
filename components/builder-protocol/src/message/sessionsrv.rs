// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(Clone,Default)]
pub struct Account {
    // message fields
    id: ::std::option::Option<u64>,
    email: ::protobuf::SingularField<::std::string::String>,
    name: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Account {}

impl Account {
    pub fn new() -> Account {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Account {
        static mut instance: ::protobuf::lazy::Lazy<Account> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Account,
        };
        unsafe {
            instance.get(|| {
                Account {
                    id: ::std::option::Option::None,
                    email: ::protobuf::SingularField::none(),
                    name: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required uint64 id = 1;

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

    // required string email = 2;

    pub fn clear_email(&mut self) {
        self.email.clear();
    }

    pub fn has_email(&self) -> bool {
        self.email.is_some()
    }

    // Param is passed by value, moved
    pub fn set_email(&mut self, v: ::std::string::String) {
        self.email = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_email(&mut self) -> &mut ::std::string::String {
        if self.email.is_none() {
            self.email.set_default();
        };
        self.email.as_mut().unwrap()
    }

    // Take field
    pub fn take_email(&mut self) -> ::std::string::String {
        self.email.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_email(&self) -> &str {
        match self.email.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // required string name = 3;

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
        };
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
}

impl ::protobuf::Message for Account {
    fn is_initialized(&self) -> bool {
        if self.id.is_none() {
            return false;
        };
        if self.email.is_none() {
            return false;
        };
        if self.name.is_none() {
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
                    let tmp = try!(is.read_uint64());
                    self.id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.email));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name));
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
        for value in self.id.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.email.iter() {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in self.name.iter() {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id {
            try!(os.write_uint64(1, v));
        };
        if let Some(v) = self.email.as_ref() {
            try!(os.write_string(2, &v));
        };
        if let Some(v) = self.name.as_ref() {
            try!(os.write_string(3, &v));
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
        ::std::any::TypeId::of::<Account>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Account {
    fn new() -> Account {
        Account::new()
    }

    fn descriptor_static(_: ::std::option::Option<Account>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "id",
                    Account::has_id,
                    Account::get_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "email",
                    Account::has_email,
                    Account::get_email,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "name",
                    Account::has_name,
                    Account::get_name,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Account>(
                    "Account",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Account {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_email();
        self.clear_name();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Account {
    fn eq(&self, other: &Account) -> bool {
        self.id == other.id &&
        self.email == other.email &&
        self.name == other.name &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Account {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct AccountGet {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AccountGet {}

impl AccountGet {
    pub fn new() -> AccountGet {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AccountGet {
        static mut instance: ::protobuf::lazy::Lazy<AccountGet> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AccountGet,
        };
        unsafe {
            instance.get(|| {
                AccountGet {
                    name: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required string name = 1;

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
        };
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
}

impl ::protobuf::Message for AccountGet {
    fn is_initialized(&self) -> bool {
        if self.name.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name));
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
        for value in self.name.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.name.as_ref() {
            try!(os.write_string(1, &v));
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
        ::std::any::TypeId::of::<AccountGet>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AccountGet {
    fn new() -> AccountGet {
        AccountGet::new()
    }

    fn descriptor_static(_: ::std::option::Option<AccountGet>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "name",
                    AccountGet::has_name,
                    AccountGet::get_name,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AccountGet>(
                    "AccountGet",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AccountGet {
    fn clear(&mut self) {
        self.clear_name();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for AccountGet {
    fn eq(&self, other: &AccountGet) -> bool {
        self.name == other.name &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for AccountGet {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct AccountSearch {
    // message fields
    key: ::std::option::Option<AccountSearchKey>,
    value: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AccountSearch {}

impl AccountSearch {
    pub fn new() -> AccountSearch {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AccountSearch {
        static mut instance: ::protobuf::lazy::Lazy<AccountSearch> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AccountSearch,
        };
        unsafe {
            instance.get(|| {
                AccountSearch {
                    key: ::std::option::Option::None,
                    value: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .sessionsrv.AccountSearchKey key = 1;

    pub fn clear_key(&mut self) {
        self.key = ::std::option::Option::None;
    }

    pub fn has_key(&self) -> bool {
        self.key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: AccountSearchKey) {
        self.key = ::std::option::Option::Some(v);
    }

    pub fn get_key(&self) -> AccountSearchKey {
        self.key.unwrap_or(AccountSearchKey::Id)
    }

    // required string value = 2;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::string::String) {
        self.value = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut ::std::string::String {
        if self.value.is_none() {
            self.value.set_default();
        };
        self.value.as_mut().unwrap()
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::string::String {
        self.value.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_value(&self) -> &str {
        match self.value.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for AccountSearch {
    fn is_initialized(&self) -> bool {
        if self.key.is_none() {
            return false;
        };
        if self.value.is_none() {
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
                    self.key = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.value));
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
        for value in self.key.iter() {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in self.value.iter() {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.key {
            try!(os.write_enum(1, v.value()));
        };
        if let Some(v) = self.value.as_ref() {
            try!(os.write_string(2, &v));
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
        ::std::any::TypeId::of::<AccountSearch>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AccountSearch {
    fn new() -> AccountSearch {
        AccountSearch::new()
    }

    fn descriptor_static(_: ::std::option::Option<AccountSearch>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "key",
                    AccountSearch::has_key,
                    AccountSearch::get_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "value",
                    AccountSearch::has_value,
                    AccountSearch::get_value,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AccountSearch>(
                    "AccountSearch",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AccountSearch {
    fn clear(&mut self) {
        self.clear_key();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for AccountSearch {
    fn eq(&self, other: &AccountSearch) -> bool {
        self.key == other.key &&
        self.value == other.value &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for AccountSearch {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct GrantFlagToTeam {
    // message fields
    flag: ::std::option::Option<u32>,
    team_id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GrantFlagToTeam {}

impl GrantFlagToTeam {
    pub fn new() -> GrantFlagToTeam {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GrantFlagToTeam {
        static mut instance: ::protobuf::lazy::Lazy<GrantFlagToTeam> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GrantFlagToTeam,
        };
        unsafe {
            instance.get(|| {
                GrantFlagToTeam {
                    flag: ::std::option::Option::None,
                    team_id: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required uint32 flag = 1;

    pub fn clear_flag(&mut self) {
        self.flag = ::std::option::Option::None;
    }

    pub fn has_flag(&self) -> bool {
        self.flag.is_some()
    }

    // Param is passed by value, moved
    pub fn set_flag(&mut self, v: u32) {
        self.flag = ::std::option::Option::Some(v);
    }

    pub fn get_flag(&self) -> u32 {
        self.flag.unwrap_or(0)
    }

    // required uint64 team_id = 2;

    pub fn clear_team_id(&mut self) {
        self.team_id = ::std::option::Option::None;
    }

    pub fn has_team_id(&self) -> bool {
        self.team_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_team_id(&mut self, v: u64) {
        self.team_id = ::std::option::Option::Some(v);
    }

    pub fn get_team_id(&self) -> u64 {
        self.team_id.unwrap_or(0)
    }
}

impl ::protobuf::Message for GrantFlagToTeam {
    fn is_initialized(&self) -> bool {
        if self.flag.is_none() {
            return false;
        };
        if self.team_id.is_none() {
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
                    let tmp = try!(is.read_uint32());
                    self.flag = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.team_id = ::std::option::Option::Some(tmp);
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
        for value in self.flag.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.team_id.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.flag {
            try!(os.write_uint32(1, v));
        };
        if let Some(v) = self.team_id {
            try!(os.write_uint64(2, v));
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
        ::std::any::TypeId::of::<GrantFlagToTeam>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GrantFlagToTeam {
    fn new() -> GrantFlagToTeam {
        GrantFlagToTeam::new()
    }

    fn descriptor_static(_: ::std::option::Option<GrantFlagToTeam>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "flag",
                    GrantFlagToTeam::has_flag,
                    GrantFlagToTeam::get_flag,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "team_id",
                    GrantFlagToTeam::has_team_id,
                    GrantFlagToTeam::get_team_id,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GrantFlagToTeam>(
                    "GrantFlagToTeam",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GrantFlagToTeam {
    fn clear(&mut self) {
        self.clear_flag();
        self.clear_team_id();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GrantFlagToTeam {
    fn eq(&self, other: &GrantFlagToTeam) -> bool {
        self.flag == other.flag &&
        self.team_id == other.team_id &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GrantFlagToTeam {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RevokeFlagFromTeam {
    // message fields
    flag: ::std::option::Option<u32>,
    team_id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RevokeFlagFromTeam {}

impl RevokeFlagFromTeam {
    pub fn new() -> RevokeFlagFromTeam {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RevokeFlagFromTeam {
        static mut instance: ::protobuf::lazy::Lazy<RevokeFlagFromTeam> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RevokeFlagFromTeam,
        };
        unsafe {
            instance.get(|| {
                RevokeFlagFromTeam {
                    flag: ::std::option::Option::None,
                    team_id: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required uint32 flag = 1;

    pub fn clear_flag(&mut self) {
        self.flag = ::std::option::Option::None;
    }

    pub fn has_flag(&self) -> bool {
        self.flag.is_some()
    }

    // Param is passed by value, moved
    pub fn set_flag(&mut self, v: u32) {
        self.flag = ::std::option::Option::Some(v);
    }

    pub fn get_flag(&self) -> u32 {
        self.flag.unwrap_or(0)
    }

    // required uint64 team_id = 2;

    pub fn clear_team_id(&mut self) {
        self.team_id = ::std::option::Option::None;
    }

    pub fn has_team_id(&self) -> bool {
        self.team_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_team_id(&mut self, v: u64) {
        self.team_id = ::std::option::Option::Some(v);
    }

    pub fn get_team_id(&self) -> u64 {
        self.team_id.unwrap_or(0)
    }
}

impl ::protobuf::Message for RevokeFlagFromTeam {
    fn is_initialized(&self) -> bool {
        if self.flag.is_none() {
            return false;
        };
        if self.team_id.is_none() {
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
                    let tmp = try!(is.read_uint32());
                    self.flag = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.team_id = ::std::option::Option::Some(tmp);
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
        for value in self.flag.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.team_id.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.flag {
            try!(os.write_uint32(1, v));
        };
        if let Some(v) = self.team_id {
            try!(os.write_uint64(2, v));
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
        ::std::any::TypeId::of::<RevokeFlagFromTeam>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RevokeFlagFromTeam {
    fn new() -> RevokeFlagFromTeam {
        RevokeFlagFromTeam::new()
    }

    fn descriptor_static(_: ::std::option::Option<RevokeFlagFromTeam>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "flag",
                    RevokeFlagFromTeam::has_flag,
                    RevokeFlagFromTeam::get_flag,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "team_id",
                    RevokeFlagFromTeam::has_team_id,
                    RevokeFlagFromTeam::get_team_id,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RevokeFlagFromTeam>(
                    "RevokeFlagFromTeam",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RevokeFlagFromTeam {
    fn clear(&mut self) {
        self.clear_flag();
        self.clear_team_id();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RevokeFlagFromTeam {
    fn eq(&self, other: &RevokeFlagFromTeam) -> bool {
        self.flag == other.flag &&
        self.team_id == other.team_id &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RevokeFlagFromTeam {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ListFlagGrants {
    // message fields
    flag: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ListFlagGrants {}

impl ListFlagGrants {
    pub fn new() -> ListFlagGrants {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ListFlagGrants {
        static mut instance: ::protobuf::lazy::Lazy<ListFlagGrants> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ListFlagGrants,
        };
        unsafe {
            instance.get(|| {
                ListFlagGrants {
                    flag: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required uint32 flag = 1;

    pub fn clear_flag(&mut self) {
        self.flag = ::std::option::Option::None;
    }

    pub fn has_flag(&self) -> bool {
        self.flag.is_some()
    }

    // Param is passed by value, moved
    pub fn set_flag(&mut self, v: u32) {
        self.flag = ::std::option::Option::Some(v);
    }

    pub fn get_flag(&self) -> u32 {
        self.flag.unwrap_or(0)
    }
}

impl ::protobuf::Message for ListFlagGrants {
    fn is_initialized(&self) -> bool {
        if self.flag.is_none() {
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
                    let tmp = try!(is.read_uint32());
                    self.flag = ::std::option::Option::Some(tmp);
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
        for value in self.flag.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.flag {
            try!(os.write_uint32(1, v));
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
        ::std::any::TypeId::of::<ListFlagGrants>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ListFlagGrants {
    fn new() -> ListFlagGrants {
        ListFlagGrants::new()
    }

    fn descriptor_static(_: ::std::option::Option<ListFlagGrants>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "flag",
                    ListFlagGrants::has_flag,
                    ListFlagGrants::get_flag,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ListFlagGrants>(
                    "ListFlagGrants",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ListFlagGrants {
    fn clear(&mut self) {
        self.clear_flag();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ListFlagGrants {
    fn eq(&self, other: &ListFlagGrants) -> bool {
        self.flag == other.flag &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ListFlagGrants {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct FlagGrants {
    // message fields
    teams: ::std::vec::Vec<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for FlagGrants {}

impl FlagGrants {
    pub fn new() -> FlagGrants {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FlagGrants {
        static mut instance: ::protobuf::lazy::Lazy<FlagGrants> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FlagGrants,
        };
        unsafe {
            instance.get(|| {
                FlagGrants {
                    teams: ::std::vec::Vec::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated uint64 teams = 1;

    pub fn clear_teams(&mut self) {
        self.teams.clear();
    }

    // Param is passed by value, moved
    pub fn set_teams(&mut self, v: ::std::vec::Vec<u64>) {
        self.teams = v;
    }

    // Mutable pointer to the field.
    pub fn mut_teams(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.teams
    }

    // Take field
    pub fn take_teams(&mut self) -> ::std::vec::Vec<u64> {
        ::std::mem::replace(&mut self.teams, ::std::vec::Vec::new())
    }

    pub fn get_teams(&self) -> &[u64] {
        &self.teams
    }
}

impl ::protobuf::Message for FlagGrants {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_uint64_into(wire_type, is, &mut self.teams));
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
        for value in self.teams.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in self.teams.iter() {
            try!(os.write_uint64(1, *v));
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
        ::std::any::TypeId::of::<FlagGrants>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for FlagGrants {
    fn new() -> FlagGrants {
        FlagGrants::new()
    }

    fn descriptor_static(_: ::std::option::Option<FlagGrants>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_u64_accessor(
                    "teams",
                    FlagGrants::get_teams,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FlagGrants>(
                    "FlagGrants",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FlagGrants {
    fn clear(&mut self) {
        self.clear_teams();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for FlagGrants {
    fn eq(&self, other: &FlagGrants) -> bool {
        self.teams == other.teams &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for FlagGrants {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Session {
    // message fields
    id: ::std::option::Option<u64>,
    email: ::protobuf::SingularField<::std::string::String>,
    name: ::protobuf::SingularField<::std::string::String>,
    token: ::protobuf::SingularField<::std::string::String>,
    flags: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Session {}

impl Session {
    pub fn new() -> Session {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Session {
        static mut instance: ::protobuf::lazy::Lazy<Session> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Session,
        };
        unsafe {
            instance.get(|| {
                Session {
                    id: ::std::option::Option::None,
                    email: ::protobuf::SingularField::none(),
                    name: ::protobuf::SingularField::none(),
                    token: ::protobuf::SingularField::none(),
                    flags: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required uint64 id = 1;

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

    // required string email = 2;

    pub fn clear_email(&mut self) {
        self.email.clear();
    }

    pub fn has_email(&self) -> bool {
        self.email.is_some()
    }

    // Param is passed by value, moved
    pub fn set_email(&mut self, v: ::std::string::String) {
        self.email = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_email(&mut self) -> &mut ::std::string::String {
        if self.email.is_none() {
            self.email.set_default();
        };
        self.email.as_mut().unwrap()
    }

    // Take field
    pub fn take_email(&mut self) -> ::std::string::String {
        self.email.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_email(&self) -> &str {
        match self.email.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // required string name = 3;

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
        };
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

    // required string token = 4;

    pub fn clear_token(&mut self) {
        self.token.clear();
    }

    pub fn has_token(&self) -> bool {
        self.token.is_some()
    }

    // Param is passed by value, moved
    pub fn set_token(&mut self, v: ::std::string::String) {
        self.token = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_token(&mut self) -> &mut ::std::string::String {
        if self.token.is_none() {
            self.token.set_default();
        };
        self.token.as_mut().unwrap()
    }

    // Take field
    pub fn take_token(&mut self) -> ::std::string::String {
        self.token.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_token(&self) -> &str {
        match self.token.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // required uint32 flags = 5;

    pub fn clear_flags(&mut self) {
        self.flags = ::std::option::Option::None;
    }

    pub fn has_flags(&self) -> bool {
        self.flags.is_some()
    }

    // Param is passed by value, moved
    pub fn set_flags(&mut self, v: u32) {
        self.flags = ::std::option::Option::Some(v);
    }

    pub fn get_flags(&self) -> u32 {
        self.flags.unwrap_or(0)
    }
}

impl ::protobuf::Message for Session {
    fn is_initialized(&self) -> bool {
        if self.id.is_none() {
            return false;
        };
        if self.email.is_none() {
            return false;
        };
        if self.name.is_none() {
            return false;
        };
        if self.token.is_none() {
            return false;
        };
        if self.flags.is_none() {
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
                    let tmp = try!(is.read_uint64());
                    self.id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.email));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name));
                },
                4 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.token));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.flags = ::std::option::Option::Some(tmp);
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
        for value in self.id.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.email.iter() {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in self.name.iter() {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        for value in self.token.iter() {
            my_size += ::protobuf::rt::string_size(4, &value);
        };
        for value in self.flags.iter() {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id {
            try!(os.write_uint64(1, v));
        };
        if let Some(v) = self.email.as_ref() {
            try!(os.write_string(2, &v));
        };
        if let Some(v) = self.name.as_ref() {
            try!(os.write_string(3, &v));
        };
        if let Some(v) = self.token.as_ref() {
            try!(os.write_string(4, &v));
        };
        if let Some(v) = self.flags {
            try!(os.write_uint32(5, v));
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
        ::std::any::TypeId::of::<Session>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Session {
    fn new() -> Session {
        Session::new()
    }

    fn descriptor_static(_: ::std::option::Option<Session>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "id",
                    Session::has_id,
                    Session::get_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "email",
                    Session::has_email,
                    Session::get_email,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "name",
                    Session::has_name,
                    Session::get_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "token",
                    Session::has_token,
                    Session::get_token,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "flags",
                    Session::has_flags,
                    Session::get_flags,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Session>(
                    "Session",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Session {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_email();
        self.clear_name();
        self.clear_token();
        self.clear_flags();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Session {
    fn eq(&self, other: &Session) -> bool {
        self.id == other.id &&
        self.email == other.email &&
        self.name == other.name &&
        self.token == other.token &&
        self.flags == other.flags &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Session {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct SessionToken {
    // message fields
    token: ::protobuf::SingularField<::std::string::String>,
    owner_id: ::std::option::Option<u64>,
    provider: ::std::option::Option<OAuthProvider>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SessionToken {}

impl SessionToken {
    pub fn new() -> SessionToken {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SessionToken {
        static mut instance: ::protobuf::lazy::Lazy<SessionToken> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SessionToken,
        };
        unsafe {
            instance.get(|| {
                SessionToken {
                    token: ::protobuf::SingularField::none(),
                    owner_id: ::std::option::Option::None,
                    provider: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required string token = 1;

    pub fn clear_token(&mut self) {
        self.token.clear();
    }

    pub fn has_token(&self) -> bool {
        self.token.is_some()
    }

    // Param is passed by value, moved
    pub fn set_token(&mut self, v: ::std::string::String) {
        self.token = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_token(&mut self) -> &mut ::std::string::String {
        if self.token.is_none() {
            self.token.set_default();
        };
        self.token.as_mut().unwrap()
    }

    // Take field
    pub fn take_token(&mut self) -> ::std::string::String {
        self.token.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_token(&self) -> &str {
        match self.token.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // required uint64 owner_id = 2;

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

    // required .sessionsrv.OAuthProvider provider = 3;

    pub fn clear_provider(&mut self) {
        self.provider = ::std::option::Option::None;
    }

    pub fn has_provider(&self) -> bool {
        self.provider.is_some()
    }

    // Param is passed by value, moved
    pub fn set_provider(&mut self, v: OAuthProvider) {
        self.provider = ::std::option::Option::Some(v);
    }

    pub fn get_provider(&self) -> OAuthProvider {
        self.provider.unwrap_or(OAuthProvider::GitHub)
    }
}

impl ::protobuf::Message for SessionToken {
    fn is_initialized(&self) -> bool {
        if self.token.is_none() {
            return false;
        };
        if self.owner_id.is_none() {
            return false;
        };
        if self.provider.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.token));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.owner_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.provider = ::std::option::Option::Some(tmp);
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
        for value in self.token.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in self.owner_id.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.provider.iter() {
            my_size += ::protobuf::rt::enum_size(3, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.token.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.owner_id {
            try!(os.write_uint64(2, v));
        };
        if let Some(v) = self.provider {
            try!(os.write_enum(3, v.value()));
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
        ::std::any::TypeId::of::<SessionToken>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SessionToken {
    fn new() -> SessionToken {
        SessionToken::new()
    }

    fn descriptor_static(_: ::std::option::Option<SessionToken>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "token",
                    SessionToken::has_token,
                    SessionToken::get_token,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "owner_id",
                    SessionToken::has_owner_id,
                    SessionToken::get_owner_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "provider",
                    SessionToken::has_provider,
                    SessionToken::get_provider,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SessionToken>(
                    "SessionToken",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SessionToken {
    fn clear(&mut self) {
        self.clear_token();
        self.clear_owner_id();
        self.clear_provider();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for SessionToken {
    fn eq(&self, other: &SessionToken) -> bool {
        self.token == other.token &&
        self.owner_id == other.owner_id &&
        self.provider == other.provider &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for SessionToken {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct SessionCreate {
    // message fields
    token: ::protobuf::SingularField<::std::string::String>,
    extern_id: ::std::option::Option<u64>,
    email: ::protobuf::SingularField<::std::string::String>,
    name: ::protobuf::SingularField<::std::string::String>,
    provider: ::std::option::Option<OAuthProvider>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SessionCreate {}

impl SessionCreate {
    pub fn new() -> SessionCreate {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SessionCreate {
        static mut instance: ::protobuf::lazy::Lazy<SessionCreate> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SessionCreate,
        };
        unsafe {
            instance.get(|| {
                SessionCreate {
                    token: ::protobuf::SingularField::none(),
                    extern_id: ::std::option::Option::None,
                    email: ::protobuf::SingularField::none(),
                    name: ::protobuf::SingularField::none(),
                    provider: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required string token = 1;

    pub fn clear_token(&mut self) {
        self.token.clear();
    }

    pub fn has_token(&self) -> bool {
        self.token.is_some()
    }

    // Param is passed by value, moved
    pub fn set_token(&mut self, v: ::std::string::String) {
        self.token = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_token(&mut self) -> &mut ::std::string::String {
        if self.token.is_none() {
            self.token.set_default();
        };
        self.token.as_mut().unwrap()
    }

    // Take field
    pub fn take_token(&mut self) -> ::std::string::String {
        self.token.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_token(&self) -> &str {
        match self.token.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // required uint64 extern_id = 2;

    pub fn clear_extern_id(&mut self) {
        self.extern_id = ::std::option::Option::None;
    }

    pub fn has_extern_id(&self) -> bool {
        self.extern_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_extern_id(&mut self, v: u64) {
        self.extern_id = ::std::option::Option::Some(v);
    }

    pub fn get_extern_id(&self) -> u64 {
        self.extern_id.unwrap_or(0)
    }

    // required string email = 3;

    pub fn clear_email(&mut self) {
        self.email.clear();
    }

    pub fn has_email(&self) -> bool {
        self.email.is_some()
    }

    // Param is passed by value, moved
    pub fn set_email(&mut self, v: ::std::string::String) {
        self.email = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_email(&mut self) -> &mut ::std::string::String {
        if self.email.is_none() {
            self.email.set_default();
        };
        self.email.as_mut().unwrap()
    }

    // Take field
    pub fn take_email(&mut self) -> ::std::string::String {
        self.email.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_email(&self) -> &str {
        match self.email.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // required string name = 4;

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
        };
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

    // required .sessionsrv.OAuthProvider provider = 5;

    pub fn clear_provider(&mut self) {
        self.provider = ::std::option::Option::None;
    }

    pub fn has_provider(&self) -> bool {
        self.provider.is_some()
    }

    // Param is passed by value, moved
    pub fn set_provider(&mut self, v: OAuthProvider) {
        self.provider = ::std::option::Option::Some(v);
    }

    pub fn get_provider(&self) -> OAuthProvider {
        self.provider.unwrap_or(OAuthProvider::GitHub)
    }
}

impl ::protobuf::Message for SessionCreate {
    fn is_initialized(&self) -> bool {
        if self.token.is_none() {
            return false;
        };
        if self.extern_id.is_none() {
            return false;
        };
        if self.email.is_none() {
            return false;
        };
        if self.name.is_none() {
            return false;
        };
        if self.provider.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.token));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.extern_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.email));
                },
                4 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.provider = ::std::option::Option::Some(tmp);
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
        for value in self.token.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in self.extern_id.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.email.iter() {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        for value in self.name.iter() {
            my_size += ::protobuf::rt::string_size(4, &value);
        };
        for value in self.provider.iter() {
            my_size += ::protobuf::rt::enum_size(5, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.token.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.extern_id {
            try!(os.write_uint64(2, v));
        };
        if let Some(v) = self.email.as_ref() {
            try!(os.write_string(3, &v));
        };
        if let Some(v) = self.name.as_ref() {
            try!(os.write_string(4, &v));
        };
        if let Some(v) = self.provider {
            try!(os.write_enum(5, v.value()));
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
        ::std::any::TypeId::of::<SessionCreate>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SessionCreate {
    fn new() -> SessionCreate {
        SessionCreate::new()
    }

    fn descriptor_static(_: ::std::option::Option<SessionCreate>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "token",
                    SessionCreate::has_token,
                    SessionCreate::get_token,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "extern_id",
                    SessionCreate::has_extern_id,
                    SessionCreate::get_extern_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "email",
                    SessionCreate::has_email,
                    SessionCreate::get_email,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "name",
                    SessionCreate::has_name,
                    SessionCreate::get_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "provider",
                    SessionCreate::has_provider,
                    SessionCreate::get_provider,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SessionCreate>(
                    "SessionCreate",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SessionCreate {
    fn clear(&mut self) {
        self.clear_token();
        self.clear_extern_id();
        self.clear_email();
        self.clear_name();
        self.clear_provider();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for SessionCreate {
    fn eq(&self, other: &SessionCreate) -> bool {
        self.token == other.token &&
        self.extern_id == other.extern_id &&
        self.email == other.email &&
        self.name == other.name &&
        self.provider == other.provider &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for SessionCreate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct SessionGet {
    // message fields
    token: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SessionGet {}

impl SessionGet {
    pub fn new() -> SessionGet {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SessionGet {
        static mut instance: ::protobuf::lazy::Lazy<SessionGet> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SessionGet,
        };
        unsafe {
            instance.get(|| {
                SessionGet {
                    token: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required string token = 1;

    pub fn clear_token(&mut self) {
        self.token.clear();
    }

    pub fn has_token(&self) -> bool {
        self.token.is_some()
    }

    // Param is passed by value, moved
    pub fn set_token(&mut self, v: ::std::string::String) {
        self.token = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_token(&mut self) -> &mut ::std::string::String {
        if self.token.is_none() {
            self.token.set_default();
        };
        self.token.as_mut().unwrap()
    }

    // Take field
    pub fn take_token(&mut self) -> ::std::string::String {
        self.token.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_token(&self) -> &str {
        match self.token.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for SessionGet {
    fn is_initialized(&self) -> bool {
        if self.token.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.token));
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
        for value in self.token.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.token.as_ref() {
            try!(os.write_string(1, &v));
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
        ::std::any::TypeId::of::<SessionGet>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SessionGet {
    fn new() -> SessionGet {
        SessionGet::new()
    }

    fn descriptor_static(_: ::std::option::Option<SessionGet>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "token",
                    SessionGet::has_token,
                    SessionGet::get_token,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SessionGet>(
                    "SessionGet",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SessionGet {
    fn clear(&mut self) {
        self.clear_token();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for SessionGet {
    fn eq(&self, other: &SessionGet) -> bool {
        self.token == other.token &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for SessionGet {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum OAuthProvider {
    GitHub = 0,
}

impl ::protobuf::ProtobufEnum for OAuthProvider {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<OAuthProvider> {
        match value {
            0 => ::std::option::Option::Some(OAuthProvider::GitHub),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [OAuthProvider] = &[
            OAuthProvider::GitHub,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<OAuthProvider>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("OAuthProvider", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for OAuthProvider {
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum AccountSearchKey {
    Id = 0,
    Name = 1,
}

impl ::protobuf::ProtobufEnum for AccountSearchKey {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<AccountSearchKey> {
        match value {
            0 => ::std::option::Option::Some(AccountSearchKey::Id),
            1 => ::std::option::Option::Some(AccountSearchKey::Name),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [AccountSearchKey] = &[
            AccountSearchKey::Id,
            AccountSearchKey::Name,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<AccountSearchKey>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("AccountSearchKey", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for AccountSearchKey {
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x1a, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x73, 0x2f, 0x73, 0x65, 0x73, 0x73,
    0x69, 0x6f, 0x6e, 0x73, 0x72, 0x76, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0a, 0x73, 0x65,
    0x73, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x72, 0x76, 0x22, 0x32, 0x0a, 0x07, 0x41, 0x63, 0x63, 0x6f,
    0x75, 0x6e, 0x74, 0x12, 0x0a, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x04, 0x12,
    0x0d, 0x0a, 0x05, 0x65, 0x6d, 0x61, 0x69, 0x6c, 0x18, 0x02, 0x20, 0x02, 0x28, 0x09, 0x12, 0x0c,
    0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x03, 0x20, 0x02, 0x28, 0x09, 0x22, 0x1a, 0x0a, 0x0a,
    0x41, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x47, 0x65, 0x74, 0x12, 0x0c, 0x0a, 0x04, 0x6e, 0x61,
    0x6d, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x22, 0x49, 0x0a, 0x0d, 0x41, 0x63, 0x63, 0x6f,
    0x75, 0x6e, 0x74, 0x53, 0x65, 0x61, 0x72, 0x63, 0x68, 0x12, 0x29, 0x0a, 0x03, 0x6b, 0x65, 0x79,
    0x18, 0x01, 0x20, 0x02, 0x28, 0x0e, 0x32, 0x1c, 0x2e, 0x73, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e,
    0x73, 0x72, 0x76, 0x2e, 0x41, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x53, 0x65, 0x61, 0x72, 0x63,
    0x68, 0x4b, 0x65, 0x79, 0x12, 0x0d, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x02, 0x20,
    0x02, 0x28, 0x09, 0x22, 0x30, 0x0a, 0x0f, 0x47, 0x72, 0x61, 0x6e, 0x74, 0x46, 0x6c, 0x61, 0x67,
    0x54, 0x6f, 0x54, 0x65, 0x61, 0x6d, 0x12, 0x0c, 0x0a, 0x04, 0x66, 0x6c, 0x61, 0x67, 0x18, 0x01,
    0x20, 0x02, 0x28, 0x0d, 0x12, 0x0f, 0x0a, 0x07, 0x74, 0x65, 0x61, 0x6d, 0x5f, 0x69, 0x64, 0x18,
    0x02, 0x20, 0x02, 0x28, 0x04, 0x22, 0x33, 0x0a, 0x12, 0x52, 0x65, 0x76, 0x6f, 0x6b, 0x65, 0x46,
    0x6c, 0x61, 0x67, 0x46, 0x72, 0x6f, 0x6d, 0x54, 0x65, 0x61, 0x6d, 0x12, 0x0c, 0x0a, 0x04, 0x66,
    0x6c, 0x61, 0x67, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0d, 0x12, 0x0f, 0x0a, 0x07, 0x74, 0x65, 0x61,
    0x6d, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x02, 0x28, 0x04, 0x22, 0x1e, 0x0a, 0x0e, 0x4c, 0x69,
    0x73, 0x74, 0x46, 0x6c, 0x61, 0x67, 0x47, 0x72, 0x61, 0x6e, 0x74, 0x73, 0x12, 0x0c, 0x0a, 0x04,
    0x66, 0x6c, 0x61, 0x67, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0d, 0x22, 0x1b, 0x0a, 0x0a, 0x46, 0x6c,
    0x61, 0x67, 0x47, 0x72, 0x61, 0x6e, 0x74, 0x73, 0x12, 0x0d, 0x0a, 0x05, 0x74, 0x65, 0x61, 0x6d,
    0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x04, 0x22, 0x50, 0x0a, 0x07, 0x53, 0x65, 0x73, 0x73, 0x69,
    0x6f, 0x6e, 0x12, 0x0a, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x04, 0x12, 0x0d,
    0x0a, 0x05, 0x65, 0x6d, 0x61, 0x69, 0x6c, 0x18, 0x02, 0x20, 0x02, 0x28, 0x09, 0x12, 0x0c, 0x0a,
    0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x03, 0x20, 0x02, 0x28, 0x09, 0x12, 0x0d, 0x0a, 0x05, 0x74,
    0x6f, 0x6b, 0x65, 0x6e, 0x18, 0x04, 0x20, 0x02, 0x28, 0x09, 0x12, 0x0d, 0x0a, 0x05, 0x66, 0x6c,
    0x61, 0x67, 0x73, 0x18, 0x05, 0x20, 0x02, 0x28, 0x0d, 0x22, 0x5c, 0x0a, 0x0c, 0x53, 0x65, 0x73,
    0x73, 0x69, 0x6f, 0x6e, 0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x12, 0x0d, 0x0a, 0x05, 0x74, 0x6f, 0x6b,
    0x65, 0x6e, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x12, 0x10, 0x0a, 0x08, 0x6f, 0x77, 0x6e, 0x65,
    0x72, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x02, 0x28, 0x04, 0x12, 0x2b, 0x0a, 0x08, 0x70, 0x72,
    0x6f, 0x76, 0x69, 0x64, 0x65, 0x72, 0x18, 0x03, 0x20, 0x02, 0x28, 0x0e, 0x32, 0x19, 0x2e, 0x73,
    0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x72, 0x76, 0x2e, 0x4f, 0x41, 0x75, 0x74, 0x68, 0x50,
    0x72, 0x6f, 0x76, 0x69, 0x64, 0x65, 0x72, 0x22, 0x7b, 0x0a, 0x0d, 0x53, 0x65, 0x73, 0x73, 0x69,
    0x6f, 0x6e, 0x43, 0x72, 0x65, 0x61, 0x74, 0x65, 0x12, 0x0d, 0x0a, 0x05, 0x74, 0x6f, 0x6b, 0x65,
    0x6e, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x12, 0x11, 0x0a, 0x09, 0x65, 0x78, 0x74, 0x65, 0x72,
    0x6e, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x02, 0x28, 0x04, 0x12, 0x0d, 0x0a, 0x05, 0x65, 0x6d,
    0x61, 0x69, 0x6c, 0x18, 0x03, 0x20, 0x02, 0x28, 0x09, 0x12, 0x0c, 0x0a, 0x04, 0x6e, 0x61, 0x6d,
    0x65, 0x18, 0x04, 0x20, 0x02, 0x28, 0x09, 0x12, 0x2b, 0x0a, 0x08, 0x70, 0x72, 0x6f, 0x76, 0x69,
    0x64, 0x65, 0x72, 0x18, 0x05, 0x20, 0x02, 0x28, 0x0e, 0x32, 0x19, 0x2e, 0x73, 0x65, 0x73, 0x73,
    0x69, 0x6f, 0x6e, 0x73, 0x72, 0x76, 0x2e, 0x4f, 0x41, 0x75, 0x74, 0x68, 0x50, 0x72, 0x6f, 0x76,
    0x69, 0x64, 0x65, 0x72, 0x22, 0x1b, 0x0a, 0x0a, 0x53, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x47,
    0x65, 0x74, 0x12, 0x0d, 0x0a, 0x05, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x18, 0x01, 0x20, 0x02, 0x28,
    0x09, 0x2a, 0x1b, 0x0a, 0x0d, 0x4f, 0x41, 0x75, 0x74, 0x68, 0x50, 0x72, 0x6f, 0x76, 0x69, 0x64,
    0x65, 0x72, 0x12, 0x0a, 0x0a, 0x06, 0x47, 0x69, 0x74, 0x48, 0x75, 0x62, 0x10, 0x00, 0x2a, 0x24,
    0x0a, 0x10, 0x41, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x53, 0x65, 0x61, 0x72, 0x63, 0x68, 0x4b,
    0x65, 0x79, 0x12, 0x06, 0x0a, 0x02, 0x49, 0x64, 0x10, 0x00, 0x12, 0x08, 0x0a, 0x04, 0x4e, 0x61,
    0x6d, 0x65, 0x10, 0x01, 0x4a, 0xe8, 0x11, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x45, 0x01, 0x0a,
    0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x00, 0x08, 0x12, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x00, 0x12,
    0x04, 0x02, 0x00, 0x04, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x00, 0x01, 0x12, 0x03, 0x02, 0x05,
    0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x00, 0x12, 0x03, 0x03, 0x02, 0x0d, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x03, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x03, 0x0b, 0x0c, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00,
    0x12, 0x04, 0x06, 0x00, 0x0a, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x06,
    0x08, 0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x07, 0x02, 0x19, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x07, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x07, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x07, 0x12, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x07, 0x17, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12,
    0x03, 0x08, 0x02, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x08,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x08, 0x0b, 0x11,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x08, 0x12, 0x17, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x08, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x09, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x02, 0x04, 0x12, 0x03, 0x09, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05,
    0x12, 0x03, 0x09, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03,
    0x09, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x09, 0x19,
    0x1a, 0x0a, 0x2b, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x0d, 0x00, 0x0f, 0x01, 0x1a, 0x1f, 0x20,
    0x67, 0x65, 0x74, 0x20, 0x61, 0x6e, 0x20, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x20, 0x62,
    0x79, 0x20, 0x47, 0x48, 0x20, 0x75, 0x73, 0x65, 0x72, 0x6e, 0x61, 0x6d, 0x65, 0x0a, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x0d, 0x08, 0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01,
    0x02, 0x00, 0x12, 0x03, 0x0e, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04,
    0x12, 0x03, 0x0e, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03,
    0x0e, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0e, 0x12,
    0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0e, 0x19, 0x1a, 0x0a,
    0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x11, 0x00, 0x14, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x02, 0x01, 0x12, 0x03, 0x11, 0x08, 0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12,
    0x03, 0x12, 0x02, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x12,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x06, 0x12, 0x03, 0x12, 0x0b, 0x1b,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x12, 0x1c, 0x1f, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x12, 0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x13, 0x02, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x01, 0x04, 0x12, 0x03, 0x13, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x05,
    0x12, 0x03, 0x13, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x13, 0x12, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x13, 0x1a,
    0x1b, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x01, 0x12, 0x04, 0x16, 0x00, 0x19, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x05, 0x01, 0x01, 0x12, 0x03, 0x16, 0x05, 0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02,
    0x00, 0x12, 0x03, 0x17, 0x02, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x17, 0x02, 0x04, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x00, 0x02, 0x12, 0x03, 0x17,
    0x07, 0x08, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x01, 0x12, 0x03, 0x18, 0x02, 0x0b, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x18, 0x02, 0x06, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x01, 0x02, 0x01, 0x02, 0x12, 0x03, 0x18, 0x09, 0x0a, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x03, 0x12, 0x04, 0x1b, 0x00, 0x1e, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03,
    0x1b, 0x08, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x1c, 0x02, 0x1b,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x03, 0x1c, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x05, 0x12, 0x03, 0x1c, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1c, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x1c, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x01,
    0x12, 0x03, 0x1d, 0x02, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x04, 0x12, 0x03,
    0x1d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x05, 0x12, 0x03, 0x1d, 0x0b,
    0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x01, 0x12, 0x03, 0x1d, 0x12, 0x19, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x03, 0x12, 0x03, 0x1d, 0x1c, 0x1d, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x04, 0x12, 0x04, 0x20, 0x00, 0x23, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01,
    0x12, 0x03, 0x20, 0x08, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x21,
    0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x04, 0x12, 0x03, 0x21, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x05, 0x12, 0x03, 0x21, 0x0b, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x21, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x21, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04,
    0x02, 0x01, 0x12, 0x03, 0x22, 0x02, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x04,
    0x12, 0x03, 0x22, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x05, 0x12, 0x03,
    0x22, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x01, 0x12, 0x03, 0x22, 0x12,
    0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x03, 0x12, 0x03, 0x22, 0x1c, 0x1d, 0x0a,
    0x0a, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x25, 0x00, 0x27, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x05, 0x01, 0x12, 0x03, 0x25, 0x08, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x00, 0x12,
    0x03, 0x26, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x04, 0x12, 0x03, 0x26,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x05, 0x12, 0x03, 0x26, 0x0b, 0x11,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12, 0x03, 0x26, 0x12, 0x16, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x26, 0x19, 0x1a, 0x0a, 0x0a, 0x0a, 0x02,
    0x04, 0x06, 0x12, 0x04, 0x29, 0x00, 0x2b, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x06, 0x01, 0x12,
    0x03, 0x29, 0x08, 0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x00, 0x12, 0x03, 0x2a, 0x02,
    0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x04, 0x12, 0x03, 0x2a, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x05, 0x12, 0x03, 0x2a, 0x0b, 0x11, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2a, 0x12, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x06, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2a, 0x1a, 0x1b, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x07, 0x12,
    0x04, 0x2d, 0x00, 0x33, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x07, 0x01, 0x12, 0x03, 0x2d, 0x08,
    0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x00, 0x12, 0x03, 0x2e, 0x02, 0x19, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x04, 0x12, 0x03, 0x2e, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x07, 0x02, 0x00, 0x05, 0x12, 0x03, 0x2e, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x2e, 0x12, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x2e, 0x17, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x01, 0x12, 0x03,
    0x2f, 0x02, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x04, 0x12, 0x03, 0x2f, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x05, 0x12, 0x03, 0x2f, 0x0b, 0x11, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x01, 0x12, 0x03, 0x2f, 0x12, 0x17, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x07, 0x02, 0x01, 0x03, 0x12, 0x03, 0x2f, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x07, 0x02, 0x02, 0x12, 0x03, 0x30, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02,
    0x04, 0x12, 0x03, 0x30, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x05, 0x12,
    0x03, 0x30, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x01, 0x12, 0x03, 0x30,
    0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x03, 0x12, 0x03, 0x30, 0x19, 0x1a,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x03, 0x12, 0x03, 0x31, 0x02, 0x1c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x07, 0x02, 0x03, 0x04, 0x12, 0x03, 0x31, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x07, 0x02, 0x03, 0x05, 0x12, 0x03, 0x31, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02,
    0x03, 0x01, 0x12, 0x03, 0x31, 0x12, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x03, 0x03,
    0x12, 0x03, 0x31, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x04, 0x12, 0x03, 0x32,
    0x02, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x04, 0x04, 0x12, 0x03, 0x32, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x04, 0x05, 0x12, 0x03, 0x32, 0x0b, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x07, 0x02, 0x04, 0x01, 0x12, 0x03, 0x32, 0x12, 0x17, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x07, 0x02, 0x04, 0x03, 0x12, 0x03, 0x32, 0x1a, 0x1b, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x08,
    0x12, 0x04, 0x35, 0x00, 0x39, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x08, 0x01, 0x12, 0x03, 0x35,
    0x08, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x00, 0x12, 0x03, 0x36, 0x02, 0x1c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x04, 0x12, 0x03, 0x36, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x08, 0x02, 0x00, 0x05, 0x12, 0x03, 0x36, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x08, 0x02, 0x00, 0x01, 0x12, 0x03, 0x36, 0x12, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x36, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x01, 0x12,
    0x03, 0x37, 0x02, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x04, 0x12, 0x03, 0x37,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x05, 0x12, 0x03, 0x37, 0x0b, 0x11,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x01, 0x12, 0x03, 0x37, 0x12, 0x1a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x03, 0x12, 0x03, 0x37, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x08, 0x02, 0x02, 0x12, 0x03, 0x38, 0x02, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02,
    0x02, 0x04, 0x12, 0x03, 0x38, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x06,
    0x12, 0x03, 0x38, 0x0b, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x01, 0x12, 0x03,
    0x38, 0x19, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x03, 0x12, 0x03, 0x38, 0x24,
    0x25, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x09, 0x12, 0x04, 0x3b, 0x00, 0x41, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x09, 0x01, 0x12, 0x03, 0x3b, 0x08, 0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02,
    0x00, 0x12, 0x03, 0x3c, 0x02, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x04, 0x12,
    0x03, 0x3c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x05, 0x12, 0x03, 0x3c,
    0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x01, 0x12, 0x03, 0x3c, 0x12, 0x17,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x03, 0x12, 0x03, 0x3c, 0x1a, 0x1b, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x09, 0x02, 0x01, 0x12, 0x03, 0x3d, 0x02, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x09, 0x02, 0x01, 0x04, 0x12, 0x03, 0x3d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02,
    0x01, 0x05, 0x12, 0x03, 0x3d, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x3d, 0x12, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x3d, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x02, 0x12, 0x03, 0x3e, 0x02, 0x1c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x02, 0x04, 0x12, 0x03, 0x3e, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x09, 0x02, 0x02, 0x05, 0x12, 0x03, 0x3e, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x09, 0x02, 0x02, 0x01, 0x12, 0x03, 0x3e, 0x12, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09,
    0x02, 0x02, 0x03, 0x12, 0x03, 0x3e, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x03,
    0x12, 0x03, 0x3f, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x03, 0x04, 0x12, 0x03,
    0x3f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x03, 0x05, 0x12, 0x03, 0x3f, 0x0b,
    0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x03, 0x01, 0x12, 0x03, 0x3f, 0x12, 0x16, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x03, 0x03, 0x12, 0x03, 0x3f, 0x19, 0x1a, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x09, 0x02, 0x04, 0x12, 0x03, 0x40, 0x02, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09,
    0x02, 0x04, 0x04, 0x12, 0x03, 0x40, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x04,
    0x06, 0x12, 0x03, 0x40, 0x0b, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x04, 0x01, 0x12,
    0x03, 0x40, 0x19, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x04, 0x03, 0x12, 0x03, 0x40,
    0x24, 0x25, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0a, 0x12, 0x04, 0x43, 0x00, 0x45, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x0a, 0x01, 0x12, 0x03, 0x43, 0x08, 0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0a,
    0x02, 0x00, 0x12, 0x03, 0x44, 0x02, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x04,
    0x12, 0x03, 0x44, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x05, 0x12, 0x03,
    0x44, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x01, 0x12, 0x03, 0x44, 0x12,
    0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x03, 0x12, 0x03, 0x44, 0x1a, 0x1b,
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
