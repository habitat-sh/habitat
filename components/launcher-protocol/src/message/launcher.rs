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
pub struct Register {
    // message fields
    pipe: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Register {}

impl Register {
    pub fn new() -> Register {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Register {
        static mut instance: ::protobuf::lazy::Lazy<Register> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Register,
        };
        unsafe {
            instance.get(Register::new)
        }
    }

    // optional string pipe = 1;

    pub fn clear_pipe(&mut self) {
        self.pipe.clear();
    }

    pub fn has_pipe(&self) -> bool {
        self.pipe.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pipe(&mut self, v: ::std::string::String) {
        self.pipe = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_pipe(&mut self) -> &mut ::std::string::String {
        if self.pipe.is_none() {
            self.pipe.set_default();
        }
        self.pipe.as_mut().unwrap()
    }

    // Take field
    pub fn take_pipe(&mut self) -> ::std::string::String {
        self.pipe.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_pipe(&self) -> &str {
        match self.pipe.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_pipe_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.pipe
    }

    fn mut_pipe_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.pipe
    }
}

impl ::protobuf::Message for Register {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.pipe)?;
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
        if let Some(ref v) = self.pipe.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.pipe.as_ref() {
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

impl ::protobuf::MessageStatic for Register {
    fn new() -> Register {
        Register::new()
    }

    fn descriptor_static(_: ::std::option::Option<Register>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "pipe",
                    Register::get_pipe_for_reflect,
                    Register::mut_pipe_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Register>(
                    "Register",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Register {
    fn clear(&mut self) {
        self.clear_pipe();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Register {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Register {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Restart {
    // message fields
    pid: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Restart {}

impl Restart {
    pub fn new() -> Restart {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Restart {
        static mut instance: ::protobuf::lazy::Lazy<Restart> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Restart,
        };
        unsafe {
            instance.get(Restart::new)
        }
    }

    // optional int64 pid = 1;

    pub fn clear_pid(&mut self) {
        self.pid = ::std::option::Option::None;
    }

    pub fn has_pid(&self) -> bool {
        self.pid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pid(&mut self, v: i64) {
        self.pid = ::std::option::Option::Some(v);
    }

    pub fn get_pid(&self) -> i64 {
        self.pid.unwrap_or(0)
    }

    fn get_pid_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.pid
    }

    fn mut_pid_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.pid
    }
}

impl ::protobuf::Message for Restart {
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
                    let tmp = is.read_int64()?;
                    self.pid = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.pid {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.pid {
            os.write_int64(1, v)?;
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

impl ::protobuf::MessageStatic for Restart {
    fn new() -> Restart {
        Restart::new()
    }

    fn descriptor_static(_: ::std::option::Option<Restart>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "pid",
                    Restart::get_pid_for_reflect,
                    Restart::mut_pid_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Restart>(
                    "Restart",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Restart {
    fn clear(&mut self) {
        self.clear_pid();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Restart {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Restart {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Spawn {
    // message fields
    id: ::protobuf::SingularField<::std::string::String>,
    binary: ::protobuf::SingularField<::std::string::String>,
    svc_user: ::protobuf::SingularField<::std::string::String>,
    svc_group: ::protobuf::SingularField<::std::string::String>,
    svc_password: ::protobuf::SingularField<::std::string::String>,
    pub env: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Spawn {}

impl Spawn {
    pub fn new() -> Spawn {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Spawn {
        static mut instance: ::protobuf::lazy::Lazy<Spawn> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Spawn,
        };
        unsafe {
            instance.get(Spawn::new)
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
        }
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

    fn get_id_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.id
    }

    // optional string binary = 2;

    pub fn clear_binary(&mut self) {
        self.binary.clear();
    }

    pub fn has_binary(&self) -> bool {
        self.binary.is_some()
    }

    // Param is passed by value, moved
    pub fn set_binary(&mut self, v: ::std::string::String) {
        self.binary = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_binary(&mut self) -> &mut ::std::string::String {
        if self.binary.is_none() {
            self.binary.set_default();
        }
        self.binary.as_mut().unwrap()
    }

    // Take field
    pub fn take_binary(&mut self) -> ::std::string::String {
        self.binary.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_binary(&self) -> &str {
        match self.binary.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_binary_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.binary
    }

    fn mut_binary_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.binary
    }

    // optional string svc_user = 3;

    pub fn clear_svc_user(&mut self) {
        self.svc_user.clear();
    }

    pub fn has_svc_user(&self) -> bool {
        self.svc_user.is_some()
    }

    // Param is passed by value, moved
    pub fn set_svc_user(&mut self, v: ::std::string::String) {
        self.svc_user = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_svc_user(&mut self) -> &mut ::std::string::String {
        if self.svc_user.is_none() {
            self.svc_user.set_default();
        }
        self.svc_user.as_mut().unwrap()
    }

    // Take field
    pub fn take_svc_user(&mut self) -> ::std::string::String {
        self.svc_user.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_svc_user(&self) -> &str {
        match self.svc_user.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_svc_user_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.svc_user
    }

    fn mut_svc_user_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.svc_user
    }

    // optional string svc_group = 4;

    pub fn clear_svc_group(&mut self) {
        self.svc_group.clear();
    }

    pub fn has_svc_group(&self) -> bool {
        self.svc_group.is_some()
    }

    // Param is passed by value, moved
    pub fn set_svc_group(&mut self, v: ::std::string::String) {
        self.svc_group = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_svc_group(&mut self) -> &mut ::std::string::String {
        if self.svc_group.is_none() {
            self.svc_group.set_default();
        }
        self.svc_group.as_mut().unwrap()
    }

    // Take field
    pub fn take_svc_group(&mut self) -> ::std::string::String {
        self.svc_group.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_svc_group(&self) -> &str {
        match self.svc_group.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_svc_group_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.svc_group
    }

    fn mut_svc_group_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.svc_group
    }

    // optional string svc_password = 5;

    pub fn clear_svc_password(&mut self) {
        self.svc_password.clear();
    }

    pub fn has_svc_password(&self) -> bool {
        self.svc_password.is_some()
    }

    // Param is passed by value, moved
    pub fn set_svc_password(&mut self, v: ::std::string::String) {
        self.svc_password = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_svc_password(&mut self) -> &mut ::std::string::String {
        if self.svc_password.is_none() {
            self.svc_password.set_default();
        }
        self.svc_password.as_mut().unwrap()
    }

    // Take field
    pub fn take_svc_password(&mut self) -> ::std::string::String {
        self.svc_password.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_svc_password(&self) -> &str {
        match self.svc_password.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_svc_password_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.svc_password
    }

    fn mut_svc_password_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.svc_password
    }

    // repeated .launcher.Spawn.EnvEntry env = 6;

    pub fn clear_env(&mut self) {
        self.env.clear();
    }

    // Param is passed by value, moved
    pub fn set_env(&mut self, v: ::std::collections::HashMap<::std::string::String, ::std::string::String>) {
        self.env = v;
    }

    // Mutable pointer to the field.
    pub fn mut_env(&mut self) -> &mut ::std::collections::HashMap<::std::string::String, ::std::string::String> {
        &mut self.env
    }

    // Take field
    pub fn take_env(&mut self) -> ::std::collections::HashMap<::std::string::String, ::std::string::String> {
        ::std::mem::replace(&mut self.env, ::std::collections::HashMap::new())
    }

    pub fn get_env(&self) -> &::std::collections::HashMap<::std::string::String, ::std::string::String> {
        &self.env
    }

    fn get_env_for_reflect(&self) -> &::std::collections::HashMap<::std::string::String, ::std::string::String> {
        &self.env
    }

    fn mut_env_for_reflect(&mut self) -> &mut ::std::collections::HashMap<::std::string::String, ::std::string::String> {
        &mut self.env
    }
}

impl ::protobuf::Message for Spawn {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.id)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.binary)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.svc_user)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.svc_group)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.svc_password)?;
                },
                6 => {
                    ::protobuf::rt::read_map_into::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(wire_type, is, &mut self.env)?;
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
        if let Some(ref v) = self.id.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.binary.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(ref v) = self.svc_user.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        if let Some(ref v) = self.svc_group.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        }
        if let Some(ref v) = self.svc_password.as_ref() {
            my_size += ::protobuf::rt::string_size(5, &v);
        }
        my_size += ::protobuf::rt::compute_map_size::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(6, &self.env);
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.id.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.binary.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(ref v) = self.svc_user.as_ref() {
            os.write_string(3, &v)?;
        }
        if let Some(ref v) = self.svc_group.as_ref() {
            os.write_string(4, &v)?;
        }
        if let Some(ref v) = self.svc_password.as_ref() {
            os.write_string(5, &v)?;
        }
        ::protobuf::rt::write_map_with_cached_sizes::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(6, &self.env, os)?;
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

impl ::protobuf::MessageStatic for Spawn {
    fn new() -> Spawn {
        Spawn::new()
    }

    fn descriptor_static(_: ::std::option::Option<Spawn>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "id",
                    Spawn::get_id_for_reflect,
                    Spawn::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "binary",
                    Spawn::get_binary_for_reflect,
                    Spawn::mut_binary_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "svc_user",
                    Spawn::get_svc_user_for_reflect,
                    Spawn::mut_svc_user_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "svc_group",
                    Spawn::get_svc_group_for_reflect,
                    Spawn::mut_svc_group_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "svc_password",
                    Spawn::get_svc_password_for_reflect,
                    Spawn::mut_svc_password_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_map_accessor::<_, ::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(
                    "env",
                    Spawn::get_env_for_reflect,
                    Spawn::mut_env_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Spawn>(
                    "Spawn",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Spawn {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_binary();
        self.clear_svc_user();
        self.clear_svc_group();
        self.clear_svc_password();
        self.clear_env();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Spawn {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Spawn {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SpawnOk {
    // message fields
    pid: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SpawnOk {}

impl SpawnOk {
    pub fn new() -> SpawnOk {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SpawnOk {
        static mut instance: ::protobuf::lazy::Lazy<SpawnOk> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SpawnOk,
        };
        unsafe {
            instance.get(SpawnOk::new)
        }
    }

    // optional int64 pid = 1;

    pub fn clear_pid(&mut self) {
        self.pid = ::std::option::Option::None;
    }

    pub fn has_pid(&self) -> bool {
        self.pid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pid(&mut self, v: i64) {
        self.pid = ::std::option::Option::Some(v);
    }

    pub fn get_pid(&self) -> i64 {
        self.pid.unwrap_or(0)
    }

    fn get_pid_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.pid
    }

    fn mut_pid_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.pid
    }
}

impl ::protobuf::Message for SpawnOk {
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
                    let tmp = is.read_int64()?;
                    self.pid = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.pid {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.pid {
            os.write_int64(1, v)?;
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

impl ::protobuf::MessageStatic for SpawnOk {
    fn new() -> SpawnOk {
        SpawnOk::new()
    }

    fn descriptor_static(_: ::std::option::Option<SpawnOk>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "pid",
                    SpawnOk::get_pid_for_reflect,
                    SpawnOk::mut_pid_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SpawnOk>(
                    "SpawnOk",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SpawnOk {
    fn clear(&mut self) {
        self.clear_pid();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SpawnOk {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SpawnOk {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Terminate {
    // message fields
    pid: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Terminate {}

impl Terminate {
    pub fn new() -> Terminate {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Terminate {
        static mut instance: ::protobuf::lazy::Lazy<Terminate> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Terminate,
        };
        unsafe {
            instance.get(Terminate::new)
        }
    }

    // optional int64 pid = 1;

    pub fn clear_pid(&mut self) {
        self.pid = ::std::option::Option::None;
    }

    pub fn has_pid(&self) -> bool {
        self.pid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pid(&mut self, v: i64) {
        self.pid = ::std::option::Option::Some(v);
    }

    pub fn get_pid(&self) -> i64 {
        self.pid.unwrap_or(0)
    }

    fn get_pid_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.pid
    }

    fn mut_pid_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.pid
    }
}

impl ::protobuf::Message for Terminate {
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
                    let tmp = is.read_int64()?;
                    self.pid = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.pid {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.pid {
            os.write_int64(1, v)?;
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

impl ::protobuf::MessageStatic for Terminate {
    fn new() -> Terminate {
        Terminate::new()
    }

    fn descriptor_static(_: ::std::option::Option<Terminate>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "pid",
                    Terminate::get_pid_for_reflect,
                    Terminate::mut_pid_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Terminate>(
                    "Terminate",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Terminate {
    fn clear(&mut self) {
        self.clear_pid();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Terminate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Terminate {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TerminateOk {
    // message fields
    exit_code: ::std::option::Option<i32>,
    shutdown_method: ::std::option::Option<ShutdownMethod>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TerminateOk {}

impl TerminateOk {
    pub fn new() -> TerminateOk {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TerminateOk {
        static mut instance: ::protobuf::lazy::Lazy<TerminateOk> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TerminateOk,
        };
        unsafe {
            instance.get(TerminateOk::new)
        }
    }

    // optional int32 exit_code = 1;

    pub fn clear_exit_code(&mut self) {
        self.exit_code = ::std::option::Option::None;
    }

    pub fn has_exit_code(&self) -> bool {
        self.exit_code.is_some()
    }

    // Param is passed by value, moved
    pub fn set_exit_code(&mut self, v: i32) {
        self.exit_code = ::std::option::Option::Some(v);
    }

    pub fn get_exit_code(&self) -> i32 {
        self.exit_code.unwrap_or(0)
    }

    fn get_exit_code_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.exit_code
    }

    fn mut_exit_code_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.exit_code
    }

    // optional .launcher.ShutdownMethod shutdown_method = 2;

    pub fn clear_shutdown_method(&mut self) {
        self.shutdown_method = ::std::option::Option::None;
    }

    pub fn has_shutdown_method(&self) -> bool {
        self.shutdown_method.is_some()
    }

    // Param is passed by value, moved
    pub fn set_shutdown_method(&mut self, v: ShutdownMethod) {
        self.shutdown_method = ::std::option::Option::Some(v);
    }

    pub fn get_shutdown_method(&self) -> ShutdownMethod {
        self.shutdown_method.unwrap_or(ShutdownMethod::AlreadyExited)
    }

    fn get_shutdown_method_for_reflect(&self) -> &::std::option::Option<ShutdownMethod> {
        &self.shutdown_method
    }

    fn mut_shutdown_method_for_reflect(&mut self) -> &mut ::std::option::Option<ShutdownMethod> {
        &mut self.shutdown_method
    }
}

impl ::protobuf::Message for TerminateOk {
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
                    let tmp = is.read_int32()?;
                    self.exit_code = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.shutdown_method = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.exit_code {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.shutdown_method {
            my_size += ::protobuf::rt::enum_size(2, v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.exit_code {
            os.write_int32(1, v)?;
        }
        if let Some(v) = self.shutdown_method {
            os.write_enum(2, v.value())?;
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

impl ::protobuf::MessageStatic for TerminateOk {
    fn new() -> TerminateOk {
        TerminateOk::new()
    }

    fn descriptor_static(_: ::std::option::Option<TerminateOk>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "exit_code",
                    TerminateOk::get_exit_code_for_reflect,
                    TerminateOk::mut_exit_code_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<ShutdownMethod>>(
                    "shutdown_method",
                    TerminateOk::get_shutdown_method_for_reflect,
                    TerminateOk::mut_shutdown_method_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TerminateOk>(
                    "TerminateOk",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TerminateOk {
    fn clear(&mut self) {
        self.clear_exit_code();
        self.clear_shutdown_method();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TerminateOk {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TerminateOk {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ShutdownMethod {
    AlreadyExited = 0,
    GracefulTermination = 1,
    Killed = 2,
}

impl ::protobuf::ProtobufEnum for ShutdownMethod {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ShutdownMethod> {
        match value {
            0 => ::std::option::Option::Some(ShutdownMethod::AlreadyExited),
            1 => ::std::option::Option::Some(ShutdownMethod::GracefulTermination),
            2 => ::std::option::Option::Some(ShutdownMethod::Killed),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ShutdownMethod] = &[
            ShutdownMethod::AlreadyExited,
            ShutdownMethod::GracefulTermination,
            ShutdownMethod::Killed,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<ShutdownMethod>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ShutdownMethod", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ShutdownMethod {
}

impl ::protobuf::reflect::ProtobufValue for ShutdownMethod {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x18protocols/launcher.proto\x12\x08launcher\"\x1e\n\x08Register\x12\
    \x12\n\x04pipe\x18\x01\x20\x01(\tR\x04pipe\"\x1b\n\x07Restart\x12\x10\n\
    \x03pid\x18\x01\x20\x01(\x03R\x03pid\"\xee\x01\n\x05Spawn\x12\x0e\n\x02i\
    d\x18\x01\x20\x01(\tR\x02id\x12\x16\n\x06binary\x18\x02\x20\x01(\tR\x06b\
    inary\x12\x19\n\x08svc_user\x18\x03\x20\x01(\tR\x07svcUser\x12\x1b\n\tsv\
    c_group\x18\x04\x20\x01(\tR\x08svcGroup\x12!\n\x0csvc_password\x18\x05\
    \x20\x01(\tR\x0bsvcPassword\x12*\n\x03env\x18\x06\x20\x03(\x0b2\x18.laun\
    cher.Spawn.EnvEntryR\x03env\x1a6\n\x08EnvEntry\x12\x10\n\x03key\x18\x01\
    \x20\x01(\tR\x03key\x12\x14\n\x05value\x18\x02\x20\x01(\tR\x05value:\x02\
    8\x01\"\x1b\n\x07SpawnOk\x12\x10\n\x03pid\x18\x01\x20\x01(\x03R\x03pid\"\
    \x1d\n\tTerminate\x12\x10\n\x03pid\x18\x01\x20\x01(\x03R\x03pid\"m\n\x0b\
    TerminateOk\x12\x1b\n\texit_code\x18\x01\x20\x01(\x05R\x08exitCode\x12A\
    \n\x0fshutdown_method\x18\x02\x20\x01(\x0e2\x18.launcher.ShutdownMethodR\
    \x0eshutdownMethod*H\n\x0eShutdownMethod\x12\x11\n\rAlreadyExited\x10\0\
    \x12\x17\n\x13GracefulTermination\x10\x01\x12\n\n\x06Killed\x10\x02J\xfc\
    \x08\n\x06\x12\x04\0\0&\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\x08\n\x01\
    \x02\x12\x03\x02\x08\x10\n\n\n\x02\x04\0\x12\x04\x04\0\x06\x01\n\n\n\x03\
    \x04\0\x01\x12\x03\x04\x08\x10\n\x0b\n\x04\x04\0\x02\0\x12\x03\x05\x02\
    \x1b\n\x0c\n\x05\x04\0\x02\0\x04\x12\x03\x05\x02\n\n\x0c\n\x05\x04\0\x02\
    \0\x05\x12\x03\x05\x0b\x11\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x05\x12\
    \x16\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x05\x19\x1a\n\n\n\x02\x04\x01\
    \x12\x04\x08\0\n\x01\n\n\n\x03\x04\x01\x01\x12\x03\x08\x08\x0f\n\x0b\n\
    \x04\x04\x01\x02\0\x12\x03\t\x02\x19\n\x0c\n\x05\x04\x01\x02\0\x04\x12\
    \x03\t\x02\n\n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03\t\x0b\x10\n\x0c\n\x05\
    \x04\x01\x02\0\x01\x12\x03\t\x11\x14\n\x0c\n\x05\x04\x01\x02\0\x03\x12\
    \x03\t\x17\x18\n\n\n\x02\x04\x02\x12\x04\x0c\0\x13\x01\n\n\n\x03\x04\x02\
    \x01\x12\x03\x0c\x08\r\n\x0b\n\x04\x04\x02\x02\0\x12\x03\r\x02\x19\n\x0c\
    \n\x05\x04\x02\x02\0\x04\x12\x03\r\x02\n\n\x0c\n\x05\x04\x02\x02\0\x05\
    \x12\x03\r\x0b\x11\n\x0c\n\x05\x04\x02\x02\0\x01\x12\x03\r\x12\x14\n\x0c\
    \n\x05\x04\x02\x02\0\x03\x12\x03\r\x17\x18\n\x0b\n\x04\x04\x02\x02\x01\
    \x12\x03\x0e\x02\x1d\n\x0c\n\x05\x04\x02\x02\x01\x04\x12\x03\x0e\x02\n\n\
    \x0c\n\x05\x04\x02\x02\x01\x05\x12\x03\x0e\x0b\x11\n\x0c\n\x05\x04\x02\
    \x02\x01\x01\x12\x03\x0e\x12\x18\n\x0c\n\x05\x04\x02\x02\x01\x03\x12\x03\
    \x0e\x1b\x1c\n\x0b\n\x04\x04\x02\x02\x02\x12\x03\x0f\x02\x1f\n\x0c\n\x05\
    \x04\x02\x02\x02\x04\x12\x03\x0f\x02\n\n\x0c\n\x05\x04\x02\x02\x02\x05\
    \x12\x03\x0f\x0b\x11\n\x0c\n\x05\x04\x02\x02\x02\x01\x12\x03\x0f\x12\x1a\
    \n\x0c\n\x05\x04\x02\x02\x02\x03\x12\x03\x0f\x1d\x1e\n\x0b\n\x04\x04\x02\
    \x02\x03\x12\x03\x10\x02\x20\n\x0c\n\x05\x04\x02\x02\x03\x04\x12\x03\x10\
    \x02\n\n\x0c\n\x05\x04\x02\x02\x03\x05\x12\x03\x10\x0b\x11\n\x0c\n\x05\
    \x04\x02\x02\x03\x01\x12\x03\x10\x12\x1b\n\x0c\n\x05\x04\x02\x02\x03\x03\
    \x12\x03\x10\x1e\x1f\n\x0b\n\x04\x04\x02\x02\x04\x12\x03\x11\x02#\n\x0c\
    \n\x05\x04\x02\x02\x04\x04\x12\x03\x11\x02\n\n\x0c\n\x05\x04\x02\x02\x04\
    \x05\x12\x03\x11\x0b\x11\n\x0c\n\x05\x04\x02\x02\x04\x01\x12\x03\x11\x12\
    \x1e\n\x0c\n\x05\x04\x02\x02\x04\x03\x12\x03\x11!\"\n\x0b\n\x04\x04\x02\
    \x02\x05\x12\x03\x12\x02\x1e\n\r\n\x05\x04\x02\x02\x05\x04\x12\x04\x12\
    \x02\x11#\n\x0c\n\x05\x04\x02\x02\x05\x06\x12\x03\x12\x02\x15\n\x0c\n\
    \x05\x04\x02\x02\x05\x01\x12\x03\x12\x16\x19\n\x0c\n\x05\x04\x02\x02\x05\
    \x03\x12\x03\x12\x1c\x1d\n\n\n\x02\x04\x03\x12\x04\x15\0\x17\x01\n\n\n\
    \x03\x04\x03\x01\x12\x03\x15\x08\x0f\n\x0b\n\x04\x04\x03\x02\0\x12\x03\
    \x16\x02\x19\n\x0c\n\x05\x04\x03\x02\0\x04\x12\x03\x16\x02\n\n\x0c\n\x05\
    \x04\x03\x02\0\x05\x12\x03\x16\x0b\x10\n\x0c\n\x05\x04\x03\x02\0\x01\x12\
    \x03\x16\x11\x14\n\x0c\n\x05\x04\x03\x02\0\x03\x12\x03\x16\x17\x18\n\n\n\
    \x02\x04\x04\x12\x04\x19\0\x1b\x01\n\n\n\x03\x04\x04\x01\x12\x03\x19\x08\
    \x11\n\x0b\n\x04\x04\x04\x02\0\x12\x03\x1a\x02\x19\n\x0c\n\x05\x04\x04\
    \x02\0\x04\x12\x03\x1a\x02\n\n\x0c\n\x05\x04\x04\x02\0\x05\x12\x03\x1a\
    \x0b\x10\n\x0c\n\x05\x04\x04\x02\0\x01\x12\x03\x1a\x11\x14\n\x0c\n\x05\
    \x04\x04\x02\0\x03\x12\x03\x1a\x17\x18\n\n\n\x02\x04\x05\x12\x04\x1d\0\
    \x20\x01\n\n\n\x03\x04\x05\x01\x12\x03\x1d\x08\x13\n\x0b\n\x04\x04\x05\
    \x02\0\x12\x03\x1e\x02\x1f\n\x0c\n\x05\x04\x05\x02\0\x04\x12\x03\x1e\x02\
    \n\n\x0c\n\x05\x04\x05\x02\0\x05\x12\x03\x1e\x0b\x10\n\x0c\n\x05\x04\x05\
    \x02\0\x01\x12\x03\x1e\x11\x1a\n\x0c\n\x05\x04\x05\x02\0\x03\x12\x03\x1e\
    \x1d\x1e\n\x0b\n\x04\x04\x05\x02\x01\x12\x03\x1f\x02.\n\x0c\n\x05\x04\
    \x05\x02\x01\x04\x12\x03\x1f\x02\n\n\x0c\n\x05\x04\x05\x02\x01\x06\x12\
    \x03\x1f\x0b\x19\n\x0c\n\x05\x04\x05\x02\x01\x01\x12\x03\x1f\x1a)\n\x0c\
    \n\x05\x04\x05\x02\x01\x03\x12\x03\x1f,-\n\n\n\x02\x05\0\x12\x04\"\0&\
    \x01\n\n\n\x03\x05\0\x01\x12\x03\"\x05\x13\n\x0b\n\x04\x05\0\x02\0\x12\
    \x03#\x02\x14\n\x0c\n\x05\x05\0\x02\0\x01\x12\x03#\x02\x0f\n\x0c\n\x05\
    \x05\0\x02\0\x02\x12\x03#\x12\x13\n\x0b\n\x04\x05\0\x02\x01\x12\x03$\x02\
    \x1a\n\x0c\n\x05\x05\0\x02\x01\x01\x12\x03$\x02\x15\n\x0c\n\x05\x05\0\
    \x02\x01\x02\x12\x03$\x18\x19\n\x0b\n\x04\x05\0\x02\x02\x12\x03%\x02\r\n\
    \x0c\n\x05\x05\0\x02\x02\x01\x12\x03%\x02\x08\n\x0c\n\x05\x05\0\x02\x02\
    \x02\x12\x03%\x0b\x0c\
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
