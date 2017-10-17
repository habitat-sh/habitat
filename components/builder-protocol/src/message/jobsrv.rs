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
        }
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
                    }
                    let tmp = is.read_enum()?;
                    self.os = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
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
        if let Some(ref v) = self.endpoint.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(v) = self.os {
            my_size += ::protobuf::rt::enum_size(2, v);
        }
        if let Some(v) = self.state {
            my_size += ::protobuf::rt::enum_size(3, v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.endpoint.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(v) = self.os {
            os.write_enum(2, v.value())?;
        }
        if let Some(v) = self.state {
            os.write_enum(3, v.value())?;
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
pub struct BusyWorker {
    // message fields
    ident: ::protobuf::SingularField<::std::string::String>,
    job_id: ::std::option::Option<u64>,
    quarantined: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for BusyWorker {}

impl BusyWorker {
    pub fn new() -> BusyWorker {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BusyWorker {
        static mut instance: ::protobuf::lazy::Lazy<BusyWorker> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BusyWorker,
        };
        unsafe {
            instance.get(BusyWorker::new)
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

    // optional uint64 job_id = 2;

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

    // optional bool quarantined = 3;

    pub fn clear_quarantined(&mut self) {
        self.quarantined = ::std::option::Option::None;
    }

    pub fn has_quarantined(&self) -> bool {
        self.quarantined.is_some()
    }

    // Param is passed by value, moved
    pub fn set_quarantined(&mut self, v: bool) {
        self.quarantined = ::std::option::Option::Some(v);
    }

    pub fn get_quarantined(&self) -> bool {
        self.quarantined.unwrap_or(false)
    }

    fn get_quarantined_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.quarantined
    }

    fn mut_quarantined_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.quarantined
    }
}

impl ::protobuf::Message for BusyWorker {
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.job_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.quarantined = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.job_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.quarantined {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.ident.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(v) = self.job_id {
            os.write_uint64(2, v)?;
        }
        if let Some(v) = self.quarantined {
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

impl ::protobuf::MessageStatic for BusyWorker {
    fn new() -> BusyWorker {
        BusyWorker::new()
    }

    fn descriptor_static(_: ::std::option::Option<BusyWorker>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "ident",
                    BusyWorker::get_ident_for_reflect,
                    BusyWorker::mut_ident_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "job_id",
                    BusyWorker::get_job_id_for_reflect,
                    BusyWorker::mut_job_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "quarantined",
                    BusyWorker::get_quarantined_for_reflect,
                    BusyWorker::mut_quarantined_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BusyWorker>(
                    "BusyWorker",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BusyWorker {
    fn clear(&mut self) {
        self.clear_ident();
        self.clear_job_id();
        self.clear_quarantined();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for BusyWorker {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BusyWorker {
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
    integrations: ::protobuf::RepeatedField<super::originsrv::OriginIntegration>,
    channel: ::protobuf::SingularField<::std::string::String>,
    project_integrations: ::protobuf::RepeatedField<super::originsrv::OriginProjectIntegration>,
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
        }
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
        }
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
        }
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
        }
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
        }
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

    // repeated .originsrv.OriginIntegration integrations = 12;

    pub fn clear_integrations(&mut self) {
        self.integrations.clear();
    }

    // Param is passed by value, moved
    pub fn set_integrations(&mut self, v: ::protobuf::RepeatedField<super::originsrv::OriginIntegration>) {
        self.integrations = v;
    }

    // Mutable pointer to the field.
    pub fn mut_integrations(&mut self) -> &mut ::protobuf::RepeatedField<super::originsrv::OriginIntegration> {
        &mut self.integrations
    }

    // Take field
    pub fn take_integrations(&mut self) -> ::protobuf::RepeatedField<super::originsrv::OriginIntegration> {
        ::std::mem::replace(&mut self.integrations, ::protobuf::RepeatedField::new())
    }

    pub fn get_integrations(&self) -> &[super::originsrv::OriginIntegration] {
        &self.integrations
    }

    fn get_integrations_for_reflect(&self) -> &::protobuf::RepeatedField<super::originsrv::OriginIntegration> {
        &self.integrations
    }

    fn mut_integrations_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::originsrv::OriginIntegration> {
        &mut self.integrations
    }

    // optional string channel = 13;

    pub fn clear_channel(&mut self) {
        self.channel.clear();
    }

    pub fn has_channel(&self) -> bool {
        self.channel.is_some()
    }

    // Param is passed by value, moved
    pub fn set_channel(&mut self, v: ::std::string::String) {
        self.channel = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_channel(&mut self) -> &mut ::std::string::String {
        if self.channel.is_none() {
            self.channel.set_default();
        }
        self.channel.as_mut().unwrap()
    }

    // Take field
    pub fn take_channel(&mut self) -> ::std::string::String {
        self.channel.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_channel(&self) -> &str {
        match self.channel.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_channel_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.channel
    }

    fn mut_channel_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.channel
    }

    // repeated .originsrv.OriginProjectIntegration project_integrations = 14;

    pub fn clear_project_integrations(&mut self) {
        self.project_integrations.clear();
    }

    // Param is passed by value, moved
    pub fn set_project_integrations(&mut self, v: ::protobuf::RepeatedField<super::originsrv::OriginProjectIntegration>) {
        self.project_integrations = v;
    }

    // Mutable pointer to the field.
    pub fn mut_project_integrations(&mut self) -> &mut ::protobuf::RepeatedField<super::originsrv::OriginProjectIntegration> {
        &mut self.project_integrations
    }

    // Take field
    pub fn take_project_integrations(&mut self) -> ::protobuf::RepeatedField<super::originsrv::OriginProjectIntegration> {
        ::std::mem::replace(&mut self.project_integrations, ::protobuf::RepeatedField::new())
    }

    pub fn get_project_integrations(&self) -> &[super::originsrv::OriginProjectIntegration] {
        &self.project_integrations
    }

    fn get_project_integrations_for_reflect(&self) -> &::protobuf::RepeatedField<super::originsrv::OriginProjectIntegration> {
        &self.project_integrations
    }

    fn mut_project_integrations_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::originsrv::OriginProjectIntegration> {
        &mut self.project_integrations
    }
}

impl ::protobuf::Message for Job {
    fn is_initialized(&self) -> bool {
        for v in &self.project {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.error {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.package_ident {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.integrations {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.project_integrations {
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
                    }
                    let tmp = is.read_bool()?;
                    self.is_archived = ::std::option::Option::Some(tmp);
                },
                12 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.integrations)?;
                },
                13 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.channel)?;
                },
                14 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.project_integrations)?;
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
        if let Some(v) = self.state {
            my_size += ::protobuf::rt::enum_size(3, v);
        }
        if let Some(ref v) = self.project.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.error.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.created_at.as_ref() {
            my_size += ::protobuf::rt::string_size(6, &v);
        }
        if let Some(ref v) = self.build_started_at.as_ref() {
            my_size += ::protobuf::rt::string_size(7, &v);
        }
        if let Some(ref v) = self.build_finished_at.as_ref() {
            my_size += ::protobuf::rt::string_size(8, &v);
        }
        if let Some(ref v) = self.package_ident.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.is_archived {
            my_size += 2;
        }
        for value in &self.integrations {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(ref v) = self.channel.as_ref() {
            my_size += ::protobuf::rt::string_size(13, &v);
        }
        for value in &self.project_integrations {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
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
        if let Some(v) = self.state {
            os.write_enum(3, v.value())?;
        }
        if let Some(ref v) = self.project.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.error.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.created_at.as_ref() {
            os.write_string(6, &v)?;
        }
        if let Some(ref v) = self.build_started_at.as_ref() {
            os.write_string(7, &v)?;
        }
        if let Some(ref v) = self.build_finished_at.as_ref() {
            os.write_string(8, &v)?;
        }
        if let Some(ref v) = self.package_ident.as_ref() {
            os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.is_archived {
            os.write_bool(11, v)?;
        }
        for v in &self.integrations {
            os.write_tag(12, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(ref v) = self.channel.as_ref() {
            os.write_string(13, &v)?;
        }
        for v in &self.project_integrations {
            os.write_tag(14, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::originsrv::OriginIntegration>>(
                    "integrations",
                    Job::get_integrations_for_reflect,
                    Job::mut_integrations_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "channel",
                    Job::get_channel_for_reflect,
                    Job::mut_channel_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::originsrv::OriginProjectIntegration>>(
                    "project_integrations",
                    Job::get_project_integrations_for_reflect,
                    Job::mut_project_integrations_for_reflect,
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
        self.clear_integrations();
        self.clear_channel();
        self.clear_project_integrations();
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
                    }
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
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id {
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
    channel: ::protobuf::SingularField<::std::string::String>,
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
        }
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

    // optional string channel = 3;

    pub fn clear_channel(&mut self) {
        self.channel.clear();
    }

    pub fn has_channel(&self) -> bool {
        self.channel.is_some()
    }

    // Param is passed by value, moved
    pub fn set_channel(&mut self, v: ::std::string::String) {
        self.channel = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_channel(&mut self) -> &mut ::std::string::String {
        if self.channel.is_none() {
            self.channel.set_default();
        }
        self.channel.as_mut().unwrap()
    }

    // Take field
    pub fn take_channel(&mut self) -> ::std::string::String {
        self.channel.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_channel(&self) -> &str {
        match self.channel.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_channel_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.channel
    }

    fn mut_channel_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.channel
    }
}

impl ::protobuf::Message for JobSpec {
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
                    self.owner_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.project)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.channel)?;
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
        if let Some(ref v) = self.project.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.channel.as_ref() {
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
        if let Some(ref v) = self.project.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.channel.as_ref() {
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
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "channel",
                    JobSpec::get_channel_for_reflect,
                    JobSpec::mut_channel_for_reflect,
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
        self.clear_channel();
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
    start: ::std::option::Option<u64>,
    stop: ::std::option::Option<u64>,
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
        if let Some(ref v) = self.name.as_ref() {
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
        if let Some(ref v) = self.name.as_ref() {
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
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "start",
                    ProjectJobsGet::get_start_for_reflect,
                    ProjectJobsGet::mut_start_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "stop",
                    ProjectJobsGet::get_stop_for_reflect,
                    ProjectJobsGet::mut_stop_for_reflect,
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
        self.clear_start();
        self.clear_stop();
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
    start: ::std::option::Option<u64>,
    stop: ::std::option::Option<u64>,
    count: ::std::option::Option<u64>,
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

    // optional uint64 count = 4;

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
}

impl ::protobuf::Message for ProjectJobsGetResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.jobs {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.jobs)?;
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
                    let tmp = is.read_uint64()?;
                    self.count = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.start {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.stop {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.count {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
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
        if let Some(v) = self.start {
            os.write_uint64(2, v)?;
        }
        if let Some(v) = self.stop {
            os.write_uint64(3, v)?;
        }
        if let Some(v) = self.count {
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
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "start",
                    ProjectJobsGetResponse::get_start_for_reflect,
                    ProjectJobsGetResponse::mut_start_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "stop",
                    ProjectJobsGetResponse::get_stop_for_reflect,
                    ProjectJobsGetResponse::mut_stop_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "count",
                    ProjectJobsGetResponse::get_count_for_reflect,
                    ProjectJobsGetResponse::mut_count_for_reflect,
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
        self.clear_start();
        self.clear_stop();
        self.clear_count();
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
        }
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
                    }
                    let tmp = is.read_uint64()?;
                    self.job_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
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
        }
        if let Some(v) = self.seq {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.content.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.job_id {
            os.write_uint64(1, v)?;
        }
        if let Some(v) = self.seq {
            os.write_uint64(2, v)?;
        }
        if let Some(ref v) = self.content.as_ref() {
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
        if let Some(v) = self.job_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.job_id {
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
                    }
                    let tmp = is.read_uint64()?;
                    self.id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
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
        }
        if let Some(v) = self.start {
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
        if let Some(v) = self.start {
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
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.content)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
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
        }
        if let Some(v) = self.stop {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.content {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        if let Some(v) = self.is_complete {
            my_size += 2;
        }
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
        for v in &self.content {
            os.write_string(3, &v)?;
        };
        if let Some(v) = self.is_complete {
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

#[derive(PartialEq,Clone,Default)]
pub struct JobGroupSpec {
    // message fields
    origin: ::protobuf::SingularField<::std::string::String>,
    package: ::protobuf::SingularField<::std::string::String>,
    deps_only: ::std::option::Option<bool>,
    target: ::protobuf::SingularField<::std::string::String>,
    origin_only: ::std::option::Option<bool>,
    package_only: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for JobGroupSpec {}

impl JobGroupSpec {
    pub fn new() -> JobGroupSpec {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static JobGroupSpec {
        static mut instance: ::protobuf::lazy::Lazy<JobGroupSpec> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const JobGroupSpec,
        };
        unsafe {
            instance.get(JobGroupSpec::new)
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

    // optional string target = 4;

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

    // optional bool origin_only = 5;

    pub fn clear_origin_only(&mut self) {
        self.origin_only = ::std::option::Option::None;
    }

    pub fn has_origin_only(&self) -> bool {
        self.origin_only.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin_only(&mut self, v: bool) {
        self.origin_only = ::std::option::Option::Some(v);
    }

    pub fn get_origin_only(&self) -> bool {
        self.origin_only.unwrap_or(false)
    }

    fn get_origin_only_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.origin_only
    }

    fn mut_origin_only_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.origin_only
    }

    // optional bool package_only = 6;

    pub fn clear_package_only(&mut self) {
        self.package_only = ::std::option::Option::None;
    }

    pub fn has_package_only(&self) -> bool {
        self.package_only.is_some()
    }

    // Param is passed by value, moved
    pub fn set_package_only(&mut self, v: bool) {
        self.package_only = ::std::option::Option::Some(v);
    }

    pub fn get_package_only(&self) -> bool {
        self.package_only.unwrap_or(false)
    }

    fn get_package_only_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.package_only
    }

    fn mut_package_only_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.package_only
    }
}

impl ::protobuf::Message for JobGroupSpec {
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
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.target)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.origin_only = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.package_only = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.target.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        }
        if let Some(v) = self.origin_only {
            my_size += 2;
        }
        if let Some(v) = self.package_only {
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
        if let Some(ref v) = self.target.as_ref() {
            os.write_string(4, &v)?;
        }
        if let Some(v) = self.origin_only {
            os.write_bool(5, v)?;
        }
        if let Some(v) = self.package_only {
            os.write_bool(6, v)?;
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

impl ::protobuf::MessageStatic for JobGroupSpec {
    fn new() -> JobGroupSpec {
        JobGroupSpec::new()
    }

    fn descriptor_static(_: ::std::option::Option<JobGroupSpec>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "origin",
                    JobGroupSpec::get_origin_for_reflect,
                    JobGroupSpec::mut_origin_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "package",
                    JobGroupSpec::get_package_for_reflect,
                    JobGroupSpec::mut_package_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "deps_only",
                    JobGroupSpec::get_deps_only_for_reflect,
                    JobGroupSpec::mut_deps_only_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "target",
                    JobGroupSpec::get_target_for_reflect,
                    JobGroupSpec::mut_target_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "origin_only",
                    JobGroupSpec::get_origin_only_for_reflect,
                    JobGroupSpec::mut_origin_only_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "package_only",
                    JobGroupSpec::get_package_only_for_reflect,
                    JobGroupSpec::mut_package_only_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<JobGroupSpec>(
                    "JobGroupSpec",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for JobGroupSpec {
    fn clear(&mut self) {
        self.clear_origin();
        self.clear_package();
        self.clear_deps_only();
        self.clear_target();
        self.clear_origin_only();
        self.clear_package_only();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for JobGroupSpec {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for JobGroupSpec {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct JobGroupProject {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    ident: ::protobuf::SingularField<::std::string::String>,
    state: ::std::option::Option<JobGroupProjectState>,
    job_id: ::std::option::Option<u64>,
    target: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for JobGroupProject {}

impl JobGroupProject {
    pub fn new() -> JobGroupProject {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static JobGroupProject {
        static mut instance: ::protobuf::lazy::Lazy<JobGroupProject> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const JobGroupProject,
        };
        unsafe {
            instance.get(JobGroupProject::new)
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

    // optional .jobsrv.JobGroupProjectState state = 3;

    pub fn clear_state(&mut self) {
        self.state = ::std::option::Option::None;
    }

    pub fn has_state(&self) -> bool {
        self.state.is_some()
    }

    // Param is passed by value, moved
    pub fn set_state(&mut self, v: JobGroupProjectState) {
        self.state = ::std::option::Option::Some(v);
    }

    pub fn get_state(&self) -> JobGroupProjectState {
        self.state.unwrap_or(JobGroupProjectState::NotStarted)
    }

    fn get_state_for_reflect(&self) -> &::std::option::Option<JobGroupProjectState> {
        &self.state
    }

    fn mut_state_for_reflect(&mut self) -> &mut ::std::option::Option<JobGroupProjectState> {
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

    // optional string target = 5;

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

impl ::protobuf::Message for JobGroupProject {
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
                5 => {
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
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(v) = self.state {
            my_size += ::protobuf::rt::enum_size(3, v);
        }
        if let Some(v) = self.job_id {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.target.as_ref() {
            my_size += ::protobuf::rt::string_size(5, &v);
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
        if let Some(ref v) = self.target.as_ref() {
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

impl ::protobuf::MessageStatic for JobGroupProject {
    fn new() -> JobGroupProject {
        JobGroupProject::new()
    }

    fn descriptor_static(_: ::std::option::Option<JobGroupProject>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    JobGroupProject::get_name_for_reflect,
                    JobGroupProject::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "ident",
                    JobGroupProject::get_ident_for_reflect,
                    JobGroupProject::mut_ident_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<JobGroupProjectState>>(
                    "state",
                    JobGroupProject::get_state_for_reflect,
                    JobGroupProject::mut_state_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "job_id",
                    JobGroupProject::get_job_id_for_reflect,
                    JobGroupProject::mut_job_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "target",
                    JobGroupProject::get_target_for_reflect,
                    JobGroupProject::mut_target_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<JobGroupProject>(
                    "JobGroupProject",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for JobGroupProject {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_ident();
        self.clear_state();
        self.clear_job_id();
        self.clear_target();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for JobGroupProject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for JobGroupProject {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct JobGroupAbort {
    // message fields
    group_id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for JobGroupAbort {}

impl JobGroupAbort {
    pub fn new() -> JobGroupAbort {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static JobGroupAbort {
        static mut instance: ::protobuf::lazy::Lazy<JobGroupAbort> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const JobGroupAbort,
        };
        unsafe {
            instance.get(JobGroupAbort::new)
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

impl ::protobuf::Message for JobGroupAbort {
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

impl ::protobuf::MessageStatic for JobGroupAbort {
    fn new() -> JobGroupAbort {
        JobGroupAbort::new()
    }

    fn descriptor_static(_: ::std::option::Option<JobGroupAbort>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "group_id",
                    JobGroupAbort::get_group_id_for_reflect,
                    JobGroupAbort::mut_group_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<JobGroupAbort>(
                    "JobGroupAbort",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for JobGroupAbort {
    fn clear(&mut self) {
        self.clear_group_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for JobGroupAbort {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for JobGroupAbort {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct JobGroupGet {
    // message fields
    group_id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for JobGroupGet {}

impl JobGroupGet {
    pub fn new() -> JobGroupGet {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static JobGroupGet {
        static mut instance: ::protobuf::lazy::Lazy<JobGroupGet> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const JobGroupGet,
        };
        unsafe {
            instance.get(JobGroupGet::new)
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

impl ::protobuf::Message for JobGroupGet {
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

impl ::protobuf::MessageStatic for JobGroupGet {
    fn new() -> JobGroupGet {
        JobGroupGet::new()
    }

    fn descriptor_static(_: ::std::option::Option<JobGroupGet>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "group_id",
                    JobGroupGet::get_group_id_for_reflect,
                    JobGroupGet::mut_group_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<JobGroupGet>(
                    "JobGroupGet",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for JobGroupGet {
    fn clear(&mut self) {
        self.clear_group_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for JobGroupGet {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for JobGroupGet {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct JobGroup {
    // message fields
    id: ::std::option::Option<u64>,
    state: ::std::option::Option<JobGroupState>,
    projects: ::protobuf::RepeatedField<JobGroupProject>,
    created_at: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for JobGroup {}

impl JobGroup {
    pub fn new() -> JobGroup {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static JobGroup {
        static mut instance: ::protobuf::lazy::Lazy<JobGroup> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const JobGroup,
        };
        unsafe {
            instance.get(JobGroup::new)
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

    // optional .jobsrv.JobGroupState state = 2;

    pub fn clear_state(&mut self) {
        self.state = ::std::option::Option::None;
    }

    pub fn has_state(&self) -> bool {
        self.state.is_some()
    }

    // Param is passed by value, moved
    pub fn set_state(&mut self, v: JobGroupState) {
        self.state = ::std::option::Option::Some(v);
    }

    pub fn get_state(&self) -> JobGroupState {
        self.state.unwrap_or(JobGroupState::GroupPending)
    }

    fn get_state_for_reflect(&self) -> &::std::option::Option<JobGroupState> {
        &self.state
    }

    fn mut_state_for_reflect(&mut self) -> &mut ::std::option::Option<JobGroupState> {
        &mut self.state
    }

    // repeated .jobsrv.JobGroupProject projects = 3;

    pub fn clear_projects(&mut self) {
        self.projects.clear();
    }

    // Param is passed by value, moved
    pub fn set_projects(&mut self, v: ::protobuf::RepeatedField<JobGroupProject>) {
        self.projects = v;
    }

    // Mutable pointer to the field.
    pub fn mut_projects(&mut self) -> &mut ::protobuf::RepeatedField<JobGroupProject> {
        &mut self.projects
    }

    // Take field
    pub fn take_projects(&mut self) -> ::protobuf::RepeatedField<JobGroupProject> {
        ::std::mem::replace(&mut self.projects, ::protobuf::RepeatedField::new())
    }

    pub fn get_projects(&self) -> &[JobGroupProject] {
        &self.projects
    }

    fn get_projects_for_reflect(&self) -> &::protobuf::RepeatedField<JobGroupProject> {
        &self.projects
    }

    fn mut_projects_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<JobGroupProject> {
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

impl ::protobuf::Message for JobGroup {
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

impl ::protobuf::MessageStatic for JobGroup {
    fn new() -> JobGroup {
        JobGroup::new()
    }

    fn descriptor_static(_: ::std::option::Option<JobGroup>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "id",
                    JobGroup::get_id_for_reflect,
                    JobGroup::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<JobGroupState>>(
                    "state",
                    JobGroup::get_state_for_reflect,
                    JobGroup::mut_state_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<JobGroupProject>>(
                    "projects",
                    JobGroup::get_projects_for_reflect,
                    JobGroup::mut_projects_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "created_at",
                    JobGroup::get_created_at_for_reflect,
                    JobGroup::mut_created_at_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<JobGroup>(
                    "JobGroup",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for JobGroup {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_state();
        self.clear_projects();
        self.clear_created_at();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for JobGroup {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for JobGroup {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct JobGraphPackage {
    // message fields
    ident: ::protobuf::SingularField<::std::string::String>,
    deps: ::protobuf::RepeatedField<::std::string::String>,
    target: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for JobGraphPackage {}

impl JobGraphPackage {
    pub fn new() -> JobGraphPackage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static JobGraphPackage {
        static mut instance: ::protobuf::lazy::Lazy<JobGraphPackage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const JobGraphPackage,
        };
        unsafe {
            instance.get(JobGraphPackage::new)
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

impl ::protobuf::Message for JobGraphPackage {
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
        if let Some(ref v) = self.ident.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        for value in &self.deps {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        if let Some(ref v) = self.target.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
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

impl ::protobuf::MessageStatic for JobGraphPackage {
    fn new() -> JobGraphPackage {
        JobGraphPackage::new()
    }

    fn descriptor_static(_: ::std::option::Option<JobGraphPackage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "ident",
                    JobGraphPackage::get_ident_for_reflect,
                    JobGraphPackage::mut_ident_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "deps",
                    JobGraphPackage::get_deps_for_reflect,
                    JobGraphPackage::mut_deps_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "target",
                    JobGraphPackage::get_target_for_reflect,
                    JobGraphPackage::mut_target_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<JobGraphPackage>(
                    "JobGraphPackage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for JobGraphPackage {
    fn clear(&mut self) {
        self.clear_ident();
        self.clear_deps();
        self.clear_target();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for JobGraphPackage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for JobGraphPackage {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct JobGraphPackagePreCreate {
    // message fields
    ident: ::protobuf::SingularField<::std::string::String>,
    deps: ::protobuf::RepeatedField<::std::string::String>,
    target: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for JobGraphPackagePreCreate {}

impl JobGraphPackagePreCreate {
    pub fn new() -> JobGraphPackagePreCreate {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static JobGraphPackagePreCreate {
        static mut instance: ::protobuf::lazy::Lazy<JobGraphPackagePreCreate> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const JobGraphPackagePreCreate,
        };
        unsafe {
            instance.get(JobGraphPackagePreCreate::new)
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

impl ::protobuf::Message for JobGraphPackagePreCreate {
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
        if let Some(ref v) = self.ident.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        for value in &self.deps {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        if let Some(ref v) = self.target.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
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

impl ::protobuf::MessageStatic for JobGraphPackagePreCreate {
    fn new() -> JobGraphPackagePreCreate {
        JobGraphPackagePreCreate::new()
    }

    fn descriptor_static(_: ::std::option::Option<JobGraphPackagePreCreate>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "ident",
                    JobGraphPackagePreCreate::get_ident_for_reflect,
                    JobGraphPackagePreCreate::mut_ident_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "deps",
                    JobGraphPackagePreCreate::get_deps_for_reflect,
                    JobGraphPackagePreCreate::mut_deps_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "target",
                    JobGraphPackagePreCreate::get_target_for_reflect,
                    JobGraphPackagePreCreate::mut_target_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<JobGraphPackagePreCreate>(
                    "JobGraphPackagePreCreate",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for JobGraphPackagePreCreate {
    fn clear(&mut self) {
        self.clear_ident();
        self.clear_deps();
        self.clear_target();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for JobGraphPackagePreCreate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for JobGraphPackagePreCreate {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct JobGraphPackageCreate {
    // message fields
    ident: ::protobuf::SingularField<::std::string::String>,
    deps: ::protobuf::RepeatedField<::std::string::String>,
    target: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for JobGraphPackageCreate {}

impl JobGraphPackageCreate {
    pub fn new() -> JobGraphPackageCreate {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static JobGraphPackageCreate {
        static mut instance: ::protobuf::lazy::Lazy<JobGraphPackageCreate> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const JobGraphPackageCreate,
        };
        unsafe {
            instance.get(JobGraphPackageCreate::new)
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

impl ::protobuf::Message for JobGraphPackageCreate {
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
        if let Some(ref v) = self.ident.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        for value in &self.deps {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        if let Some(ref v) = self.target.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
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

impl ::protobuf::MessageStatic for JobGraphPackageCreate {
    fn new() -> JobGraphPackageCreate {
        JobGraphPackageCreate::new()
    }

    fn descriptor_static(_: ::std::option::Option<JobGraphPackageCreate>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "ident",
                    JobGraphPackageCreate::get_ident_for_reflect,
                    JobGraphPackageCreate::mut_ident_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "deps",
                    JobGraphPackageCreate::get_deps_for_reflect,
                    JobGraphPackageCreate::mut_deps_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "target",
                    JobGraphPackageCreate::get_target_for_reflect,
                    JobGraphPackageCreate::mut_target_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<JobGraphPackageCreate>(
                    "JobGraphPackageCreate",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for JobGraphPackageCreate {
    fn clear(&mut self) {
        self.clear_ident();
        self.clear_deps();
        self.clear_target();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for JobGraphPackageCreate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for JobGraphPackageCreate {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct JobGraphPackageReverseDependenciesGet {
    // message fields
    origin: ::protobuf::SingularField<::std::string::String>,
    name: ::protobuf::SingularField<::std::string::String>,
    target: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for JobGraphPackageReverseDependenciesGet {}

impl JobGraphPackageReverseDependenciesGet {
    pub fn new() -> JobGraphPackageReverseDependenciesGet {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static JobGraphPackageReverseDependenciesGet {
        static mut instance: ::protobuf::lazy::Lazy<JobGraphPackageReverseDependenciesGet> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const JobGraphPackageReverseDependenciesGet,
        };
        unsafe {
            instance.get(JobGraphPackageReverseDependenciesGet::new)
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

impl ::protobuf::Message for JobGraphPackageReverseDependenciesGet {
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
        if let Some(ref v) = self.origin.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(ref v) = self.target.as_ref() {
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
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(2, &v)?;
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

impl ::protobuf::MessageStatic for JobGraphPackageReverseDependenciesGet {
    fn new() -> JobGraphPackageReverseDependenciesGet {
        JobGraphPackageReverseDependenciesGet::new()
    }

    fn descriptor_static(_: ::std::option::Option<JobGraphPackageReverseDependenciesGet>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "origin",
                    JobGraphPackageReverseDependenciesGet::get_origin_for_reflect,
                    JobGraphPackageReverseDependenciesGet::mut_origin_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    JobGraphPackageReverseDependenciesGet::get_name_for_reflect,
                    JobGraphPackageReverseDependenciesGet::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "target",
                    JobGraphPackageReverseDependenciesGet::get_target_for_reflect,
                    JobGraphPackageReverseDependenciesGet::mut_target_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<JobGraphPackageReverseDependenciesGet>(
                    "JobGraphPackageReverseDependenciesGet",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for JobGraphPackageReverseDependenciesGet {
    fn clear(&mut self) {
        self.clear_origin();
        self.clear_name();
        self.clear_target();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for JobGraphPackageReverseDependenciesGet {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for JobGraphPackageReverseDependenciesGet {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct JobGraphPackageReverseDependencies {
    // message fields
    origin: ::protobuf::SingularField<::std::string::String>,
    name: ::protobuf::SingularField<::std::string::String>,
    rdeps: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for JobGraphPackageReverseDependencies {}

impl JobGraphPackageReverseDependencies {
    pub fn new() -> JobGraphPackageReverseDependencies {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static JobGraphPackageReverseDependencies {
        static mut instance: ::protobuf::lazy::Lazy<JobGraphPackageReverseDependencies> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const JobGraphPackageReverseDependencies,
        };
        unsafe {
            instance.get(JobGraphPackageReverseDependencies::new)
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

    // repeated string rdeps = 3;

    pub fn clear_rdeps(&mut self) {
        self.rdeps.clear();
    }

    // Param is passed by value, moved
    pub fn set_rdeps(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.rdeps = v;
    }

    // Mutable pointer to the field.
    pub fn mut_rdeps(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.rdeps
    }

    // Take field
    pub fn take_rdeps(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.rdeps, ::protobuf::RepeatedField::new())
    }

    pub fn get_rdeps(&self) -> &[::std::string::String] {
        &self.rdeps
    }

    fn get_rdeps_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.rdeps
    }

    fn mut_rdeps_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.rdeps
    }
}

impl ::protobuf::Message for JobGraphPackageReverseDependencies {
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
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.rdeps)?;
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
        for value in &self.rdeps {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
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
        for v in &self.rdeps {
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

impl ::protobuf::MessageStatic for JobGraphPackageReverseDependencies {
    fn new() -> JobGraphPackageReverseDependencies {
        JobGraphPackageReverseDependencies::new()
    }

    fn descriptor_static(_: ::std::option::Option<JobGraphPackageReverseDependencies>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "origin",
                    JobGraphPackageReverseDependencies::get_origin_for_reflect,
                    JobGraphPackageReverseDependencies::mut_origin_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    JobGraphPackageReverseDependencies::get_name_for_reflect,
                    JobGraphPackageReverseDependencies::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "rdeps",
                    JobGraphPackageReverseDependencies::get_rdeps_for_reflect,
                    JobGraphPackageReverseDependencies::mut_rdeps_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<JobGraphPackageReverseDependencies>(
                    "JobGraphPackageReverseDependencies",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for JobGraphPackageReverseDependencies {
    fn clear(&mut self) {
        self.clear_origin();
        self.clear_name();
        self.clear_rdeps();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for JobGraphPackageReverseDependencies {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for JobGraphPackageReverseDependencies {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct JobGraphPackageStatsGet {
    // message fields
    origin: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for JobGraphPackageStatsGet {}

impl JobGraphPackageStatsGet {
    pub fn new() -> JobGraphPackageStatsGet {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static JobGraphPackageStatsGet {
        static mut instance: ::protobuf::lazy::Lazy<JobGraphPackageStatsGet> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const JobGraphPackageStatsGet,
        };
        unsafe {
            instance.get(JobGraphPackageStatsGet::new)
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

impl ::protobuf::Message for JobGraphPackageStatsGet {
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

impl ::protobuf::MessageStatic for JobGraphPackageStatsGet {
    fn new() -> JobGraphPackageStatsGet {
        JobGraphPackageStatsGet::new()
    }

    fn descriptor_static(_: ::std::option::Option<JobGraphPackageStatsGet>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "origin",
                    JobGraphPackageStatsGet::get_origin_for_reflect,
                    JobGraphPackageStatsGet::mut_origin_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<JobGraphPackageStatsGet>(
                    "JobGraphPackageStatsGet",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for JobGraphPackageStatsGet {
    fn clear(&mut self) {
        self.clear_origin();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for JobGraphPackageStatsGet {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for JobGraphPackageStatsGet {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct JobGraphPackageStats {
    // message fields
    plans: ::std::option::Option<u64>,
    builds: ::std::option::Option<u64>,
    unique_packages: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for JobGraphPackageStats {}

impl JobGraphPackageStats {
    pub fn new() -> JobGraphPackageStats {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static JobGraphPackageStats {
        static mut instance: ::protobuf::lazy::Lazy<JobGraphPackageStats> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const JobGraphPackageStats,
        };
        unsafe {
            instance.get(JobGraphPackageStats::new)
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

    // optional uint64 unique_packages = 3;

    pub fn clear_unique_packages(&mut self) {
        self.unique_packages = ::std::option::Option::None;
    }

    pub fn has_unique_packages(&self) -> bool {
        self.unique_packages.is_some()
    }

    // Param is passed by value, moved
    pub fn set_unique_packages(&mut self, v: u64) {
        self.unique_packages = ::std::option::Option::Some(v);
    }

    pub fn get_unique_packages(&self) -> u64 {
        self.unique_packages.unwrap_or(0)
    }

    fn get_unique_packages_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.unique_packages
    }

    fn mut_unique_packages_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.unique_packages
    }
}

impl ::protobuf::Message for JobGraphPackageStats {
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
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.unique_packages = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.unique_packages {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
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
        if let Some(v) = self.unique_packages {
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

impl ::protobuf::MessageStatic for JobGraphPackageStats {
    fn new() -> JobGraphPackageStats {
        JobGraphPackageStats::new()
    }

    fn descriptor_static(_: ::std::option::Option<JobGraphPackageStats>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "plans",
                    JobGraphPackageStats::get_plans_for_reflect,
                    JobGraphPackageStats::mut_plans_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "builds",
                    JobGraphPackageStats::get_builds_for_reflect,
                    JobGraphPackageStats::mut_builds_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "unique_packages",
                    JobGraphPackageStats::get_unique_packages_for_reflect,
                    JobGraphPackageStats::mut_unique_packages_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<JobGraphPackageStats>(
                    "JobGraphPackageStats",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for JobGraphPackageStats {
    fn clear(&mut self) {
        self.clear_plans();
        self.clear_builds();
        self.clear_unique_packages();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for JobGraphPackageStats {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for JobGraphPackageStats {
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

    fn enum_descriptor_static(_: ::std::option::Option<Os>) -> &'static ::protobuf::reflect::EnumDescriptor {
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

    fn enum_descriptor_static(_: ::std::option::Option<WorkerState>) -> &'static ::protobuf::reflect::EnumDescriptor {
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

    fn enum_descriptor_static(_: ::std::option::Option<JobState>) -> &'static ::protobuf::reflect::EnumDescriptor {
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

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum JobGroupProjectState {
    NotStarted = 0,
    InProgress = 1,
    Success = 2,
    Failure = 3,
    Skipped = 4,
}

impl ::protobuf::ProtobufEnum for JobGroupProjectState {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<JobGroupProjectState> {
        match value {
            0 => ::std::option::Option::Some(JobGroupProjectState::NotStarted),
            1 => ::std::option::Option::Some(JobGroupProjectState::InProgress),
            2 => ::std::option::Option::Some(JobGroupProjectState::Success),
            3 => ::std::option::Option::Some(JobGroupProjectState::Failure),
            4 => ::std::option::Option::Some(JobGroupProjectState::Skipped),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [JobGroupProjectState] = &[
            JobGroupProjectState::NotStarted,
            JobGroupProjectState::InProgress,
            JobGroupProjectState::Success,
            JobGroupProjectState::Failure,
            JobGroupProjectState::Skipped,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<JobGroupProjectState>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("JobGroupProjectState", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for JobGroupProjectState {
}

impl ::protobuf::reflect::ProtobufValue for JobGroupProjectState {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum JobGroupState {
    GroupPending = 0,
    GroupDispatching = 1,
    GroupComplete = 2,
    GroupFailed = 3,
}

impl ::protobuf::ProtobufEnum for JobGroupState {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<JobGroupState> {
        match value {
            0 => ::std::option::Option::Some(JobGroupState::GroupPending),
            1 => ::std::option::Option::Some(JobGroupState::GroupDispatching),
            2 => ::std::option::Option::Some(JobGroupState::GroupComplete),
            3 => ::std::option::Option::Some(JobGroupState::GroupFailed),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [JobGroupState] = &[
            JobGroupState::GroupPending,
            JobGroupState::GroupDispatching,
            JobGroupState::GroupComplete,
            JobGroupState::GroupFailed,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<JobGroupState>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("JobGroupState", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for JobGroupState {
}

impl ::protobuf::reflect::ProtobufValue for JobGroupState {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x16protocols/jobsrv.proto\x12\x06jobsrv\x1a\x13protocols/net.proto\
    \x1a\x19protocols/originsrv.proto\"n\n\tHeartbeat\x12\x1a\n\x08endpoint\
    \x18\x01\x20\x01(\tR\x08endpoint\x12\x1a\n\x02os\x18\x02\x20\x01(\x0e2\n\
    .jobsrv.OsR\x02os\x12)\n\x05state\x18\x03\x20\x01(\x0e2\x13.jobsrv.Worke\
    rStateR\x05state\"[\n\nBusyWorker\x12\x14\n\x05ident\x18\x01\x20\x01(\tR\
    \x05ident\x12\x15\n\x06job_id\x18\x02\x20\x01(\x04R\x05jobId\x12\x20\n\
    \x0bquarantined\x18\x03\x20\x01(\x08R\x0bquarantined\"\xce\x04\n\x03Job\
    \x12\x0e\n\x02id\x18\x01\x20\x01(\x04R\x02id\x12\x19\n\x08owner_id\x18\
    \x02\x20\x01(\x04R\x07ownerId\x12&\n\x05state\x18\x03\x20\x01(\x0e2\x10.\
    jobsrv.JobStateR\x05state\x122\n\x07project\x18\x04\x20\x01(\x0b2\x18.or\
    iginsrv.OriginProjectR\x07project\x12#\n\x05error\x18\x05\x20\x01(\x0b2\
    \r.net.NetErrorR\x05error\x12\x1d\n\ncreated_at\x18\x06\x20\x01(\tR\tcre\
    atedAt\x12(\n\x10build_started_at\x18\x07\x20\x01(\tR\x0ebuildStartedAt\
    \x12*\n\x11build_finished_at\x18\x08\x20\x01(\tR\x0fbuildFinishedAt\x12B\
    \n\rpackage_ident\x18\t\x20\x01(\x0b2\x1d.originsrv.OriginPackageIdentR\
    \x0cpackageIdent\x12\x1f\n\x0bis_archived\x18\x0b\x20\x01(\x08R\nisArchi\
    ved\x12@\n\x0cintegrations\x18\x0c\x20\x03(\x0b2\x1c.originsrv.OriginInt\
    egrationR\x0cintegrations\x12\x18\n\x07channel\x18\r\x20\x01(\tR\x07chan\
    nel\x12V\n\x14project_integrations\x18\x0e\x20\x03(\x0b2#.originsrv.Orig\
    inProjectIntegrationR\x13projectIntegrationsJ\x04\x08\n\x10\x0bR\x07log_\
    url\"\x18\n\x06JobGet\x12\x0e\n\x02id\x18\x01\x20\x01(\x04R\x02id\"r\n\
    \x07JobSpec\x12\x19\n\x08owner_id\x18\x01\x20\x01(\x04R\x07ownerId\x122\
    \n\x07project\x18\x02\x20\x01(\x0b2\x18.originsrv.OriginProjectR\x07proj\
    ect\x12\x18\n\x07channel\x18\x03\x20\x01(\tR\x07channel\"N\n\x0eProjectJ\
    obsGet\x12\x12\n\x04name\x18\x01\x20\x01(\tR\x04name\x12\x14\n\x05start\
    \x18\x02\x20\x01(\x04R\x05start\x12\x12\n\x04stop\x18\x03\x20\x01(\x04R\
    \x04stop\"y\n\x16ProjectJobsGetResponse\x12\x1f\n\x04jobs\x18\x01\x20\
    \x03(\x0b2\x0b.jobsrv.JobR\x04jobs\x12\x14\n\x05start\x18\x02\x20\x01(\
    \x04R\x05start\x12\x12\n\x04stop\x18\x03\x20\x01(\x04R\x04stop\x12\x14\n\
    \x05count\x18\x04\x20\x01(\x04R\x05count\"P\n\x0bJobLogChunk\x12\x15\n\
    \x06job_id\x18\x01\x20\x01(\x04R\x05jobId\x12\x10\n\x03seq\x18\x02\x20\
    \x01(\x04R\x03seq\x12\x18\n\x07content\x18\x03\x20\x01(\tR\x07content\"'\
    \n\x0eJobLogComplete\x12\x15\n\x06job_id\x18\x01\x20\x01(\x04R\x05jobId\
    \"1\n\tJobLogGet\x12\x0e\n\x02id\x18\x01\x20\x01(\x04R\x02id\x12\x14\n\
    \x05start\x18\x02\x20\x01(\x04R\x05start\"m\n\x06JobLog\x12\x14\n\x05sta\
    rt\x18\x01\x20\x01(\x04R\x05start\x12\x12\n\x04stop\x18\x02\x20\x01(\x04\
    R\x04stop\x12\x18\n\x07content\x18\x03\x20\x03(\tR\x07content\x12\x1f\n\
    \x0bis_complete\x18\x04\x20\x01(\x08R\nisComplete\"\xb9\x01\n\x0cJobGrou\
    pSpec\x12\x16\n\x06origin\x18\x01\x20\x01(\tR\x06origin\x12\x18\n\x07pac\
    kage\x18\x02\x20\x01(\tR\x07package\x12\x1b\n\tdeps_only\x18\x03\x20\x01\
    (\x08R\x08depsOnly\x12\x16\n\x06target\x18\x04\x20\x01(\tR\x06target\x12\
    \x1f\n\x0borigin_only\x18\x05\x20\x01(\x08R\noriginOnly\x12!\n\x0cpackag\
    e_only\x18\x06\x20\x01(\x08R\x0bpackageOnly\"\x9e\x01\n\x0fJobGroupProje\
    ct\x12\x12\n\x04name\x18\x01\x20\x01(\tR\x04name\x12\x14\n\x05ident\x18\
    \x02\x20\x01(\tR\x05ident\x122\n\x05state\x18\x03\x20\x01(\x0e2\x1c.jobs\
    rv.JobGroupProjectStateR\x05state\x12\x15\n\x06job_id\x18\x04\x20\x01(\
    \x04R\x05jobId\x12\x16\n\x06target\x18\x05\x20\x01(\tR\x06target\"*\n\rJ\
    obGroupAbort\x12\x19\n\x08group_id\x18\x01\x20\x01(\x04R\x07groupId\"(\n\
    \x0bJobGroupGet\x12\x19\n\x08group_id\x18\x01\x20\x01(\x04R\x07groupId\"\
    \x9b\x01\n\x08JobGroup\x12\x0e\n\x02id\x18\x01\x20\x01(\x04R\x02id\x12+\
    \n\x05state\x18\x02\x20\x01(\x0e2\x15.jobsrv.JobGroupStateR\x05state\x12\
    3\n\x08projects\x18\x03\x20\x03(\x0b2\x17.jobsrv.JobGroupProjectR\x08pro\
    jects\x12\x1d\n\ncreated_at\x18\x04\x20\x01(\tR\tcreatedAt\"S\n\x0fJobGr\
    aphPackage\x12\x14\n\x05ident\x18\x01\x20\x01(\tR\x05ident\x12\x12\n\x04\
    deps\x18\x02\x20\x03(\tR\x04deps\x12\x16\n\x06target\x18\x03\x20\x01(\tR\
    \x06target\"\\\n\x18JobGraphPackagePreCreate\x12\x14\n\x05ident\x18\x01\
    \x20\x01(\tR\x05ident\x12\x12\n\x04deps\x18\x02\x20\x03(\tR\x04deps\x12\
    \x16\n\x06target\x18\x03\x20\x01(\tR\x06target\"Y\n\x15JobGraphPackageCr\
    eate\x12\x14\n\x05ident\x18\x01\x20\x01(\tR\x05ident\x12\x12\n\x04deps\
    \x18\x02\x20\x03(\tR\x04deps\x12\x16\n\x06target\x18\x03\x20\x01(\tR\x06\
    target\"k\n%JobGraphPackageReverseDependenciesGet\x12\x16\n\x06origin\
    \x18\x01\x20\x01(\tR\x06origin\x12\x12\n\x04name\x18\x02\x20\x01(\tR\x04\
    name\x12\x16\n\x06target\x18\x03\x20\x01(\tR\x06target\"f\n\"JobGraphPac\
    kageReverseDependencies\x12\x16\n\x06origin\x18\x01\x20\x01(\tR\x06origi\
    n\x12\x12\n\x04name\x18\x02\x20\x01(\tR\x04name\x12\x14\n\x05rdeps\x18\
    \x03\x20\x03(\tR\x05rdeps\"1\n\x17JobGraphPackageStatsGet\x12\x16\n\x06o\
    rigin\x18\x01\x20\x01(\tR\x06origin\"m\n\x14JobGraphPackageStats\x12\x14\
    \n\x05plans\x18\x01\x20\x01(\x04R\x05plans\x12\x16\n\x06builds\x18\x02\
    \x20\x01(\x04R\x06builds\x12'\n\x0funique_packages\x18\x03\x20\x01(\x04R\
    \x0euniquePackages*(\n\x02Os\x12\t\n\x05Linux\x10\x01\x12\n\n\x06Darwin\
    \x10\x02\x12\x0b\n\x07Windows\x10\x03*\"\n\x0bWorkerState\x12\t\n\x05Rea\
    dy\x10\0\x12\x08\n\x04Busy\x10\x01*_\n\x08JobState\x12\x0b\n\x07Pending\
    \x10\0\x12\x0e\n\nProcessing\x10\x01\x12\x0c\n\x08Complete\x10\x02\x12\
    \x0c\n\x08Rejected\x10\x03\x12\n\n\x06Failed\x10\x04\x12\x0e\n\nDispatch\
    ed\x10\x05*]\n\x14JobGroupProjectState\x12\x0e\n\nNotStarted\x10\0\x12\
    \x0e\n\nInProgress\x10\x01\x12\x0b\n\x07Success\x10\x02\x12\x0b\n\x07Fai\
    lure\x10\x03\x12\x0b\n\x07Skipped\x10\x04*[\n\rJobGroupState\x12\x10\n\
    \x0cGroupPending\x10\0\x12\x14\n\x10GroupDispatching\x10\x01\x12\x11\n\r\
    GroupComplete\x10\x02\x12\x0f\n\x0bGroupFailed\x10\x03J\xd68\n\x07\x12\
    \x05\0\0\xb9\x01\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\x08\n\x01\x02\x12\
    \x03\x01\x08\x0e\n\t\n\x02\x03\0\x12\x03\x02\x07\x1c\n\t\n\x02\x03\x01\
    \x12\x03\x03\x07\"\n\n\n\x02\x05\0\x12\x04\x05\0\t\x01\n\n\n\x03\x05\0\
    \x01\x12\x03\x05\x05\x07\n\x0b\n\x04\x05\0\x02\0\x12\x03\x06\x02\x0c\n\
    \x0c\n\x05\x05\0\x02\0\x01\x12\x03\x06\x02\x07\n\x0c\n\x05\x05\0\x02\0\
    \x02\x12\x03\x06\n\x0b\n\x0b\n\x04\x05\0\x02\x01\x12\x03\x07\x02\r\n\x0c\
    \n\x05\x05\0\x02\x01\x01\x12\x03\x07\x02\x08\n\x0c\n\x05\x05\0\x02\x01\
    \x02\x12\x03\x07\x0b\x0c\n\x0b\n\x04\x05\0\x02\x02\x12\x03\x08\x02\x0e\n\
    \x0c\n\x05\x05\0\x02\x02\x01\x12\x03\x08\x02\t\n\x0c\n\x05\x05\0\x02\x02\
    \x02\x12\x03\x08\x0c\r\n\n\n\x02\x05\x01\x12\x04\x0b\0\x0e\x01\n\n\n\x03\
    \x05\x01\x01\x12\x03\x0b\x05\x10\n\x0b\n\x04\x05\x01\x02\0\x12\x03\x0c\
    \x02\x0c\n\x0c\n\x05\x05\x01\x02\0\x01\x12\x03\x0c\x02\x07\n\x0c\n\x05\
    \x05\x01\x02\0\x02\x12\x03\x0c\n\x0b\n\x0b\n\x04\x05\x01\x02\x01\x12\x03\
    \r\x02\x0b\n\x0c\n\x05\x05\x01\x02\x01\x01\x12\x03\r\x02\x06\n\x0c\n\x05\
    \x05\x01\x02\x01\x02\x12\x03\r\t\n\n\n\n\x02\x05\x02\x12\x04\x10\0\x17\
    \x01\n\n\n\x03\x05\x02\x01\x12\x03\x10\x05\r\n\x0b\n\x04\x05\x02\x02\0\
    \x12\x03\x11\x02\x0e\n\x0c\n\x05\x05\x02\x02\0\x01\x12\x03\x11\x02\t\n\
    \x0c\n\x05\x05\x02\x02\0\x02\x12\x03\x11\x0c\r\n\x0b\n\x04\x05\x02\x02\
    \x01\x12\x03\x12\x02\x11\n\x0c\n\x05\x05\x02\x02\x01\x01\x12\x03\x12\x02\
    \x0c\n\x0c\n\x05\x05\x02\x02\x01\x02\x12\x03\x12\x0f\x10\n\x0b\n\x04\x05\
    \x02\x02\x02\x12\x03\x13\x02\x0f\n\x0c\n\x05\x05\x02\x02\x02\x01\x12\x03\
    \x13\x02\n\n\x0c\n\x05\x05\x02\x02\x02\x02\x12\x03\x13\r\x0e\n\x0b\n\x04\
    \x05\x02\x02\x03\x12\x03\x14\x02\x0f\n\x0c\n\x05\x05\x02\x02\x03\x01\x12\
    \x03\x14\x02\n\n\x0c\n\x05\x05\x02\x02\x03\x02\x12\x03\x14\r\x0e\n\x0b\n\
    \x04\x05\x02\x02\x04\x12\x03\x15\x02\r\n\x0c\n\x05\x05\x02\x02\x04\x01\
    \x12\x03\x15\x02\x08\n\x0c\n\x05\x05\x02\x02\x04\x02\x12\x03\x15\x0b\x0c\
    \n\x0b\n\x04\x05\x02\x02\x05\x12\x03\x16\x02\x11\n\x0c\n\x05\x05\x02\x02\
    \x05\x01\x12\x03\x16\x02\x0c\n\x0c\n\x05\x05\x02\x02\x05\x02\x12\x03\x16\
    \x0f\x10\n\n\n\x02\x04\0\x12\x04\x19\0\x1d\x01\n\n\n\x03\x04\0\x01\x12\
    \x03\x19\x08\x11\n\x0b\n\x04\x04\0\x02\0\x12\x03\x1a\x02\x1f\n\x0c\n\x05\
    \x04\0\x02\0\x04\x12\x03\x1a\x02\n\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\
    \x1a\x0b\x11\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x1a\x12\x1a\n\x0c\n\x05\
    \x04\0\x02\0\x03\x12\x03\x1a\x1d\x1e\n\x0b\n\x04\x04\0\x02\x01\x12\x03\
    \x1b\x02\x15\n\x0c\n\x05\x04\0\x02\x01\x04\x12\x03\x1b\x02\n\n\x0c\n\x05\
    \x04\0\x02\x01\x06\x12\x03\x1b\x0b\r\n\x0c\n\x05\x04\0\x02\x01\x01\x12\
    \x03\x1b\x0e\x10\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x1b\x13\x14\n\x0b\
    \n\x04\x04\0\x02\x02\x12\x03\x1c\x02!\n\x0c\n\x05\x04\0\x02\x02\x04\x12\
    \x03\x1c\x02\n\n\x0c\n\x05\x04\0\x02\x02\x06\x12\x03\x1c\x0b\x16\n\x0c\n\
    \x05\x04\0\x02\x02\x01\x12\x03\x1c\x17\x1c\n\x0c\n\x05\x04\0\x02\x02\x03\
    \x12\x03\x1c\x1f\x20\n\n\n\x02\x04\x01\x12\x04\x1f\0#\x01\n\n\n\x03\x04\
    \x01\x01\x12\x03\x1f\x08\x12\n\x0b\n\x04\x04\x01\x02\0\x12\x03\x20\x02\
    \x1c\n\x0c\n\x05\x04\x01\x02\0\x04\x12\x03\x20\x02\n\n\x0c\n\x05\x04\x01\
    \x02\0\x05\x12\x03\x20\x0b\x11\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\x20\
    \x12\x17\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03\x20\x1a\x1b\n\x0b\n\x04\
    \x04\x01\x02\x01\x12\x03!\x02\x1d\n\x0c\n\x05\x04\x01\x02\x01\x04\x12\
    \x03!\x02\n\n\x0c\n\x05\x04\x01\x02\x01\x05\x12\x03!\x0b\x11\n\x0c\n\x05\
    \x04\x01\x02\x01\x01\x12\x03!\x12\x18\n\x0c\n\x05\x04\x01\x02\x01\x03\
    \x12\x03!\x1b\x1c\n\x0b\n\x04\x04\x01\x02\x02\x12\x03\"\x02\x20\n\x0c\n\
    \x05\x04\x01\x02\x02\x04\x12\x03\"\x02\n\n\x0c\n\x05\x04\x01\x02\x02\x05\
    \x12\x03\"\x0b\x0f\n\x0c\n\x05\x04\x01\x02\x02\x01\x12\x03\"\x10\x1b\n\
    \x0c\n\x05\x04\x01\x02\x02\x03\x12\x03\"\x1e\x1f\n\n\n\x02\x04\x02\x12\
    \x04%\05\x01\n\n\n\x03\x04\x02\x01\x12\x03%\x08\x0b\n\n\n\x03\x04\x02\t\
    \x12\x03&\x0b\x0e\n\x0b\n\x04\x04\x02\t\0\x12\x03&\x0b\r\n\x0c\n\x05\x04\
    \x02\t\0\x01\x12\x03&\x0b\r\n\x0c\n\x05\x04\x02\t\0\x02\x12\x03&\x0b\r\n\
    \n\n\x03\x04\x02\n\x12\x03'\x0b\x15\n\x0b\n\x04\x04\x02\n\0\x12\x03'\x0b\
    \x14\n\x0b\n\x04\x04\x02\x02\0\x12\x03(\x02\x19\n\x0c\n\x05\x04\x02\x02\
    \0\x04\x12\x03(\x02\n\n\x0c\n\x05\x04\x02\x02\0\x05\x12\x03(\x0b\x11\n\
    \x0c\n\x05\x04\x02\x02\0\x01\x12\x03(\x12\x14\n\x0c\n\x05\x04\x02\x02\0\
    \x03\x12\x03(\x17\x18\n\x0b\n\x04\x04\x02\x02\x01\x12\x03)\x02\x1f\n\x0c\
    \n\x05\x04\x02\x02\x01\x04\x12\x03)\x02\n\n\x0c\n\x05\x04\x02\x02\x01\
    \x05\x12\x03)\x0b\x11\n\x0c\n\x05\x04\x02\x02\x01\x01\x12\x03)\x12\x1a\n\
    \x0c\n\x05\x04\x02\x02\x01\x03\x12\x03)\x1d\x1e\n\x0b\n\x04\x04\x02\x02\
    \x02\x12\x03*\x02\x1e\n\x0c\n\x05\x04\x02\x02\x02\x04\x12\x03*\x02\n\n\
    \x0c\n\x05\x04\x02\x02\x02\x06\x12\x03*\x0b\x13\n\x0c\n\x05\x04\x02\x02\
    \x02\x01\x12\x03*\x14\x19\n\x0c\n\x05\x04\x02\x02\x02\x03\x12\x03*\x1c\
    \x1d\n\x0b\n\x04\x04\x02\x02\x03\x12\x03+\x02/\n\x0c\n\x05\x04\x02\x02\
    \x03\x04\x12\x03+\x02\n\n\x0c\n\x05\x04\x02\x02\x03\x06\x12\x03+\x0b\"\n\
    \x0c\n\x05\x04\x02\x02\x03\x01\x12\x03+#*\n\x0c\n\x05\x04\x02\x02\x03\
    \x03\x12\x03+-.\n\x0b\n\x04\x04\x02\x02\x04\x12\x03,\x02\"\n\x0c\n\x05\
    \x04\x02\x02\x04\x04\x12\x03,\x02\n\n\x0c\n\x05\x04\x02\x02\x04\x06\x12\
    \x03,\x0b\x17\n\x0c\n\x05\x04\x02\x02\x04\x01\x12\x03,\x18\x1d\n\x0c\n\
    \x05\x04\x02\x02\x04\x03\x12\x03,\x20!\n%\n\x04\x04\x02\x02\x05\x12\x03-\
    \x02!\"\x18\x20RFC3339-formatted\x20time\n\n\x0c\n\x05\x04\x02\x02\x05\
    \x04\x12\x03-\x02\n\n\x0c\n\x05\x04\x02\x02\x05\x05\x12\x03-\x0b\x11\n\
    \x0c\n\x05\x04\x02\x02\x05\x01\x12\x03-\x12\x1c\n\x0c\n\x05\x04\x02\x02\
    \x05\x03\x12\x03-\x1f\x20\n%\n\x04\x04\x02\x02\x06\x12\x03.\x02'\"\x18\
    \x20RFC3339-formatted\x20time\n\n\x0c\n\x05\x04\x02\x02\x06\x04\x12\x03.\
    \x02\n\n\x0c\n\x05\x04\x02\x02\x06\x05\x12\x03.\x0b\x11\n\x0c\n\x05\x04\
    \x02\x02\x06\x01\x12\x03.\x12\"\n\x0c\n\x05\x04\x02\x02\x06\x03\x12\x03.\
    %&\n\x0b\n\x04\x04\x02\x02\x07\x12\x03/\x02(\n\x0c\n\x05\x04\x02\x02\x07\
    \x04\x12\x03/\x02\n\n\x0c\n\x05\x04\x02\x02\x07\x05\x12\x03/\x0b\x11\n\
    \x0c\n\x05\x04\x02\x02\x07\x01\x12\x03/\x12#\n\x0c\n\x05\x04\x02\x02\x07\
    \x03\x12\x03/&'\n\x0b\n\x04\x04\x02\x02\x08\x12\x030\x02:\n\x0c\n\x05\
    \x04\x02\x02\x08\x04\x12\x030\x02\n\n\x0c\n\x05\x04\x02\x02\x08\x06\x12\
    \x030\x0b'\n\x0c\n\x05\x04\x02\x02\x08\x01\x12\x030(5\n\x0c\n\x05\x04\
    \x02\x02\x08\x03\x12\x03089\n\x0b\n\x04\x04\x02\x02\t\x12\x031\x02!\n\
    \x0c\n\x05\x04\x02\x02\t\x04\x12\x031\x02\n\n\x0c\n\x05\x04\x02\x02\t\
    \x05\x12\x031\x0b\x0f\n\x0c\n\x05\x04\x02\x02\t\x01\x12\x031\x10\x1b\n\
    \x0c\n\x05\x04\x02\x02\t\x03\x12\x031\x1e\x20\n\x0b\n\x04\x04\x02\x02\n\
    \x12\x032\x029\n\x0c\n\x05\x04\x02\x02\n\x04\x12\x032\x02\n\n\x0c\n\x05\
    \x04\x02\x02\n\x06\x12\x032\x0b&\n\x0c\n\x05\x04\x02\x02\n\x01\x12\x032'\
    3\n\x0c\n\x05\x04\x02\x02\n\x03\x12\x03268\n\x0b\n\x04\x04\x02\x02\x0b\
    \x12\x033\x02\x1f\n\x0c\n\x05\x04\x02\x02\x0b\x04\x12\x033\x02\n\n\x0c\n\
    \x05\x04\x02\x02\x0b\x05\x12\x033\x0b\x11\n\x0c\n\x05\x04\x02\x02\x0b\
    \x01\x12\x033\x12\x19\n\x0c\n\x05\x04\x02\x02\x0b\x03\x12\x033\x1c\x1e\n\
    \x0b\n\x04\x04\x02\x02\x0c\x12\x034\x02H\n\x0c\n\x05\x04\x02\x02\x0c\x04\
    \x12\x034\x02\n\n\x0c\n\x05\x04\x02\x02\x0c\x06\x12\x034\x0b-\n\x0c\n\
    \x05\x04\x02\x02\x0c\x01\x12\x034.B\n\x0c\n\x05\x04\x02\x02\x0c\x03\x12\
    \x034EG\n\n\n\x02\x04\x03\x12\x047\09\x01\n\n\n\x03\x04\x03\x01\x12\x037\
    \x08\x0e\n\x0b\n\x04\x04\x03\x02\0\x12\x038\x02\x19\n\x0c\n\x05\x04\x03\
    \x02\0\x04\x12\x038\x02\n\n\x0c\n\x05\x04\x03\x02\0\x05\x12\x038\x0b\x11\
    \n\x0c\n\x05\x04\x03\x02\0\x01\x12\x038\x12\x14\n\x0c\n\x05\x04\x03\x02\
    \0\x03\x12\x038\x17\x18\n\n\n\x02\x04\x04\x12\x04;\0?\x01\n\n\n\x03\x04\
    \x04\x01\x12\x03;\x08\x0f\n\x0b\n\x04\x04\x04\x02\0\x12\x03<\x02\x1f\n\
    \x0c\n\x05\x04\x04\x02\0\x04\x12\x03<\x02\n\n\x0c\n\x05\x04\x04\x02\0\
    \x05\x12\x03<\x0b\x11\n\x0c\n\x05\x04\x04\x02\0\x01\x12\x03<\x12\x1a\n\
    \x0c\n\x05\x04\x04\x02\0\x03\x12\x03<\x1d\x1e\n\x0b\n\x04\x04\x04\x02\
    \x01\x12\x03=\x02/\n\x0c\n\x05\x04\x04\x02\x01\x04\x12\x03=\x02\n\n\x0c\
    \n\x05\x04\x04\x02\x01\x06\x12\x03=\x0b\"\n\x0c\n\x05\x04\x04\x02\x01\
    \x01\x12\x03=#*\n\x0c\n\x05\x04\x04\x02\x01\x03\x12\x03=-.\n\x0b\n\x04\
    \x04\x04\x02\x02\x12\x03>\x02\x1e\n\x0c\n\x05\x04\x04\x02\x02\x04\x12\
    \x03>\x02\n\n\x0c\n\x05\x04\x04\x02\x02\x05\x12\x03>\x0b\x11\n\x0c\n\x05\
    \x04\x04\x02\x02\x01\x12\x03>\x12\x19\n\x0c\n\x05\x04\x04\x02\x02\x03\
    \x12\x03>\x1c\x1d\n\n\n\x02\x04\x05\x12\x04A\0E\x01\n\n\n\x03\x04\x05\
    \x01\x12\x03A\x08\x16\n\x0b\n\x04\x04\x05\x02\0\x12\x03B\x02\x1b\n\x0c\n\
    \x05\x04\x05\x02\0\x04\x12\x03B\x02\n\n\x0c\n\x05\x04\x05\x02\0\x05\x12\
    \x03B\x0b\x11\n\x0c\n\x05\x04\x05\x02\0\x01\x12\x03B\x12\x16\n\x0c\n\x05\
    \x04\x05\x02\0\x03\x12\x03B\x19\x1a\n\x0b\n\x04\x04\x05\x02\x01\x12\x03C\
    \x02\x1c\n\x0c\n\x05\x04\x05\x02\x01\x04\x12\x03C\x02\n\n\x0c\n\x05\x04\
    \x05\x02\x01\x05\x12\x03C\x0b\x11\n\x0c\n\x05\x04\x05\x02\x01\x01\x12\
    \x03C\x12\x17\n\x0c\n\x05\x04\x05\x02\x01\x03\x12\x03C\x1a\x1b\n\x0b\n\
    \x04\x04\x05\x02\x02\x12\x03D\x02\x1b\n\x0c\n\x05\x04\x05\x02\x02\x04\
    \x12\x03D\x02\n\n\x0c\n\x05\x04\x05\x02\x02\x05\x12\x03D\x0b\x11\n\x0c\n\
    \x05\x04\x05\x02\x02\x01\x12\x03D\x12\x16\n\x0c\n\x05\x04\x05\x02\x02\
    \x03\x12\x03D\x19\x1a\n\n\n\x02\x04\x06\x12\x04G\0L\x01\n\n\n\x03\x04\
    \x06\x01\x12\x03G\x08\x1e\n\x0b\n\x04\x04\x06\x02\0\x12\x03H\x02\x18\n\
    \x0c\n\x05\x04\x06\x02\0\x04\x12\x03H\x02\n\n\x0c\n\x05\x04\x06\x02\0\
    \x06\x12\x03H\x0b\x0e\n\x0c\n\x05\x04\x06\x02\0\x01\x12\x03H\x0f\x13\n\
    \x0c\n\x05\x04\x06\x02\0\x03\x12\x03H\x16\x17\n\x0b\n\x04\x04\x06\x02\
    \x01\x12\x03I\x02\x1c\n\x0c\n\x05\x04\x06\x02\x01\x04\x12\x03I\x02\n\n\
    \x0c\n\x05\x04\x06\x02\x01\x05\x12\x03I\x0b\x11\n\x0c\n\x05\x04\x06\x02\
    \x01\x01\x12\x03I\x12\x17\n\x0c\n\x05\x04\x06\x02\x01\x03\x12\x03I\x1a\
    \x1b\n\x0b\n\x04\x04\x06\x02\x02\x12\x03J\x02\x1b\n\x0c\n\x05\x04\x06\
    \x02\x02\x04\x12\x03J\x02\n\n\x0c\n\x05\x04\x06\x02\x02\x05\x12\x03J\x0b\
    \x11\n\x0c\n\x05\x04\x06\x02\x02\x01\x12\x03J\x12\x16\n\x0c\n\x05\x04\
    \x06\x02\x02\x03\x12\x03J\x19\x1a\n\x0b\n\x04\x04\x06\x02\x03\x12\x03K\
    \x02\x1c\n\x0c\n\x05\x04\x06\x02\x03\x04\x12\x03K\x02\n\n\x0c\n\x05\x04\
    \x06\x02\x03\x05\x12\x03K\x0b\x11\n\x0c\n\x05\x04\x06\x02\x03\x01\x12\
    \x03K\x12\x17\n\x0c\n\x05\x04\x06\x02\x03\x03\x12\x03K\x1a\x1b\n\n\n\x02\
    \x04\x07\x12\x04N\0R\x01\n\n\n\x03\x04\x07\x01\x12\x03N\x08\x13\n\x0b\n\
    \x04\x04\x07\x02\0\x12\x03O\x02\x1d\n\x0c\n\x05\x04\x07\x02\0\x04\x12\
    \x03O\x02\n\n\x0c\n\x05\x04\x07\x02\0\x05\x12\x03O\x0b\x11\n\x0c\n\x05\
    \x04\x07\x02\0\x01\x12\x03O\x12\x18\n\x0c\n\x05\x04\x07\x02\0\x03\x12\
    \x03O\x1b\x1c\n+\n\x04\x04\x07\x02\x01\x12\x03P\x02\x1a\"\x1e\x20Chunk\
    \x20ordering\x20(line\x20number)\n\n\x0c\n\x05\x04\x07\x02\x01\x04\x12\
    \x03P\x02\n\n\x0c\n\x05\x04\x07\x02\x01\x05\x12\x03P\x0b\x11\n\x0c\n\x05\
    \x04\x07\x02\x01\x01\x12\x03P\x12\x15\n\x0c\n\x05\x04\x07\x02\x01\x03\
    \x12\x03P\x18\x19\n5\n\x04\x04\x07\x02\x02\x12\x03Q\x02\x1e\"(\x20Log\
    \x20content\x20(TODO:\x20Make\x20repeatedfield)\n\n\x0c\n\x05\x04\x07\
    \x02\x02\x04\x12\x03Q\x02\n\n\x0c\n\x05\x04\x07\x02\x02\x05\x12\x03Q\x0b\
    \x11\n\x0c\n\x05\x04\x07\x02\x02\x01\x12\x03Q\x12\x19\n\x0c\n\x05\x04\
    \x07\x02\x02\x03\x12\x03Q\x1c\x1d\n\n\n\x02\x04\x08\x12\x04T\0V\x01\n\n\
    \n\x03\x04\x08\x01\x12\x03T\x08\x16\n\x0b\n\x04\x04\x08\x02\0\x12\x03U\
    \x02\x1d\n\x0c\n\x05\x04\x08\x02\0\x04\x12\x03U\x02\n\n\x0c\n\x05\x04\
    \x08\x02\0\x05\x12\x03U\x0b\x11\n\x0c\n\x05\x04\x08\x02\0\x01\x12\x03U\
    \x12\x18\n\x0c\n\x05\x04\x08\x02\0\x03\x12\x03U\x1b\x1c\n\n\n\x02\x04\t\
    \x12\x04X\0[\x01\n\n\n\x03\x04\t\x01\x12\x03X\x08\x11\n\x0b\n\x04\x04\t\
    \x02\0\x12\x03Y\x02\x19\n\x0c\n\x05\x04\t\x02\0\x04\x12\x03Y\x02\n\n\x0c\
    \n\x05\x04\t\x02\0\x05\x12\x03Y\x0b\x11\n\x0c\n\x05\x04\t\x02\0\x01\x12\
    \x03Y\x12\x14\n\x0c\n\x05\x04\t\x02\0\x03\x12\x03Y\x17\x18\n.\n\x04\x04\
    \t\x02\x01\x12\x03Z\x02\x1c\"!\x20Zero-indexed\x20line\x20of\x20log\x20o\
    utput\n\n\x0c\n\x05\x04\t\x02\x01\x04\x12\x03Z\x02\n\n\x0c\n\x05\x04\t\
    \x02\x01\x05\x12\x03Z\x0b\x11\n\x0c\n\x05\x04\t\x02\x01\x01\x12\x03Z\x12\
    \x17\n\x0c\n\x05\x04\t\x02\x01\x03\x12\x03Z\x1a\x1b\n\n\n\x02\x04\n\x12\
    \x04]\0b\x01\n\n\n\x03\x04\n\x01\x12\x03]\x08\x0e\n,\n\x04\x04\n\x02\0\
    \x12\x03^\x02\x1c\"\x1f\x20Zero-indexed\x20(inclusive)\x20line\n\n\x0c\n\
    \x05\x04\n\x02\0\x04\x12\x03^\x02\n\n\x0c\n\x05\x04\n\x02\0\x05\x12\x03^\
    \x0b\x11\n\x0c\n\x05\x04\n\x02\0\x01\x12\x03^\x12\x17\n\x0c\n\x05\x04\n\
    \x02\0\x03\x12\x03^\x1a\x1b\n,\n\x04\x04\n\x02\x01\x12\x03_\x02\x1b\"\
    \x1f\x20Zero-indexed\x20(exclusive)\x20line\n\n\x0c\n\x05\x04\n\x02\x01\
    \x04\x12\x03_\x02\n\n\x0c\n\x05\x04\n\x02\x01\x05\x12\x03_\x0b\x11\n\x0c\
    \n\x05\x04\n\x02\x01\x01\x12\x03_\x12\x16\n\x0c\n\x05\x04\n\x02\x01\x03\
    \x12\x03_\x19\x1a\n\x0b\n\x04\x04\n\x02\x02\x12\x03`\x02\x1e\n\x0c\n\x05\
    \x04\n\x02\x02\x04\x12\x03`\x02\n\n\x0c\n\x05\x04\n\x02\x02\x05\x12\x03`\
    \x0b\x11\n\x0c\n\x05\x04\n\x02\x02\x01\x12\x03`\x12\x19\n\x0c\n\x05\x04\
    \n\x02\x02\x03\x12\x03`\x1c\x1d\n\x0b\n\x04\x04\n\x02\x03\x12\x03a\x02\
    \x20\n\x0c\n\x05\x04\n\x02\x03\x04\x12\x03a\x02\n\n\x0c\n\x05\x04\n\x02\
    \x03\x05\x12\x03a\x0b\x0f\n\x0c\n\x05\x04\n\x02\x03\x01\x12\x03a\x10\x1b\
    \n\x0c\n\x05\x04\n\x02\x03\x03\x12\x03a\x1e\x1f\n\n\n\x02\x04\x0b\x12\
    \x04d\0k\x01\n\n\n\x03\x04\x0b\x01\x12\x03d\x08\x14\n\x0b\n\x04\x04\x0b\
    \x02\0\x12\x03e\x02\x1d\n\x0c\n\x05\x04\x0b\x02\0\x04\x12\x03e\x02\n\n\
    \x0c\n\x05\x04\x0b\x02\0\x05\x12\x03e\x0b\x11\n\x0c\n\x05\x04\x0b\x02\0\
    \x01\x12\x03e\x12\x18\n\x0c\n\x05\x04\x0b\x02\0\x03\x12\x03e\x1b\x1c\n\
    \x0b\n\x04\x04\x0b\x02\x01\x12\x03f\x02\x1e\n\x0c\n\x05\x04\x0b\x02\x01\
    \x04\x12\x03f\x02\n\n\x0c\n\x05\x04\x0b\x02\x01\x05\x12\x03f\x0b\x11\n\
    \x0c\n\x05\x04\x0b\x02\x01\x01\x12\x03f\x12\x19\n\x0c\n\x05\x04\x0b\x02\
    \x01\x03\x12\x03f\x1c\x1d\n\x0b\n\x04\x04\x0b\x02\x02\x12\x03g\x02\x1e\n\
    \x0c\n\x05\x04\x0b\x02\x02\x04\x12\x03g\x02\n\n\x0c\n\x05\x04\x0b\x02\
    \x02\x05\x12\x03g\x0b\x0f\n\x0c\n\x05\x04\x0b\x02\x02\x01\x12\x03g\x10\
    \x19\n\x0c\n\x05\x04\x0b\x02\x02\x03\x12\x03g\x1c\x1d\n\x0b\n\x04\x04\
    \x0b\x02\x03\x12\x03h\x02\x1d\n\x0c\n\x05\x04\x0b\x02\x03\x04\x12\x03h\
    \x02\n\n\x0c\n\x05\x04\x0b\x02\x03\x05\x12\x03h\x0b\x11\n\x0c\n\x05\x04\
    \x0b\x02\x03\x01\x12\x03h\x12\x18\n\x0c\n\x05\x04\x0b\x02\x03\x03\x12\
    \x03h\x1b\x1c\n\x0b\n\x04\x04\x0b\x02\x04\x12\x03i\x02\x20\n\x0c\n\x05\
    \x04\x0b\x02\x04\x04\x12\x03i\x02\n\n\x0c\n\x05\x04\x0b\x02\x04\x05\x12\
    \x03i\x0b\x0f\n\x0c\n\x05\x04\x0b\x02\x04\x01\x12\x03i\x10\x1b\n\x0c\n\
    \x05\x04\x0b\x02\x04\x03\x12\x03i\x1e\x1f\n\x0b\n\x04\x04\x0b\x02\x05\
    \x12\x03j\x02!\n\x0c\n\x05\x04\x0b\x02\x05\x04\x12\x03j\x02\n\n\x0c\n\
    \x05\x04\x0b\x02\x05\x05\x12\x03j\x0b\x0f\n\x0c\n\x05\x04\x0b\x02\x05\
    \x01\x12\x03j\x10\x1c\n\x0c\n\x05\x04\x0b\x02\x05\x03\x12\x03j\x1f\x20\n\
    \n\n\x02\x05\x03\x12\x04m\0s\x01\n\n\n\x03\x05\x03\x01\x12\x03m\x05\x19\
    \n\x0b\n\x04\x05\x03\x02\0\x12\x03n\x02\x11\n\x0c\n\x05\x05\x03\x02\0\
    \x01\x12\x03n\x02\x0c\n\x0c\n\x05\x05\x03\x02\0\x02\x12\x03n\x0f\x10\n\
    \x0b\n\x04\x05\x03\x02\x01\x12\x03o\x02\x11\n\x0c\n\x05\x05\x03\x02\x01\
    \x01\x12\x03o\x02\x0c\n\x0c\n\x05\x05\x03\x02\x01\x02\x12\x03o\x0f\x10\n\
    \x0b\n\x04\x05\x03\x02\x02\x12\x03p\x02\x0e\n\x0c\n\x05\x05\x03\x02\x02\
    \x01\x12\x03p\x02\t\n\x0c\n\x05\x05\x03\x02\x02\x02\x12\x03p\x0c\r\n\x0b\
    \n\x04\x05\x03\x02\x03\x12\x03q\x02\x0e\n\x0c\n\x05\x05\x03\x02\x03\x01\
    \x12\x03q\x02\t\n\x0c\n\x05\x05\x03\x02\x03\x02\x12\x03q\x0c\r\n\x0b\n\
    \x04\x05\x03\x02\x04\x12\x03r\x02\x0e\n\x0c\n\x05\x05\x03\x02\x04\x01\
    \x12\x03r\x02\t\n\x0c\n\x05\x05\x03\x02\x04\x02\x12\x03r\x0c\r\n\n\n\x02\
    \x04\x0c\x12\x04u\0{\x01\n\n\n\x03\x04\x0c\x01\x12\x03u\x08\x17\n\x0b\n\
    \x04\x04\x0c\x02\0\x12\x03v\x02\x1b\n\x0c\n\x05\x04\x0c\x02\0\x04\x12\
    \x03v\x02\n\n\x0c\n\x05\x04\x0c\x02\0\x05\x12\x03v\x0b\x11\n\x0c\n\x05\
    \x04\x0c\x02\0\x01\x12\x03v\x12\x16\n\x0c\n\x05\x04\x0c\x02\0\x03\x12\
    \x03v\x19\x1a\n\x0b\n\x04\x04\x0c\x02\x01\x12\x03w\x02\x1c\n\x0c\n\x05\
    \x04\x0c\x02\x01\x04\x12\x03w\x02\n\n\x0c\n\x05\x04\x0c\x02\x01\x05\x12\
    \x03w\x0b\x11\n\x0c\n\x05\x04\x0c\x02\x01\x01\x12\x03w\x12\x17\n\x0c\n\
    \x05\x04\x0c\x02\x01\x03\x12\x03w\x1a\x1b\n\x0b\n\x04\x04\x0c\x02\x02\
    \x12\x03x\x02*\n\x0c\n\x05\x04\x0c\x02\x02\x04\x12\x03x\x02\n\n\x0c\n\
    \x05\x04\x0c\x02\x02\x06\x12\x03x\x0b\x1f\n\x0c\n\x05\x04\x0c\x02\x02\
    \x01\x12\x03x\x20%\n\x0c\n\x05\x04\x0c\x02\x02\x03\x12\x03x()\n\x0b\n\
    \x04\x04\x0c\x02\x03\x12\x03y\x02\x1d\n\x0c\n\x05\x04\x0c\x02\x03\x04\
    \x12\x03y\x02\n\n\x0c\n\x05\x04\x0c\x02\x03\x05\x12\x03y\x0b\x11\n\x0c\n\
    \x05\x04\x0c\x02\x03\x01\x12\x03y\x12\x18\n\x0c\n\x05\x04\x0c\x02\x03\
    \x03\x12\x03y\x1b\x1c\n\x0b\n\x04\x04\x0c\x02\x04\x12\x03z\x02\x1d\n\x0c\
    \n\x05\x04\x0c\x02\x04\x04\x12\x03z\x02\n\n\x0c\n\x05\x04\x0c\x02\x04\
    \x05\x12\x03z\x0b\x11\n\x0c\n\x05\x04\x0c\x02\x04\x01\x12\x03z\x12\x18\n\
    \x0c\n\x05\x04\x0c\x02\x04\x03\x12\x03z\x1b\x1c\n\x0b\n\x02\x05\x04\x12\
    \x05}\0\x82\x01\x01\n\n\n\x03\x05\x04\x01\x12\x03}\x05\x12\n\x0b\n\x04\
    \x05\x04\x02\0\x12\x03~\x02\x13\n\x0c\n\x05\x05\x04\x02\0\x01\x12\x03~\
    \x02\x0e\n\x0c\n\x05\x05\x04\x02\0\x02\x12\x03~\x11\x12\n\x0b\n\x04\x05\
    \x04\x02\x01\x12\x03\x7f\x02\x17\n\x0c\n\x05\x05\x04\x02\x01\x01\x12\x03\
    \x7f\x02\x12\n\x0c\n\x05\x05\x04\x02\x01\x02\x12\x03\x7f\x15\x16\n\x0c\n\
    \x04\x05\x04\x02\x02\x12\x04\x80\x01\x02\x14\n\r\n\x05\x05\x04\x02\x02\
    \x01\x12\x04\x80\x01\x02\x0f\n\r\n\x05\x05\x04\x02\x02\x02\x12\x04\x80\
    \x01\x12\x13\n\x0c\n\x04\x05\x04\x02\x03\x12\x04\x81\x01\x02\x12\n\r\n\
    \x05\x05\x04\x02\x03\x01\x12\x04\x81\x01\x02\r\n\r\n\x05\x05\x04\x02\x03\
    \x02\x12\x04\x81\x01\x10\x11\n\x0c\n\x02\x04\r\x12\x06\x84\x01\0\x86\x01\
    \x01\n\x0b\n\x03\x04\r\x01\x12\x04\x84\x01\x08\x15\n\x0c\n\x04\x04\r\x02\
    \0\x12\x04\x85\x01\x02\x1f\n\r\n\x05\x04\r\x02\0\x04\x12\x04\x85\x01\x02\
    \n\n\r\n\x05\x04\r\x02\0\x05\x12\x04\x85\x01\x0b\x11\n\r\n\x05\x04\r\x02\
    \0\x01\x12\x04\x85\x01\x12\x1a\n\r\n\x05\x04\r\x02\0\x03\x12\x04\x85\x01\
    \x1d\x1e\n\x0c\n\x02\x04\x0e\x12\x06\x88\x01\0\x8a\x01\x01\n\x0b\n\x03\
    \x04\x0e\x01\x12\x04\x88\x01\x08\x13\n\x0c\n\x04\x04\x0e\x02\0\x12\x04\
    \x89\x01\x02\x1f\n\r\n\x05\x04\x0e\x02\0\x04\x12\x04\x89\x01\x02\n\n\r\n\
    \x05\x04\x0e\x02\0\x05\x12\x04\x89\x01\x0b\x11\n\r\n\x05\x04\x0e\x02\0\
    \x01\x12\x04\x89\x01\x12\x1a\n\r\n\x05\x04\x0e\x02\0\x03\x12\x04\x89\x01\
    \x1d\x1e\n\x0c\n\x02\x04\x0f\x12\x06\x8c\x01\0\x91\x01\x01\n\x0b\n\x03\
    \x04\x0f\x01\x12\x04\x8c\x01\x08\x10\n\x0c\n\x04\x04\x0f\x02\0\x12\x04\
    \x8d\x01\x02\x19\n\r\n\x05\x04\x0f\x02\0\x04\x12\x04\x8d\x01\x02\n\n\r\n\
    \x05\x04\x0f\x02\0\x05\x12\x04\x8d\x01\x0b\x11\n\r\n\x05\x04\x0f\x02\0\
    \x01\x12\x04\x8d\x01\x12\x14\n\r\n\x05\x04\x0f\x02\0\x03\x12\x04\x8d\x01\
    \x17\x18\n\x0c\n\x04\x04\x0f\x02\x01\x12\x04\x8e\x01\x02#\n\r\n\x05\x04\
    \x0f\x02\x01\x04\x12\x04\x8e\x01\x02\n\n\r\n\x05\x04\x0f\x02\x01\x06\x12\
    \x04\x8e\x01\x0b\x18\n\r\n\x05\x04\x0f\x02\x01\x01\x12\x04\x8e\x01\x19\
    \x1e\n\r\n\x05\x04\x0f\x02\x01\x03\x12\x04\x8e\x01!\"\n\x0c\n\x04\x04\
    \x0f\x02\x02\x12\x04\x8f\x01\x02(\n\r\n\x05\x04\x0f\x02\x02\x04\x12\x04\
    \x8f\x01\x02\n\n\r\n\x05\x04\x0f\x02\x02\x06\x12\x04\x8f\x01\x0b\x1a\n\r\
    \n\x05\x04\x0f\x02\x02\x01\x12\x04\x8f\x01\x1b#\n\r\n\x05\x04\x0f\x02\
    \x02\x03\x12\x04\x8f\x01&'\n\x0c\n\x04\x04\x0f\x02\x03\x12\x04\x90\x01\
    \x02!\n\r\n\x05\x04\x0f\x02\x03\x04\x12\x04\x90\x01\x02\n\n\r\n\x05\x04\
    \x0f\x02\x03\x05\x12\x04\x90\x01\x0b\x11\n\r\n\x05\x04\x0f\x02\x03\x01\
    \x12\x04\x90\x01\x12\x1c\n\r\n\x05\x04\x0f\x02\x03\x03\x12\x04\x90\x01\
    \x1f\x20\n\x0c\n\x02\x04\x10\x12\x06\x93\x01\0\x97\x01\x01\n\x0b\n\x03\
    \x04\x10\x01\x12\x04\x93\x01\x08\x17\n\x0c\n\x04\x04\x10\x02\0\x12\x04\
    \x94\x01\x02\x1c\n\r\n\x05\x04\x10\x02\0\x04\x12\x04\x94\x01\x02\n\n\r\n\
    \x05\x04\x10\x02\0\x05\x12\x04\x94\x01\x0b\x11\n\r\n\x05\x04\x10\x02\0\
    \x01\x12\x04\x94\x01\x12\x17\n\r\n\x05\x04\x10\x02\0\x03\x12\x04\x94\x01\
    \x1a\x1b\n\x0c\n\x04\x04\x10\x02\x01\x12\x04\x95\x01\x02\x1b\n\r\n\x05\
    \x04\x10\x02\x01\x04\x12\x04\x95\x01\x02\n\n\r\n\x05\x04\x10\x02\x01\x05\
    \x12\x04\x95\x01\x0b\x11\n\r\n\x05\x04\x10\x02\x01\x01\x12\x04\x95\x01\
    \x12\x16\n\r\n\x05\x04\x10\x02\x01\x03\x12\x04\x95\x01\x19\x1a\n\x0c\n\
    \x04\x04\x10\x02\x02\x12\x04\x96\x01\x02\x1d\n\r\n\x05\x04\x10\x02\x02\
    \x04\x12\x04\x96\x01\x02\n\n\r\n\x05\x04\x10\x02\x02\x05\x12\x04\x96\x01\
    \x0b\x11\n\r\n\x05\x04\x10\x02\x02\x01\x12\x04\x96\x01\x12\x18\n\r\n\x05\
    \x04\x10\x02\x02\x03\x12\x04\x96\x01\x1b\x1c\n\x0c\n\x02\x04\x11\x12\x06\
    \x99\x01\0\x9d\x01\x01\n\x0b\n\x03\x04\x11\x01\x12\x04\x99\x01\x08\x20\n\
    \x0c\n\x04\x04\x11\x02\0\x12\x04\x9a\x01\x02\x1c\n\r\n\x05\x04\x11\x02\0\
    \x04\x12\x04\x9a\x01\x02\n\n\r\n\x05\x04\x11\x02\0\x05\x12\x04\x9a\x01\
    \x0b\x11\n\r\n\x05\x04\x11\x02\0\x01\x12\x04\x9a\x01\x12\x17\n\r\n\x05\
    \x04\x11\x02\0\x03\x12\x04\x9a\x01\x1a\x1b\n\x0c\n\x04\x04\x11\x02\x01\
    \x12\x04\x9b\x01\x02\x1b\n\r\n\x05\x04\x11\x02\x01\x04\x12\x04\x9b\x01\
    \x02\n\n\r\n\x05\x04\x11\x02\x01\x05\x12\x04\x9b\x01\x0b\x11\n\r\n\x05\
    \x04\x11\x02\x01\x01\x12\x04\x9b\x01\x12\x16\n\r\n\x05\x04\x11\x02\x01\
    \x03\x12\x04\x9b\x01\x19\x1a\n\x0c\n\x04\x04\x11\x02\x02\x12\x04\x9c\x01\
    \x02\x1d\n\r\n\x05\x04\x11\x02\x02\x04\x12\x04\x9c\x01\x02\n\n\r\n\x05\
    \x04\x11\x02\x02\x05\x12\x04\x9c\x01\x0b\x11\n\r\n\x05\x04\x11\x02\x02\
    \x01\x12\x04\x9c\x01\x12\x18\n\r\n\x05\x04\x11\x02\x02\x03\x12\x04\x9c\
    \x01\x1b\x1c\n\x0c\n\x02\x04\x12\x12\x06\x9f\x01\0\xa3\x01\x01\n\x0b\n\
    \x03\x04\x12\x01\x12\x04\x9f\x01\x08\x1d\n\x0c\n\x04\x04\x12\x02\0\x12\
    \x04\xa0\x01\x02\x1c\n\r\n\x05\x04\x12\x02\0\x04\x12\x04\xa0\x01\x02\n\n\
    \r\n\x05\x04\x12\x02\0\x05\x12\x04\xa0\x01\x0b\x11\n\r\n\x05\x04\x12\x02\
    \0\x01\x12\x04\xa0\x01\x12\x17\n\r\n\x05\x04\x12\x02\0\x03\x12\x04\xa0\
    \x01\x1a\x1b\n\x0c\n\x04\x04\x12\x02\x01\x12\x04\xa1\x01\x02\x1b\n\r\n\
    \x05\x04\x12\x02\x01\x04\x12\x04\xa1\x01\x02\n\n\r\n\x05\x04\x12\x02\x01\
    \x05\x12\x04\xa1\x01\x0b\x11\n\r\n\x05\x04\x12\x02\x01\x01\x12\x04\xa1\
    \x01\x12\x16\n\r\n\x05\x04\x12\x02\x01\x03\x12\x04\xa1\x01\x19\x1a\n\x0c\
    \n\x04\x04\x12\x02\x02\x12\x04\xa2\x01\x02\x1d\n\r\n\x05\x04\x12\x02\x02\
    \x04\x12\x04\xa2\x01\x02\n\n\r\n\x05\x04\x12\x02\x02\x05\x12\x04\xa2\x01\
    \x0b\x11\n\r\n\x05\x04\x12\x02\x02\x01\x12\x04\xa2\x01\x12\x18\n\r\n\x05\
    \x04\x12\x02\x02\x03\x12\x04\xa2\x01\x1b\x1c\n\x0c\n\x02\x04\x13\x12\x06\
    \xa5\x01\0\xa9\x01\x01\n\x0b\n\x03\x04\x13\x01\x12\x04\xa5\x01\x08-\n\
    \x0c\n\x04\x04\x13\x02\0\x12\x04\xa6\x01\x02\x1d\n\r\n\x05\x04\x13\x02\0\
    \x04\x12\x04\xa6\x01\x02\n\n\r\n\x05\x04\x13\x02\0\x05\x12\x04\xa6\x01\
    \x0b\x11\n\r\n\x05\x04\x13\x02\0\x01\x12\x04\xa6\x01\x12\x18\n\r\n\x05\
    \x04\x13\x02\0\x03\x12\x04\xa6\x01\x1b\x1c\n\x0c\n\x04\x04\x13\x02\x01\
    \x12\x04\xa7\x01\x02\x1b\n\r\n\x05\x04\x13\x02\x01\x04\x12\x04\xa7\x01\
    \x02\n\n\r\n\x05\x04\x13\x02\x01\x05\x12\x04\xa7\x01\x0b\x11\n\r\n\x05\
    \x04\x13\x02\x01\x01\x12\x04\xa7\x01\x12\x16\n\r\n\x05\x04\x13\x02\x01\
    \x03\x12\x04\xa7\x01\x19\x1a\n\x0c\n\x04\x04\x13\x02\x02\x12\x04\xa8\x01\
    \x02\x1d\n\r\n\x05\x04\x13\x02\x02\x04\x12\x04\xa8\x01\x02\n\n\r\n\x05\
    \x04\x13\x02\x02\x05\x12\x04\xa8\x01\x0b\x11\n\r\n\x05\x04\x13\x02\x02\
    \x01\x12\x04\xa8\x01\x12\x18\n\r\n\x05\x04\x13\x02\x02\x03\x12\x04\xa8\
    \x01\x1b\x1c\n\x0c\n\x02\x04\x14\x12\x06\xab\x01\0\xaf\x01\x01\n\x0b\n\
    \x03\x04\x14\x01\x12\x04\xab\x01\x08*\n\x0c\n\x04\x04\x14\x02\0\x12\x04\
    \xac\x01\x02\x1d\n\r\n\x05\x04\x14\x02\0\x04\x12\x04\xac\x01\x02\n\n\r\n\
    \x05\x04\x14\x02\0\x05\x12\x04\xac\x01\x0b\x11\n\r\n\x05\x04\x14\x02\0\
    \x01\x12\x04\xac\x01\x12\x18\n\r\n\x05\x04\x14\x02\0\x03\x12\x04\xac\x01\
    \x1b\x1c\n\x0c\n\x04\x04\x14\x02\x01\x12\x04\xad\x01\x02\x1b\n\r\n\x05\
    \x04\x14\x02\x01\x04\x12\x04\xad\x01\x02\n\n\r\n\x05\x04\x14\x02\x01\x05\
    \x12\x04\xad\x01\x0b\x11\n\r\n\x05\x04\x14\x02\x01\x01\x12\x04\xad\x01\
    \x12\x16\n\r\n\x05\x04\x14\x02\x01\x03\x12\x04\xad\x01\x19\x1a\n\x0c\n\
    \x04\x04\x14\x02\x02\x12\x04\xae\x01\x02\x1c\n\r\n\x05\x04\x14\x02\x02\
    \x04\x12\x04\xae\x01\x02\n\n\r\n\x05\x04\x14\x02\x02\x05\x12\x04\xae\x01\
    \x0b\x11\n\r\n\x05\x04\x14\x02\x02\x01\x12\x04\xae\x01\x12\x17\n\r\n\x05\
    \x04\x14\x02\x02\x03\x12\x04\xae\x01\x1a\x1b\n\x0c\n\x02\x04\x15\x12\x06\
    \xb1\x01\0\xb3\x01\x01\n\x0b\n\x03\x04\x15\x01\x12\x04\xb1\x01\x08\x1f\n\
    \x0c\n\x04\x04\x15\x02\0\x12\x04\xb2\x01\x02\x1d\n\r\n\x05\x04\x15\x02\0\
    \x04\x12\x04\xb2\x01\x02\n\n\r\n\x05\x04\x15\x02\0\x05\x12\x04\xb2\x01\
    \x0b\x11\n\r\n\x05\x04\x15\x02\0\x01\x12\x04\xb2\x01\x12\x18\n\r\n\x05\
    \x04\x15\x02\0\x03\x12\x04\xb2\x01\x1b\x1c\n\x0c\n\x02\x04\x16\x12\x06\
    \xb5\x01\0\xb9\x01\x01\n\x0b\n\x03\x04\x16\x01\x12\x04\xb5\x01\x08\x1c\n\
    \x0c\n\x04\x04\x16\x02\0\x12\x04\xb6\x01\x02\x1c\n\r\n\x05\x04\x16\x02\0\
    \x04\x12\x04\xb6\x01\x02\n\n\r\n\x05\x04\x16\x02\0\x05\x12\x04\xb6\x01\
    \x0b\x11\n\r\n\x05\x04\x16\x02\0\x01\x12\x04\xb6\x01\x12\x17\n\r\n\x05\
    \x04\x16\x02\0\x03\x12\x04\xb6\x01\x1a\x1b\n\x0c\n\x04\x04\x16\x02\x01\
    \x12\x04\xb7\x01\x02\x1d\n\r\n\x05\x04\x16\x02\x01\x04\x12\x04\xb7\x01\
    \x02\n\n\r\n\x05\x04\x16\x02\x01\x05\x12\x04\xb7\x01\x0b\x11\n\r\n\x05\
    \x04\x16\x02\x01\x01\x12\x04\xb7\x01\x12\x18\n\r\n\x05\x04\x16\x02\x01\
    \x03\x12\x04\xb7\x01\x1b\x1c\n\x0c\n\x04\x04\x16\x02\x02\x12\x04\xb8\x01\
    \x02&\n\r\n\x05\x04\x16\x02\x02\x04\x12\x04\xb8\x01\x02\n\n\r\n\x05\x04\
    \x16\x02\x02\x05\x12\x04\xb8\x01\x0b\x11\n\r\n\x05\x04\x16\x02\x02\x01\
    \x12\x04\xb8\x01\x12!\n\r\n\x05\x04\x16\x02\x02\x03\x12\x04\xb8\x01$%\
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
