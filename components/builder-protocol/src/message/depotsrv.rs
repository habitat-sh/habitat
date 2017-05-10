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
pub struct PackageIdent {
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
unsafe impl ::std::marker::Sync for PackageIdent {}

impl PackageIdent {
    pub fn new() -> PackageIdent {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PackageIdent {
        static mut instance: ::protobuf::lazy::Lazy<PackageIdent> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PackageIdent,
        };
        unsafe { instance.get(PackageIdent::new) }
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
        };
        self.origin.as_mut().unwrap()
    }

    // Take field
    pub fn take_origin(&mut self) -> ::std::string::String {
        self.origin
            .take()
            .unwrap_or_else(|| ::std::string::String::new())
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
        };
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name
            .take()
            .unwrap_or_else(|| ::std::string::String::new())
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
        };
        self.version.as_mut().unwrap()
    }

    // Take field
    pub fn take_version(&mut self) -> ::std::string::String {
        self.version
            .take()
            .unwrap_or_else(|| ::std::string::String::new())
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
        };
        self.release.as_mut().unwrap()
    }

    // Take field
    pub fn take_release(&mut self) -> ::std::string::String {
        self.release
            .take()
            .unwrap_or_else(|| ::std::string::String::new())
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

impl ::protobuf::Message for PackageIdent {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self,
                  is: &mut ::protobuf::CodedInputStream)
                  -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.origin)?;
                }
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                }
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.version)?;
                }
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.release)?;
                }
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number,
                                                               wire_type,
                                                               is,
                                                               self.mut_unknown_fields())?;
                }
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.origin.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        if let Some(v) = self.version.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        };
        if let Some(v) = self.release.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self,
                                  os: &mut ::protobuf::CodedOutputStream)
                                  -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.origin.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.name.as_ref() {
            os.write_string(2, &v)?;
        };
        if let Some(v) = self.version.as_ref() {
            os.write_string(3, &v)?;
        };
        if let Some(v) = self.release.as_ref() {
            os.write_string(4, &v)?;
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

impl ::protobuf::MessageStatic for PackageIdent {
    fn new() -> PackageIdent {
        PackageIdent::new()
    }

    fn descriptor_static(_: ::std::option::Option<PackageIdent>)
                         -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> =
            ::protobuf::lazy::Lazy {
                lock: ::protobuf::lazy::ONCE_INIT,
                ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
            };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "origin",
                    PackageIdent::get_origin_for_reflect,
                    PackageIdent::mut_origin_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    PackageIdent::get_name_for_reflect,
                    PackageIdent::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "version",
                    PackageIdent::get_version_for_reflect,
                    PackageIdent::mut_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "release",
                    PackageIdent::get_release_for_reflect,
                    PackageIdent::mut_release_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PackageIdent>(
                    "PackageIdent",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PackageIdent {
    fn clear(&mut self) {
        self.clear_origin();
        self.clear_name();
        self.clear_version();
        self.clear_release();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PackageIdent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PackageIdent {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Package {
    // message fields
    ident: ::protobuf::SingularPtrField<PackageIdent>,
    checksum: ::protobuf::SingularField<::std::string::String>,
    manifest: ::protobuf::SingularField<::std::string::String>,
    deps: ::protobuf::RepeatedField<PackageIdent>,
    tdeps: ::protobuf::RepeatedField<PackageIdent>,
    exposes: ::std::vec::Vec<u32>,
    config: ::protobuf::SingularField<::std::string::String>,
    target: ::protobuf::SingularField<::std::string::String>,
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
        unsafe { instance.get(Package::new) }
    }

    // optional .depotsrv.PackageIdent ident = 1;

    pub fn clear_ident(&mut self) {
        self.ident.clear();
    }

    pub fn has_ident(&self) -> bool {
        self.ident.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ident(&mut self, v: PackageIdent) {
        self.ident = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ident(&mut self) -> &mut PackageIdent {
        if self.ident.is_none() {
            self.ident.set_default();
        };
        self.ident.as_mut().unwrap()
    }

    // Take field
    pub fn take_ident(&mut self) -> PackageIdent {
        self.ident.take().unwrap_or_else(|| PackageIdent::new())
    }

    pub fn get_ident(&self) -> &PackageIdent {
        self.ident
            .as_ref()
            .unwrap_or_else(|| PackageIdent::default_instance())
    }

    fn get_ident_for_reflect(&self) -> &::protobuf::SingularPtrField<PackageIdent> {
        &self.ident
    }

    fn mut_ident_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<PackageIdent> {
        &mut self.ident
    }

    // optional string checksum = 2;

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
        };
        self.checksum.as_mut().unwrap()
    }

    // Take field
    pub fn take_checksum(&mut self) -> ::std::string::String {
        self.checksum
            .take()
            .unwrap_or_else(|| ::std::string::String::new())
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

    fn mut_checksum_for_reflect(&mut self)
                                -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.checksum
    }

    // optional string manifest = 3;

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
        };
        self.manifest.as_mut().unwrap()
    }

    // Take field
    pub fn take_manifest(&mut self) -> ::std::string::String {
        self.manifest
            .take()
            .unwrap_or_else(|| ::std::string::String::new())
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

    fn mut_manifest_for_reflect(&mut self)
                                -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.manifest
    }

    // repeated .depotsrv.PackageIdent deps = 4;

    pub fn clear_deps(&mut self) {
        self.deps.clear();
    }

    // Param is passed by value, moved
    pub fn set_deps(&mut self, v: ::protobuf::RepeatedField<PackageIdent>) {
        self.deps = v;
    }

    // Mutable pointer to the field.
    pub fn mut_deps(&mut self) -> &mut ::protobuf::RepeatedField<PackageIdent> {
        &mut self.deps
    }

    // Take field
    pub fn take_deps(&mut self) -> ::protobuf::RepeatedField<PackageIdent> {
        ::std::mem::replace(&mut self.deps, ::protobuf::RepeatedField::new())
    }

    pub fn get_deps(&self) -> &[PackageIdent] {
        &self.deps
    }

    fn get_deps_for_reflect(&self) -> &::protobuf::RepeatedField<PackageIdent> {
        &self.deps
    }

    fn mut_deps_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<PackageIdent> {
        &mut self.deps
    }

    // repeated .depotsrv.PackageIdent tdeps = 5;

    pub fn clear_tdeps(&mut self) {
        self.tdeps.clear();
    }

    // Param is passed by value, moved
    pub fn set_tdeps(&mut self, v: ::protobuf::RepeatedField<PackageIdent>) {
        self.tdeps = v;
    }

    // Mutable pointer to the field.
    pub fn mut_tdeps(&mut self) -> &mut ::protobuf::RepeatedField<PackageIdent> {
        &mut self.tdeps
    }

    // Take field
    pub fn take_tdeps(&mut self) -> ::protobuf::RepeatedField<PackageIdent> {
        ::std::mem::replace(&mut self.tdeps, ::protobuf::RepeatedField::new())
    }

    pub fn get_tdeps(&self) -> &[PackageIdent] {
        &self.tdeps
    }

    fn get_tdeps_for_reflect(&self) -> &::protobuf::RepeatedField<PackageIdent> {
        &self.tdeps
    }

    fn mut_tdeps_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<PackageIdent> {
        &mut self.tdeps
    }

    // repeated uint32 exposes = 6;

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

    // optional string config = 7;

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
        };
        self.config.as_mut().unwrap()
    }

    // Take field
    pub fn take_config(&mut self) -> ::std::string::String {
        self.config
            .take()
            .unwrap_or_else(|| ::std::string::String::new())
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

    // optional string target = 8;

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
        };
        self.target.as_mut().unwrap()
    }

    // Take field
    pub fn take_target(&mut self) -> ::std::string::String {
        self.target
            .take()
            .unwrap_or_else(|| ::std::string::String::new())
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

impl ::protobuf::Message for Package {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self,
                  is: &mut ::protobuf::CodedInputStream)
                  -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.ident)?;
                }
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.checksum)?;
                }
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.manifest)?;
                }
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.deps)?;
                }
                5 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.tdeps)?;
                }
                6 => {
                    ::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.exposes)?;
                }
                7 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.config)?;
                }
                8 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.target)?;
                }
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number,
                                                               wire_type,
                                                               is,
                                                               self.mut_unknown_fields())?;
                }
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.ident.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.checksum.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        if let Some(v) = self.manifest.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        };
        for value in &self.deps {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        for value in &self.tdeps {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.exposes.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_size(6, &self.exposes);
        };
        if let Some(v) = self.config.as_ref() {
            my_size += ::protobuf::rt::string_size(7, &v);
        };
        if let Some(v) = self.target.as_ref() {
            my_size += ::protobuf::rt::string_size(8, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self,
                                  os: &mut ::protobuf::CodedOutputStream)
                                  -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.ident.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.checksum.as_ref() {
            os.write_string(2, &v)?;
        };
        if let Some(v) = self.manifest.as_ref() {
            os.write_string(3, &v)?;
        };
        for v in &self.deps {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        for v in &self.tdeps {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if !self.exposes.is_empty() {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            // TODO: Data size is computed again, it should be cached
            os.write_raw_varint32(::protobuf::rt::vec_packed_varint_data_size(&self.exposes))?;
            for v in &self.exposes {
                os.write_uint32_no_tag(*v)?;
            }
        };
        if let Some(v) = self.config.as_ref() {
            os.write_string(7, &v)?;
        };
        if let Some(v) = self.target.as_ref() {
            os.write_string(8, &v)?;
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

    fn descriptor_static(_: ::std::option::Option<Package>)
                         -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> =
            ::protobuf::lazy::Lazy {
                lock: ::protobuf::lazy::ONCE_INIT,
                ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
            };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<PackageIdent>>(
                    "ident",
                    Package::get_ident_for_reflect,
                    Package::mut_ident_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "checksum",
                    Package::get_checksum_for_reflect,
                    Package::mut_checksum_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "manifest",
                    Package::get_manifest_for_reflect,
                    Package::mut_manifest_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<PackageIdent>>(
                    "deps",
                    Package::get_deps_for_reflect,
                    Package::mut_deps_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<PackageIdent>>(
                    "tdeps",
                    Package::get_tdeps_for_reflect,
                    Package::mut_tdeps_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "exposes",
                    Package::get_exposes_for_reflect,
                    Package::mut_exposes_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "config",
                    Package::get_config_for_reflect,
                    Package::mut_config_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "target",
                    Package::get_target_for_reflect,
                    Package::mut_target_for_reflect,
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
pub struct View {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for View {}

impl View {
    pub fn new() -> View {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static View {
        static mut instance: ::protobuf::lazy::Lazy<View> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const View,
        };
        unsafe { instance.get(View::new) }
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
        self.name
            .take()
            .unwrap_or_else(|| ::std::string::String::new())
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

impl ::protobuf::Message for View {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self,
                  is: &mut ::protobuf::CodedInputStream)
                  -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                }
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number,
                                                               wire_type,
                                                               is,
                                                               self.mut_unknown_fields())?;
                }
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

    fn write_to_with_cached_sizes(&self,
                                  os: &mut ::protobuf::CodedOutputStream)
                                  -> ::protobuf::ProtobufResult<()> {
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

impl ::protobuf::MessageStatic for View {
    fn new() -> View {
        View::new()
    }

    fn descriptor_static(_: ::std::option::Option<View>)
                         -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> =
            ::protobuf::lazy::Lazy {
                lock: ::protobuf::lazy::ONCE_INIT,
                ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
            };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    View::get_name_for_reflect,
                    View::mut_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<View>(
                    "View",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for View {
    fn clear(&mut self) {
        self.clear_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for View {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for View {
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
        unsafe { instance.get(OriginKeyIdent::new) }
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
        };
        self.origin.as_mut().unwrap()
    }

    // Take field
    pub fn take_origin(&mut self) -> ::std::string::String {
        self.origin
            .take()
            .unwrap_or_else(|| ::std::string::String::new())
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
        };
        self.revision.as_mut().unwrap()
    }

    // Take field
    pub fn take_revision(&mut self) -> ::std::string::String {
        self.revision
            .take()
            .unwrap_or_else(|| ::std::string::String::new())
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

    fn mut_revision_for_reflect(&mut self)
                                -> &mut ::protobuf::SingularField<::std::string::String> {
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
        };
        self.location.as_mut().unwrap()
    }

    // Take field
    pub fn take_location(&mut self) -> ::std::string::String {
        self.location
            .take()
            .unwrap_or_else(|| ::std::string::String::new())
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

    fn mut_location_for_reflect(&mut self)
                                -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.location
    }
}

impl ::protobuf::Message for OriginKeyIdent {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self,
                  is: &mut ::protobuf::CodedInputStream)
                  -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.origin)?;
                }
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.revision)?;
                }
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.location)?;
                }
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number,
                                                               wire_type,
                                                               is,
                                                               self.mut_unknown_fields())?;
                }
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.origin.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.revision.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        if let Some(v) = self.location.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self,
                                  os: &mut ::protobuf::CodedOutputStream)
                                  -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.origin.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.revision.as_ref() {
            os.write_string(2, &v)?;
        };
        if let Some(v) = self.location.as_ref() {
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

impl ::protobuf::MessageStatic for OriginKeyIdent {
    fn new() -> OriginKeyIdent {
        OriginKeyIdent::new()
    }

    fn descriptor_static(_: ::std::option::Option<OriginKeyIdent>)
                         -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> =
            ::protobuf::lazy::Lazy {
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

static file_descriptor_proto_data: &'static [u8] =
    &[0x0a, 0x18, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x73, 0x2f, 0x64, 0x65, 0x70,
      0x6f, 0x74, 0x73, 0x72, 0x76, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x08, 0x64, 0x65,
      0x70, 0x6f, 0x74, 0x73, 0x72, 0x76, 0x22, 0x4e, 0x0a, 0x0c, 0x50, 0x61, 0x63, 0x6b, 0x61,
      0x67, 0x65, 0x49, 0x64, 0x65, 0x6e, 0x74, 0x12, 0x0e, 0x0a, 0x06, 0x6f, 0x72, 0x69, 0x67,
      0x69, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x12, 0x0c, 0x0a, 0x04, 0x6e, 0x61, 0x6d,
      0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x12, 0x0f, 0x0a, 0x07, 0x76, 0x65, 0x72, 0x73,
      0x69, 0x6f, 0x6e, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x12, 0x0f, 0x0a, 0x07, 0x72, 0x65,
      0x6c, 0x65, 0x61, 0x73, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09, 0x22, 0xd6, 0x01, 0x0a,
      0x07, 0x50, 0x61, 0x63, 0x6b, 0x61, 0x67, 0x65, 0x12, 0x25, 0x0a, 0x05, 0x69, 0x64, 0x65,
      0x6e, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x16, 0x2e, 0x64, 0x65, 0x70, 0x6f,
      0x74, 0x73, 0x72, 0x76, 0x2e, 0x50, 0x61, 0x63, 0x6b, 0x61, 0x67, 0x65, 0x49, 0x64, 0x65,
      0x6e, 0x74, 0x12, 0x10, 0x0a, 0x08, 0x63, 0x68, 0x65, 0x63, 0x6b, 0x73, 0x75, 0x6d, 0x18,
      0x02, 0x20, 0x01, 0x28, 0x09, 0x12, 0x10, 0x0a, 0x08, 0x6d, 0x61, 0x6e, 0x69, 0x66, 0x65,
      0x73, 0x74, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x12, 0x24, 0x0a, 0x04, 0x64, 0x65, 0x70,
      0x73, 0x18, 0x04, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x16, 0x2e, 0x64, 0x65, 0x70, 0x6f, 0x74,
      0x73, 0x72, 0x76, 0x2e, 0x50, 0x61, 0x63, 0x6b, 0x61, 0x67, 0x65, 0x49, 0x64, 0x65, 0x6e,
      0x74, 0x12, 0x25, 0x0a, 0x05, 0x74, 0x64, 0x65, 0x70, 0x73, 0x18, 0x05, 0x20, 0x03, 0x28,
      0x0b, 0x32, 0x16, 0x2e, 0x64, 0x65, 0x70, 0x6f, 0x74, 0x73, 0x72, 0x76, 0x2e, 0x50, 0x61,
      0x63, 0x6b, 0x61, 0x67, 0x65, 0x49, 0x64, 0x65, 0x6e, 0x74, 0x12, 0x13, 0x0a, 0x07, 0x65,
      0x78, 0x70, 0x6f, 0x73, 0x65, 0x73, 0x18, 0x06, 0x20, 0x03, 0x28, 0x0d, 0x42, 0x02, 0x10,
      0x01, 0x12, 0x0e, 0x0a, 0x06, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x18, 0x07, 0x20, 0x01,
      0x28, 0x09, 0x12, 0x0e, 0x0a, 0x06, 0x74, 0x61, 0x72, 0x67, 0x65, 0x74, 0x18, 0x08, 0x20,
      0x01, 0x28, 0x09, 0x22, 0x14, 0x0a, 0x04, 0x56, 0x69, 0x65, 0x77, 0x12, 0x0c, 0x0a, 0x04,
      0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x22, 0x44, 0x0a, 0x0e, 0x4f,
      0x72, 0x69, 0x67, 0x69, 0x6e, 0x4b, 0x65, 0x79, 0x49, 0x64, 0x65, 0x6e, 0x74, 0x12, 0x0e,
      0x0a, 0x06, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x12,
      0x10, 0x0a, 0x08, 0x72, 0x65, 0x76, 0x69, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x02, 0x20, 0x01,
      0x28, 0x09, 0x12, 0x10, 0x0a, 0x08, 0x6c, 0x6f, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x18,
      0x03, 0x20, 0x01, 0x28, 0x09, 0x4a, 0xac, 0x0a, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x1c,
      0x01, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x00, 0x08, 0x10, 0x0a, 0x0a, 0x0a, 0x02,
      0x04, 0x00, 0x12, 0x04, 0x02, 0x00, 0x07, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01,
      0x12, 0x03, 0x02, 0x08, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03,
      0x03, 0x02, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x03,
      0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x03, 0x0b,
      0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x03, 0x12, 0x18,
      0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x03, 0x1b, 0x1c, 0x0a,
      0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x04, 0x02, 0x1b, 0x0a, 0x0c, 0x0a,
      0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x04, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
      0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x04, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
      0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x04, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
      0x02, 0x01, 0x03, 0x12, 0x03, 0x04, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
      0x02, 0x12, 0x03, 0x05, 0x02, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04,
      0x12, 0x03, 0x05, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12,
      0x03, 0x05, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03,
      0x05, 0x12, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x05,
      0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x06, 0x02, 0x1e,
      0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x04, 0x12, 0x03, 0x06, 0x02, 0x0a, 0x0a,
      0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x05, 0x12, 0x03, 0x06, 0x0b, 0x11, 0x0a, 0x0c,
      0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x06, 0x12, 0x19, 0x0a, 0x0c, 0x0a,
      0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x06, 0x1c, 0x1d, 0x0a, 0x0a, 0x0a, 0x02,
      0x04, 0x01, 0x12, 0x04, 0x09, 0x00, 0x12, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01,
      0x12, 0x03, 0x09, 0x08, 0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03,
      0x0a, 0x02, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x0a,
      0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x06, 0x12, 0x03, 0x0a, 0x0b,
      0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0a, 0x18, 0x1d,
      0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0a, 0x20, 0x21, 0x0a,
      0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x0b, 0x02, 0x1f, 0x0a, 0x0c, 0x0a,
      0x05, 0x04, 0x01, 0x02, 0x01, 0x04, 0x12, 0x03, 0x0b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
      0x04, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x0b, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
      0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0b, 0x12, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
      0x02, 0x01, 0x03, 0x12, 0x03, 0x0b, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02,
      0x02, 0x12, 0x03, 0x0c, 0x02, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x04,
      0x12, 0x03, 0x0c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x05, 0x12,
      0x03, 0x0c, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03,
      0x0c, 0x12, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x03, 0x12, 0x03, 0x0c,
      0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x03, 0x12, 0x03, 0x0d, 0x02, 0x21,
      0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x04, 0x12, 0x03, 0x0d, 0x02, 0x0a, 0x0a,
      0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x06, 0x12, 0x03, 0x0d, 0x0b, 0x17, 0x0a, 0x0c,
      0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x01, 0x12, 0x03, 0x0d, 0x18, 0x1c, 0x0a, 0x0c, 0x0a,
      0x05, 0x04, 0x01, 0x02, 0x03, 0x03, 0x12, 0x03, 0x0d, 0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04,
      0x04, 0x01, 0x02, 0x04, 0x12, 0x03, 0x0e, 0x02, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
      0x02, 0x04, 0x04, 0x12, 0x03, 0x0e, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
      0x04, 0x06, 0x12, 0x03, 0x0e, 0x0b, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04,
      0x01, 0x12, 0x03, 0x0e, 0x18, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x03,
      0x12, 0x03, 0x0e, 0x20, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x05, 0x12, 0x03,
      0x0f, 0x02, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x04, 0x12, 0x03, 0x0f,
      0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x05, 0x12, 0x03, 0x0f, 0x0b,
      0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x01, 0x12, 0x03, 0x0f, 0x12, 0x19,
      0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x03, 0x12, 0x03, 0x0f, 0x1c, 0x1d, 0x0a,
      0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x08, 0x12, 0x03, 0x0f, 0x1e, 0x2b, 0x0a, 0x0f,
      0x0a, 0x08, 0x04, 0x01, 0x02, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x0f, 0x1f, 0x2a,
      0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x02, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03,
      0x0f, 0x1f, 0x25, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x01, 0x02, 0x05, 0x08, 0xe7, 0x07, 0x00,
      0x02, 0x00, 0x12, 0x03, 0x0f, 0x1f, 0x25, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x01, 0x02, 0x05,
      0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0f, 0x1f, 0x25, 0x0a, 0x10, 0x0a,
      0x09, 0x04, 0x01, 0x02, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x0f, 0x26, 0x2a,
      0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x06, 0x12, 0x03, 0x10, 0x02, 0x1d, 0x0a, 0x0c,
      0x0a, 0x05, 0x04, 0x01, 0x02, 0x06, 0x04, 0x12, 0x03, 0x10, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
      0x05, 0x04, 0x01, 0x02, 0x06, 0x05, 0x12, 0x03, 0x10, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05,
      0x04, 0x01, 0x02, 0x06, 0x01, 0x12, 0x03, 0x10, 0x12, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
      0x01, 0x02, 0x06, 0x03, 0x12, 0x03, 0x10, 0x1b, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01,
      0x02, 0x07, 0x12, 0x03, 0x11, 0x02, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x07,
      0x04, 0x12, 0x03, 0x11, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x07, 0x05,
      0x12, 0x03, 0x11, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x07, 0x01, 0x12,
      0x03, 0x11, 0x12, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x07, 0x03, 0x12, 0x03,
      0x11, 0x1b, 0x1c, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x14, 0x00, 0x16, 0x01,
      0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x14, 0x08, 0x0c, 0x0a, 0x0b, 0x0a,
      0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x15, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
      0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x15, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
      0x02, 0x00, 0x05, 0x12, 0x03, 0x15, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
      0x00, 0x01, 0x12, 0x03, 0x15, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00,
      0x03, 0x12, 0x03, 0x15, 0x19, 0x1a, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x18,
      0x00, 0x1c, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x18, 0x08, 0x16,
      0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x19, 0x02, 0x1d, 0x0a, 0x0c,
      0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x03, 0x19, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
      0x05, 0x04, 0x03, 0x02, 0x00, 0x05, 0x12, 0x03, 0x19, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05,
      0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x19, 0x12, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
      0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x19, 0x1b, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03,
      0x02, 0x01, 0x12, 0x03, 0x1a, 0x02, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01,
      0x04, 0x12, 0x03, 0x1a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x05,
      0x12, 0x03, 0x1a, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x01, 0x12,
      0x03, 0x1a, 0x12, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x03, 0x12, 0x03,
      0x1a, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x02, 0x12, 0x03, 0x1b, 0x02,
      0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x04, 0x12, 0x03, 0x1b, 0x02, 0x0a,
      0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x05, 0x12, 0x03, 0x1b, 0x0b, 0x11, 0x0a,
      0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x01, 0x12, 0x03, 0x1b, 0x12, 0x1a, 0x0a, 0x0c,
      0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x03, 0x12, 0x03, 0x1b, 0x1d, 0x1e];

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe { file_descriptor_proto_lazy.get(|| parse_descriptor_proto()) }
}
