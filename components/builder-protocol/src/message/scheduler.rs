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
pub struct Project {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    ident: ::protobuf::SingularField<::std::string::String>,
    state: ::std::option::Option<ProjectState>,
    job_id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Project {}

impl Project {
    pub fn new() -> Project {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Project {
        static mut instance: ::protobuf::lazy::Lazy<Project> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Project,
        };
        unsafe {
            instance.get(Project::new)
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

    // optional string ident = 2;

    pub fn clear_ident(&mut self) {
        self.ident.clear();
    }

    pub fn has_ident(&self) -> bool {
        self.ident.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ident(&mut self, v: ::std::string::String) {
        self.ident = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ident(&mut self) -> &mut ::std::string::String {
        if self.ident.is_none() {
            self.ident.set_default();
        }
        self.ident.as_mut().unwrap()
    }

    // Take field
    pub fn take_ident(&mut self) -> ::std::string::String {
        self.ident.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_ident(&self) -> &str {
        match self.ident.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_ident_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.ident
    }

    fn mut_ident_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.ident
    }

    // optional .scheduler.ProjectState state = 3;

    pub fn clear_state(&mut self) {
        self.state = ::std::option::Option::None;
    }

    pub fn has_state(&self) -> bool {
        self.state.is_some()
    }

    // Param is passed by value, moved
    pub fn set_state(&mut self, v: ProjectState) {
        self.state = ::std::option::Option::Some(v);
    }

    pub fn get_state(&self) -> ProjectState {
        self.state.unwrap_or(ProjectState::NotStarted)
    }

    fn get_state_for_reflect(&self) -> &::std::option::Option<ProjectState> {
        &self.state
    }

    fn mut_state_for_reflect(&mut self) -> &mut ::std::option::Option<ProjectState> {
        &mut self.state
    }

    // optional uint64 job_id = 4;

    pub fn clear_job_id(&mut self) {
        self.job_id = ::std::option::Option::None;
    }

    pub fn has_job_id(&self) -> bool {
        self.job_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_job_id(&mut self, v: u64) {
        self.job_id = ::std::option::Option::Some(v);
    }

    pub fn get_job_id(&self) -> u64 {
        self.job_id.unwrap_or(0)
    }

    fn get_job_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.job_id
    }

    fn mut_job_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.job_id
    }
}

impl ::protobuf::Message for Project {
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.ident)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.state = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.job_id = ::std::option::Option::Some(tmp);
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
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(v) = self.state {
            my_size += ::protobuf::rt::enum_size(3, v);
        }
        if let Some(v) = self.job_id {
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
            os.write_string(2, &v)?;
        }
        if let Some(v) = self.state {
            os.write_enum(3, v.value())?;
        }
        if let Some(v) = self.job_id {
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

impl ::protobuf::MessageStatic for Project {
    fn new() -> Project {
        Project::new()
    }

    fn descriptor_static(_: ::std::option::Option<Project>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    Project::get_name_for_reflect,
                    Project::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "ident",
                    Project::get_ident_for_reflect,
                    Project::mut_ident_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<ProjectState>>(
                    "state",
                    Project::get_state_for_reflect,
                    Project::mut_state_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "job_id",
                    Project::get_job_id_for_reflect,
                    Project::mut_job_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Project>(
                    "Project",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Project {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_ident();
        self.clear_state();
        self.clear_job_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Project {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Project {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GroupCreate {
    // message fields
    origin: ::protobuf::SingularField<::std::string::String>,
    package: ::protobuf::SingularField<::std::string::String>,
    deps_only: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GroupCreate {}

impl GroupCreate {
    pub fn new() -> GroupCreate {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GroupCreate {
        static mut instance: ::protobuf::lazy::Lazy<GroupCreate> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GroupCreate,
        };
        unsafe {
            instance.get(GroupCreate::new)
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

    // optional string package = 2;

    pub fn clear_package(&mut self) {
        self.package.clear();
    }

    pub fn has_package(&self) -> bool {
        self.package.is_some()
    }

    // Param is passed by value, moved
    pub fn set_package(&mut self, v: ::std::string::String) {
        self.package = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_package(&mut self) -> &mut ::std::string::String {
        if self.package.is_none() {
            self.package.set_default();
        }
        self.package.as_mut().unwrap()
    }

    // Take field
    pub fn take_package(&mut self) -> ::std::string::String {
        self.package.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_package(&self) -> &str {
        match self.package.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_package_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.package
    }

    fn mut_package_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.package
    }

    // optional bool deps_only = 3;

    pub fn clear_deps_only(&mut self) {
        self.deps_only = ::std::option::Option::None;
    }

    pub fn has_deps_only(&self) -> bool {
        self.deps_only.is_some()
    }

    // Param is passed by value, moved
    pub fn set_deps_only(&mut self, v: bool) {
        self.deps_only = ::std::option::Option::Some(v);
    }

    pub fn get_deps_only(&self) -> bool {
        self.deps_only.unwrap_or(false)
    }

    fn get_deps_only_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.deps_only
    }

    fn mut_deps_only_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.deps_only
    }
}

impl ::protobuf::Message for GroupCreate {
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.package)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.deps_only = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.package.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(v) = self.deps_only {
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
        if let Some(ref v) = self.package.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(v) = self.deps_only {
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

impl ::protobuf::MessageStatic for GroupCreate {
    fn new() -> GroupCreate {
        GroupCreate::new()
    }

    fn descriptor_static(_: ::std::option::Option<GroupCreate>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "origin",
                    GroupCreate::get_origin_for_reflect,
                    GroupCreate::mut_origin_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "package",
                    GroupCreate::get_package_for_reflect,
                    GroupCreate::mut_package_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "deps_only",
                    GroupCreate::get_deps_only_for_reflect,
                    GroupCreate::mut_deps_only_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GroupCreate>(
                    "GroupCreate",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GroupCreate {
    fn clear(&mut self) {
        self.clear_origin();
        self.clear_package();
        self.clear_deps_only();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GroupCreate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GroupCreate {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GroupGet {
    // message fields
    group_id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GroupGet {}

impl GroupGet {
    pub fn new() -> GroupGet {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GroupGet {
        static mut instance: ::protobuf::lazy::Lazy<GroupGet> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GroupGet,
        };
        unsafe {
            instance.get(GroupGet::new)
        }
    }

    // optional uint64 group_id = 1;

    pub fn clear_group_id(&mut self) {
        self.group_id = ::std::option::Option::None;
    }

    pub fn has_group_id(&self) -> bool {
        self.group_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_group_id(&mut self, v: u64) {
        self.group_id = ::std::option::Option::Some(v);
    }

    pub fn get_group_id(&self) -> u64 {
        self.group_id.unwrap_or(0)
    }

    fn get_group_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.group_id
    }

    fn mut_group_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.group_id
    }
}

impl ::protobuf::Message for GroupGet {
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
                    self.group_id = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.group_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.group_id {
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

impl ::protobuf::MessageStatic for GroupGet {
    fn new() -> GroupGet {
        GroupGet::new()
    }

    fn descriptor_static(_: ::std::option::Option<GroupGet>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "group_id",
                    GroupGet::get_group_id_for_reflect,
                    GroupGet::mut_group_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GroupGet>(
                    "GroupGet",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GroupGet {
    fn clear(&mut self) {
        self.clear_group_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GroupGet {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GroupGet {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Group {
    // message fields
    id: ::std::option::Option<u64>,
    state: ::std::option::Option<GroupState>,
    projects: ::protobuf::RepeatedField<Project>,
    created_at: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Group {}

impl Group {
    pub fn new() -> Group {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Group {
        static mut instance: ::protobuf::lazy::Lazy<Group> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Group,
        };
        unsafe {
            instance.get(Group::new)
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

    // optional .scheduler.GroupState state = 2;

    pub fn clear_state(&mut self) {
        self.state = ::std::option::Option::None;
    }

    pub fn has_state(&self) -> bool {
        self.state.is_some()
    }

    // Param is passed by value, moved
    pub fn set_state(&mut self, v: GroupState) {
        self.state = ::std::option::Option::Some(v);
    }

    pub fn get_state(&self) -> GroupState {
        self.state.unwrap_or(GroupState::Pending)
    }

    fn get_state_for_reflect(&self) -> &::std::option::Option<GroupState> {
        &self.state
    }

    fn mut_state_for_reflect(&mut self) -> &mut ::std::option::Option<GroupState> {
        &mut self.state
    }

    // repeated .scheduler.Project projects = 3;

    pub fn clear_projects(&mut self) {
        self.projects.clear();
    }

    // Param is passed by value, moved
    pub fn set_projects(&mut self, v: ::protobuf::RepeatedField<Project>) {
        self.projects = v;
    }

    // Mutable pointer to the field.
    pub fn mut_projects(&mut self) -> &mut ::protobuf::RepeatedField<Project> {
        &mut self.projects
    }

    // Take field
    pub fn take_projects(&mut self) -> ::protobuf::RepeatedField<Project> {
        ::std::mem::replace(&mut self.projects, ::protobuf::RepeatedField::new())
    }

    pub fn get_projects(&self) -> &[Project] {
        &self.projects
    }

    fn get_projects_for_reflect(&self) -> &::protobuf::RepeatedField<Project> {
        &self.projects
    }

    fn mut_projects_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Project> {
        &mut self.projects
    }

    // optional string created_at = 4;

    pub fn clear_created_at(&mut self) {
        self.created_at.clear();
    }

    pub fn has_created_at(&self) -> bool {
        self.created_at.is_some()
    }

    // Param is passed by value, moved
    pub fn set_created_at(&mut self, v: ::std::string::String) {
        self.created_at = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_created_at(&mut self) -> &mut ::std::string::String {
        if self.created_at.is_none() {
            self.created_at.set_default();
        }
        self.created_at.as_mut().unwrap()
    }

    // Take field
    pub fn take_created_at(&mut self) -> ::std::string::String {
        self.created_at.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_created_at(&self) -> &str {
        match self.created_at.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_created_at_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.created_at
    }

    fn mut_created_at_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.created_at
    }
}

impl ::protobuf::Message for Group {
    fn is_initialized(&self) -> bool {
        for v in &self.projects {
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
                    let tmp = is.read_enum()?;
                    self.state = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.projects)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.created_at)?;
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
        if let Some(v) = self.state {
            my_size += ::protobuf::rt::enum_size(2, v);
        }
        for value in &self.projects {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(ref v) = self.created_at.as_ref() {
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
        if let Some(v) = self.state {
            os.write_enum(2, v.value())?;
        }
        for v in &self.projects {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(ref v) = self.created_at.as_ref() {
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

impl ::protobuf::MessageStatic for Group {
    fn new() -> Group {
        Group::new()
    }

    fn descriptor_static(_: ::std::option::Option<Group>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "id",
                    Group::get_id_for_reflect,
                    Group::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<GroupState>>(
                    "state",
                    Group::get_state_for_reflect,
                    Group::mut_state_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Project>>(
                    "projects",
                    Group::get_projects_for_reflect,
                    Group::mut_projects_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "created_at",
                    Group::get_created_at_for_reflect,
                    Group::mut_created_at_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Group>(
                    "Group",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Group {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_state();
        self.clear_projects();
        self.clear_created_at();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Group {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Group {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Package {
    // message fields
    ident: ::protobuf::SingularField<::std::string::String>,
    deps: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Package {}

impl Package {
    pub fn new() -> Package {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Package {
        static mut instance: ::protobuf::lazy::Lazy<Package> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Package,
        };
        unsafe {
            instance.get(Package::new)
        }
    }

    // optional string ident = 1;

    pub fn clear_ident(&mut self) {
        self.ident.clear();
    }

    pub fn has_ident(&self) -> bool {
        self.ident.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ident(&mut self, v: ::std::string::String) {
        self.ident = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ident(&mut self) -> &mut ::std::string::String {
        if self.ident.is_none() {
            self.ident.set_default();
        }
        self.ident.as_mut().unwrap()
    }

    // Take field
    pub fn take_ident(&mut self) -> ::std::string::String {
        self.ident.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_ident(&self) -> &str {
        match self.ident.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_ident_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.ident
    }

    fn mut_ident_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.ident
    }

    // repeated string deps = 2;

    pub fn clear_deps(&mut self) {
        self.deps.clear();
    }

    // Param is passed by value, moved
    pub fn set_deps(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.deps = v;
    }

    // Mutable pointer to the field.
    pub fn mut_deps(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.deps
    }

    // Take field
    pub fn take_deps(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.deps, ::protobuf::RepeatedField::new())
    }

    pub fn get_deps(&self) -> &[::std::string::String] {
        &self.deps
    }

    fn get_deps_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.deps
    }

    fn mut_deps_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.deps
    }
}

impl ::protobuf::Message for Package {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.ident)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.deps)?;
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
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        for value in &self.deps {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.ident.as_ref() {
            os.write_string(1, &v)?;
        }
        for v in &self.deps {
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

impl ::protobuf::MessageStatic for Package {
    fn new() -> Package {
        Package::new()
    }

    fn descriptor_static(_: ::std::option::Option<Package>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "ident",
                    Package::get_ident_for_reflect,
                    Package::mut_ident_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "deps",
                    Package::get_deps_for_reflect,
                    Package::mut_deps_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Package>(
                    "Package",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Package {
    fn clear(&mut self) {
        self.clear_ident();
        self.clear_deps();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Package {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Package {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PackagePreCreate {
    // message fields
    ident: ::protobuf::SingularField<::std::string::String>,
    deps: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PackagePreCreate {}

impl PackagePreCreate {
    pub fn new() -> PackagePreCreate {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PackagePreCreate {
        static mut instance: ::protobuf::lazy::Lazy<PackagePreCreate> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PackagePreCreate,
        };
        unsafe {
            instance.get(PackagePreCreate::new)
        }
    }

    // optional string ident = 1;

    pub fn clear_ident(&mut self) {
        self.ident.clear();
    }

    pub fn has_ident(&self) -> bool {
        self.ident.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ident(&mut self, v: ::std::string::String) {
        self.ident = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ident(&mut self) -> &mut ::std::string::String {
        if self.ident.is_none() {
            self.ident.set_default();
        }
        self.ident.as_mut().unwrap()
    }

    // Take field
    pub fn take_ident(&mut self) -> ::std::string::String {
        self.ident.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_ident(&self) -> &str {
        match self.ident.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_ident_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.ident
    }

    fn mut_ident_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.ident
    }

    // repeated string deps = 2;

    pub fn clear_deps(&mut self) {
        self.deps.clear();
    }

    // Param is passed by value, moved
    pub fn set_deps(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.deps = v;
    }

    // Mutable pointer to the field.
    pub fn mut_deps(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.deps
    }

    // Take field
    pub fn take_deps(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.deps, ::protobuf::RepeatedField::new())
    }

    pub fn get_deps(&self) -> &[::std::string::String] {
        &self.deps
    }

    fn get_deps_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.deps
    }

    fn mut_deps_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.deps
    }
}

impl ::protobuf::Message for PackagePreCreate {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.ident)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.deps)?;
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
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        for value in &self.deps {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.ident.as_ref() {
            os.write_string(1, &v)?;
        }
        for v in &self.deps {
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

impl ::protobuf::MessageStatic for PackagePreCreate {
    fn new() -> PackagePreCreate {
        PackagePreCreate::new()
    }

    fn descriptor_static(_: ::std::option::Option<PackagePreCreate>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "ident",
                    PackagePreCreate::get_ident_for_reflect,
                    PackagePreCreate::mut_ident_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "deps",
                    PackagePreCreate::get_deps_for_reflect,
                    PackagePreCreate::mut_deps_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PackagePreCreate>(
                    "PackagePreCreate",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PackagePreCreate {
    fn clear(&mut self) {
        self.clear_ident();
        self.clear_deps();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PackagePreCreate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PackagePreCreate {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PackageCreate {
    // message fields
    ident: ::protobuf::SingularField<::std::string::String>,
    deps: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PackageCreate {}

impl PackageCreate {
    pub fn new() -> PackageCreate {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PackageCreate {
        static mut instance: ::protobuf::lazy::Lazy<PackageCreate> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PackageCreate,
        };
        unsafe {
            instance.get(PackageCreate::new)
        }
    }

    // optional string ident = 1;

    pub fn clear_ident(&mut self) {
        self.ident.clear();
    }

    pub fn has_ident(&self) -> bool {
        self.ident.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ident(&mut self, v: ::std::string::String) {
        self.ident = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ident(&mut self) -> &mut ::std::string::String {
        if self.ident.is_none() {
            self.ident.set_default();
        }
        self.ident.as_mut().unwrap()
    }

    // Take field
    pub fn take_ident(&mut self) -> ::std::string::String {
        self.ident.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_ident(&self) -> &str {
        match self.ident.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_ident_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.ident
    }

    fn mut_ident_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.ident
    }

    // repeated string deps = 2;

    pub fn clear_deps(&mut self) {
        self.deps.clear();
    }

    // Param is passed by value, moved
    pub fn set_deps(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.deps = v;
    }

    // Mutable pointer to the field.
    pub fn mut_deps(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.deps
    }

    // Take field
    pub fn take_deps(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.deps, ::protobuf::RepeatedField::new())
    }

    pub fn get_deps(&self) -> &[::std::string::String] {
        &self.deps
    }

    fn get_deps_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.deps
    }

    fn mut_deps_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.deps
    }
}

impl ::protobuf::Message for PackageCreate {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.ident)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.deps)?;
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
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        for value in &self.deps {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.ident.as_ref() {
            os.write_string(1, &v)?;
        }
        for v in &self.deps {
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

impl ::protobuf::MessageStatic for PackageCreate {
    fn new() -> PackageCreate {
        PackageCreate::new()
    }

    fn descriptor_static(_: ::std::option::Option<PackageCreate>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "ident",
                    PackageCreate::get_ident_for_reflect,
                    PackageCreate::mut_ident_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "deps",
                    PackageCreate::get_deps_for_reflect,
                    PackageCreate::mut_deps_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PackageCreate>(
                    "PackageCreate",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PackageCreate {
    fn clear(&mut self) {
        self.clear_ident();
        self.clear_deps();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PackageCreate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PackageCreate {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PackageStatsGet {
    // message fields
    origin: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PackageStatsGet {}

impl PackageStatsGet {
    pub fn new() -> PackageStatsGet {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PackageStatsGet {
        static mut instance: ::protobuf::lazy::Lazy<PackageStatsGet> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PackageStatsGet,
        };
        unsafe {
            instance.get(PackageStatsGet::new)
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
}

impl ::protobuf::Message for PackageStatsGet {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.origin.as_ref() {
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

impl ::protobuf::MessageStatic for PackageStatsGet {
    fn new() -> PackageStatsGet {
        PackageStatsGet::new()
    }

    fn descriptor_static(_: ::std::option::Option<PackageStatsGet>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "origin",
                    PackageStatsGet::get_origin_for_reflect,
                    PackageStatsGet::mut_origin_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PackageStatsGet>(
                    "PackageStatsGet",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PackageStatsGet {
    fn clear(&mut self) {
        self.clear_origin();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PackageStatsGet {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PackageStatsGet {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PackageStats {
    // message fields
    plans: ::std::option::Option<u64>,
    builds: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PackageStats {}

impl PackageStats {
    pub fn new() -> PackageStats {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PackageStats {
        static mut instance: ::protobuf::lazy::Lazy<PackageStats> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PackageStats,
        };
        unsafe {
            instance.get(PackageStats::new)
        }
    }

    // optional uint64 plans = 1;

    pub fn clear_plans(&mut self) {
        self.plans = ::std::option::Option::None;
    }

    pub fn has_plans(&self) -> bool {
        self.plans.is_some()
    }

    // Param is passed by value, moved
    pub fn set_plans(&mut self, v: u64) {
        self.plans = ::std::option::Option::Some(v);
    }

    pub fn get_plans(&self) -> u64 {
        self.plans.unwrap_or(0)
    }

    fn get_plans_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.plans
    }

    fn mut_plans_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.plans
    }

    // optional uint64 builds = 2;

    pub fn clear_builds(&mut self) {
        self.builds = ::std::option::Option::None;
    }

    pub fn has_builds(&self) -> bool {
        self.builds.is_some()
    }

    // Param is passed by value, moved
    pub fn set_builds(&mut self, v: u64) {
        self.builds = ::std::option::Option::Some(v);
    }

    pub fn get_builds(&self) -> u64 {
        self.builds.unwrap_or(0)
    }

    fn get_builds_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.builds
    }

    fn mut_builds_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.builds
    }
}

impl ::protobuf::Message for PackageStats {
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
                    self.plans = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.builds = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.plans {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.builds {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.plans {
            os.write_uint64(1, v)?;
        }
        if let Some(v) = self.builds {
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

impl ::protobuf::MessageStatic for PackageStats {
    fn new() -> PackageStats {
        PackageStats::new()
    }

    fn descriptor_static(_: ::std::option::Option<PackageStats>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "plans",
                    PackageStats::get_plans_for_reflect,
                    PackageStats::mut_plans_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "builds",
                    PackageStats::get_builds_for_reflect,
                    PackageStats::mut_builds_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PackageStats>(
                    "PackageStats",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PackageStats {
    fn clear(&mut self) {
        self.clear_plans();
        self.clear_builds();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PackageStats {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PackageStats {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct JobStatus {
    // message fields
    job: ::protobuf::SingularPtrField<super::jobsrv::Job>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for JobStatus {}

impl JobStatus {
    pub fn new() -> JobStatus {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static JobStatus {
        static mut instance: ::protobuf::lazy::Lazy<JobStatus> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const JobStatus,
        };
        unsafe {
            instance.get(JobStatus::new)
        }
    }

    // optional .jobsrv.Job job = 1;

    pub fn clear_job(&mut self) {
        self.job.clear();
    }

    pub fn has_job(&self) -> bool {
        self.job.is_some()
    }

    // Param is passed by value, moved
    pub fn set_job(&mut self, v: super::jobsrv::Job) {
        self.job = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_job(&mut self) -> &mut super::jobsrv::Job {
        if self.job.is_none() {
            self.job.set_default();
        }
        self.job.as_mut().unwrap()
    }

    // Take field
    pub fn take_job(&mut self) -> super::jobsrv::Job {
        self.job.take().unwrap_or_else(|| super::jobsrv::Job::new())
    }

    pub fn get_job(&self) -> &super::jobsrv::Job {
        self.job.as_ref().unwrap_or_else(|| super::jobsrv::Job::default_instance())
    }

    fn get_job_for_reflect(&self) -> &::protobuf::SingularPtrField<super::jobsrv::Job> {
        &self.job
    }

    fn mut_job_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::jobsrv::Job> {
        &mut self.job
    }
}

impl ::protobuf::Message for JobStatus {
    fn is_initialized(&self) -> bool {
        for v in &self.job {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.job)?;
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
        if let Some(ref v) = self.job.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.job.as_ref() {
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

impl ::protobuf::MessageStatic for JobStatus {
    fn new() -> JobStatus {
        JobStatus::new()
    }

    fn descriptor_static(_: ::std::option::Option<JobStatus>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::jobsrv::Job>>(
                    "job",
                    JobStatus::get_job_for_reflect,
                    JobStatus::mut_job_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<JobStatus>(
                    "JobStatus",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for JobStatus {
    fn clear(&mut self) {
        self.clear_job();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for JobStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for JobStatus {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ProjectState {
    NotStarted = 0,
    InProgress = 1,
    Success = 2,
    Failure = 3,
    Skipped = 4,
}

impl ::protobuf::ProtobufEnum for ProjectState {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ProjectState> {
        match value {
            0 => ::std::option::Option::Some(ProjectState::NotStarted),
            1 => ::std::option::Option::Some(ProjectState::InProgress),
            2 => ::std::option::Option::Some(ProjectState::Success),
            3 => ::std::option::Option::Some(ProjectState::Failure),
            4 => ::std::option::Option::Some(ProjectState::Skipped),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ProjectState] = &[
            ProjectState::NotStarted,
            ProjectState::InProgress,
            ProjectState::Success,
            ProjectState::Failure,
            ProjectState::Skipped,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<ProjectState>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ProjectState", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ProjectState {
}

impl ::protobuf::reflect::ProtobufValue for ProjectState {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum GroupState {
    Pending = 0,
    Dispatching = 1,
    Complete = 2,
    Failed = 3,
}

impl ::protobuf::ProtobufEnum for GroupState {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<GroupState> {
        match value {
            0 => ::std::option::Option::Some(GroupState::Pending),
            1 => ::std::option::Option::Some(GroupState::Dispatching),
            2 => ::std::option::Option::Some(GroupState::Complete),
            3 => ::std::option::Option::Some(GroupState::Failed),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [GroupState] = &[
            GroupState::Pending,
            GroupState::Dispatching,
            GroupState::Complete,
            GroupState::Failed,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<GroupState>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("GroupState", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for GroupState {
}

impl ::protobuf::reflect::ProtobufValue for GroupState {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x19protocols/scheduler.proto\x12\tscheduler\x1a\x16protocols/jobsrv.p\
    roto\"y\n\x07Project\x12\x12\n\x04name\x18\x01\x20\x01(\tR\x04name\x12\
    \x14\n\x05ident\x18\x02\x20\x01(\tR\x05ident\x12-\n\x05state\x18\x03\x20\
    \x01(\x0e2\x17.scheduler.ProjectStateR\x05state\x12\x15\n\x06job_id\x18\
    \x04\x20\x01(\x04R\x05jobId\"\\\n\x0bGroupCreate\x12\x16\n\x06origin\x18\
    \x01\x20\x01(\tR\x06origin\x12\x18\n\x07package\x18\x02\x20\x01(\tR\x07p\
    ackage\x12\x1b\n\tdeps_only\x18\x03\x20\x01(\x08R\x08depsOnly\"%\n\x08Gr\
    oupGet\x12\x19\n\x08group_id\x18\x01\x20\x01(\x04R\x07groupId\"\x93\x01\
    \n\x05Group\x12\x0e\n\x02id\x18\x01\x20\x01(\x04R\x02id\x12+\n\x05state\
    \x18\x02\x20\x01(\x0e2\x15.scheduler.GroupStateR\x05state\x12.\n\x08proj\
    ects\x18\x03\x20\x03(\x0b2\x12.scheduler.ProjectR\x08projects\x12\x1d\n\
    \ncreated_at\x18\x04\x20\x01(\tR\tcreatedAt\"3\n\x07Package\x12\x14\n\
    \x05ident\x18\x01\x20\x01(\tR\x05ident\x12\x12\n\x04deps\x18\x02\x20\x03\
    (\tR\x04deps\"<\n\x10PackagePreCreate\x12\x14\n\x05ident\x18\x01\x20\x01\
    (\tR\x05ident\x12\x12\n\x04deps\x18\x02\x20\x03(\tR\x04deps\"9\n\rPackag\
    eCreate\x12\x14\n\x05ident\x18\x01\x20\x01(\tR\x05ident\x12\x12\n\x04dep\
    s\x18\x02\x20\x03(\tR\x04deps\")\n\x0fPackageStatsGet\x12\x16\n\x06origi\
    n\x18\x01\x20\x01(\tR\x06origin\"<\n\x0cPackageStats\x12\x14\n\x05plans\
    \x18\x01\x20\x01(\x04R\x05plans\x12\x16\n\x06builds\x18\x02\x20\x01(\x04\
    R\x06builds\"*\n\tJobStatus\x12\x1d\n\x03job\x18\x01\x20\x01(\x0b2\x0b.j\
    obsrv.JobR\x03job*U\n\x0cProjectState\x12\x0e\n\nNotStarted\x10\0\x12\
    \x0e\n\nInProgress\x10\x01\x12\x0b\n\x07Success\x10\x02\x12\x0b\n\x07Fai\
    lure\x10\x03\x12\x0b\n\x07Skipped\x10\x04*D\n\nGroupState\x12\x0b\n\x07P\
    ending\x10\0\x12\x0f\n\x0bDispatching\x10\x01\x12\x0c\n\x08Complete\x10\
    \x02\x12\n\n\x06Failed\x10\x03J\x9c\x11\n\x06\x12\x04\0\0D\x01\n\x08\n\
    \x01\x02\x12\x03\0\x08\x11\n\t\n\x02\x03\0\x12\x03\x01\x07\x1f\n\n\n\x02\
    \x05\0\x12\x04\x03\0\t\x01\n\n\n\x03\x05\0\x01\x12\x03\x03\x05\x11\n\x0b\
    \n\x04\x05\0\x02\0\x12\x03\x04\x02\x11\n\x0c\n\x05\x05\0\x02\0\x01\x12\
    \x03\x04\x02\x0c\n\x0c\n\x05\x05\0\x02\0\x02\x12\x03\x04\x0f\x10\n\x0b\n\
    \x04\x05\0\x02\x01\x12\x03\x05\x02\x11\n\x0c\n\x05\x05\0\x02\x01\x01\x12\
    \x03\x05\x02\x0c\n\x0c\n\x05\x05\0\x02\x01\x02\x12\x03\x05\x0f\x10\n\x0b\
    \n\x04\x05\0\x02\x02\x12\x03\x06\x02\x0e\n\x0c\n\x05\x05\0\x02\x02\x01\
    \x12\x03\x06\x02\t\n\x0c\n\x05\x05\0\x02\x02\x02\x12\x03\x06\x0c\r\n\x0b\
    \n\x04\x05\0\x02\x03\x12\x03\x07\x02\x0e\n\x0c\n\x05\x05\0\x02\x03\x01\
    \x12\x03\x07\x02\t\n\x0c\n\x05\x05\0\x02\x03\x02\x12\x03\x07\x0c\r\n\x0b\
    \n\x04\x05\0\x02\x04\x12\x03\x08\x02\x0e\n\x0c\n\x05\x05\0\x02\x04\x01\
    \x12\x03\x08\x02\t\n\x0c\n\x05\x05\0\x02\x04\x02\x12\x03\x08\x0c\r\n\n\n\
    \x02\x04\0\x12\x04\x0b\0\x10\x01\n\n\n\x03\x04\0\x01\x12\x03\x0b\x08\x0f\
    \n\x0b\n\x04\x04\0\x02\0\x12\x03\x0c\x02\x1b\n\x0c\n\x05\x04\0\x02\0\x04\
    \x12\x03\x0c\x02\n\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\x0c\x0b\x11\n\x0c\
    \n\x05\x04\0\x02\0\x01\x12\x03\x0c\x12\x16\n\x0c\n\x05\x04\0\x02\0\x03\
    \x12\x03\x0c\x19\x1a\n\x0b\n\x04\x04\0\x02\x01\x12\x03\r\x02\x1c\n\x0c\n\
    \x05\x04\0\x02\x01\x04\x12\x03\r\x02\n\n\x0c\n\x05\x04\0\x02\x01\x05\x12\
    \x03\r\x0b\x11\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\r\x12\x17\n\x0c\n\
    \x05\x04\0\x02\x01\x03\x12\x03\r\x1a\x1b\n\x0b\n\x04\x04\0\x02\x02\x12\
    \x03\x0e\x02\"\n\x0c\n\x05\x04\0\x02\x02\x04\x12\x03\x0e\x02\n\n\x0c\n\
    \x05\x04\0\x02\x02\x06\x12\x03\x0e\x0b\x17\n\x0c\n\x05\x04\0\x02\x02\x01\
    \x12\x03\x0e\x18\x1d\n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03\x0e\x20!\n\
    \x0b\n\x04\x04\0\x02\x03\x12\x03\x0f\x02\x1d\n\x0c\n\x05\x04\0\x02\x03\
    \x04\x12\x03\x0f\x02\n\n\x0c\n\x05\x04\0\x02\x03\x05\x12\x03\x0f\x0b\x11\
    \n\x0c\n\x05\x04\0\x02\x03\x01\x12\x03\x0f\x12\x18\n\x0c\n\x05\x04\0\x02\
    \x03\x03\x12\x03\x0f\x1b\x1c\n\n\n\x02\x05\x01\x12\x04\x12\0\x17\x01\n\n\
    \n\x03\x05\x01\x01\x12\x03\x12\x05\x0f\n\x0b\n\x04\x05\x01\x02\0\x12\x03\
    \x13\x02\x0e\n\x0c\n\x05\x05\x01\x02\0\x01\x12\x03\x13\x02\t\n\x0c\n\x05\
    \x05\x01\x02\0\x02\x12\x03\x13\x0c\r\n\x0b\n\x04\x05\x01\x02\x01\x12\x03\
    \x14\x02\x12\n\x0c\n\x05\x05\x01\x02\x01\x01\x12\x03\x14\x02\r\n\x0c\n\
    \x05\x05\x01\x02\x01\x02\x12\x03\x14\x10\x11\n\x0b\n\x04\x05\x01\x02\x02\
    \x12\x03\x15\x02\x0f\n\x0c\n\x05\x05\x01\x02\x02\x01\x12\x03\x15\x02\n\n\
    \x0c\n\x05\x05\x01\x02\x02\x02\x12\x03\x15\r\x0e\n\x0b\n\x04\x05\x01\x02\
    \x03\x12\x03\x16\x02\r\n\x0c\n\x05\x05\x01\x02\x03\x01\x12\x03\x16\x02\
    \x08\n\x0c\n\x05\x05\x01\x02\x03\x02\x12\x03\x16\x0b\x0c\n\n\n\x02\x04\
    \x01\x12\x04\x19\0\x1d\x01\n\n\n\x03\x04\x01\x01\x12\x03\x19\x08\x13\n\
    \x0b\n\x04\x04\x01\x02\0\x12\x03\x1a\x02\x1d\n\x0c\n\x05\x04\x01\x02\0\
    \x04\x12\x03\x1a\x02\n\n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03\x1a\x0b\x11\
    \n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\x1a\x12\x18\n\x0c\n\x05\x04\x01\
    \x02\0\x03\x12\x03\x1a\x1b\x1c\n\x0b\n\x04\x04\x01\x02\x01\x12\x03\x1b\
    \x02\x1e\n\x0c\n\x05\x04\x01\x02\x01\x04\x12\x03\x1b\x02\n\n\x0c\n\x05\
    \x04\x01\x02\x01\x05\x12\x03\x1b\x0b\x11\n\x0c\n\x05\x04\x01\x02\x01\x01\
    \x12\x03\x1b\x12\x19\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x03\x1b\x1c\x1d\
    \n\x0b\n\x04\x04\x01\x02\x02\x12\x03\x1c\x02\x1e\n\x0c\n\x05\x04\x01\x02\
    \x02\x04\x12\x03\x1c\x02\n\n\x0c\n\x05\x04\x01\x02\x02\x05\x12\x03\x1c\
    \x0b\x0f\n\x0c\n\x05\x04\x01\x02\x02\x01\x12\x03\x1c\x10\x19\n\x0c\n\x05\
    \x04\x01\x02\x02\x03\x12\x03\x1c\x1c\x1d\n\n\n\x02\x04\x02\x12\x04\x1f\0\
    !\x01\n\n\n\x03\x04\x02\x01\x12\x03\x1f\x08\x10\n\x0b\n\x04\x04\x02\x02\
    \0\x12\x03\x20\x02\x1f\n\x0c\n\x05\x04\x02\x02\0\x04\x12\x03\x20\x02\n\n\
    \x0c\n\x05\x04\x02\x02\0\x05\x12\x03\x20\x0b\x11\n\x0c\n\x05\x04\x02\x02\
    \0\x01\x12\x03\x20\x12\x1a\n\x0c\n\x05\x04\x02\x02\0\x03\x12\x03\x20\x1d\
    \x1e\n\n\n\x02\x04\x03\x12\x04#\0(\x01\n\n\n\x03\x04\x03\x01\x12\x03#\
    \x08\r\n\x0b\n\x04\x04\x03\x02\0\x12\x03$\x02\x19\n\x0c\n\x05\x04\x03\
    \x02\0\x04\x12\x03$\x02\n\n\x0c\n\x05\x04\x03\x02\0\x05\x12\x03$\x0b\x11\
    \n\x0c\n\x05\x04\x03\x02\0\x01\x12\x03$\x12\x14\n\x0c\n\x05\x04\x03\x02\
    \0\x03\x12\x03$\x17\x18\n\x0b\n\x04\x04\x03\x02\x01\x12\x03%\x02\x20\n\
    \x0c\n\x05\x04\x03\x02\x01\x04\x12\x03%\x02\n\n\x0c\n\x05\x04\x03\x02\
    \x01\x06\x12\x03%\x0b\x15\n\x0c\n\x05\x04\x03\x02\x01\x01\x12\x03%\x16\
    \x1b\n\x0c\n\x05\x04\x03\x02\x01\x03\x12\x03%\x1e\x1f\n\x0b\n\x04\x04\
    \x03\x02\x02\x12\x03&\x02\x20\n\x0c\n\x05\x04\x03\x02\x02\x04\x12\x03&\
    \x02\n\n\x0c\n\x05\x04\x03\x02\x02\x06\x12\x03&\x0b\x12\n\x0c\n\x05\x04\
    \x03\x02\x02\x01\x12\x03&\x13\x1b\n\x0c\n\x05\x04\x03\x02\x02\x03\x12\
    \x03&\x1e\x1f\n\x0b\n\x04\x04\x03\x02\x03\x12\x03'\x02!\n\x0c\n\x05\x04\
    \x03\x02\x03\x04\x12\x03'\x02\n\n\x0c\n\x05\x04\x03\x02\x03\x05\x12\x03'\
    \x0b\x11\n\x0c\n\x05\x04\x03\x02\x03\x01\x12\x03'\x12\x1c\n\x0c\n\x05\
    \x04\x03\x02\x03\x03\x12\x03'\x1f\x20\n\n\n\x02\x04\x04\x12\x04*\0-\x01\
    \n\n\n\x03\x04\x04\x01\x12\x03*\x08\x0f\n\x0b\n\x04\x04\x04\x02\0\x12\
    \x03+\x02\x1c\n\x0c\n\x05\x04\x04\x02\0\x04\x12\x03+\x02\n\n\x0c\n\x05\
    \x04\x04\x02\0\x05\x12\x03+\x0b\x11\n\x0c\n\x05\x04\x04\x02\0\x01\x12\
    \x03+\x12\x17\n\x0c\n\x05\x04\x04\x02\0\x03\x12\x03+\x1a\x1b\n\x0b\n\x04\
    \x04\x04\x02\x01\x12\x03,\x02\x1b\n\x0c\n\x05\x04\x04\x02\x01\x04\x12\
    \x03,\x02\n\n\x0c\n\x05\x04\x04\x02\x01\x05\x12\x03,\x0b\x11\n\x0c\n\x05\
    \x04\x04\x02\x01\x01\x12\x03,\x12\x16\n\x0c\n\x05\x04\x04\x02\x01\x03\
    \x12\x03,\x19\x1a\n\n\n\x02\x04\x05\x12\x04/\02\x01\n\n\n\x03\x04\x05\
    \x01\x12\x03/\x08\x18\n\x0b\n\x04\x04\x05\x02\0\x12\x030\x02\x1c\n\x0c\n\
    \x05\x04\x05\x02\0\x04\x12\x030\x02\n\n\x0c\n\x05\x04\x05\x02\0\x05\x12\
    \x030\x0b\x11\n\x0c\n\x05\x04\x05\x02\0\x01\x12\x030\x12\x17\n\x0c\n\x05\
    \x04\x05\x02\0\x03\x12\x030\x1a\x1b\n\x0b\n\x04\x04\x05\x02\x01\x12\x031\
    \x02\x1b\n\x0c\n\x05\x04\x05\x02\x01\x04\x12\x031\x02\n\n\x0c\n\x05\x04\
    \x05\x02\x01\x05\x12\x031\x0b\x11\n\x0c\n\x05\x04\x05\x02\x01\x01\x12\
    \x031\x12\x16\n\x0c\n\x05\x04\x05\x02\x01\x03\x12\x031\x19\x1a\n\n\n\x02\
    \x04\x06\x12\x044\07\x01\n\n\n\x03\x04\x06\x01\x12\x034\x08\x15\n\x0b\n\
    \x04\x04\x06\x02\0\x12\x035\x02\x1c\n\x0c\n\x05\x04\x06\x02\0\x04\x12\
    \x035\x02\n\n\x0c\n\x05\x04\x06\x02\0\x05\x12\x035\x0b\x11\n\x0c\n\x05\
    \x04\x06\x02\0\x01\x12\x035\x12\x17\n\x0c\n\x05\x04\x06\x02\0\x03\x12\
    \x035\x1a\x1b\n\x0b\n\x04\x04\x06\x02\x01\x12\x036\x02\x1b\n\x0c\n\x05\
    \x04\x06\x02\x01\x04\x12\x036\x02\n\n\x0c\n\x05\x04\x06\x02\x01\x05\x12\
    \x036\x0b\x11\n\x0c\n\x05\x04\x06\x02\x01\x01\x12\x036\x12\x16\n\x0c\n\
    \x05\x04\x06\x02\x01\x03\x12\x036\x19\x1a\n\n\n\x02\x04\x07\x12\x049\0;\
    \x01\n\n\n\x03\x04\x07\x01\x12\x039\x08\x17\n\x0b\n\x04\x04\x07\x02\0\
    \x12\x03:\x02\x1d\n\x0c\n\x05\x04\x07\x02\0\x04\x12\x03:\x02\n\n\x0c\n\
    \x05\x04\x07\x02\0\x05\x12\x03:\x0b\x11\n\x0c\n\x05\x04\x07\x02\0\x01\
    \x12\x03:\x12\x18\n\x0c\n\x05\x04\x07\x02\0\x03\x12\x03:\x1b\x1c\n\n\n\
    \x02\x04\x08\x12\x04=\0@\x01\n\n\n\x03\x04\x08\x01\x12\x03=\x08\x14\n\
    \x0b\n\x04\x04\x08\x02\0\x12\x03>\x02\x1c\n\x0c\n\x05\x04\x08\x02\0\x04\
    \x12\x03>\x02\n\n\x0c\n\x05\x04\x08\x02\0\x05\x12\x03>\x0b\x11\n\x0c\n\
    \x05\x04\x08\x02\0\x01\x12\x03>\x12\x17\n\x0c\n\x05\x04\x08\x02\0\x03\
    \x12\x03>\x1a\x1b\n\x0b\n\x04\x04\x08\x02\x01\x12\x03?\x02\x1d\n\x0c\n\
    \x05\x04\x08\x02\x01\x04\x12\x03?\x02\n\n\x0c\n\x05\x04\x08\x02\x01\x05\
    \x12\x03?\x0b\x11\n\x0c\n\x05\x04\x08\x02\x01\x01\x12\x03?\x12\x18\n\x0c\
    \n\x05\x04\x08\x02\x01\x03\x12\x03?\x1b\x1c\n\n\n\x02\x04\t\x12\x04B\0D\
    \x01\n\n\n\x03\x04\t\x01\x12\x03B\x08\x11\n\x0b\n\x04\x04\t\x02\0\x12\
    \x03C\x02\x1e\n\x0c\n\x05\x04\t\x02\0\x04\x12\x03C\x02\n\n\x0c\n\x05\x04\
    \t\x02\0\x06\x12\x03C\x0b\x15\n\x0c\n\x05\x04\t\x02\0\x01\x12\x03C\x16\
    \x19\n\x0c\n\x05\x04\t\x02\0\x03\x12\x03C\x1c\x1d\
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
