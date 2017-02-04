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
    project: ::protobuf::SingularPtrField<super::vault::OriginProject>,
    error: ::protobuf::SingularPtrField<super::net::NetError>,
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

    // optional .vault.OriginProject project = 4;

    pub fn clear_project(&mut self) {
        self.project.clear();
    }

    pub fn has_project(&self) -> bool {
        self.project.is_some()
    }

    // Param is passed by value, moved
    pub fn set_project(&mut self, v: super::vault::OriginProject) {
        self.project = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_project(&mut self) -> &mut super::vault::OriginProject {
        if self.project.is_none() {
            self.project.set_default();
        };
        self.project.as_mut().unwrap()
    }

    // Take field
    pub fn take_project(&mut self) -> super::vault::OriginProject {
        self.project.take().unwrap_or_else(|| super::vault::OriginProject::new())
    }

    pub fn get_project(&self) -> &super::vault::OriginProject {
        self.project.as_ref().unwrap_or_else(|| super::vault::OriginProject::default_instance())
    }

    fn get_project_for_reflect(&self) -> &::protobuf::SingularPtrField<super::vault::OriginProject> {
        &self.project
    }

    fn mut_project_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::vault::OriginProject> {
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
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::vault::OriginProject>>(
                    "project",
                    Job::get_project_for_reflect,
                    Job::mut_project_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::net::NetError>>(
                    "error",
                    Job::get_error_for_reflect,
                    Job::mut_error_for_reflect,
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
    project: ::protobuf::SingularPtrField<super::vault::OriginProject>,
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

    // optional .vault.OriginProject project = 2;

    pub fn clear_project(&mut self) {
        self.project.clear();
    }

    pub fn has_project(&self) -> bool {
        self.project.is_some()
    }

    // Param is passed by value, moved
    pub fn set_project(&mut self, v: super::vault::OriginProject) {
        self.project = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_project(&mut self) -> &mut super::vault::OriginProject {
        if self.project.is_none() {
            self.project.set_default();
        };
        self.project.as_mut().unwrap()
    }

    // Take field
    pub fn take_project(&mut self) -> super::vault::OriginProject {
        self.project.take().unwrap_or_else(|| super::vault::OriginProject::new())
    }

    pub fn get_project(&self) -> &super::vault::OriginProject {
        self.project.as_ref().unwrap_or_else(|| super::vault::OriginProject::default_instance())
    }

    fn get_project_for_reflect(&self) -> &::protobuf::SingularPtrField<super::vault::OriginProject> {
        &self.project
    }

    fn mut_project_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::vault::OriginProject> {
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
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::vault::OriginProject>>(
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
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x15, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x73,
    0x2f, 0x76, 0x61, 0x75, 0x6c, 0x74, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x6e, 0x0a, 0x09,
    0x48, 0x65, 0x61, 0x72, 0x74, 0x62, 0x65, 0x61, 0x74, 0x12, 0x1a, 0x0a, 0x08, 0x65, 0x6e, 0x64,
    0x70, 0x6f, 0x69, 0x6e, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x08, 0x65, 0x6e, 0x64,
    0x70, 0x6f, 0x69, 0x6e, 0x74, 0x12, 0x1a, 0x0a, 0x02, 0x6f, 0x73, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x0e, 0x32, 0x0a, 0x2e, 0x6a, 0x6f, 0x62, 0x73, 0x72, 0x76, 0x2e, 0x4f, 0x73, 0x52, 0x02, 0x6f,
    0x73, 0x12, 0x29, 0x0a, 0x05, 0x73, 0x74, 0x61, 0x74, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0e,
    0x32, 0x13, 0x2e, 0x6a, 0x6f, 0x62, 0x73, 0x72, 0x76, 0x2e, 0x57, 0x6f, 0x72, 0x6b, 0x65, 0x72,
    0x53, 0x74, 0x61, 0x74, 0x65, 0x52, 0x05, 0x73, 0x74, 0x61, 0x74, 0x65, 0x22, 0xad, 0x01, 0x0a,
    0x03, 0x4a, 0x6f, 0x62, 0x12, 0x0e, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04,
    0x52, 0x02, 0x69, 0x64, 0x12, 0x19, 0x0a, 0x08, 0x6f, 0x77, 0x6e, 0x65, 0x72, 0x5f, 0x69, 0x64,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x04, 0x52, 0x07, 0x6f, 0x77, 0x6e, 0x65, 0x72, 0x49, 0x64, 0x12,
    0x26, 0x0a, 0x05, 0x73, 0x74, 0x61, 0x74, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x10,
    0x2e, 0x6a, 0x6f, 0x62, 0x73, 0x72, 0x76, 0x2e, 0x4a, 0x6f, 0x62, 0x53, 0x74, 0x61, 0x74, 0x65,
    0x52, 0x05, 0x73, 0x74, 0x61, 0x74, 0x65, 0x12, 0x2e, 0x0a, 0x07, 0x70, 0x72, 0x6f, 0x6a, 0x65,
    0x63, 0x74, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x14, 0x2e, 0x76, 0x61, 0x75, 0x6c, 0x74,
    0x2e, 0x4f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x50, 0x72, 0x6f, 0x6a, 0x65, 0x63, 0x74, 0x52, 0x07,
    0x70, 0x72, 0x6f, 0x6a, 0x65, 0x63, 0x74, 0x12, 0x23, 0x0a, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72,
    0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0d, 0x2e, 0x6e, 0x65, 0x74, 0x2e, 0x4e, 0x65, 0x74,
    0x45, 0x72, 0x72, 0x6f, 0x72, 0x52, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x22, 0x18, 0x0a, 0x06,
    0x4a, 0x6f, 0x62, 0x47, 0x65, 0x74, 0x12, 0x0e, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x04, 0x52, 0x02, 0x69, 0x64, 0x22, 0x54, 0x0a, 0x07, 0x4a, 0x6f, 0x62, 0x53, 0x70, 0x65,
    0x63, 0x12, 0x19, 0x0a, 0x08, 0x6f, 0x77, 0x6e, 0x65, 0x72, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x04, 0x52, 0x07, 0x6f, 0x77, 0x6e, 0x65, 0x72, 0x49, 0x64, 0x12, 0x2e, 0x0a, 0x07,
    0x70, 0x72, 0x6f, 0x6a, 0x65, 0x63, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x14, 0x2e,
    0x76, 0x61, 0x75, 0x6c, 0x74, 0x2e, 0x4f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x50, 0x72, 0x6f, 0x6a,
    0x65, 0x63, 0x74, 0x52, 0x07, 0x70, 0x72, 0x6f, 0x6a, 0x65, 0x63, 0x74, 0x2a, 0x28, 0x0a, 0x02,
    0x4f, 0x73, 0x12, 0x09, 0x0a, 0x05, 0x4c, 0x69, 0x6e, 0x75, 0x78, 0x10, 0x01, 0x12, 0x0a, 0x0a,
    0x06, 0x44, 0x61, 0x72, 0x77, 0x69, 0x6e, 0x10, 0x02, 0x12, 0x0b, 0x0a, 0x07, 0x57, 0x69, 0x6e,
    0x64, 0x6f, 0x77, 0x73, 0x10, 0x03, 0x2a, 0x22, 0x0a, 0x0b, 0x57, 0x6f, 0x72, 0x6b, 0x65, 0x72,
    0x53, 0x74, 0x61, 0x74, 0x65, 0x12, 0x09, 0x0a, 0x05, 0x52, 0x65, 0x61, 0x64, 0x79, 0x10, 0x00,
    0x12, 0x08, 0x0a, 0x04, 0x42, 0x75, 0x73, 0x79, 0x10, 0x01, 0x2a, 0x5f, 0x0a, 0x08, 0x4a, 0x6f,
    0x62, 0x53, 0x74, 0x61, 0x74, 0x65, 0x12, 0x0b, 0x0a, 0x07, 0x50, 0x65, 0x6e, 0x64, 0x69, 0x6e,
    0x67, 0x10, 0x00, 0x12, 0x0e, 0x0a, 0x0a, 0x50, 0x72, 0x6f, 0x63, 0x65, 0x73, 0x73, 0x69, 0x6e,
    0x67, 0x10, 0x01, 0x12, 0x0c, 0x0a, 0x08, 0x43, 0x6f, 0x6d, 0x70, 0x6c, 0x65, 0x74, 0x65, 0x10,
    0x02, 0x12, 0x0c, 0x0a, 0x08, 0x52, 0x65, 0x6a, 0x65, 0x63, 0x74, 0x65, 0x64, 0x10, 0x03, 0x12,
    0x0a, 0x0a, 0x06, 0x46, 0x61, 0x69, 0x6c, 0x65, 0x64, 0x10, 0x04, 0x12, 0x0e, 0x0a, 0x0a, 0x44,
    0x69, 0x73, 0x70, 0x61, 0x74, 0x63, 0x68, 0x65, 0x64, 0x10, 0x05, 0x4a, 0x8a, 0x0b, 0x0a, 0x06,
    0x12, 0x04, 0x00, 0x00, 0x2d, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x00, 0x08, 0x0e,
    0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x01, 0x07, 0x1c, 0x0a, 0x09, 0x0a, 0x02, 0x03,
    0x01, 0x12, 0x03, 0x02, 0x07, 0x1e, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x00, 0x12, 0x04, 0x04, 0x00,
    0x08, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x00, 0x01, 0x12, 0x03, 0x04, 0x05, 0x07, 0x0a, 0x0b,
    0x0a, 0x04, 0x05, 0x00, 0x02, 0x00, 0x12, 0x03, 0x05, 0x02, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x05, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02,
    0x00, 0x02, 0x12, 0x03, 0x05, 0x0a, 0x0b, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x01, 0x12,
    0x03, 0x06, 0x02, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x06,
    0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x06, 0x0b, 0x0c,
    0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x02, 0x12, 0x03, 0x07, 0x02, 0x0e, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x07, 0x02, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x07, 0x0c, 0x0d, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x01, 0x12,
    0x04, 0x0a, 0x00, 0x0d, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x01, 0x01, 0x12, 0x03, 0x0a, 0x05,
    0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x00, 0x12, 0x03, 0x0b, 0x02, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0b, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x01, 0x02, 0x00, 0x02, 0x12, 0x03, 0x0b, 0x0a, 0x0b, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01,
    0x02, 0x01, 0x12, 0x03, 0x0c, 0x02, 0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x0c, 0x02, 0x06, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x01, 0x02, 0x12, 0x03,
    0x0c, 0x09, 0x0a, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x02, 0x12, 0x04, 0x0f, 0x00, 0x16, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x05, 0x02, 0x01, 0x12, 0x03, 0x0f, 0x05, 0x0d, 0x0a, 0x0b, 0x0a, 0x04, 0x05,
    0x02, 0x02, 0x00, 0x12, 0x03, 0x10, 0x02, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x10, 0x02, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x00, 0x02, 0x12,
    0x03, 0x10, 0x0c, 0x0d, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x01, 0x12, 0x03, 0x11, 0x02,
    0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x11, 0x02, 0x0c, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x01, 0x02, 0x12, 0x03, 0x11, 0x0f, 0x10, 0x0a, 0x0b, 0x0a,
    0x04, 0x05, 0x02, 0x02, 0x02, 0x12, 0x03, 0x12, 0x02, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02,
    0x02, 0x02, 0x01, 0x12, 0x03, 0x12, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x02,
    0x02, 0x12, 0x03, 0x12, 0x0d, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x03, 0x12, 0x03,
    0x13, 0x02, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x03, 0x01, 0x12, 0x03, 0x13, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x03, 0x02, 0x12, 0x03, 0x13, 0x0d, 0x0e, 0x0a,
    0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x04, 0x12, 0x03, 0x14, 0x02, 0x0d, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x02, 0x02, 0x04, 0x01, 0x12, 0x03, 0x14, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02,
    0x02, 0x04, 0x02, 0x12, 0x03, 0x14, 0x0b, 0x0c, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x05,
    0x12, 0x03, 0x15, 0x02, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x05, 0x01, 0x12, 0x03,
    0x15, 0x02, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x05, 0x02, 0x12, 0x03, 0x15, 0x0f,
    0x10, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x18, 0x00, 0x1c, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x18, 0x08, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x00, 0x12, 0x03, 0x19, 0x02, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12,
    0x03, 0x19, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x19,
    0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x19, 0x12, 0x1a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x19, 0x1d, 0x1e, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x1a, 0x02, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x1a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x06, 0x12, 0x03, 0x1a, 0x0b, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x1a, 0x0e, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x1a, 0x13, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x1b, 0x02, 0x21,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x1b, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x06, 0x12, 0x03, 0x1b, 0x0b, 0x16, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x1b, 0x17, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x02, 0x03, 0x12, 0x03, 0x1b, 0x1f, 0x20, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04,
    0x1e, 0x00, 0x24, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x1e, 0x08, 0x0b,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x1f, 0x02, 0x19, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x1f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x1f, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x1f, 0x12, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x1f, 0x17, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x20,
    0x02, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x04, 0x12, 0x03, 0x20, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x20, 0x0b, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x20, 0x12, 0x1a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x20, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01,
    0x02, 0x02, 0x12, 0x03, 0x21, 0x02, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x04,
    0x12, 0x03, 0x21, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x06, 0x12, 0x03,
    0x21, 0x0b, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x21, 0x14,
    0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x03, 0x12, 0x03, 0x21, 0x1c, 0x1d, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x03, 0x12, 0x03, 0x22, 0x02, 0x2b, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x03, 0x04, 0x12, 0x03, 0x22, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x03, 0x06, 0x12, 0x03, 0x22, 0x0b, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03,
    0x01, 0x12, 0x03, 0x22, 0x1f, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x03, 0x12,
    0x03, 0x22, 0x29, 0x2a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x04, 0x12, 0x03, 0x23, 0x02,
    0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x04, 0x12, 0x03, 0x23, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x06, 0x12, 0x03, 0x23, 0x0b, 0x17, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x04, 0x01, 0x12, 0x03, 0x23, 0x18, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x04, 0x03, 0x12, 0x03, 0x23, 0x20, 0x21, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12,
    0x04, 0x26, 0x00, 0x28, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x26, 0x08,
    0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x27, 0x02, 0x19, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x27, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x27, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x27, 0x12, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x27, 0x17, 0x18, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x2a, 0x00,
    0x2d, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x2a, 0x08, 0x0f, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x2b, 0x02, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x00, 0x04, 0x12, 0x03, 0x2b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x2b, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x2b, 0x12, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x2b, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x01, 0x12, 0x03, 0x2c, 0x02, 0x2b,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x04, 0x12, 0x03, 0x2c, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x06, 0x12, 0x03, 0x2c, 0x0b, 0x1e, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x01, 0x01, 0x12, 0x03, 0x2c, 0x1f, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x2c, 0x29, 0x2a,
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
