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
    \x0bis_complete\x18\x04\x20\x01(\x08R\nisComplete*(\n\x02Os\x12\t\n\x05L\
    inux\x10\x01\x12\n\n\x06Darwin\x10\x02\x12\x0b\n\x07Windows\x10\x03*\"\n\
    \x0bWorkerState\x12\t\n\x05Ready\x10\0\x12\x08\n\x04Busy\x10\x01*_\n\x08\
    JobState\x12\x0b\n\x07Pending\x10\0\x12\x0e\n\nProcessing\x10\x01\x12\
    \x0c\n\x08Complete\x10\x02\x12\x0c\n\x08Rejected\x10\x03\x12\n\n\x06Fail\
    ed\x10\x04\x12\x0e\n\nDispatched\x10\x05J\xf7'\n\x07\x12\x05\0\0\x80\x01\
    \x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\x08\n\x01\x02\x12\x03\x01\x08\x0e\
    \n\t\n\x02\x03\0\x12\x03\x02\x07\x1c\n\t\n\x02\x03\x01\x12\x03\x03\x07\"\
    \n\n\n\x02\x05\0\x12\x04\x05\0\t\x01\n\n\n\x03\x05\0\x01\x12\x03\x05\x05\
    \x07\n\x0b\n\x04\x05\0\x02\0\x12\x03\x06\x02\x0c\n\x0c\n\x05\x05\0\x02\0\
    \x01\x12\x03\x06\x02\x07\n\x0c\n\x05\x05\0\x02\0\x02\x12\x03\x06\n\x0b\n\
    \x0b\n\x04\x05\0\x02\x01\x12\x03\x07\x02\r\n\x0c\n\x05\x05\0\x02\x01\x01\
    \x12\x03\x07\x02\x08\n\x0c\n\x05\x05\0\x02\x01\x02\x12\x03\x07\x0b\x0c\n\
    \x0b\n\x04\x05\0\x02\x02\x12\x03\x08\x02\x0e\n\x0c\n\x05\x05\0\x02\x02\
    \x01\x12\x03\x08\x02\t\n\x0c\n\x05\x05\0\x02\x02\x02\x12\x03\x08\x0c\r\n\
    \n\n\x02\x05\x01\x12\x04\x0b\0\x0e\x01\n\n\n\x03\x05\x01\x01\x12\x03\x0b\
    \x05\x10\n\x0b\n\x04\x05\x01\x02\0\x12\x03\x0c\x02\x0c\n\x0c\n\x05\x05\
    \x01\x02\0\x01\x12\x03\x0c\x02\x07\n\x0c\n\x05\x05\x01\x02\0\x02\x12\x03\
    \x0c\n\x0b\n\x0b\n\x04\x05\x01\x02\x01\x12\x03\r\x02\x0b\n\x0c\n\x05\x05\
    \x01\x02\x01\x01\x12\x03\r\x02\x06\n\x0c\n\x05\x05\x01\x02\x01\x02\x12\
    \x03\r\t\n\n\n\n\x02\x05\x02\x12\x04\x10\0\x17\x01\n\n\n\x03\x05\x02\x01\
    \x12\x03\x10\x05\r\n\x0b\n\x04\x05\x02\x02\0\x12\x03\x11\x02\x0e\n\x0c\n\
    \x05\x05\x02\x02\0\x01\x12\x03\x11\x02\t\n\x0c\n\x05\x05\x02\x02\0\x02\
    \x12\x03\x11\x0c\r\n\x0b\n\x04\x05\x02\x02\x01\x12\x03\x12\x02\x11\n\x0c\
    \n\x05\x05\x02\x02\x01\x01\x12\x03\x12\x02\x0c\n\x0c\n\x05\x05\x02\x02\
    \x01\x02\x12\x03\x12\x0f\x10\n\x0b\n\x04\x05\x02\x02\x02\x12\x03\x13\x02\
    \x0f\n\x0c\n\x05\x05\x02\x02\x02\x01\x12\x03\x13\x02\n\n\x0c\n\x05\x05\
    \x02\x02\x02\x02\x12\x03\x13\r\x0e\n\x0b\n\x04\x05\x02\x02\x03\x12\x03\
    \x14\x02\x0f\n\x0c\n\x05\x05\x02\x02\x03\x01\x12\x03\x14\x02\n\n\x0c\n\
    \x05\x05\x02\x02\x03\x02\x12\x03\x14\r\x0e\n\x0b\n\x04\x05\x02\x02\x04\
    \x12\x03\x15\x02\r\n\x0c\n\x05\x05\x02\x02\x04\x01\x12\x03\x15\x02\x08\n\
    \x0c\n\x05\x05\x02\x02\x04\x02\x12\x03\x15\x0b\x0c\n\x0b\n\x04\x05\x02\
    \x02\x05\x12\x03\x16\x02\x11\n\x0c\n\x05\x05\x02\x02\x05\x01\x12\x03\x16\
    \x02\x0c\n\x0c\n\x05\x05\x02\x02\x05\x02\x12\x03\x16\x0f\x10\n\n\n\x02\
    \x04\0\x12\x04\x19\0\x1d\x01\n\n\n\x03\x04\0\x01\x12\x03\x19\x08\x11\n\
    \x0b\n\x04\x04\0\x02\0\x12\x03\x1a\x02\x1f\n\x0c\n\x05\x04\0\x02\0\x04\
    \x12\x03\x1a\x02\n\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\x1a\x0b\x11\n\x0c\
    \n\x05\x04\0\x02\0\x01\x12\x03\x1a\x12\x1a\n\x0c\n\x05\x04\0\x02\0\x03\
    \x12\x03\x1a\x1d\x1e\n\x0b\n\x04\x04\0\x02\x01\x12\x03\x1b\x02\x15\n\x0c\
    \n\x05\x04\0\x02\x01\x04\x12\x03\x1b\x02\n\n\x0c\n\x05\x04\0\x02\x01\x06\
    \x12\x03\x1b\x0b\r\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x1b\x0e\x10\n\
    \x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x1b\x13\x14\n\x0b\n\x04\x04\0\x02\
    \x02\x12\x03\x1c\x02!\n\x0c\n\x05\x04\0\x02\x02\x04\x12\x03\x1c\x02\n\n\
    \x0c\n\x05\x04\0\x02\x02\x06\x12\x03\x1c\x0b\x16\n\x0c\n\x05\x04\0\x02\
    \x02\x01\x12\x03\x1c\x17\x1c\n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03\x1c\
    \x1f\x20\n\n\n\x02\x04\x01\x12\x04\x1f\0#\x01\n\n\n\x03\x04\x01\x01\x12\
    \x03\x1f\x08\x12\n\x0b\n\x04\x04\x01\x02\0\x12\x03\x20\x02\x1c\n\x0c\n\
    \x05\x04\x01\x02\0\x04\x12\x03\x20\x02\n\n\x0c\n\x05\x04\x01\x02\0\x05\
    \x12\x03\x20\x0b\x11\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\x20\x12\x17\n\
    \x0c\n\x05\x04\x01\x02\0\x03\x12\x03\x20\x1a\x1b\n\x0b\n\x04\x04\x01\x02\
    \x01\x12\x03!\x02\x1d\n\x0c\n\x05\x04\x01\x02\x01\x04\x12\x03!\x02\n\n\
    \x0c\n\x05\x04\x01\x02\x01\x05\x12\x03!\x0b\x11\n\x0c\n\x05\x04\x01\x02\
    \x01\x01\x12\x03!\x12\x18\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x03!\x1b\
    \x1c\n\x0b\n\x04\x04\x01\x02\x02\x12\x03\"\x02\x20\n\x0c\n\x05\x04\x01\
    \x02\x02\x04\x12\x03\"\x02\n\n\x0c\n\x05\x04\x01\x02\x02\x05\x12\x03\"\
    \x0b\x0f\n\x0c\n\x05\x04\x01\x02\x02\x01\x12\x03\"\x10\x1b\n\x0c\n\x05\
    \x04\x01\x02\x02\x03\x12\x03\"\x1e\x1f\n\n\n\x02\x04\x02\x12\x04%\0>\x01\
    \n\n\n\x03\x04\x02\x01\x12\x03%\x08\x0b\n\n\n\x03\x04\x02\t\x12\x03&\x0b\
    \x0e\n\x0b\n\x04\x04\x02\t\0\x12\x03&\x0b\r\n\x0c\n\x05\x04\x02\t\0\x01\
    \x12\x03&\x0b\r\n\x0c\n\x05\x04\x02\t\0\x02\x12\x03&\x0b\r\n\n\n\x03\x04\
    \x02\n\x12\x03'\x0b\x15\n\x0b\n\x04\x04\x02\n\0\x12\x03'\x0b\x14\n\x0b\n\
    \x04\x04\x02\x02\0\x12\x03(\x02\x19\n\x0c\n\x05\x04\x02\x02\0\x04\x12\
    \x03(\x02\n\n\x0c\n\x05\x04\x02\x02\0\x05\x12\x03(\x0b\x11\n\x0c\n\x05\
    \x04\x02\x02\0\x01\x12\x03(\x12\x14\n\x0c\n\x05\x04\x02\x02\0\x03\x12\
    \x03(\x17\x18\n\x0b\n\x04\x04\x02\x02\x01\x12\x03)\x02\x1f\n\x0c\n\x05\
    \x04\x02\x02\x01\x04\x12\x03)\x02\n\n\x0c\n\x05\x04\x02\x02\x01\x05\x12\
    \x03)\x0b\x11\n\x0c\n\x05\x04\x02\x02\x01\x01\x12\x03)\x12\x1a\n\x0c\n\
    \x05\x04\x02\x02\x01\x03\x12\x03)\x1d\x1e\n\x0b\n\x04\x04\x02\x02\x02\
    \x12\x03*\x02\x1e\n\x0c\n\x05\x04\x02\x02\x02\x04\x12\x03*\x02\n\n\x0c\n\
    \x05\x04\x02\x02\x02\x06\x12\x03*\x0b\x13\n\x0c\n\x05\x04\x02\x02\x02\
    \x01\x12\x03*\x14\x19\n\x0c\n\x05\x04\x02\x02\x02\x03\x12\x03*\x1c\x1d\n\
    \x0b\n\x04\x04\x02\x02\x03\x12\x03+\x02/\n\x0c\n\x05\x04\x02\x02\x03\x04\
    \x12\x03+\x02\n\n\x0c\n\x05\x04\x02\x02\x03\x06\x12\x03+\x0b\"\n\x0c\n\
    \x05\x04\x02\x02\x03\x01\x12\x03+#*\n\x0c\n\x05\x04\x02\x02\x03\x03\x12\
    \x03+-.\n\x0b\n\x04\x04\x02\x02\x04\x12\x03,\x02\"\n\x0c\n\x05\x04\x02\
    \x02\x04\x04\x12\x03,\x02\n\n\x0c\n\x05\x04\x02\x02\x04\x06\x12\x03,\x0b\
    \x17\n\x0c\n\x05\x04\x02\x02\x04\x01\x12\x03,\x18\x1d\n\x0c\n\x05\x04\
    \x02\x02\x04\x03\x12\x03,\x20!\n\xa5\x01\n\x04\x04\x02\x02\x05\x12\x030\
    \x02!\x1a\x97\x01\x20The\x20RFC3339-formatted\x20time\x20the\x20job\x20w\
    as\x20entered\x20into\x20the\n\x20system.\x20It\x20may\x20not\x20begin\
    \x20processing\x20for\x20some\x20time\x20after\x20this,\n\x20based\x20on\
    \x20current\x20system\x20load.\n\n\x0c\n\x05\x04\x02\x02\x05\x04\x12\x03\
    0\x02\n\n\x0c\n\x05\x04\x02\x02\x05\x05\x12\x030\x0b\x11\n\x0c\n\x05\x04\
    \x02\x02\x05\x01\x12\x030\x12\x1c\n\x0c\n\x05\x04\x02\x02\x05\x03\x12\
    \x030\x1f\x20\nR\n\x04\x04\x02\x02\x06\x12\x033\x02'\x1aE\x20The\x20RFC3\
    339-formatted\x20time\x20the\x20`hab\x20studio\x20build`\x20process\n\
    \x20started.\n\n\x0c\n\x05\x04\x02\x02\x06\x04\x12\x033\x02\n\n\x0c\n\
    \x05\x04\x02\x02\x06\x05\x12\x033\x0b\x11\n\x0c\n\x05\x04\x02\x02\x06\
    \x01\x12\x033\x12\"\n\x0c\n\x05\x04\x02\x02\x06\x03\x12\x033%&\ne\n\x04\
    \x04\x02\x02\x07\x12\x036\x02(\x1aX\x20The\x20RFC3339-formatted\x20time\
    \x20the\x20`hab\x20studio\x20build`\x20process\n\x20stopped,\x20successf\
    ul\x20or\x20not.\n\n\x0c\n\x05\x04\x02\x02\x07\x04\x12\x036\x02\n\n\x0c\
    \n\x05\x04\x02\x02\x07\x05\x12\x036\x0b\x11\n\x0c\n\x05\x04\x02\x02\x07\
    \x01\x12\x036\x12#\n\x0c\n\x05\x04\x02\x02\x07\x03\x12\x036&'\nH\n\x04\
    \x04\x02\x02\x08\x12\x038\x02:\x1a;\x20The\x20identifier\x20of\x20the\
    \x20package\x20built/attempted\x20by\x20the\x20job.\n\n\x0c\n\x05\x04\
    \x02\x02\x08\x04\x12\x038\x02\n\n\x0c\n\x05\x04\x02\x02\x08\x06\x12\x038\
    \x0b'\n\x0c\n\x05\x04\x02\x02\x08\x01\x12\x038(5\n\x0c\n\x05\x04\x02\x02\
    \x08\x03\x12\x03889\nC\n\x04\x04\x02\x02\t\x12\x03:\x02!\x1a6\x20Whether\
    \x20or\x20not\x20the\x20log\x20for\x20the\x20job\x20has\x20been\x20archi\
    ved\n\n\x0c\n\x05\x04\x02\x02\t\x04\x12\x03:\x02\n\n\x0c\n\x05\x04\x02\
    \x02\t\x05\x12\x03:\x0b\x0f\n\x0c\n\x05\x04\x02\x02\t\x01\x12\x03:\x10\
    \x1b\n\x0c\n\x05\x04\x02\x02\t\x03\x12\x03:\x1e\x20\n\x0b\n\x04\x04\x02\
    \x02\n\x12\x03;\x029\n\x0c\n\x05\x04\x02\x02\n\x04\x12\x03;\x02\n\n\x0c\
    \n\x05\x04\x02\x02\n\x06\x12\x03;\x0b&\n\x0c\n\x05\x04\x02\x02\n\x01\x12\
    \x03;'3\n\x0c\n\x05\x04\x02\x02\n\x03\x12\x03;68\n\x0b\n\x04\x04\x02\x02\
    \x0b\x12\x03<\x02\x1f\n\x0c\n\x05\x04\x02\x02\x0b\x04\x12\x03<\x02\n\n\
    \x0c\n\x05\x04\x02\x02\x0b\x05\x12\x03<\x0b\x11\n\x0c\n\x05\x04\x02\x02\
    \x0b\x01\x12\x03<\x12\x19\n\x0c\n\x05\x04\x02\x02\x0b\x03\x12\x03<\x1c\
    \x1e\n\x0b\n\x04\x04\x02\x02\x0c\x12\x03=\x02H\n\x0c\n\x05\x04\x02\x02\
    \x0c\x04\x12\x03=\x02\n\n\x0c\n\x05\x04\x02\x02\x0c\x06\x12\x03=\x0b-\n\
    \x0c\n\x05\x04\x02\x02\x0c\x01\x12\x03=.B\n\x0c\n\x05\x04\x02\x02\x0c\
    \x03\x12\x03=EG\n)\n\x02\x04\x03\x12\x04A\0C\x01\x1a\x1d\x20Retrieve\x20\
    a\x20single\x20job\x20by\x20ID\n\n\n\n\x03\x04\x03\x01\x12\x03A\x08\x0e\
    \n\x0b\n\x04\x04\x03\x02\0\x12\x03B\x02\x19\n\x0c\n\x05\x04\x03\x02\0\
    \x04\x12\x03B\x02\n\n\x0c\n\x05\x04\x03\x02\0\x05\x12\x03B\x0b\x11\n\x0c\
    \n\x05\x04\x03\x02\0\x01\x12\x03B\x12\x14\n\x0c\n\x05\x04\x03\x02\0\x03\
    \x12\x03B\x17\x18\n\n\n\x02\x04\x04\x12\x04E\0I\x01\n\n\n\x03\x04\x04\
    \x01\x12\x03E\x08\x0f\n\x0b\n\x04\x04\x04\x02\0\x12\x03F\x02\x1f\n\x0c\n\
    \x05\x04\x04\x02\0\x04\x12\x03F\x02\n\n\x0c\n\x05\x04\x04\x02\0\x05\x12\
    \x03F\x0b\x11\n\x0c\n\x05\x04\x04\x02\0\x01\x12\x03F\x12\x1a\n\x0c\n\x05\
    \x04\x04\x02\0\x03\x12\x03F\x1d\x1e\n\x0b\n\x04\x04\x04\x02\x01\x12\x03G\
    \x02/\n\x0c\n\x05\x04\x04\x02\x01\x04\x12\x03G\x02\n\n\x0c\n\x05\x04\x04\
    \x02\x01\x06\x12\x03G\x0b\"\n\x0c\n\x05\x04\x04\x02\x01\x01\x12\x03G#*\n\
    \x0c\n\x05\x04\x04\x02\x01\x03\x12\x03G-.\n\x0b\n\x04\x04\x04\x02\x02\
    \x12\x03H\x02\x1e\n\x0c\n\x05\x04\x04\x02\x02\x04\x12\x03H\x02\n\n\x0c\n\
    \x05\x04\x04\x02\x02\x05\x12\x03H\x0b\x11\n\x0c\n\x05\x04\x04\x02\x02\
    \x01\x12\x03H\x12\x19\n\x0c\n\x05\x04\x04\x02\x02\x03\x12\x03H\x1c\x1d\n\
    3\n\x02\x04\x05\x12\x04L\0Q\x01\x1a'\x20Retrieve\x20jobs\x20for\x20a\x20\
    specific\x20project.\n\n\n\n\x03\x04\x05\x01\x12\x03L\x08\x16\nH\n\x04\
    \x04\x05\x02\0\x12\x03N\x02\x1b\x1a;\x20The\x20origin-qualified\x20name\
    \x20of\x20a\x20project,\x20e.g.\x20\"core/nginx\"\n\n\x0c\n\x05\x04\x05\
    \x02\0\x04\x12\x03N\x02\n\n\x0c\n\x05\x04\x05\x02\0\x05\x12\x03N\x0b\x11\
    \n\x0c\n\x05\x04\x05\x02\0\x01\x12\x03N\x12\x16\n\x0c\n\x05\x04\x05\x02\
    \0\x03\x12\x03N\x19\x1a\n\x0b\n\x04\x04\x05\x02\x01\x12\x03O\x02\x1c\n\
    \x0c\n\x05\x04\x05\x02\x01\x04\x12\x03O\x02\n\n\x0c\n\x05\x04\x05\x02\
    \x01\x05\x12\x03O\x0b\x11\n\x0c\n\x05\x04\x05\x02\x01\x01\x12\x03O\x12\
    \x17\n\x0c\n\x05\x04\x05\x02\x01\x03\x12\x03O\x1a\x1b\n\x0b\n\x04\x04\
    \x05\x02\x02\x12\x03P\x02\x1b\n\x0c\n\x05\x04\x05\x02\x02\x04\x12\x03P\
    \x02\n\n\x0c\n\x05\x04\x05\x02\x02\x05\x12\x03P\x0b\x11\n\x0c\n\x05\x04\
    \x05\x02\x02\x01\x12\x03P\x12\x16\n\x0c\n\x05\x04\x05\x02\x02\x03\x12\
    \x03P\x19\x1a\n\n\n\x02\x04\x06\x12\x04S\0X\x01\n\n\n\x03\x04\x06\x01\
    \x12\x03S\x08\x1e\n\x0b\n\x04\x04\x06\x02\0\x12\x03T\x02\x18\n\x0c\n\x05\
    \x04\x06\x02\0\x04\x12\x03T\x02\n\n\x0c\n\x05\x04\x06\x02\0\x06\x12\x03T\
    \x0b\x0e\n\x0c\n\x05\x04\x06\x02\0\x01\x12\x03T\x0f\x13\n\x0c\n\x05\x04\
    \x06\x02\0\x03\x12\x03T\x16\x17\n\x0b\n\x04\x04\x06\x02\x01\x12\x03U\x02\
    \x1c\n\x0c\n\x05\x04\x06\x02\x01\x04\x12\x03U\x02\n\n\x0c\n\x05\x04\x06\
    \x02\x01\x05\x12\x03U\x0b\x11\n\x0c\n\x05\x04\x06\x02\x01\x01\x12\x03U\
    \x12\x17\n\x0c\n\x05\x04\x06\x02\x01\x03\x12\x03U\x1a\x1b\n\x0b\n\x04\
    \x04\x06\x02\x02\x12\x03V\x02\x1b\n\x0c\n\x05\x04\x06\x02\x02\x04\x12\
    \x03V\x02\n\n\x0c\n\x05\x04\x06\x02\x02\x05\x12\x03V\x0b\x11\n\x0c\n\x05\
    \x04\x06\x02\x02\x01\x12\x03V\x12\x16\n\x0c\n\x05\x04\x06\x02\x02\x03\
    \x12\x03V\x19\x1a\n\x0b\n\x04\x04\x06\x02\x03\x12\x03W\x02\x1c\n\x0c\n\
    \x05\x04\x06\x02\x03\x04\x12\x03W\x02\n\n\x0c\n\x05\x04\x06\x02\x03\x05\
    \x12\x03W\x0b\x11\n\x0c\n\x05\x04\x06\x02\x03\x01\x12\x03W\x12\x17\n\x0c\
    \n\x05\x04\x06\x02\x03\x03\x12\x03W\x1a\x1b\nQ\n\x02\x04\x07\x12\x04[\0d\
    \x01\x1aE\x20Sent\x20from\x20a\x20worker\x20to\x20the\x20job\x20server's\
    \x20log\x20ingester\x20during\x20a\x20build.\n\n\n\n\x03\x04\x07\x01\x12\
    \x03[\x08\x13\n\x0b\n\x04\x04\x07\x02\0\x12\x03\\\x02\x1d\n\x0c\n\x05\
    \x04\x07\x02\0\x04\x12\x03\\\x02\n\n\x0c\n\x05\x04\x07\x02\0\x05\x12\x03\
    \\\x0b\x11\n\x0c\n\x05\x04\x07\x02\0\x01\x12\x03\\\x12\x18\n\x0c\n\x05\
    \x04\x07\x02\0\x03\x12\x03\\\x1b\x1c\n\x9c\x01\n\x04\x04\x07\x02\x01\x12\
    \x03`\x02\x1a\x1a\x8e\x01\x20Ordering\x20of\x20this\x20chunk\x20in\x20th\
    e\x20overall\x20log\x20output,\x20starting\x20at\x201;\n\x20We\x20curren\
    tly\x20send\x20one\x20line\x20at\x20a\x20time,\x20so\x20this\x20correspo\
    nds\x20to\x20line\n\x20number.\n\n\x0c\n\x05\x04\x07\x02\x01\x04\x12\x03\
    `\x02\n\n\x0c\n\x05\x04\x07\x02\x01\x05\x12\x03`\x0b\x11\n\x0c\n\x05\x04\
    \x07\x02\x01\x01\x12\x03`\x12\x15\n\x0c\n\x05\x04\x07\x02\x01\x03\x12\
    \x03`\x18\x19\nd\n\x04\x04\x07\x02\x02\x12\x03c\x02\x1e\x1aW\x20The\x20l\
    og\x20content\x20being\x20sent\n\x20TODO:\x20Make\x20this\x20a\x20repeat\
    ed\x20field\x20for\x20future\x20compatibility\n\n\x0c\n\x05\x04\x07\x02\
    \x02\x04\x12\x03c\x02\n\n\x0c\n\x05\x04\x07\x02\x02\x05\x12\x03c\x0b\x11\
    \n\x0c\n\x05\x04\x07\x02\x02\x01\x12\x03c\x12\x19\n\x0c\n\x05\x04\x07\
    \x02\x02\x03\x12\x03c\x1c\x1d\n\\\n\x02\x04\x08\x12\x04h\0j\x01\x1aP\x20\
    Sent\x20from\x20a\x20worker\x20to\x20the\x20job\x20server's\x20log\x20in\
    gester\x20when\x20a\x20build\x20is\n\x20complete.\n\n\n\n\x03\x04\x08\
    \x01\x12\x03h\x08\x16\n\x0b\n\x04\x04\x08\x02\0\x12\x03i\x02\x1d\n\x0c\n\
    \x05\x04\x08\x02\0\x04\x12\x03i\x02\n\n\x0c\n\x05\x04\x08\x02\0\x05\x12\
    \x03i\x0b\x11\n\x0c\n\x05\x04\x08\x02\0\x01\x12\x03i\x12\x18\n\x0c\n\x05\
    \x04\x08\x02\0\x03\x12\x03i\x1b\x1c\nl\n\x02\x04\t\x12\x04n\0r\x01\x1a`\
    \x20Initiated\x20by\x20API\x20request\x20to\x20retrieve\x20a\x20portion\
    \x20of\x20a\x20job's\x20log\x20data,\n\x20beginning\x20at\x20line\x20`st\
    art`\n\n\n\n\x03\x04\t\x01\x12\x03n\x08\x11\n\x0b\n\x04\x04\t\x02\0\x12\
    \x03o\x02\x19\n\x0c\n\x05\x04\t\x02\0\x04\x12\x03o\x02\n\n\x0c\n\x05\x04\
    \t\x02\0\x05\x12\x03o\x0b\x11\n\x0c\n\x05\x04\t\x02\0\x01\x12\x03o\x12\
    \x14\n\x0c\n\x05\x04\t\x02\0\x03\x12\x03o\x17\x18\n@\n\x04\x04\t\x02\x01\
    \x12\x03q\x02\x1c\x1a3\x20Zero-indexed\x20line\x20of\x20the\x20log\x20ou\
    tput\x20to\x20start\x20with\n\n\x0c\n\x05\x04\t\x02\x01\x04\x12\x03q\x02\
    \n\n\x0c\n\x05\x04\t\x02\x01\x05\x12\x03q\x0b\x11\n\x0c\n\x05\x04\t\x02\
    \x01\x01\x12\x03q\x12\x17\n\x0c\n\x05\x04\t\x02\x01\x03\x12\x03q\x1a\x1b\
    \n+\n\x02\x04\n\x12\x05u\0\x80\x01\x01\x1a\x1e\x20Sent\x20in\x20reply\
    \x20to\x20a\x20JobLogGet\n\n\n\n\x03\x04\n\x01\x12\x03u\x08\x0e\nK\n\x04\
    \x04\n\x02\0\x12\x03w\x02\x1c\x1a>\x20Zero-indexed\x20(inclusive)\x20lin\
    e\x20of\x20the\x20log\x20output\x20in\x20`content`\n\n\x0c\n\x05\x04\n\
    \x02\0\x04\x12\x03w\x02\n\n\x0c\n\x05\x04\n\x02\0\x05\x12\x03w\x0b\x11\n\
    \x0c\n\x05\x04\n\x02\0\x01\x12\x03w\x12\x17\n\x0c\n\x05\x04\n\x02\0\x03\
    \x12\x03w\x1a\x1b\nK\n\x04\x04\n\x02\x01\x12\x03y\x02\x1b\x1a>\x20Zero-i\
    ndexed\x20(exclusive)\x20line\x20of\x20the\x20log\x20output\x20in\x20`co\
    ntent`\n\n\x0c\n\x05\x04\n\x02\x01\x04\x12\x03y\x02\n\n\x0c\n\x05\x04\n\
    \x02\x01\x05\x12\x03y\x0b\x11\n\x0c\n\x05\x04\n\x02\x01\x01\x12\x03y\x12\
    \x16\n\x0c\n\x05\x04\n\x02\x01\x03\x12\x03y\x19\x1a\n\"\n\x04\x04\n\x02\
    \x02\x12\x03{\x02\x1e\x1a\x15\x20Lines\x20of\x20log\x20output\n\n\x0c\n\
    \x05\x04\n\x02\x02\x04\x12\x03{\x02\n\n\x0c\n\x05\x04\n\x02\x02\x05\x12\
    \x03{\x0b\x11\n\x0c\n\x05\x04\n\x02\x02\x01\x12\x03{\x12\x19\n\x0c\n\x05\
    \x04\n\x02\x02\x03\x12\x03{\x1c\x1d\n\x9c\x01\n\x04\x04\n\x02\x03\x12\
    \x03\x7f\x02\x20\x1a\x8e\x01\x20While\x20we\x20need\x20to\x20poll\x20for\
    \x20logs,\x20this\x20serves\x20as\x20an\x20indicator\x20to\n\x20clients\
    \x20if\x20they\x20need\x20to\x20continue\x20polling\x20to\x20receive\x20\
    more\x20logs,\x20or\n\x20can\x20stop.\n\n\x0c\n\x05\x04\n\x02\x03\x04\
    \x12\x03\x7f\x02\n\n\x0c\n\x05\x04\n\x02\x03\x05\x12\x03\x7f\x0b\x0f\n\
    \x0c\n\x05\x04\n\x02\x03\x01\x12\x03\x7f\x10\x1b\n\x0c\n\x05\x04\n\x02\
    \x03\x03\x12\x03\x7f\x1e\x1f\
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
