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
pub struct Heartbeat {
    // message fields
    endpoint: ::protobuf::SingularField<::std::string::String>,
    os: ::std::option::Option<Os>,
    state: ::std::option::Option<WorkerState>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Heartbeat {}

impl Heartbeat {
    pub fn new() -> Heartbeat {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Heartbeat {
        static mut instance: ::protobuf::lazy::Lazy<Heartbeat> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Heartbeat,
        };
        unsafe {
            instance.get(Heartbeat::new)
        }
    }

    // optional string endpoint = 1;

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
        };
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

    // optional .jobsrv.Os os = 2;

    pub fn clear_os(&mut self) {
        self.os = ::std::option::Option::None;
    }

    pub fn has_os(&self) -> bool {
        self.os.is_some()
    }

    // Param is passed by value, moved
    pub fn set_os(&mut self, v: Os) {
        self.os = ::std::option::Option::Some(v);
    }

    pub fn get_os(&self) -> Os {
        self.os.unwrap_or(Os::Linux)
    }

    fn get_os_for_reflect(&self) -> &::std::option::Option<Os> {
        &self.os
    }

    fn mut_os_for_reflect(&mut self) -> &mut ::std::option::Option<Os> {
        &mut self.os
    }

    // optional .jobsrv.WorkerState state = 3;

    pub fn clear_state(&mut self) {
        self.state = ::std::option::Option::None;
    }

    pub fn has_state(&self) -> bool {
        self.state.is_some()
    }

    // Param is passed by value, moved
    pub fn set_state(&mut self, v: WorkerState) {
        self.state = ::std::option::Option::Some(v);
    }

    pub fn get_state(&self) -> WorkerState {
        self.state.unwrap_or(WorkerState::Ready)
    }

    fn get_state_for_reflect(&self) -> &::std::option::Option<WorkerState> {
        &self.state
    }

    fn mut_state_for_reflect(&mut self) -> &mut ::std::option::Option<WorkerState> {
        &mut self.state
    }
}

impl ::protobuf::Message for Heartbeat {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.endpoint)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.os = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.state = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.endpoint.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.os {
            my_size += ::protobuf::rt::enum_size(2, v);
        };
        if let Some(v) = self.state {
            my_size += ::protobuf::rt::enum_size(3, v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.endpoint.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.os {
            os.write_enum(2, v.value())?;
        };
        if let Some(v) = self.state {
            os.write_enum(3, v.value())?;
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

impl ::protobuf::MessageStatic for Heartbeat {
    fn new() -> Heartbeat {
        Heartbeat::new()
    }

    fn descriptor_static(_: ::std::option::Option<Heartbeat>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "endpoint",
                    Heartbeat::get_endpoint_for_reflect,
                    Heartbeat::mut_endpoint_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Os>>(
                    "os",
                    Heartbeat::get_os_for_reflect,
                    Heartbeat::mut_os_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<WorkerState>>(
                    "state",
                    Heartbeat::get_state_for_reflect,
                    Heartbeat::mut_state_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Heartbeat>(
                    "Heartbeat",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Heartbeat {
    fn clear(&mut self) {
        self.clear_endpoint();
        self.clear_os();
        self.clear_state();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Heartbeat {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Heartbeat {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Job {
    // message fields
    id: ::std::option::Option<u64>,
    owner_id: ::std::option::Option<u64>,
    state: ::std::option::Option<JobState>,
    project: ::protobuf::SingularPtrField<super::originsrv::OriginProject>,
    error: ::protobuf::SingularPtrField<super::net::NetError>,
    created_at: ::protobuf::SingularField<::std::string::String>,
    build_started_at: ::protobuf::SingularField<::std::string::String>,
    build_finished_at: ::protobuf::SingularField<::std::string::String>,
    package_ident: ::protobuf::SingularPtrField<super::originsrv::OriginPackageIdent>,
    is_archived: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Job {}

impl Job {
    pub fn new() -> Job {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Job {
        static mut instance: ::protobuf::lazy::Lazy<Job> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Job,
        };
        unsafe {
            instance.get(Job::new)
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

    // optional .jobsrv.JobState state = 3;

    pub fn clear_state(&mut self) {
        self.state = ::std::option::Option::None;
    }

    pub fn has_state(&self) -> bool {
        self.state.is_some()
    }

    // Param is passed by value, moved
    pub fn set_state(&mut self, v: JobState) {
        self.state = ::std::option::Option::Some(v);
    }

    pub fn get_state(&self) -> JobState {
        self.state.unwrap_or(JobState::Pending)
    }

    fn get_state_for_reflect(&self) -> &::std::option::Option<JobState> {
        &self.state
    }

    fn mut_state_for_reflect(&mut self) -> &mut ::std::option::Option<JobState> {
        &mut self.state
    }

    // optional .originsrv.OriginProject project = 4;

    pub fn clear_project(&mut self) {
        self.project.clear();
    }

    pub fn has_project(&self) -> bool {
        self.project.is_some()
    }

    // Param is passed by value, moved
    pub fn set_project(&mut self, v: super::originsrv::OriginProject) {
        self.project = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_project(&mut self) -> &mut super::originsrv::OriginProject {
        if self.project.is_none() {
            self.project.set_default();
        };
        self.project.as_mut().unwrap()
    }

    // Take field
    pub fn take_project(&mut self) -> super::originsrv::OriginProject {
        self.project.take().unwrap_or_else(|| super::originsrv::OriginProject::new())
    }

    pub fn get_project(&self) -> &super::originsrv::OriginProject {
        self.project.as_ref().unwrap_or_else(|| super::originsrv::OriginProject::default_instance())
    }

    fn get_project_for_reflect(&self) -> &::protobuf::SingularPtrField<super::originsrv::OriginProject> {
        &self.project
    }

    fn mut_project_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::originsrv::OriginProject> {
        &mut self.project
    }

    // optional .net.NetError error = 5;

    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: super::net::NetError) {
        self.error = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut super::net::NetError {
        if self.error.is_none() {
            self.error.set_default();
        };
        self.error.as_mut().unwrap()
    }

    // Take field
    pub fn take_error(&mut self) -> super::net::NetError {
        self.error.take().unwrap_or_else(|| super::net::NetError::new())
    }

    pub fn get_error(&self) -> &super::net::NetError {
        self.error.as_ref().unwrap_or_else(|| super::net::NetError::default_instance())
    }

    fn get_error_for_reflect(&self) -> &::protobuf::SingularPtrField<super::net::NetError> {
        &self.error
    }

    fn mut_error_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::net::NetError> {
        &mut self.error
    }

    // optional string created_at = 6;

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
        };
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

    // optional string build_started_at = 7;

    pub fn clear_build_started_at(&mut self) {
        self.build_started_at.clear();
    }

    pub fn has_build_started_at(&self) -> bool {
        self.build_started_at.is_some()
    }

    // Param is passed by value, moved
    pub fn set_build_started_at(&mut self, v: ::std::string::String) {
        self.build_started_at = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_build_started_at(&mut self) -> &mut ::std::string::String {
        if self.build_started_at.is_none() {
            self.build_started_at.set_default();
        };
        self.build_started_at.as_mut().unwrap()
    }

    // Take field
    pub fn take_build_started_at(&mut self) -> ::std::string::String {
        self.build_started_at.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_build_started_at(&self) -> &str {
        match self.build_started_at.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_build_started_at_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.build_started_at
    }

    fn mut_build_started_at_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.build_started_at
    }

    // optional string build_finished_at = 8;

    pub fn clear_build_finished_at(&mut self) {
        self.build_finished_at.clear();
    }

    pub fn has_build_finished_at(&self) -> bool {
        self.build_finished_at.is_some()
    }

    // Param is passed by value, moved
    pub fn set_build_finished_at(&mut self, v: ::std::string::String) {
        self.build_finished_at = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_build_finished_at(&mut self) -> &mut ::std::string::String {
        if self.build_finished_at.is_none() {
            self.build_finished_at.set_default();
        };
        self.build_finished_at.as_mut().unwrap()
    }

    // Take field
    pub fn take_build_finished_at(&mut self) -> ::std::string::String {
        self.build_finished_at.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_build_finished_at(&self) -> &str {
        match self.build_finished_at.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_build_finished_at_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.build_finished_at
    }

    fn mut_build_finished_at_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.build_finished_at
    }

    // optional .originsrv.OriginPackageIdent package_ident = 9;

    pub fn clear_package_ident(&mut self) {
        self.package_ident.clear();
    }

    pub fn has_package_ident(&self) -> bool {
        self.package_ident.is_some()
    }

    // Param is passed by value, moved
    pub fn set_package_ident(&mut self, v: super::originsrv::OriginPackageIdent) {
        self.package_ident = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_package_ident(&mut self) -> &mut super::originsrv::OriginPackageIdent {
        if self.package_ident.is_none() {
            self.package_ident.set_default();
        };
        self.package_ident.as_mut().unwrap()
    }

    // Take field
    pub fn take_package_ident(&mut self) -> super::originsrv::OriginPackageIdent {
        self.package_ident.take().unwrap_or_else(|| super::originsrv::OriginPackageIdent::new())
    }

    pub fn get_package_ident(&self) -> &super::originsrv::OriginPackageIdent {
        self.package_ident.as_ref().unwrap_or_else(|| super::originsrv::OriginPackageIdent::default_instance())
    }

    fn get_package_ident_for_reflect(&self) -> &::protobuf::SingularPtrField<super::originsrv::OriginPackageIdent> {
        &self.package_ident
    }

    fn mut_package_ident_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::originsrv::OriginPackageIdent> {
        &mut self.package_ident
    }

    // optional bool is_archived = 11;

    pub fn clear_is_archived(&mut self) {
        self.is_archived = ::std::option::Option::None;
    }

    pub fn has_is_archived(&self) -> bool {
        self.is_archived.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_archived(&mut self, v: bool) {
        self.is_archived = ::std::option::Option::Some(v);
    }

    pub fn get_is_archived(&self) -> bool {
        self.is_archived.unwrap_or(false)
    }

    fn get_is_archived_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_archived
    }

    fn mut_is_archived_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_archived
    }
}

impl ::protobuf::Message for Job {
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
                    let tmp = is.read_uint64()?;
                    self.id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.owner_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.state = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.project)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.error)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.created_at)?;
                },
                7 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.build_started_at)?;
                },
                8 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.build_finished_at)?;
                },
                9 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.package_ident)?;
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.is_archived = ::std::option::Option::Some(tmp);
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
        };
        if let Some(v) = self.owner_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.state {
            my_size += ::protobuf::rt::enum_size(3, v);
        };
        if let Some(v) = self.project.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.error.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.created_at.as_ref() {
            my_size += ::protobuf::rt::string_size(6, &v);
        };
        if let Some(v) = self.build_started_at.as_ref() {
            my_size += ::protobuf::rt::string_size(7, &v);
        };
        if let Some(v) = self.build_finished_at.as_ref() {
            my_size += ::protobuf::rt::string_size(8, &v);
        };
        if let Some(v) = self.package_ident.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.is_archived {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id {
            os.write_uint64(1, v)?;
        };
        if let Some(v) = self.owner_id {
            os.write_uint64(2, v)?;
        };
        if let Some(v) = self.state {
            os.write_enum(3, v.value())?;
        };
        if let Some(v) = self.project.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.error.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.created_at.as_ref() {
            os.write_string(6, &v)?;
        };
        if let Some(v) = self.build_started_at.as_ref() {
            os.write_string(7, &v)?;
        };
        if let Some(v) = self.build_finished_at.as_ref() {
            os.write_string(8, &v)?;
        };
        if let Some(v) = self.package_ident.as_ref() {
            os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.is_archived {
            os.write_bool(11, v)?;
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

impl ::protobuf::MessageStatic for Job {
    fn new() -> Job {
        Job::new()
    }

    fn descriptor_static(_: ::std::option::Option<Job>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "id",
                    Job::get_id_for_reflect,
                    Job::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "owner_id",
                    Job::get_owner_id_for_reflect,
                    Job::mut_owner_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<JobState>>(
                    "state",
                    Job::get_state_for_reflect,
                    Job::mut_state_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::originsrv::OriginProject>>(
                    "project",
                    Job::get_project_for_reflect,
                    Job::mut_project_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::net::NetError>>(
                    "error",
                    Job::get_error_for_reflect,
                    Job::mut_error_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "created_at",
                    Job::get_created_at_for_reflect,
                    Job::mut_created_at_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "build_started_at",
                    Job::get_build_started_at_for_reflect,
                    Job::mut_build_started_at_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "build_finished_at",
                    Job::get_build_finished_at_for_reflect,
                    Job::mut_build_finished_at_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::originsrv::OriginPackageIdent>>(
                    "package_ident",
                    Job::get_package_ident_for_reflect,
                    Job::mut_package_ident_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_archived",
                    Job::get_is_archived_for_reflect,
                    Job::mut_is_archived_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Job>(
                    "Job",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Job {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_owner_id();
        self.clear_state();
        self.clear_project();
        self.clear_error();
        self.clear_created_at();
        self.clear_build_started_at();
        self.clear_build_finished_at();
        self.clear_package_ident();
        self.clear_is_archived();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Job {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Job {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct JobGet {
    // message fields
    id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for JobGet {}

impl JobGet {
    pub fn new() -> JobGet {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static JobGet {
        static mut instance: ::protobuf::lazy::Lazy<JobGet> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const JobGet,
        };
        unsafe {
            instance.get(JobGet::new)
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
}

impl ::protobuf::Message for JobGet {
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
                    let tmp = is.read_uint64()?;
                    self.id = ::std::option::Option::Some(tmp);
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
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id {
            os.write_uint64(1, v)?;
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

impl ::protobuf::MessageStatic for JobGet {
    fn new() -> JobGet {
        JobGet::new()
    }

    fn descriptor_static(_: ::std::option::Option<JobGet>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "id",
                    JobGet::get_id_for_reflect,
                    JobGet::mut_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<JobGet>(
                    "JobGet",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for JobGet {
    fn clear(&mut self) {
        self.clear_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for JobGet {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for JobGet {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct JobSpec {
    // message fields
    owner_id: ::std::option::Option<u64>,
    project: ::protobuf::SingularPtrField<super::originsrv::OriginProject>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for JobSpec {}

impl JobSpec {
    pub fn new() -> JobSpec {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static JobSpec {
        static mut instance: ::protobuf::lazy::Lazy<JobSpec> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const JobSpec,
        };
        unsafe {
            instance.get(JobSpec::new)
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

    // optional .originsrv.OriginProject project = 2;

    pub fn clear_project(&mut self) {
        self.project.clear();
    }

    pub fn has_project(&self) -> bool {
        self.project.is_some()
    }

    // Param is passed by value, moved
    pub fn set_project(&mut self, v: super::originsrv::OriginProject) {
        self.project = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_project(&mut self) -> &mut super::originsrv::OriginProject {
        if self.project.is_none() {
            self.project.set_default();
        };
        self.project.as_mut().unwrap()
    }

    // Take field
    pub fn take_project(&mut self) -> super::originsrv::OriginProject {
        self.project.take().unwrap_or_else(|| super::originsrv::OriginProject::new())
    }

    pub fn get_project(&self) -> &super::originsrv::OriginProject {
        self.project.as_ref().unwrap_or_else(|| super::originsrv::OriginProject::default_instance())
    }

    fn get_project_for_reflect(&self) -> &::protobuf::SingularPtrField<super::originsrv::OriginProject> {
        &self.project
    }

    fn mut_project_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::originsrv::OriginProject> {
        &mut self.project
    }
}

impl ::protobuf::Message for JobSpec {
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
                    let tmp = is.read_uint64()?;
                    self.owner_id = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.owner_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.project.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.owner_id {
            os.write_uint64(1, v)?;
        };
        if let Some(v) = self.project.as_ref() {
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

impl ::protobuf::MessageStatic for JobSpec {
    fn new() -> JobSpec {
        JobSpec::new()
    }

    fn descriptor_static(_: ::std::option::Option<JobSpec>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "owner_id",
                    JobSpec::get_owner_id_for_reflect,
                    JobSpec::mut_owner_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::originsrv::OriginProject>>(
                    "project",
                    JobSpec::get_project_for_reflect,
                    JobSpec::mut_project_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<JobSpec>(
                    "JobSpec",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for JobSpec {
    fn clear(&mut self) {
        self.clear_owner_id();
        self.clear_project();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for JobSpec {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for JobSpec {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ProjectJobsGet {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ProjectJobsGet {}

impl ProjectJobsGet {
    pub fn new() -> ProjectJobsGet {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ProjectJobsGet {
        static mut instance: ::protobuf::lazy::Lazy<ProjectJobsGet> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ProjectJobsGet,
        };
        unsafe {
            instance.get(ProjectJobsGet::new)
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

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.name
    }
}

impl ::protobuf::Message for ProjectJobsGet {
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
        if let Some(v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.name.as_ref() {
            os.write_string(1, &v)?;
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

impl ::protobuf::MessageStatic for ProjectJobsGet {
    fn new() -> ProjectJobsGet {
        ProjectJobsGet::new()
    }

    fn descriptor_static(_: ::std::option::Option<ProjectJobsGet>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    ProjectJobsGet::get_name_for_reflect,
                    ProjectJobsGet::mut_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ProjectJobsGet>(
                    "ProjectJobsGet",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ProjectJobsGet {
    fn clear(&mut self) {
        self.clear_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ProjectJobsGet {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ProjectJobsGet {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ProjectJobsGetResponse {
    // message fields
    jobs: ::protobuf::RepeatedField<Job>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ProjectJobsGetResponse {}

impl ProjectJobsGetResponse {
    pub fn new() -> ProjectJobsGetResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ProjectJobsGetResponse {
        static mut instance: ::protobuf::lazy::Lazy<ProjectJobsGetResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ProjectJobsGetResponse,
        };
        unsafe {
            instance.get(ProjectJobsGetResponse::new)
        }
    }

    // repeated .jobsrv.Job jobs = 1;

    pub fn clear_jobs(&mut self) {
        self.jobs.clear();
    }

    // Param is passed by value, moved
    pub fn set_jobs(&mut self, v: ::protobuf::RepeatedField<Job>) {
        self.jobs = v;
    }

    // Mutable pointer to the field.
    pub fn mut_jobs(&mut self) -> &mut ::protobuf::RepeatedField<Job> {
        &mut self.jobs
    }

    // Take field
    pub fn take_jobs(&mut self) -> ::protobuf::RepeatedField<Job> {
        ::std::mem::replace(&mut self.jobs, ::protobuf::RepeatedField::new())
    }

    pub fn get_jobs(&self) -> &[Job] {
        &self.jobs
    }

    fn get_jobs_for_reflect(&self) -> &::protobuf::RepeatedField<Job> {
        &self.jobs
    }

    fn mut_jobs_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Job> {
        &mut self.jobs
    }
}

impl ::protobuf::Message for ProjectJobsGetResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.jobs)?;
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
        for value in &self.jobs {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.jobs {
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

impl ::protobuf::MessageStatic for ProjectJobsGetResponse {
    fn new() -> ProjectJobsGetResponse {
        ProjectJobsGetResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<ProjectJobsGetResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Job>>(
                    "jobs",
                    ProjectJobsGetResponse::get_jobs_for_reflect,
                    ProjectJobsGetResponse::mut_jobs_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ProjectJobsGetResponse>(
                    "ProjectJobsGetResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ProjectJobsGetResponse {
    fn clear(&mut self) {
        self.clear_jobs();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ProjectJobsGetResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ProjectJobsGetResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct JobLogChunk {
    // message fields
    job_id: ::std::option::Option<u64>,
    seq: ::std::option::Option<u64>,
    content: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for JobLogChunk {}

impl JobLogChunk {
    pub fn new() -> JobLogChunk {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static JobLogChunk {
        static mut instance: ::protobuf::lazy::Lazy<JobLogChunk> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const JobLogChunk,
        };
        unsafe {
            instance.get(JobLogChunk::new)
        }
    }

    // optional uint64 job_id = 1;

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

    // optional uint64 seq = 2;

    pub fn clear_seq(&mut self) {
        self.seq = ::std::option::Option::None;
    }

    pub fn has_seq(&self) -> bool {
        self.seq.is_some()
    }

    // Param is passed by value, moved
    pub fn set_seq(&mut self, v: u64) {
        self.seq = ::std::option::Option::Some(v);
    }

    pub fn get_seq(&self) -> u64 {
        self.seq.unwrap_or(0)
    }

    fn get_seq_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.seq
    }

    fn mut_seq_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.seq
    }

    // optional string content = 3;

    pub fn clear_content(&mut self) {
        self.content.clear();
    }

    pub fn has_content(&self) -> bool {
        self.content.is_some()
    }

    // Param is passed by value, moved
    pub fn set_content(&mut self, v: ::std::string::String) {
        self.content = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_content(&mut self) -> &mut ::std::string::String {
        if self.content.is_none() {
            self.content.set_default();
        };
        self.content.as_mut().unwrap()
    }

    // Take field
    pub fn take_content(&mut self) -> ::std::string::String {
        self.content.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_content(&self) -> &str {
        match self.content.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_content_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.content
    }

    fn mut_content_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.content
    }
}

impl ::protobuf::Message for JobLogChunk {
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
                    let tmp = is.read_uint64()?;
                    self.job_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.seq = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.content)?;
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
        if let Some(v) = self.job_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.seq {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.content.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.job_id {
            os.write_uint64(1, v)?;
        };
        if let Some(v) = self.seq {
            os.write_uint64(2, v)?;
        };
        if let Some(v) = self.content.as_ref() {
            os.write_string(3, &v)?;
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

impl ::protobuf::MessageStatic for JobLogChunk {
    fn new() -> JobLogChunk {
        JobLogChunk::new()
    }

    fn descriptor_static(_: ::std::option::Option<JobLogChunk>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "job_id",
                    JobLogChunk::get_job_id_for_reflect,
                    JobLogChunk::mut_job_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "seq",
                    JobLogChunk::get_seq_for_reflect,
                    JobLogChunk::mut_seq_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "content",
                    JobLogChunk::get_content_for_reflect,
                    JobLogChunk::mut_content_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<JobLogChunk>(
                    "JobLogChunk",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for JobLogChunk {
    fn clear(&mut self) {
        self.clear_job_id();
        self.clear_seq();
        self.clear_content();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for JobLogChunk {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for JobLogChunk {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct JobLogComplete {
    // message fields
    job_id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for JobLogComplete {}

impl JobLogComplete {
    pub fn new() -> JobLogComplete {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static JobLogComplete {
        static mut instance: ::protobuf::lazy::Lazy<JobLogComplete> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const JobLogComplete,
        };
        unsafe {
            instance.get(JobLogComplete::new)
        }
    }

    // optional uint64 job_id = 1;

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

impl ::protobuf::Message for JobLogComplete {
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
        if let Some(v) = self.job_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.job_id {
            os.write_uint64(1, v)?;
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

impl ::protobuf::MessageStatic for JobLogComplete {
    fn new() -> JobLogComplete {
        JobLogComplete::new()
    }

    fn descriptor_static(_: ::std::option::Option<JobLogComplete>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "job_id",
                    JobLogComplete::get_job_id_for_reflect,
                    JobLogComplete::mut_job_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<JobLogComplete>(
                    "JobLogComplete",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for JobLogComplete {
    fn clear(&mut self) {
        self.clear_job_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for JobLogComplete {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for JobLogComplete {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct JobLogGet {
    // message fields
    id: ::std::option::Option<u64>,
    start: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for JobLogGet {}

impl JobLogGet {
    pub fn new() -> JobLogGet {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static JobLogGet {
        static mut instance: ::protobuf::lazy::Lazy<JobLogGet> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const JobLogGet,
        };
        unsafe {
            instance.get(JobLogGet::new)
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
}

impl ::protobuf::Message for JobLogGet {
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
                    let tmp = is.read_uint64()?;
                    self.id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.start = ::std::option::Option::Some(tmp);
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
        };
        if let Some(v) = self.start {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id {
            os.write_uint64(1, v)?;
        };
        if let Some(v) = self.start {
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

impl ::protobuf::MessageStatic for JobLogGet {
    fn new() -> JobLogGet {
        JobLogGet::new()
    }

    fn descriptor_static(_: ::std::option::Option<JobLogGet>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "id",
                    JobLogGet::get_id_for_reflect,
                    JobLogGet::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "start",
                    JobLogGet::get_start_for_reflect,
                    JobLogGet::mut_start_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<JobLogGet>(
                    "JobLogGet",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for JobLogGet {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_start();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for JobLogGet {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for JobLogGet {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct JobLog {
    // message fields
    start: ::std::option::Option<u64>,
    stop: ::std::option::Option<u64>,
    content: ::protobuf::RepeatedField<::std::string::String>,
    is_complete: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for JobLog {}

impl JobLog {
    pub fn new() -> JobLog {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static JobLog {
        static mut instance: ::protobuf::lazy::Lazy<JobLog> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const JobLog,
        };
        unsafe {
            instance.get(JobLog::new)
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

    // repeated string content = 3;

    pub fn clear_content(&mut self) {
        self.content.clear();
    }

    // Param is passed by value, moved
    pub fn set_content(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.content = v;
    }

    // Mutable pointer to the field.
    pub fn mut_content(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.content
    }

    // Take field
    pub fn take_content(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.content, ::protobuf::RepeatedField::new())
    }

    pub fn get_content(&self) -> &[::std::string::String] {
        &self.content
    }

    fn get_content_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.content
    }

    fn mut_content_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.content
    }

    // optional bool is_complete = 4;

    pub fn clear_is_complete(&mut self) {
        self.is_complete = ::std::option::Option::None;
    }

    pub fn has_is_complete(&self) -> bool {
        self.is_complete.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_complete(&mut self, v: bool) {
        self.is_complete = ::std::option::Option::Some(v);
    }

    pub fn get_is_complete(&self) -> bool {
        self.is_complete.unwrap_or(false)
    }

    fn get_is_complete_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_complete
    }

    fn mut_is_complete_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_complete
    }
}

impl ::protobuf::Message for JobLog {
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
                    let tmp = is.read_uint64()?;
                    self.start = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.stop = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.content)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.is_complete = ::std::option::Option::Some(tmp);
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
        };
        if let Some(v) = self.stop {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.content {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        if let Some(v) = self.is_complete {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.start {
            os.write_uint64(1, v)?;
        };
        if let Some(v) = self.stop {
            os.write_uint64(2, v)?;
        };
        for v in &self.content {
            os.write_string(3, &v)?;
        };
        if let Some(v) = self.is_complete {
            os.write_bool(4, v)?;
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

impl ::protobuf::MessageStatic for JobLog {
    fn new() -> JobLog {
        JobLog::new()
    }

    fn descriptor_static(_: ::std::option::Option<JobLog>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "start",
                    JobLog::get_start_for_reflect,
                    JobLog::mut_start_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "stop",
                    JobLog::get_stop_for_reflect,
                    JobLog::mut_stop_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "content",
                    JobLog::get_content_for_reflect,
                    JobLog::mut_content_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_complete",
                    JobLog::get_is_complete_for_reflect,
                    JobLog::mut_is_complete_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<JobLog>(
                    "JobLog",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for JobLog {
    fn clear(&mut self) {
        self.clear_start();
        self.clear_stop();
        self.clear_content();
        self.clear_is_complete();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for JobLog {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for JobLog {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Os {
    Linux = 1,
    Darwin = 2,
    Windows = 3,
}

impl ::protobuf::ProtobufEnum for Os {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Os> {
        match value {
            1 => ::std::option::Option::Some(Os::Linux),
            2 => ::std::option::Option::Some(Os::Darwin),
            3 => ::std::option::Option::Some(Os::Windows),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Os] = &[
            Os::Linux,
            Os::Darwin,
            Os::Windows,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<Os>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Os", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Os {
}

impl ::protobuf::reflect::ProtobufValue for Os {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum WorkerState {
    Ready = 0,
    Busy = 1,
}

impl ::protobuf::ProtobufEnum for WorkerState {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<WorkerState> {
        match value {
            0 => ::std::option::Option::Some(WorkerState::Ready),
            1 => ::std::option::Option::Some(WorkerState::Busy),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [WorkerState] = &[
            WorkerState::Ready,
            WorkerState::Busy,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<WorkerState>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("WorkerState", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for WorkerState {
}

impl ::protobuf::reflect::ProtobufValue for WorkerState {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum JobState {
    Pending = 0,
    Processing = 1,
    Complete = 2,
    Rejected = 3,
    Failed = 4,
    Dispatched = 5,
}

impl ::protobuf::ProtobufEnum for JobState {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<JobState> {
        match value {
            0 => ::std::option::Option::Some(JobState::Pending),
            1 => ::std::option::Option::Some(JobState::Processing),
            2 => ::std::option::Option::Some(JobState::Complete),
            3 => ::std::option::Option::Some(JobState::Rejected),
            4 => ::std::option::Option::Some(JobState::Failed),
            5 => ::std::option::Option::Some(JobState::Dispatched),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [JobState] = &[
            JobState::Pending,
            JobState::Processing,
            JobState::Complete,
            JobState::Rejected,
            JobState::Failed,
            JobState::Dispatched,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<JobState>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("JobState", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for JobState {
}

impl ::protobuf::reflect::ProtobufValue for JobState {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x16, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x73, 0x2f, 0x6a, 0x6f, 0x62, 0x73,
    0x72, 0x76, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x06, 0x6a, 0x6f, 0x62, 0x73, 0x72, 0x76,
    0x1a, 0x13, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x73, 0x2f, 0x6e, 0x65, 0x74, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x19, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x73,
    0x2f, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x73, 0x72, 0x76, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x22, 0x6e, 0x0a, 0x09, 0x48, 0x65, 0x61, 0x72, 0x74, 0x62, 0x65, 0x61, 0x74, 0x12, 0x1a, 0x0a,
    0x08, 0x65, 0x6e, 0x64, 0x70, 0x6f, 0x69, 0x6e, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52,
    0x08, 0x65, 0x6e, 0x64, 0x70, 0x6f, 0x69, 0x6e, 0x74, 0x12, 0x1a, 0x0a, 0x02, 0x6f, 0x73, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x0a, 0x2e, 0x6a, 0x6f, 0x62, 0x73, 0x72, 0x76, 0x2e, 0x4f,
    0x73, 0x52, 0x02, 0x6f, 0x73, 0x12, 0x29, 0x0a, 0x05, 0x73, 0x74, 0x61, 0x74, 0x65, 0x18, 0x03,
    0x20, 0x01, 0x28, 0x0e, 0x32, 0x13, 0x2e, 0x6a, 0x6f, 0x62, 0x73, 0x72, 0x76, 0x2e, 0x57, 0x6f,
    0x72, 0x6b, 0x65, 0x72, 0x53, 0x74, 0x61, 0x74, 0x65, 0x52, 0x05, 0x73, 0x74, 0x61, 0x74, 0x65,
    0x22, 0x9a, 0x03, 0x0a, 0x03, 0x4a, 0x6f, 0x62, 0x12, 0x0e, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x04, 0x52, 0x02, 0x69, 0x64, 0x12, 0x19, 0x0a, 0x08, 0x6f, 0x77, 0x6e, 0x65,
    0x72, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x04, 0x52, 0x07, 0x6f, 0x77, 0x6e, 0x65,
    0x72, 0x49, 0x64, 0x12, 0x26, 0x0a, 0x05, 0x73, 0x74, 0x61, 0x74, 0x65, 0x18, 0x03, 0x20, 0x01,
    0x28, 0x0e, 0x32, 0x10, 0x2e, 0x6a, 0x6f, 0x62, 0x73, 0x72, 0x76, 0x2e, 0x4a, 0x6f, 0x62, 0x53,
    0x74, 0x61, 0x74, 0x65, 0x52, 0x05, 0x73, 0x74, 0x61, 0x74, 0x65, 0x12, 0x32, 0x0a, 0x07, 0x70,
    0x72, 0x6f, 0x6a, 0x65, 0x63, 0x74, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x18, 0x2e, 0x6f,
    0x72, 0x69, 0x67, 0x69, 0x6e, 0x73, 0x72, 0x76, 0x2e, 0x4f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x50,
    0x72, 0x6f, 0x6a, 0x65, 0x63, 0x74, 0x52, 0x07, 0x70, 0x72, 0x6f, 0x6a, 0x65, 0x63, 0x74, 0x12,
    0x23, 0x0a, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0d,
    0x2e, 0x6e, 0x65, 0x74, 0x2e, 0x4e, 0x65, 0x74, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x52, 0x05, 0x65,
    0x72, 0x72, 0x6f, 0x72, 0x12, 0x1d, 0x0a, 0x0a, 0x63, 0x72, 0x65, 0x61, 0x74, 0x65, 0x64, 0x5f,
    0x61, 0x74, 0x18, 0x06, 0x20, 0x01, 0x28, 0x09, 0x52, 0x09, 0x63, 0x72, 0x65, 0x61, 0x74, 0x65,
    0x64, 0x41, 0x74, 0x12, 0x28, 0x0a, 0x10, 0x62, 0x75, 0x69, 0x6c, 0x64, 0x5f, 0x73, 0x74, 0x61,
    0x72, 0x74, 0x65, 0x64, 0x5f, 0x61, 0x74, 0x18, 0x07, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0e, 0x62,
    0x75, 0x69, 0x6c, 0x64, 0x53, 0x74, 0x61, 0x72, 0x74, 0x65, 0x64, 0x41, 0x74, 0x12, 0x2a, 0x0a,
    0x11, 0x62, 0x75, 0x69, 0x6c, 0x64, 0x5f, 0x66, 0x69, 0x6e, 0x69, 0x73, 0x68, 0x65, 0x64, 0x5f,
    0x61, 0x74, 0x18, 0x08, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0f, 0x62, 0x75, 0x69, 0x6c, 0x64, 0x46,
    0x69, 0x6e, 0x69, 0x73, 0x68, 0x65, 0x64, 0x41, 0x74, 0x12, 0x42, 0x0a, 0x0d, 0x70, 0x61, 0x63,
    0x6b, 0x61, 0x67, 0x65, 0x5f, 0x69, 0x64, 0x65, 0x6e, 0x74, 0x18, 0x09, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x1d, 0x2e, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x73, 0x72, 0x76, 0x2e, 0x4f, 0x72, 0x69,
    0x67, 0x69, 0x6e, 0x50, 0x61, 0x63, 0x6b, 0x61, 0x67, 0x65, 0x49, 0x64, 0x65, 0x6e, 0x74, 0x52,
    0x0c, 0x70, 0x61, 0x63, 0x6b, 0x61, 0x67, 0x65, 0x49, 0x64, 0x65, 0x6e, 0x74, 0x12, 0x1f, 0x0a,
    0x0b, 0x69, 0x73, 0x5f, 0x61, 0x72, 0x63, 0x68, 0x69, 0x76, 0x65, 0x64, 0x18, 0x0b, 0x20, 0x01,
    0x28, 0x08, 0x52, 0x0a, 0x69, 0x73, 0x41, 0x72, 0x63, 0x68, 0x69, 0x76, 0x65, 0x64, 0x4a, 0x04,
    0x08, 0x0a, 0x10, 0x0b, 0x52, 0x07, 0x6c, 0x6f, 0x67, 0x5f, 0x75, 0x72, 0x6c, 0x22, 0x18, 0x0a,
    0x06, 0x4a, 0x6f, 0x62, 0x47, 0x65, 0x74, 0x12, 0x0e, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x04, 0x52, 0x02, 0x69, 0x64, 0x22, 0x58, 0x0a, 0x07, 0x4a, 0x6f, 0x62, 0x53, 0x70,
    0x65, 0x63, 0x12, 0x19, 0x0a, 0x08, 0x6f, 0x77, 0x6e, 0x65, 0x72, 0x5f, 0x69, 0x64, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x04, 0x52, 0x07, 0x6f, 0x77, 0x6e, 0x65, 0x72, 0x49, 0x64, 0x12, 0x32, 0x0a,
    0x07, 0x70, 0x72, 0x6f, 0x6a, 0x65, 0x63, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x18,
    0x2e, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x73, 0x72, 0x76, 0x2e, 0x4f, 0x72, 0x69, 0x67, 0x69,
    0x6e, 0x50, 0x72, 0x6f, 0x6a, 0x65, 0x63, 0x74, 0x52, 0x07, 0x70, 0x72, 0x6f, 0x6a, 0x65, 0x63,
    0x74, 0x22, 0x24, 0x0a, 0x0e, 0x50, 0x72, 0x6f, 0x6a, 0x65, 0x63, 0x74, 0x4a, 0x6f, 0x62, 0x73,
    0x47, 0x65, 0x74, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x22, 0x39, 0x0a, 0x16, 0x50, 0x72, 0x6f, 0x6a, 0x65,
    0x63, 0x74, 0x4a, 0x6f, 0x62, 0x73, 0x47, 0x65, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x12, 0x1f, 0x0a, 0x04, 0x6a, 0x6f, 0x62, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32,
    0x0b, 0x2e, 0x6a, 0x6f, 0x62, 0x73, 0x72, 0x76, 0x2e, 0x4a, 0x6f, 0x62, 0x52, 0x04, 0x6a, 0x6f,
    0x62, 0x73, 0x22, 0x50, 0x0a, 0x0b, 0x4a, 0x6f, 0x62, 0x4c, 0x6f, 0x67, 0x43, 0x68, 0x75, 0x6e,
    0x6b, 0x12, 0x15, 0x0a, 0x06, 0x6a, 0x6f, 0x62, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x04, 0x52, 0x05, 0x6a, 0x6f, 0x62, 0x49, 0x64, 0x12, 0x10, 0x0a, 0x03, 0x73, 0x65, 0x71, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x04, 0x52, 0x03, 0x73, 0x65, 0x71, 0x12, 0x18, 0x0a, 0x07, 0x63, 0x6f,
    0x6e, 0x74, 0x65, 0x6e, 0x74, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x07, 0x63, 0x6f, 0x6e,
    0x74, 0x65, 0x6e, 0x74, 0x22, 0x27, 0x0a, 0x0e, 0x4a, 0x6f, 0x62, 0x4c, 0x6f, 0x67, 0x43, 0x6f,
    0x6d, 0x70, 0x6c, 0x65, 0x74, 0x65, 0x12, 0x15, 0x0a, 0x06, 0x6a, 0x6f, 0x62, 0x5f, 0x69, 0x64,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52, 0x05, 0x6a, 0x6f, 0x62, 0x49, 0x64, 0x22, 0x31, 0x0a,
    0x09, 0x4a, 0x6f, 0x62, 0x4c, 0x6f, 0x67, 0x47, 0x65, 0x74, 0x12, 0x0e, 0x0a, 0x02, 0x69, 0x64,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52, 0x02, 0x69, 0x64, 0x12, 0x14, 0x0a, 0x05, 0x73, 0x74,
    0x61, 0x72, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x04, 0x52, 0x05, 0x73, 0x74, 0x61, 0x72, 0x74,
    0x22, 0x6d, 0x0a, 0x06, 0x4a, 0x6f, 0x62, 0x4c, 0x6f, 0x67, 0x12, 0x14, 0x0a, 0x05, 0x73, 0x74,
    0x61, 0x72, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52, 0x05, 0x73, 0x74, 0x61, 0x72, 0x74,
    0x12, 0x12, 0x0a, 0x04, 0x73, 0x74, 0x6f, 0x70, 0x18, 0x02, 0x20, 0x01, 0x28, 0x04, 0x52, 0x04,
    0x73, 0x74, 0x6f, 0x70, 0x12, 0x18, 0x0a, 0x07, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x18,
    0x03, 0x20, 0x03, 0x28, 0x09, 0x52, 0x07, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x12, 0x1f,
    0x0a, 0x0b, 0x69, 0x73, 0x5f, 0x63, 0x6f, 0x6d, 0x70, 0x6c, 0x65, 0x74, 0x65, 0x18, 0x04, 0x20,
    0x01, 0x28, 0x08, 0x52, 0x0a, 0x69, 0x73, 0x43, 0x6f, 0x6d, 0x70, 0x6c, 0x65, 0x74, 0x65, 0x2a,
    0x28, 0x0a, 0x02, 0x4f, 0x73, 0x12, 0x09, 0x0a, 0x05, 0x4c, 0x69, 0x6e, 0x75, 0x78, 0x10, 0x01,
    0x12, 0x0a, 0x0a, 0x06, 0x44, 0x61, 0x72, 0x77, 0x69, 0x6e, 0x10, 0x02, 0x12, 0x0b, 0x0a, 0x07,
    0x57, 0x69, 0x6e, 0x64, 0x6f, 0x77, 0x73, 0x10, 0x03, 0x2a, 0x22, 0x0a, 0x0b, 0x57, 0x6f, 0x72,
    0x6b, 0x65, 0x72, 0x53, 0x74, 0x61, 0x74, 0x65, 0x12, 0x09, 0x0a, 0x05, 0x52, 0x65, 0x61, 0x64,
    0x79, 0x10, 0x00, 0x12, 0x08, 0x0a, 0x04, 0x42, 0x75, 0x73, 0x79, 0x10, 0x01, 0x2a, 0x5f, 0x0a,
    0x08, 0x4a, 0x6f, 0x62, 0x53, 0x74, 0x61, 0x74, 0x65, 0x12, 0x0b, 0x0a, 0x07, 0x50, 0x65, 0x6e,
    0x64, 0x69, 0x6e, 0x67, 0x10, 0x00, 0x12, 0x0e, 0x0a, 0x0a, 0x50, 0x72, 0x6f, 0x63, 0x65, 0x73,
    0x73, 0x69, 0x6e, 0x67, 0x10, 0x01, 0x12, 0x0c, 0x0a, 0x08, 0x43, 0x6f, 0x6d, 0x70, 0x6c, 0x65,
    0x74, 0x65, 0x10, 0x02, 0x12, 0x0c, 0x0a, 0x08, 0x52, 0x65, 0x6a, 0x65, 0x63, 0x74, 0x65, 0x64,
    0x10, 0x03, 0x12, 0x0a, 0x0a, 0x06, 0x46, 0x61, 0x69, 0x6c, 0x65, 0x64, 0x10, 0x04, 0x12, 0x0e,
    0x0a, 0x0a, 0x44, 0x69, 0x73, 0x70, 0x61, 0x74, 0x63, 0x68, 0x65, 0x64, 0x10, 0x05, 0x4a, 0x87,
    0x22, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x74, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03,
    0x00, 0x08, 0x0e, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x01, 0x07, 0x1c, 0x0a, 0x09,
    0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x02, 0x07, 0x22, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x00, 0x12,
    0x04, 0x04, 0x00, 0x08, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x00, 0x01, 0x12, 0x03, 0x04, 0x05,
    0x07, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x00, 0x12, 0x03, 0x05, 0x02, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x05, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x05, 0x0a, 0x0b, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00,
    0x02, 0x01, 0x12, 0x03, 0x06, 0x02, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x06, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03,
    0x06, 0x0b, 0x0c, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x02, 0x12, 0x03, 0x07, 0x02, 0x0e,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x07, 0x02, 0x09, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x07, 0x0c, 0x0d, 0x0a, 0x0a, 0x0a, 0x02,
    0x05, 0x01, 0x12, 0x04, 0x0a, 0x00, 0x0d, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x01, 0x01, 0x12,
    0x03, 0x0a, 0x05, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x00, 0x12, 0x03, 0x0b, 0x02,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0b, 0x02, 0x07, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x00, 0x02, 0x12, 0x03, 0x0b, 0x0a, 0x0b, 0x0a, 0x0b, 0x0a,
    0x04, 0x05, 0x01, 0x02, 0x01, 0x12, 0x03, 0x0c, 0x02, 0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x0c, 0x02, 0x06, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x01,
    0x02, 0x12, 0x03, 0x0c, 0x09, 0x0a, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x02, 0x12, 0x04, 0x0f, 0x00,
    0x16, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x02, 0x01, 0x12, 0x03, 0x0f, 0x05, 0x0d, 0x0a, 0x0b,
    0x0a, 0x04, 0x05, 0x02, 0x02, 0x00, 0x12, 0x03, 0x10, 0x02, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x10, 0x02, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02,
    0x00, 0x02, 0x12, 0x03, 0x10, 0x0c, 0x0d, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x01, 0x12,
    0x03, 0x11, 0x02, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x11,
    0x02, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x01, 0x02, 0x12, 0x03, 0x11, 0x0f, 0x10,
    0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x02, 0x12, 0x03, 0x12, 0x02, 0x0f, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x02, 0x02, 0x02, 0x01, 0x12, 0x03, 0x12, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x02, 0x02, 0x02, 0x02, 0x12, 0x03, 0x12, 0x0d, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02,
    0x03, 0x12, 0x03, 0x13, 0x02, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x03, 0x01, 0x12,
    0x03, 0x13, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x03, 0x02, 0x12, 0x03, 0x13,
    0x0d, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x04, 0x12, 0x03, 0x14, 0x02, 0x0d, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x04, 0x01, 0x12, 0x03, 0x14, 0x02, 0x08, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x02, 0x02, 0x04, 0x02, 0x12, 0x03, 0x14, 0x0b, 0x0c, 0x0a, 0x0b, 0x0a, 0x04, 0x05,
    0x02, 0x02, 0x05, 0x12, 0x03, 0x15, 0x02, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x05,
    0x01, 0x12, 0x03, 0x15, 0x02, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x05, 0x02, 0x12,
    0x03, 0x15, 0x0f, 0x10, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x18, 0x00, 0x1c, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x18, 0x08, 0x11, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x19, 0x02, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x04, 0x12, 0x03, 0x19, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05,
    0x12, 0x03, 0x19, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x19, 0x12, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x19, 0x1d,
    0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x1a, 0x02, 0x15, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x1a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x01, 0x06, 0x12, 0x03, 0x1a, 0x0b, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x1a, 0x0e, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01,
    0x03, 0x12, 0x03, 0x1a, 0x13, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03,
    0x1b, 0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x1b, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x06, 0x12, 0x03, 0x1b, 0x0b, 0x16, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x1b, 0x17, 0x1c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x1b, 0x1f, 0x20, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x01, 0x12, 0x04, 0x1e, 0x00, 0x35, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03,
    0x1e, 0x08, 0x0b, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x09, 0x12, 0x03, 0x1f, 0x0b, 0x0e, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x01, 0x09, 0x00, 0x12, 0x03, 0x1f, 0x0b, 0x0d, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x09, 0x00, 0x01, 0x12, 0x03, 0x1f, 0x0b, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x09, 0x00, 0x02, 0x12, 0x03, 0x1f, 0x0b, 0x0d, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x0a, 0x12,
    0x03, 0x20, 0x0b, 0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x0a, 0x00, 0x12, 0x03, 0x20, 0x0b,
    0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x21, 0x02, 0x19, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x21, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x21, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x21, 0x12, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x21, 0x17, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03,
    0x22, 0x02, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x04, 0x12, 0x03, 0x22, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x22, 0x0b, 0x11, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x22, 0x12, 0x1a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x22, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x01, 0x02, 0x02, 0x12, 0x03, 0x23, 0x02, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02,
    0x04, 0x12, 0x03, 0x23, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x06, 0x12,
    0x03, 0x23, 0x0b, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x23,
    0x14, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x03, 0x12, 0x03, 0x23, 0x1c, 0x1d,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x03, 0x12, 0x03, 0x24, 0x02, 0x2f, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x03, 0x04, 0x12, 0x03, 0x24, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x03, 0x06, 0x12, 0x03, 0x24, 0x0b, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x03, 0x01, 0x12, 0x03, 0x24, 0x23, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x03,
    0x12, 0x03, 0x24, 0x2d, 0x2e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x04, 0x12, 0x03, 0x25,
    0x02, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x04, 0x12, 0x03, 0x25, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x06, 0x12, 0x03, 0x25, 0x0b, 0x17, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x01, 0x12, 0x03, 0x25, 0x18, 0x1d, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x04, 0x03, 0x12, 0x03, 0x25, 0x20, 0x21, 0x0a, 0xa5, 0x01, 0x0a, 0x04, 0x04,
    0x01, 0x02, 0x05, 0x12, 0x03, 0x29, 0x02, 0x21, 0x1a, 0x97, 0x01, 0x20, 0x54, 0x68, 0x65, 0x20,
    0x52, 0x46, 0x43, 0x33, 0x33, 0x33, 0x39, 0x2d, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x74, 0x65,
    0x64, 0x20, 0x74, 0x69, 0x6d, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6a, 0x6f, 0x62, 0x20, 0x77,
    0x61, 0x73, 0x20, 0x65, 0x6e, 0x74, 0x65, 0x72, 0x65, 0x64, 0x20, 0x69, 0x6e, 0x74, 0x6f, 0x20,
    0x74, 0x68, 0x65, 0x0a, 0x20, 0x73, 0x79, 0x73, 0x74, 0x65, 0x6d, 0x2e, 0x20, 0x49, 0x74, 0x20,
    0x6d, 0x61, 0x79, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x62, 0x65, 0x67, 0x69, 0x6e, 0x20, 0x70, 0x72,
    0x6f, 0x63, 0x65, 0x73, 0x73, 0x69, 0x6e, 0x67, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x73, 0x6f, 0x6d,
    0x65, 0x20, 0x74, 0x69, 0x6d, 0x65, 0x20, 0x61, 0x66, 0x74, 0x65, 0x72, 0x20, 0x74, 0x68, 0x69,
    0x73, 0x2c, 0x0a, 0x20, 0x62, 0x61, 0x73, 0x65, 0x64, 0x20, 0x6f, 0x6e, 0x20, 0x63, 0x75, 0x72,
    0x72, 0x65, 0x6e, 0x74, 0x20, 0x73, 0x79, 0x73, 0x74, 0x65, 0x6d, 0x20, 0x6c, 0x6f, 0x61, 0x64,
    0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x04, 0x12, 0x03, 0x29, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x05, 0x12, 0x03, 0x29, 0x0b, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x01, 0x12, 0x03, 0x29, 0x12, 0x1c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x05, 0x03, 0x12, 0x03, 0x29, 0x1f, 0x20, 0x0a, 0x52, 0x0a, 0x04, 0x04, 0x01,
    0x02, 0x06, 0x12, 0x03, 0x2c, 0x02, 0x27, 0x1a, 0x45, 0x20, 0x54, 0x68, 0x65, 0x20, 0x52, 0x46,
    0x43, 0x33, 0x33, 0x33, 0x39, 0x2d, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x74, 0x65, 0x64, 0x20,
    0x74, 0x69, 0x6d, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x60, 0x68, 0x61, 0x62, 0x20, 0x73, 0x74,
    0x75, 0x64, 0x69, 0x6f, 0x20, 0x62, 0x75, 0x69, 0x6c, 0x64, 0x60, 0x20, 0x70, 0x72, 0x6f, 0x63,
    0x65, 0x73, 0x73, 0x0a, 0x20, 0x73, 0x74, 0x61, 0x72, 0x74, 0x65, 0x64, 0x2e, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x06, 0x04, 0x12, 0x03, 0x2c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x06, 0x05, 0x12, 0x03, 0x2c, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x06, 0x01, 0x12, 0x03, 0x2c, 0x12, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x06,
    0x03, 0x12, 0x03, 0x2c, 0x25, 0x26, 0x0a, 0x65, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x07, 0x12, 0x03,
    0x2f, 0x02, 0x28, 0x1a, 0x58, 0x20, 0x54, 0x68, 0x65, 0x20, 0x52, 0x46, 0x43, 0x33, 0x33, 0x33,
    0x39, 0x2d, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x74, 0x65, 0x64, 0x20, 0x74, 0x69, 0x6d, 0x65,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x60, 0x68, 0x61, 0x62, 0x20, 0x73, 0x74, 0x75, 0x64, 0x69, 0x6f,
    0x20, 0x62, 0x75, 0x69, 0x6c, 0x64, 0x60, 0x20, 0x70, 0x72, 0x6f, 0x63, 0x65, 0x73, 0x73, 0x0a,
    0x20, 0x73, 0x74, 0x6f, 0x70, 0x70, 0x65, 0x64, 0x2c, 0x20, 0x73, 0x75, 0x63, 0x63, 0x65, 0x73,
    0x73, 0x66, 0x75, 0x6c, 0x20, 0x6f, 0x72, 0x20, 0x6e, 0x6f, 0x74, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x07, 0x04, 0x12, 0x03, 0x2f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x07, 0x05, 0x12, 0x03, 0x2f, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x07, 0x01, 0x12, 0x03, 0x2f, 0x12, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x07, 0x03,
    0x12, 0x03, 0x2f, 0x26, 0x27, 0x0a, 0x62, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x08, 0x12, 0x03, 0x32,
    0x02, 0x3a, 0x1a, 0x55, 0x20, 0x54, 0x68, 0x65, 0x20, 0x69, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x66,
    0x69, 0x65, 0x72, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x70, 0x61, 0x63, 0x6b, 0x61,
    0x67, 0x65, 0x20, 0x62, 0x75, 0x69, 0x6c, 0x74, 0x20, 0x62, 0x79, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x6a, 0x6f, 0x62, 0x2e, 0x20, 0x53, 0x65, 0x74, 0x20, 0x6f, 0x6e, 0x6c, 0x79, 0x20, 0x61, 0x0a,
    0x20, 0x73, 0x75, 0x63, 0x63, 0x65, 0x73, 0x73, 0x66, 0x75, 0x6c, 0x6c, 0x79, 0x2d, 0x62, 0x75,
    0x69, 0x6c, 0x74, 0x20, 0x4a, 0x6f, 0x62, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x08, 0x04, 0x12, 0x03, 0x32, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x08, 0x06,
    0x12, 0x03, 0x32, 0x0b, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x08, 0x01, 0x12, 0x03,
    0x32, 0x28, 0x35, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x08, 0x03, 0x12, 0x03, 0x32, 0x38,
    0x39, 0x0a, 0x43, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x09, 0x12, 0x03, 0x34, 0x02, 0x21, 0x1a, 0x36,
    0x20, 0x57, 0x68, 0x65, 0x74, 0x68, 0x65, 0x72, 0x20, 0x6f, 0x72, 0x20, 0x6e, 0x6f, 0x74, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x6c, 0x6f, 0x67, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x6a, 0x6f, 0x62, 0x20, 0x68, 0x61, 0x73, 0x20, 0x62, 0x65, 0x65, 0x6e, 0x20, 0x61, 0x72, 0x63,
    0x68, 0x69, 0x76, 0x65, 0x64, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x09, 0x04, 0x12,
    0x03, 0x34, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x09, 0x05, 0x12, 0x03, 0x34,
    0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x09, 0x01, 0x12, 0x03, 0x34, 0x10, 0x1b,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x09, 0x03, 0x12, 0x03, 0x34, 0x1e, 0x20, 0x0a, 0x29,
    0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x38, 0x00, 0x3a, 0x01, 0x1a, 0x1d, 0x20, 0x52, 0x65, 0x74,
    0x72, 0x69, 0x65, 0x76, 0x65, 0x20, 0x61, 0x20, 0x73, 0x69, 0x6e, 0x67, 0x6c, 0x65, 0x20, 0x6a,
    0x6f, 0x62, 0x20, 0x62, 0x79, 0x20, 0x49, 0x44, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01,
    0x12, 0x03, 0x38, 0x08, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x39,
    0x02, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x39, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x39, 0x0b, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x39, 0x12, 0x14, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x39, 0x17, 0x18, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03,
    0x12, 0x04, 0x3c, 0x00, 0x3f, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x3c,
    0x08, 0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x3d, 0x02, 0x1f, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x03, 0x3d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x00, 0x05, 0x12, 0x03, 0x3d, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x3d, 0x12, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x3d, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x01, 0x12,
    0x03, 0x3e, 0x02, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x04, 0x12, 0x03, 0x3e,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x06, 0x12, 0x03, 0x3e, 0x0b, 0x22,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x01, 0x12, 0x03, 0x3e, 0x23, 0x2a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x03, 0x12, 0x03, 0x3e, 0x2d, 0x2e, 0x0a, 0x88, 0x01, 0x0a,
    0x02, 0x04, 0x04, 0x12, 0x04, 0x45, 0x00, 0x48, 0x01, 0x1a, 0x7c, 0x20, 0x52, 0x65, 0x74, 0x72,
    0x69, 0x65, 0x76, 0x65, 0x20, 0x6a, 0x6f, 0x62, 0x73, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x61, 0x20,
    0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x63, 0x20, 0x70, 0x72, 0x6f, 0x6a, 0x65, 0x63, 0x74,
    0x2e, 0x0a, 0x0a, 0x20, 0x43, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x6c, 0x79, 0x20, 0x72, 0x65,
    0x74, 0x72, 0x69, 0x65, 0x76, 0x65, 0x73, 0x20, 0x35, 0x30, 0x20, 0x6d, 0x6f, 0x73, 0x74, 0x20,
    0x72, 0x65, 0x63, 0x65, 0x6e, 0x74, 0x3b, 0x20, 0x61, 0x64, 0x64, 0x69, 0x74, 0x69, 0x6f, 0x6e,
    0x61, 0x6c, 0x20, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x69, 0x6e, 0x67, 0x20, 0x61, 0x6e, 0x64,
    0x0a, 0x20, 0x73, 0x6f, 0x72, 0x74, 0x69, 0x6e, 0x67, 0x20, 0x63, 0x6f, 0x6d, 0x65, 0x73, 0x20,
    0x6c, 0x61, 0x74, 0x65, 0x72, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03,
    0x45, 0x08, 0x16, 0x0a, 0x48, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x47, 0x02, 0x1b,
    0x1a, 0x3b, 0x20, 0x54, 0x68, 0x65, 0x20, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x2d, 0x71, 0x75,
    0x61, 0x6c, 0x69, 0x66, 0x69, 0x65, 0x64, 0x20, 0x6e, 0x61, 0x6d, 0x65, 0x20, 0x6f, 0x66, 0x20,
    0x61, 0x20, 0x70, 0x72, 0x6f, 0x6a, 0x65, 0x63, 0x74, 0x2c, 0x20, 0x65, 0x2e, 0x67, 0x2e, 0x20,
    0x22, 0x63, 0x6f, 0x72, 0x65, 0x2f, 0x6e, 0x67, 0x69, 0x6e, 0x78, 0x22, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x00, 0x04, 0x12, 0x03, 0x47, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x00, 0x05, 0x12, 0x03, 0x47, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x47, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x47, 0x19, 0x1a, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x4a, 0x00, 0x4c,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x4a, 0x08, 0x1e, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x05, 0x02, 0x00, 0x12, 0x03, 0x4b, 0x02, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x00, 0x04, 0x12, 0x03, 0x4b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00,
    0x06, 0x12, 0x03, 0x4b, 0x0b, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x4b, 0x0f, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x4b,
    0x16, 0x17, 0x0a, 0x51, 0x0a, 0x02, 0x04, 0x06, 0x12, 0x04, 0x4f, 0x00, 0x58, 0x01, 0x1a, 0x45,
    0x20, 0x53, 0x65, 0x6e, 0x74, 0x20, 0x66, 0x72, 0x6f, 0x6d, 0x20, 0x61, 0x20, 0x77, 0x6f, 0x72,
    0x6b, 0x65, 0x72, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6a, 0x6f, 0x62, 0x20, 0x73,
    0x65, 0x72, 0x76, 0x65, 0x72, 0x27, 0x73, 0x20, 0x6c, 0x6f, 0x67, 0x20, 0x69, 0x6e, 0x67, 0x65,
    0x73, 0x74, 0x65, 0x72, 0x20, 0x64, 0x75, 0x72, 0x69, 0x6e, 0x67, 0x20, 0x61, 0x20, 0x62, 0x75,
    0x69, 0x6c, 0x64, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x06, 0x01, 0x12, 0x03, 0x4f, 0x08,
    0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x00, 0x12, 0x03, 0x50, 0x02, 0x1d, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x04, 0x12, 0x03, 0x50, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x00, 0x05, 0x12, 0x03, 0x50, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x50, 0x12, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x50, 0x1b, 0x1c, 0x0a, 0x9c, 0x01, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x01, 0x12,
    0x03, 0x54, 0x02, 0x1a, 0x1a, 0x8e, 0x01, 0x20, 0x4f, 0x72, 0x64, 0x65, 0x72, 0x69, 0x6e, 0x67,
    0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x63, 0x68, 0x75, 0x6e, 0x6b, 0x20, 0x69,
    0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6f, 0x76, 0x65, 0x72, 0x61, 0x6c, 0x6c, 0x20, 0x6c, 0x6f,
    0x67, 0x20, 0x6f, 0x75, 0x74, 0x70, 0x75, 0x74, 0x2c, 0x20, 0x73, 0x74, 0x61, 0x72, 0x74, 0x69,
    0x6e, 0x67, 0x20, 0x61, 0x74, 0x20, 0x31, 0x3b, 0x0a, 0x20, 0x57, 0x65, 0x20, 0x63, 0x75, 0x72,
    0x72, 0x65, 0x6e, 0x74, 0x6c, 0x79, 0x20, 0x73, 0x65, 0x6e, 0x64, 0x20, 0x6f, 0x6e, 0x65, 0x20,
    0x6c, 0x69, 0x6e, 0x65, 0x20, 0x61, 0x74, 0x20, 0x61, 0x20, 0x74, 0x69, 0x6d, 0x65, 0x2c, 0x20,
    0x73, 0x6f, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x63, 0x6f, 0x72, 0x72, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x64, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x6c, 0x69, 0x6e, 0x65, 0x0a, 0x20, 0x6e, 0x75, 0x6d,
    0x62, 0x65, 0x72, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x04, 0x12, 0x03,
    0x54, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x05, 0x12, 0x03, 0x54, 0x0b,
    0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x01, 0x12, 0x03, 0x54, 0x12, 0x15, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x03, 0x12, 0x03, 0x54, 0x18, 0x19, 0x0a, 0x64, 0x0a,
    0x04, 0x04, 0x06, 0x02, 0x02, 0x12, 0x03, 0x57, 0x02, 0x1e, 0x1a, 0x57, 0x20, 0x54, 0x68, 0x65,
    0x20, 0x6c, 0x6f, 0x67, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x20, 0x62, 0x65, 0x69,
    0x6e, 0x67, 0x20, 0x73, 0x65, 0x6e, 0x74, 0x0a, 0x20, 0x54, 0x4f, 0x44, 0x4f, 0x3a, 0x20, 0x4d,
    0x61, 0x6b, 0x65, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x61, 0x20, 0x72, 0x65, 0x70, 0x65, 0x61,
    0x74, 0x65, 0x64, 0x20, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x66, 0x75,
    0x74, 0x75, 0x72, 0x65, 0x20, 0x63, 0x6f, 0x6d, 0x70, 0x61, 0x74, 0x69, 0x62, 0x69, 0x6c, 0x69,
    0x74, 0x79, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x04, 0x12, 0x03, 0x57, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x05, 0x12, 0x03, 0x57, 0x0b, 0x11, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x01, 0x12, 0x03, 0x57, 0x12, 0x19, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x02, 0x03, 0x12, 0x03, 0x57, 0x1c, 0x1d, 0x0a, 0x5c, 0x0a, 0x02, 0x04,
    0x07, 0x12, 0x04, 0x5c, 0x00, 0x5e, 0x01, 0x1a, 0x50, 0x20, 0x53, 0x65, 0x6e, 0x74, 0x20, 0x66,
    0x72, 0x6f, 0x6d, 0x20, 0x61, 0x20, 0x77, 0x6f, 0x72, 0x6b, 0x65, 0x72, 0x20, 0x74, 0x6f, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x6a, 0x6f, 0x62, 0x20, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x27, 0x73,
    0x20, 0x6c, 0x6f, 0x67, 0x20, 0x69, 0x6e, 0x67, 0x65, 0x73, 0x74, 0x65, 0x72, 0x20, 0x77, 0x68,
    0x65, 0x6e, 0x20, 0x61, 0x20, 0x62, 0x75, 0x69, 0x6c, 0x64, 0x20, 0x69, 0x73, 0x0a, 0x20, 0x63,
    0x6f, 0x6d, 0x70, 0x6c, 0x65, 0x74, 0x65, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x07, 0x01,
    0x12, 0x03, 0x5c, 0x08, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x00, 0x12, 0x03, 0x5d,
    0x02, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x04, 0x12, 0x03, 0x5d, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x05, 0x12, 0x03, 0x5d, 0x0b, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x01, 0x12, 0x03, 0x5d, 0x12, 0x18, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x07, 0x02, 0x00, 0x03, 0x12, 0x03, 0x5d, 0x1b, 0x1c, 0x0a, 0x6c, 0x0a, 0x02, 0x04, 0x08,
    0x12, 0x04, 0x62, 0x00, 0x66, 0x01, 0x1a, 0x60, 0x20, 0x49, 0x6e, 0x69, 0x74, 0x69, 0x61, 0x74,
    0x65, 0x64, 0x20, 0x62, 0x79, 0x20, 0x41, 0x50, 0x49, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x20, 0x74, 0x6f, 0x20, 0x72, 0x65, 0x74, 0x72, 0x69, 0x65, 0x76, 0x65, 0x20, 0x61, 0x20,
    0x70, 0x6f, 0x72, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6f, 0x66, 0x20, 0x61, 0x20, 0x6a, 0x6f, 0x62,
    0x27, 0x73, 0x20, 0x6c, 0x6f, 0x67, 0x20, 0x64, 0x61, 0x74, 0x61, 0x2c, 0x0a, 0x20, 0x62, 0x65,
    0x67, 0x69, 0x6e, 0x6e, 0x69, 0x6e, 0x67, 0x20, 0x61, 0x74, 0x20, 0x6c, 0x69, 0x6e, 0x65, 0x20,
    0x60, 0x73, 0x74, 0x61, 0x72, 0x74, 0x60, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x08, 0x01, 0x12,
    0x03, 0x62, 0x08, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x00, 0x12, 0x03, 0x63, 0x02,
    0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x04, 0x12, 0x03, 0x63, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x05, 0x12, 0x03, 0x63, 0x0b, 0x11, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x08, 0x02, 0x00, 0x01, 0x12, 0x03, 0x63, 0x12, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x08, 0x02, 0x00, 0x03, 0x12, 0x03, 0x63, 0x17, 0x18, 0x0a, 0x40, 0x0a, 0x04, 0x04, 0x08, 0x02,
    0x01, 0x12, 0x03, 0x65, 0x02, 0x1c, 0x1a, 0x33, 0x20, 0x5a, 0x65, 0x72, 0x6f, 0x2d, 0x69, 0x6e,
    0x64, 0x65, 0x78, 0x65, 0x64, 0x20, 0x6c, 0x69, 0x6e, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x6c, 0x6f, 0x67, 0x20, 0x6f, 0x75, 0x74, 0x70, 0x75, 0x74, 0x20, 0x74, 0x6f, 0x20,
    0x73, 0x74, 0x61, 0x72, 0x74, 0x20, 0x77, 0x69, 0x74, 0x68, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x08, 0x02, 0x01, 0x04, 0x12, 0x03, 0x65, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02,
    0x01, 0x05, 0x12, 0x03, 0x65, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x65, 0x12, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x65, 0x1a, 0x1b, 0x0a, 0x2a, 0x0a, 0x02, 0x04, 0x09, 0x12, 0x04, 0x69, 0x00, 0x74, 0x01, 0x1a,
    0x1e, 0x20, 0x53, 0x65, 0x6e, 0x74, 0x20, 0x69, 0x6e, 0x20, 0x72, 0x65, 0x70, 0x6c, 0x79, 0x20,
    0x74, 0x6f, 0x20, 0x61, 0x20, 0x4a, 0x6f, 0x62, 0x4c, 0x6f, 0x67, 0x47, 0x65, 0x74, 0x0a, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x09, 0x01, 0x12, 0x03, 0x69, 0x08, 0x0e, 0x0a, 0x4b, 0x0a, 0x04, 0x04,
    0x09, 0x02, 0x00, 0x12, 0x03, 0x6b, 0x02, 0x1c, 0x1a, 0x3e, 0x20, 0x5a, 0x65, 0x72, 0x6f, 0x2d,
    0x69, 0x6e, 0x64, 0x65, 0x78, 0x65, 0x64, 0x20, 0x28, 0x69, 0x6e, 0x63, 0x6c, 0x75, 0x73, 0x69,
    0x76, 0x65, 0x29, 0x20, 0x6c, 0x69, 0x6e, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x6c, 0x6f, 0x67, 0x20, 0x6f, 0x75, 0x74, 0x70, 0x75, 0x74, 0x20, 0x69, 0x6e, 0x20, 0x60, 0x63,
    0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x60, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x6b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x6b, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x01, 0x12, 0x03, 0x6b,
    0x12, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x03, 0x12, 0x03, 0x6b, 0x1a, 0x1b,
    0x0a, 0x4b, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x01, 0x12, 0x03, 0x6d, 0x02, 0x1b, 0x1a, 0x3e, 0x20,
    0x5a, 0x65, 0x72, 0x6f, 0x2d, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x65, 0x64, 0x20, 0x28, 0x65, 0x78,
    0x63, 0x6c, 0x75, 0x73, 0x69, 0x76, 0x65, 0x29, 0x20, 0x6c, 0x69, 0x6e, 0x65, 0x20, 0x6f, 0x66,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x6c, 0x6f, 0x67, 0x20, 0x6f, 0x75, 0x74, 0x70, 0x75, 0x74, 0x20,
    0x69, 0x6e, 0x20, 0x60, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x60, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x09, 0x02, 0x01, 0x04, 0x12, 0x03, 0x6d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x09, 0x02, 0x01, 0x05, 0x12, 0x03, 0x6d, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x6d, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x6d, 0x19, 0x1a, 0x0a, 0x22, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x02, 0x12, 0x03, 0x6f,
    0x02, 0x1e, 0x1a, 0x15, 0x20, 0x4c, 0x69, 0x6e, 0x65, 0x73, 0x20, 0x6f, 0x66, 0x20, 0x6c, 0x6f,
    0x67, 0x20, 0x6f, 0x75, 0x74, 0x70, 0x75, 0x74, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02,
    0x02, 0x04, 0x12, 0x03, 0x6f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x02, 0x05,
    0x12, 0x03, 0x6f, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x02, 0x01, 0x12, 0x03,
    0x6f, 0x12, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x02, 0x03, 0x12, 0x03, 0x6f, 0x1c,
    0x1d, 0x0a, 0x9c, 0x01, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x03, 0x12, 0x03, 0x73, 0x02, 0x20, 0x1a,
    0x8e, 0x01, 0x20, 0x57, 0x68, 0x69, 0x6c, 0x65, 0x20, 0x77, 0x65, 0x20, 0x6e, 0x65, 0x65, 0x64,
    0x20, 0x74, 0x6f, 0x20, 0x70, 0x6f, 0x6c, 0x6c, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x6c, 0x6f, 0x67,
    0x73, 0x2c, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x73, 0x65, 0x72, 0x76, 0x65, 0x73, 0x20, 0x61,
    0x73, 0x20, 0x61, 0x6e, 0x20, 0x69, 0x6e, 0x64, 0x69, 0x63, 0x61, 0x74, 0x6f, 0x72, 0x20, 0x74,
    0x6f, 0x0a, 0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x73, 0x20, 0x69, 0x66, 0x20, 0x74, 0x68,
    0x65, 0x79, 0x20, 0x6e, 0x65, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x69,
    0x6e, 0x75, 0x65, 0x20, 0x70, 0x6f, 0x6c, 0x6c, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x6f, 0x20, 0x72,
    0x65, 0x63, 0x65, 0x69, 0x76, 0x65, 0x20, 0x6d, 0x6f, 0x72, 0x65, 0x20, 0x6c, 0x6f, 0x67, 0x73,
    0x2c, 0x20, 0x6f, 0x72, 0x0a, 0x20, 0x63, 0x61, 0x6e, 0x20, 0x73, 0x74, 0x6f, 0x70, 0x2e, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x03, 0x04, 0x12, 0x03, 0x73, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x09, 0x02, 0x03, 0x05, 0x12, 0x03, 0x73, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x09, 0x02, 0x03, 0x01, 0x12, 0x03, 0x73, 0x10, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09,
    0x02, 0x03, 0x03, 0x12, 0x03, 0x73, 0x1e, 0x1f,
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
