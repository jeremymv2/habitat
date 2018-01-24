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
pub struct ApplicationEnvironment {
    // message fields
    application: ::protobuf::SingularField<::std::string::String>,
    environment: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ApplicationEnvironment {}

impl ApplicationEnvironment {
    pub fn new() -> ApplicationEnvironment {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ApplicationEnvironment {
        static mut instance: ::protobuf::lazy::Lazy<ApplicationEnvironment> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ApplicationEnvironment,
        };
        unsafe {
            instance.get(ApplicationEnvironment::new)
        }
    }

    // required string application = 1;

    pub fn clear_application(&mut self) {
        self.application.clear();
    }

    pub fn has_application(&self) -> bool {
        self.application.is_some()
    }

    // Param is passed by value, moved
    pub fn set_application(&mut self, v: ::std::string::String) {
        self.application = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_application(&mut self) -> &mut ::std::string::String {
        if self.application.is_none() {
            self.application.set_default();
        }
        self.application.as_mut().unwrap()
    }

    // Take field
    pub fn take_application(&mut self) -> ::std::string::String {
        self.application.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_application(&self) -> &str {
        match self.application.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_application_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.application
    }

    fn mut_application_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.application
    }

    // optional string environment = 2;

    pub fn clear_environment(&mut self) {
        self.environment.clear();
    }

    pub fn has_environment(&self) -> bool {
        self.environment.is_some()
    }

    // Param is passed by value, moved
    pub fn set_environment(&mut self, v: ::std::string::String) {
        self.environment = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_environment(&mut self) -> &mut ::std::string::String {
        if self.environment.is_none() {
            self.environment.set_default();
        }
        self.environment.as_mut().unwrap()
    }

    // Take field
    pub fn take_environment(&mut self) -> ::std::string::String {
        self.environment.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_environment(&self) -> &str {
        match self.environment.as_ref() {
            Some(v) => &v,
            None => "default",
        }
    }

    fn get_environment_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.environment
    }

    fn mut_environment_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.environment
    }
}

impl ::protobuf::Message for ApplicationEnvironment {
    fn is_initialized(&self) -> bool {
        if self.application.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.application)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.environment)?;
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
        if let Some(ref v) = self.application.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.environment.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.application.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.environment.as_ref() {
            os.write_string(2, &v)?;
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

impl ::protobuf::MessageStatic for ApplicationEnvironment {
    fn new() -> ApplicationEnvironment {
        ApplicationEnvironment::new()
    }

    fn descriptor_static(_: ::std::option::Option<ApplicationEnvironment>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "application",
                    ApplicationEnvironment::get_application_for_reflect,
                    ApplicationEnvironment::mut_application_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "environment",
                    ApplicationEnvironment::get_environment_for_reflect,
                    ApplicationEnvironment::mut_environment_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ApplicationEnvironment>(
                    "ApplicationEnvironment",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ApplicationEnvironment {
    fn clear(&mut self) {
        self.clear_application();
        self.clear_environment();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ApplicationEnvironment {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ApplicationEnvironment {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

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
        unsafe {
            instance.get(PackageIdent::new)
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
        }
        self.version.as_mut().unwrap()
    }

    // Take field
    pub fn take_version(&mut self) -> ::std::string::String {
        self.version.take().unwrap_or_else(|| ::std::string::String::new())
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
        }
        self.release.as_mut().unwrap()
    }

    // Take field
    pub fn take_release(&mut self) -> ::std::string::String {
        self.release.take().unwrap_or_else(|| ::std::string::String::new())
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.version)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.release)?;
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
        if let Some(ref v) = self.version.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        if let Some(ref v) = self.release.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
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
        if let Some(ref v) = self.version.as_ref() {
            os.write_string(3, &v)?;
        }
        if let Some(ref v) = self.release.as_ref() {
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

impl ::protobuf::MessageStatic for PackageIdent {
    fn new() -> PackageIdent {
        PackageIdent::new()
    }

    fn descriptor_static(_: ::std::option::Option<PackageIdent>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
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
pub struct ServiceBind {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    service_group: ::protobuf::SingularPtrField<ServiceGroup>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ServiceBind {}

impl ServiceBind {
    pub fn new() -> ServiceBind {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ServiceBind {
        static mut instance: ::protobuf::lazy::Lazy<ServiceBind> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ServiceBind,
        };
        unsafe {
            instance.get(ServiceBind::new)
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

    // required .ServiceGroup service_group = 2;

    pub fn clear_service_group(&mut self) {
        self.service_group.clear();
    }

    pub fn has_service_group(&self) -> bool {
        self.service_group.is_some()
    }

    // Param is passed by value, moved
    pub fn set_service_group(&mut self, v: ServiceGroup) {
        self.service_group = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_service_group(&mut self) -> &mut ServiceGroup {
        if self.service_group.is_none() {
            self.service_group.set_default();
        }
        self.service_group.as_mut().unwrap()
    }

    // Take field
    pub fn take_service_group(&mut self) -> ServiceGroup {
        self.service_group.take().unwrap_or_else(|| ServiceGroup::new())
    }

    pub fn get_service_group(&self) -> &ServiceGroup {
        self.service_group.as_ref().unwrap_or_else(|| ServiceGroup::default_instance())
    }

    fn get_service_group_for_reflect(&self) -> &::protobuf::SingularPtrField<ServiceGroup> {
        &self.service_group
    }

    fn mut_service_group_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ServiceGroup> {
        &mut self.service_group
    }
}

impl ::protobuf::Message for ServiceBind {
    fn is_initialized(&self) -> bool {
        if self.name.is_none() {
            return false;
        }
        if self.service_group.is_none() {
            return false;
        }
        for v in &self.service_group {
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.service_group)?;
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
        if let Some(ref v) = self.service_group.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.service_group.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for ServiceBind {
    fn new() -> ServiceBind {
        ServiceBind::new()
    }

    fn descriptor_static(_: ::std::option::Option<ServiceBind>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    ServiceBind::get_name_for_reflect,
                    ServiceBind::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ServiceGroup>>(
                    "service_group",
                    ServiceBind::get_service_group_for_reflect,
                    ServiceBind::mut_service_group_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ServiceBind>(
                    "ServiceBind",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ServiceBind {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_service_group();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ServiceBind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ServiceBind {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ServiceGroup {
    // message fields
    service: ::protobuf::SingularField<::std::string::String>,
    group: ::protobuf::SingularField<::std::string::String>,
    application_environment: ::protobuf::SingularPtrField<ApplicationEnvironment>,
    organization: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ServiceGroup {}

impl ServiceGroup {
    pub fn new() -> ServiceGroup {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ServiceGroup {
        static mut instance: ::protobuf::lazy::Lazy<ServiceGroup> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ServiceGroup,
        };
        unsafe {
            instance.get(ServiceGroup::new)
        }
    }

    // required string service = 1;

    pub fn clear_service(&mut self) {
        self.service.clear();
    }

    pub fn has_service(&self) -> bool {
        self.service.is_some()
    }

    // Param is passed by value, moved
    pub fn set_service(&mut self, v: ::std::string::String) {
        self.service = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_service(&mut self) -> &mut ::std::string::String {
        if self.service.is_none() {
            self.service.set_default();
        }
        self.service.as_mut().unwrap()
    }

    // Take field
    pub fn take_service(&mut self) -> ::std::string::String {
        self.service.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_service(&self) -> &str {
        match self.service.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_service_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.service
    }

    fn mut_service_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.service
    }

    // optional string group = 2;

    pub fn clear_group(&mut self) {
        self.group.clear();
    }

    pub fn has_group(&self) -> bool {
        self.group.is_some()
    }

    // Param is passed by value, moved
    pub fn set_group(&mut self, v: ::std::string::String) {
        self.group = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_group(&mut self) -> &mut ::std::string::String {
        if self.group.is_none() {
            self.group.set_default();
        }
        self.group.as_mut().unwrap()
    }

    // Take field
    pub fn take_group(&mut self) -> ::std::string::String {
        self.group.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_group(&self) -> &str {
        match self.group.as_ref() {
            Some(v) => &v,
            None => "default",
        }
    }

    fn get_group_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.group
    }

    fn mut_group_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.group
    }

    // optional .ApplicationEnvironment application_environment = 3;

    pub fn clear_application_environment(&mut self) {
        self.application_environment.clear();
    }

    pub fn has_application_environment(&self) -> bool {
        self.application_environment.is_some()
    }

    // Param is passed by value, moved
    pub fn set_application_environment(&mut self, v: ApplicationEnvironment) {
        self.application_environment = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_application_environment(&mut self) -> &mut ApplicationEnvironment {
        if self.application_environment.is_none() {
            self.application_environment.set_default();
        }
        self.application_environment.as_mut().unwrap()
    }

    // Take field
    pub fn take_application_environment(&mut self) -> ApplicationEnvironment {
        self.application_environment.take().unwrap_or_else(|| ApplicationEnvironment::new())
    }

    pub fn get_application_environment(&self) -> &ApplicationEnvironment {
        self.application_environment.as_ref().unwrap_or_else(|| ApplicationEnvironment::default_instance())
    }

    fn get_application_environment_for_reflect(&self) -> &::protobuf::SingularPtrField<ApplicationEnvironment> {
        &self.application_environment
    }

    fn mut_application_environment_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ApplicationEnvironment> {
        &mut self.application_environment
    }

    // optional string organization = 4;

    pub fn clear_organization(&mut self) {
        self.organization.clear();
    }

    pub fn has_organization(&self) -> bool {
        self.organization.is_some()
    }

    // Param is passed by value, moved
    pub fn set_organization(&mut self, v: ::std::string::String) {
        self.organization = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_organization(&mut self) -> &mut ::std::string::String {
        if self.organization.is_none() {
            self.organization.set_default();
        }
        self.organization.as_mut().unwrap()
    }

    // Take field
    pub fn take_organization(&mut self) -> ::std::string::String {
        self.organization.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_organization(&self) -> &str {
        match self.organization.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_organization_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.organization
    }

    fn mut_organization_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.organization
    }
}

impl ::protobuf::Message for ServiceGroup {
    fn is_initialized(&self) -> bool {
        if self.service.is_none() {
            return false;
        }
        for v in &self.application_environment {
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.service)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.group)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.application_environment)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.organization)?;
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
        if let Some(ref v) = self.service.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.group.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(ref v) = self.application_environment.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.organization.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.service.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.group.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(ref v) = self.application_environment.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.organization.as_ref() {
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

impl ::protobuf::MessageStatic for ServiceGroup {
    fn new() -> ServiceGroup {
        ServiceGroup::new()
    }

    fn descriptor_static(_: ::std::option::Option<ServiceGroup>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "service",
                    ServiceGroup::get_service_for_reflect,
                    ServiceGroup::mut_service_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "group",
                    ServiceGroup::get_group_for_reflect,
                    ServiceGroup::mut_group_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ApplicationEnvironment>>(
                    "application_environment",
                    ServiceGroup::get_application_environment_for_reflect,
                    ServiceGroup::mut_application_environment_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "organization",
                    ServiceGroup::get_organization_for_reflect,
                    ServiceGroup::mut_organization_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ServiceGroup>(
                    "ServiceGroup",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ServiceGroup {
    fn clear(&mut self) {
        self.clear_service();
        self.clear_group();
        self.clear_application_environment();
        self.clear_organization();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ServiceGroup {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ServiceGroup {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum InstallSource {
    Ident = 0,
    Archive = 1,
}

impl ::protobuf::ProtobufEnum for InstallSource {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<InstallSource> {
        match value {
            0 => ::std::option::Option::Some(InstallSource::Ident),
            1 => ::std::option::Option::Some(InstallSource::Archive),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [InstallSource] = &[
            InstallSource::Ident,
            InstallSource::Archive,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<InstallSource>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("InstallSource", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for InstallSource {
}

impl ::protobuf::reflect::ProtobufValue for InstallSource {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Topology {
    Standalone = 0,
    Leader = 1,
}

impl ::protobuf::ProtobufEnum for Topology {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Topology> {
        match value {
            0 => ::std::option::Option::Some(Topology::Standalone),
            1 => ::std::option::Option::Some(Topology::Leader),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Topology] = &[
            Topology::Standalone,
            Topology::Leader,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<Topology>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Topology", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Topology {
}

impl ::protobuf::reflect::ProtobufValue for Topology {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum UpdateStrategy {
    None = 0,
    AtOnce = 1,
    Rolling = 2,
}

impl ::protobuf::ProtobufEnum for UpdateStrategy {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<UpdateStrategy> {
        match value {
            0 => ::std::option::Option::Some(UpdateStrategy::None),
            1 => ::std::option::Option::Some(UpdateStrategy::AtOnce),
            2 => ::std::option::Option::Some(UpdateStrategy::Rolling),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [UpdateStrategy] = &[
            UpdateStrategy::None,
            UpdateStrategy::AtOnce,
            UpdateStrategy::Rolling,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<UpdateStrategy>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("UpdateStrategy", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for UpdateStrategy {
}

impl ::protobuf::reflect::ProtobufValue for UpdateStrategy {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0btypes.proto\"e\n\x16ApplicationEnvironment\x12\x20\n\x0bapplicatio\
    n\x18\x01\x20\x02(\tR\x0bapplication\x12)\n\x0benvironment\x18\x02\x20\
    \x01(\t:\x07defaultR\x0benvironment\"n\n\x0cPackageIdent\x12\x16\n\x06or\
    igin\x18\x01\x20\x01(\tR\x06origin\x12\x12\n\x04name\x18\x02\x20\x01(\tR\
    \x04name\x12\x18\n\x07version\x18\x03\x20\x01(\tR\x07version\x12\x18\n\
    \x07release\x18\x04\x20\x01(\tR\x07release\"U\n\x0bServiceBind\x12\x12\n\
    \x04name\x18\x01\x20\x02(\tR\x04name\x122\n\rservice_group\x18\x02\x20\
    \x02(\x0b2\r.ServiceGroupR\x0cserviceGroup\"\xbd\x01\n\x0cServiceGroup\
    \x12\x18\n\x07service\x18\x01\x20\x02(\tR\x07service\x12\x1d\n\x05group\
    \x18\x02\x20\x01(\t:\x07defaultR\x05group\x12P\n\x17application_environm\
    ent\x18\x03\x20\x01(\x0b2\x17.ApplicationEnvironmentR\x16applicationEnvi\
    ronment\x12\"\n\x0corganization\x18\x04\x20\x01(\tR\x0corganization*'\n\
    \rInstallSource\x12\t\n\x05Ident\x10\0\x12\x0b\n\x07Archive\x10\x01*&\n\
    \x08Topology\x12\x0e\n\nStandalone\x10\0\x12\n\n\x06Leader\x10\x01*3\n\
    \x0eUpdateStrategy\x12\x08\n\x04None\x10\0\x12\n\n\x06AtOnce\x10\x01\x12\
    \x0b\n\x07Rolling\x10\x02\
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
