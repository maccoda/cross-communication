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
pub struct InitiateRequest {
    // message fields
    clientAddress: ::protobuf::SingularPtrField<Address>,
    room: ::protobuf::SingularPtrField<Room>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for InitiateRequest {}

impl InitiateRequest {
    pub fn new() -> InitiateRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static InitiateRequest {
        static mut instance: ::protobuf::lazy::Lazy<InitiateRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const InitiateRequest,
        };
        unsafe {
            instance.get(InitiateRequest::new)
        }
    }

    // .Address clientAddress = 1;

    pub fn clear_clientAddress(&mut self) {
        self.clientAddress.clear();
    }

    pub fn has_clientAddress(&self) -> bool {
        self.clientAddress.is_some()
    }

    // Param is passed by value, moved
    pub fn set_clientAddress(&mut self, v: Address) {
        self.clientAddress = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_clientAddress(&mut self) -> &mut Address {
        if self.clientAddress.is_none() {
            self.clientAddress.set_default();
        };
        self.clientAddress.as_mut().unwrap()
    }

    // Take field
    pub fn take_clientAddress(&mut self) -> Address {
        self.clientAddress.take().unwrap_or_else(|| Address::new())
    }

    pub fn get_clientAddress(&self) -> &Address {
        self.clientAddress.as_ref().unwrap_or_else(|| Address::default_instance())
    }

    fn get_clientAddress_for_reflect(&self) -> &::protobuf::SingularPtrField<Address> {
        &self.clientAddress
    }

    fn mut_clientAddress_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Address> {
        &mut self.clientAddress
    }

    // .Room room = 2;

    pub fn clear_room(&mut self) {
        self.room.clear();
    }

    pub fn has_room(&self) -> bool {
        self.room.is_some()
    }

    // Param is passed by value, moved
    pub fn set_room(&mut self, v: Room) {
        self.room = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_room(&mut self) -> &mut Room {
        if self.room.is_none() {
            self.room.set_default();
        };
        self.room.as_mut().unwrap()
    }

    // Take field
    pub fn take_room(&mut self) -> Room {
        self.room.take().unwrap_or_else(|| Room::new())
    }

    pub fn get_room(&self) -> &Room {
        self.room.as_ref().unwrap_or_else(|| Room::default_instance())
    }

    fn get_room_for_reflect(&self) -> &::protobuf::SingularPtrField<Room> {
        &self.room
    }

    fn mut_room_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Room> {
        &mut self.room
    }
}

impl ::protobuf::Message for InitiateRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.clientAddress)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.room)?;
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
        if let Some(v) = self.clientAddress.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.room.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.clientAddress.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.room.as_ref() {
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

impl ::protobuf::MessageStatic for InitiateRequest {
    fn new() -> InitiateRequest {
        InitiateRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<InitiateRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Address>>(
                    "clientAddress",
                    InitiateRequest::get_clientAddress_for_reflect,
                    InitiateRequest::mut_clientAddress_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Room>>(
                    "room",
                    InitiateRequest::get_room_for_reflect,
                    InitiateRequest::mut_room_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<InitiateRequest>(
                    "InitiateRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for InitiateRequest {
    fn clear(&mut self) {
        self.clear_clientAddress();
        self.clear_room();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for InitiateRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for InitiateRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct InitiateReply {
    // message fields
    pub success: bool,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for InitiateReply {}

impl InitiateReply {
    pub fn new() -> InitiateReply {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static InitiateReply {
        static mut instance: ::protobuf::lazy::Lazy<InitiateReply> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const InitiateReply,
        };
        unsafe {
            instance.get(InitiateReply::new)
        }
    }

    // bool success = 1;

    pub fn clear_success(&mut self) {
        self.success = false;
    }

    // Param is passed by value, moved
    pub fn set_success(&mut self, v: bool) {
        self.success = v;
    }

    pub fn get_success(&self) -> bool {
        self.success
    }

    fn get_success_for_reflect(&self) -> &bool {
        &self.success
    }

    fn mut_success_for_reflect(&mut self) -> &mut bool {
        &mut self.success
    }
}

impl ::protobuf::Message for InitiateReply {
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
                    let tmp = is.read_bool()?;
                    self.success = tmp;
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
        if self.success != false {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.success != false {
            os.write_bool(1, self.success)?;
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

impl ::protobuf::MessageStatic for InitiateReply {
    fn new() -> InitiateReply {
        InitiateReply::new()
    }

    fn descriptor_static(_: ::std::option::Option<InitiateReply>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "success",
                    InitiateReply::get_success_for_reflect,
                    InitiateReply::mut_success_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<InitiateReply>(
                    "InitiateReply",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for InitiateReply {
    fn clear(&mut self) {
        self.clear_success();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for InitiateReply {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for InitiateReply {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TerminateRequest {
    // message fields
    clientAddress: ::protobuf::SingularPtrField<Address>,
    room: ::protobuf::SingularPtrField<Room>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TerminateRequest {}

impl TerminateRequest {
    pub fn new() -> TerminateRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TerminateRequest {
        static mut instance: ::protobuf::lazy::Lazy<TerminateRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TerminateRequest,
        };
        unsafe {
            instance.get(TerminateRequest::new)
        }
    }

    // .Address clientAddress = 1;

    pub fn clear_clientAddress(&mut self) {
        self.clientAddress.clear();
    }

    pub fn has_clientAddress(&self) -> bool {
        self.clientAddress.is_some()
    }

    // Param is passed by value, moved
    pub fn set_clientAddress(&mut self, v: Address) {
        self.clientAddress = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_clientAddress(&mut self) -> &mut Address {
        if self.clientAddress.is_none() {
            self.clientAddress.set_default();
        };
        self.clientAddress.as_mut().unwrap()
    }

    // Take field
    pub fn take_clientAddress(&mut self) -> Address {
        self.clientAddress.take().unwrap_or_else(|| Address::new())
    }

    pub fn get_clientAddress(&self) -> &Address {
        self.clientAddress.as_ref().unwrap_or_else(|| Address::default_instance())
    }

    fn get_clientAddress_for_reflect(&self) -> &::protobuf::SingularPtrField<Address> {
        &self.clientAddress
    }

    fn mut_clientAddress_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Address> {
        &mut self.clientAddress
    }

    // .Room room = 2;

    pub fn clear_room(&mut self) {
        self.room.clear();
    }

    pub fn has_room(&self) -> bool {
        self.room.is_some()
    }

    // Param is passed by value, moved
    pub fn set_room(&mut self, v: Room) {
        self.room = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_room(&mut self) -> &mut Room {
        if self.room.is_none() {
            self.room.set_default();
        };
        self.room.as_mut().unwrap()
    }

    // Take field
    pub fn take_room(&mut self) -> Room {
        self.room.take().unwrap_or_else(|| Room::new())
    }

    pub fn get_room(&self) -> &Room {
        self.room.as_ref().unwrap_or_else(|| Room::default_instance())
    }

    fn get_room_for_reflect(&self) -> &::protobuf::SingularPtrField<Room> {
        &self.room
    }

    fn mut_room_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Room> {
        &mut self.room
    }
}

impl ::protobuf::Message for TerminateRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.clientAddress)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.room)?;
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
        if let Some(v) = self.clientAddress.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.room.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.clientAddress.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.room.as_ref() {
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

impl ::protobuf::MessageStatic for TerminateRequest {
    fn new() -> TerminateRequest {
        TerminateRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<TerminateRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Address>>(
                    "clientAddress",
                    TerminateRequest::get_clientAddress_for_reflect,
                    TerminateRequest::mut_clientAddress_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Room>>(
                    "room",
                    TerminateRequest::get_room_for_reflect,
                    TerminateRequest::mut_room_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TerminateRequest>(
                    "TerminateRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TerminateRequest {
    fn clear(&mut self) {
        self.clear_clientAddress();
        self.clear_room();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TerminateRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TerminateRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TerminateReply {
    // message fields
    pub success: bool,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TerminateReply {}

impl TerminateReply {
    pub fn new() -> TerminateReply {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TerminateReply {
        static mut instance: ::protobuf::lazy::Lazy<TerminateReply> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TerminateReply,
        };
        unsafe {
            instance.get(TerminateReply::new)
        }
    }

    // bool success = 1;

    pub fn clear_success(&mut self) {
        self.success = false;
    }

    // Param is passed by value, moved
    pub fn set_success(&mut self, v: bool) {
        self.success = v;
    }

    pub fn get_success(&self) -> bool {
        self.success
    }

    fn get_success_for_reflect(&self) -> &bool {
        &self.success
    }

    fn mut_success_for_reflect(&mut self) -> &mut bool {
        &mut self.success
    }
}

impl ::protobuf::Message for TerminateReply {
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
                    let tmp = is.read_bool()?;
                    self.success = tmp;
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
        if self.success != false {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.success != false {
            os.write_bool(1, self.success)?;
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

impl ::protobuf::MessageStatic for TerminateReply {
    fn new() -> TerminateReply {
        TerminateReply::new()
    }

    fn descriptor_static(_: ::std::option::Option<TerminateReply>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "success",
                    TerminateReply::get_success_for_reflect,
                    TerminateReply::mut_success_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TerminateReply>(
                    "TerminateReply",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TerminateReply {
    fn clear(&mut self) {
        self.clear_success();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TerminateReply {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TerminateReply {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct MessageRequest {
    // message fields
    clientAddress: ::protobuf::SingularPtrField<Address>,
    pub message: ::std::string::String,
    room: ::protobuf::SingularPtrField<Room>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for MessageRequest {}

impl MessageRequest {
    pub fn new() -> MessageRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static MessageRequest {
        static mut instance: ::protobuf::lazy::Lazy<MessageRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const MessageRequest,
        };
        unsafe {
            instance.get(MessageRequest::new)
        }
    }

    // .Address clientAddress = 1;

    pub fn clear_clientAddress(&mut self) {
        self.clientAddress.clear();
    }

    pub fn has_clientAddress(&self) -> bool {
        self.clientAddress.is_some()
    }

    // Param is passed by value, moved
    pub fn set_clientAddress(&mut self, v: Address) {
        self.clientAddress = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_clientAddress(&mut self) -> &mut Address {
        if self.clientAddress.is_none() {
            self.clientAddress.set_default();
        };
        self.clientAddress.as_mut().unwrap()
    }

    // Take field
    pub fn take_clientAddress(&mut self) -> Address {
        self.clientAddress.take().unwrap_or_else(|| Address::new())
    }

    pub fn get_clientAddress(&self) -> &Address {
        self.clientAddress.as_ref().unwrap_or_else(|| Address::default_instance())
    }

    fn get_clientAddress_for_reflect(&self) -> &::protobuf::SingularPtrField<Address> {
        &self.clientAddress
    }

    fn mut_clientAddress_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Address> {
        &mut self.clientAddress
    }

    // string message = 2;

    pub fn clear_message(&mut self) {
        self.message.clear();
    }

    // Param is passed by value, moved
    pub fn set_message(&mut self, v: ::std::string::String) {
        self.message = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_message(&mut self) -> &mut ::std::string::String {
        &mut self.message
    }

    // Take field
    pub fn take_message(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.message, ::std::string::String::new())
    }

    pub fn get_message(&self) -> &str {
        &self.message
    }

    fn get_message_for_reflect(&self) -> &::std::string::String {
        &self.message
    }

    fn mut_message_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.message
    }

    // .Room room = 3;

    pub fn clear_room(&mut self) {
        self.room.clear();
    }

    pub fn has_room(&self) -> bool {
        self.room.is_some()
    }

    // Param is passed by value, moved
    pub fn set_room(&mut self, v: Room) {
        self.room = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_room(&mut self) -> &mut Room {
        if self.room.is_none() {
            self.room.set_default();
        };
        self.room.as_mut().unwrap()
    }

    // Take field
    pub fn take_room(&mut self) -> Room {
        self.room.take().unwrap_or_else(|| Room::new())
    }

    pub fn get_room(&self) -> &Room {
        self.room.as_ref().unwrap_or_else(|| Room::default_instance())
    }

    fn get_room_for_reflect(&self) -> &::protobuf::SingularPtrField<Room> {
        &self.room
    }

    fn mut_room_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Room> {
        &mut self.room
    }
}

impl ::protobuf::Message for MessageRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.clientAddress)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.message)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.room)?;
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
        if let Some(v) = self.clientAddress.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if !self.message.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.message);
        };
        if let Some(v) = self.room.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.clientAddress.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if !self.message.is_empty() {
            os.write_string(2, &self.message)?;
        };
        if let Some(v) = self.room.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for MessageRequest {
    fn new() -> MessageRequest {
        MessageRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<MessageRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Address>>(
                    "clientAddress",
                    MessageRequest::get_clientAddress_for_reflect,
                    MessageRequest::mut_clientAddress_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "message",
                    MessageRequest::get_message_for_reflect,
                    MessageRequest::mut_message_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Room>>(
                    "room",
                    MessageRequest::get_room_for_reflect,
                    MessageRequest::mut_room_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<MessageRequest>(
                    "MessageRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for MessageRequest {
    fn clear(&mut self) {
        self.clear_clientAddress();
        self.clear_message();
        self.clear_room();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for MessageRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MessageRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct MessageReply {
    // message fields
    messages: ::protobuf::RepeatedField<Message>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for MessageReply {}

impl MessageReply {
    pub fn new() -> MessageReply {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static MessageReply {
        static mut instance: ::protobuf::lazy::Lazy<MessageReply> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const MessageReply,
        };
        unsafe {
            instance.get(MessageReply::new)
        }
    }

    // repeated .Message messages = 2;

    pub fn clear_messages(&mut self) {
        self.messages.clear();
    }

    // Param is passed by value, moved
    pub fn set_messages(&mut self, v: ::protobuf::RepeatedField<Message>) {
        self.messages = v;
    }

    // Mutable pointer to the field.
    pub fn mut_messages(&mut self) -> &mut ::protobuf::RepeatedField<Message> {
        &mut self.messages
    }

    // Take field
    pub fn take_messages(&mut self) -> ::protobuf::RepeatedField<Message> {
        ::std::mem::replace(&mut self.messages, ::protobuf::RepeatedField::new())
    }

    pub fn get_messages(&self) -> &[Message] {
        &self.messages
    }

    fn get_messages_for_reflect(&self) -> &::protobuf::RepeatedField<Message> {
        &self.messages
    }

    fn mut_messages_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Message> {
        &mut self.messages
    }
}

impl ::protobuf::Message for MessageReply {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.messages)?;
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
        for value in &self.messages {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.messages {
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

impl ::protobuf::MessageStatic for MessageReply {
    fn new() -> MessageReply {
        MessageReply::new()
    }

    fn descriptor_static(_: ::std::option::Option<MessageReply>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Message>>(
                    "messages",
                    MessageReply::get_messages_for_reflect,
                    MessageReply::mut_messages_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<MessageReply>(
                    "MessageReply",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for MessageReply {
    fn clear(&mut self) {
        self.clear_messages();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for MessageReply {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MessageReply {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Address {
    // message fields
    pub address: ::std::string::String,
    pub userName: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Address {}

impl Address {
    pub fn new() -> Address {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Address {
        static mut instance: ::protobuf::lazy::Lazy<Address> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Address,
        };
        unsafe {
            instance.get(Address::new)
        }
    }

    // string address = 1;

    pub fn clear_address(&mut self) {
        self.address.clear();
    }

    // Param is passed by value, moved
    pub fn set_address(&mut self, v: ::std::string::String) {
        self.address = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_address(&mut self) -> &mut ::std::string::String {
        &mut self.address
    }

    // Take field
    pub fn take_address(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.address, ::std::string::String::new())
    }

    pub fn get_address(&self) -> &str {
        &self.address
    }

    fn get_address_for_reflect(&self) -> &::std::string::String {
        &self.address
    }

    fn mut_address_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.address
    }

    // string userName = 2;

    pub fn clear_userName(&mut self) {
        self.userName.clear();
    }

    // Param is passed by value, moved
    pub fn set_userName(&mut self, v: ::std::string::String) {
        self.userName = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_userName(&mut self) -> &mut ::std::string::String {
        &mut self.userName
    }

    // Take field
    pub fn take_userName(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.userName, ::std::string::String::new())
    }

    pub fn get_userName(&self) -> &str {
        &self.userName
    }

    fn get_userName_for_reflect(&self) -> &::std::string::String {
        &self.userName
    }

    fn mut_userName_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.userName
    }
}

impl ::protobuf::Message for Address {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.address)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.userName)?;
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
        if !self.address.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.address);
        };
        if !self.userName.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.userName);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.address.is_empty() {
            os.write_string(1, &self.address)?;
        };
        if !self.userName.is_empty() {
            os.write_string(2, &self.userName)?;
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

impl ::protobuf::MessageStatic for Address {
    fn new() -> Address {
        Address::new()
    }

    fn descriptor_static(_: ::std::option::Option<Address>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "address",
                    Address::get_address_for_reflect,
                    Address::mut_address_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "userName",
                    Address::get_userName_for_reflect,
                    Address::mut_userName_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Address>(
                    "Address",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Address {
    fn clear(&mut self) {
        self.clear_address();
        self.clear_userName();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Address {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Address {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Room {
    // message fields
    pub name: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Room {}

impl Room {
    pub fn new() -> Room {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Room {
        static mut instance: ::protobuf::lazy::Lazy<Room> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Room,
        };
        unsafe {
            instance.get(Room::new)
        }
    }

    // string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    fn get_name_for_reflect(&self) -> &::std::string::String {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }
}

impl ::protobuf::Message for Room {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
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
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.name);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
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

impl ::protobuf::MessageStatic for Room {
    fn new() -> Room {
        Room::new()
    }

    fn descriptor_static(_: ::std::option::Option<Room>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    Room::get_name_for_reflect,
                    Room::mut_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Room>(
                    "Room",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Room {
    fn clear(&mut self) {
        self.clear_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Room {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Room {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Message {
    // message fields
    user: ::protobuf::SingularPtrField<Address>,
    pub content: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Message {}

impl Message {
    pub fn new() -> Message {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Message {
        static mut instance: ::protobuf::lazy::Lazy<Message> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Message,
        };
        unsafe {
            instance.get(Message::new)
        }
    }

    // .Address user = 1;

    pub fn clear_user(&mut self) {
        self.user.clear();
    }

    pub fn has_user(&self) -> bool {
        self.user.is_some()
    }

    // Param is passed by value, moved
    pub fn set_user(&mut self, v: Address) {
        self.user = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_user(&mut self) -> &mut Address {
        if self.user.is_none() {
            self.user.set_default();
        };
        self.user.as_mut().unwrap()
    }

    // Take field
    pub fn take_user(&mut self) -> Address {
        self.user.take().unwrap_or_else(|| Address::new())
    }

    pub fn get_user(&self) -> &Address {
        self.user.as_ref().unwrap_or_else(|| Address::default_instance())
    }

    fn get_user_for_reflect(&self) -> &::protobuf::SingularPtrField<Address> {
        &self.user
    }

    fn mut_user_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Address> {
        &mut self.user
    }

    // string content = 2;

    pub fn clear_content(&mut self) {
        self.content.clear();
    }

    // Param is passed by value, moved
    pub fn set_content(&mut self, v: ::std::string::String) {
        self.content = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_content(&mut self) -> &mut ::std::string::String {
        &mut self.content
    }

    // Take field
    pub fn take_content(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.content, ::std::string::String::new())
    }

    pub fn get_content(&self) -> &str {
        &self.content
    }

    fn get_content_for_reflect(&self) -> &::std::string::String {
        &self.content
    }

    fn mut_content_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.content
    }
}

impl ::protobuf::Message for Message {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.user)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.content)?;
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
        if let Some(v) = self.user.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if !self.content.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.content);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.user.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if !self.content.is_empty() {
            os.write_string(2, &self.content)?;
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

impl ::protobuf::MessageStatic for Message {
    fn new() -> Message {
        Message::new()
    }

    fn descriptor_static(_: ::std::option::Option<Message>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Address>>(
                    "user",
                    Message::get_user_for_reflect,
                    Message::mut_user_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "content",
                    Message::get_content_for_reflect,
                    Message::mut_content_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Message>(
                    "Message",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Message {
    fn clear(&mut self) {
        self.clear_user();
        self.clear_content();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Message {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Message {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x18, 0x63, 0x6f, 0x6d, 0x6d, 0x73, 0x2d, 0x73, 0x70, 0x65, 0x63, 0x2f, 0x6d, 0x65, 0x73,
    0x73, 0x61, 0x67, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x5c, 0x0a, 0x0f, 0x49, 0x6e,
    0x69, 0x74, 0x69, 0x61, 0x74, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x2e, 0x0a,
    0x0d, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x41, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x08, 0x2e, 0x41, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x52, 0x0d,
    0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x41, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x12, 0x19, 0x0a,
    0x04, 0x72, 0x6f, 0x6f, 0x6d, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x05, 0x2e, 0x52, 0x6f,
    0x6f, 0x6d, 0x52, 0x04, 0x72, 0x6f, 0x6f, 0x6d, 0x22, 0x29, 0x0a, 0x0d, 0x49, 0x6e, 0x69, 0x74,
    0x69, 0x61, 0x74, 0x65, 0x52, 0x65, 0x70, 0x6c, 0x79, 0x12, 0x18, 0x0a, 0x07, 0x73, 0x75, 0x63,
    0x63, 0x65, 0x73, 0x73, 0x18, 0x01, 0x20, 0x01, 0x28, 0x08, 0x52, 0x07, 0x73, 0x75, 0x63, 0x63,
    0x65, 0x73, 0x73, 0x22, 0x5d, 0x0a, 0x10, 0x54, 0x65, 0x72, 0x6d, 0x69, 0x6e, 0x61, 0x74, 0x65,
    0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x2e, 0x0a, 0x0d, 0x63, 0x6c, 0x69, 0x65, 0x6e,
    0x74, 0x41, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x08,
    0x2e, 0x41, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x52, 0x0d, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74,
    0x41, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x12, 0x19, 0x0a, 0x04, 0x72, 0x6f, 0x6f, 0x6d, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x05, 0x2e, 0x52, 0x6f, 0x6f, 0x6d, 0x52, 0x04, 0x72, 0x6f,
    0x6f, 0x6d, 0x22, 0x2a, 0x0a, 0x0e, 0x54, 0x65, 0x72, 0x6d, 0x69, 0x6e, 0x61, 0x74, 0x65, 0x52,
    0x65, 0x70, 0x6c, 0x79, 0x12, 0x18, 0x0a, 0x07, 0x73, 0x75, 0x63, 0x63, 0x65, 0x73, 0x73, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x08, 0x52, 0x07, 0x73, 0x75, 0x63, 0x63, 0x65, 0x73, 0x73, 0x22, 0x75,
    0x0a, 0x0e, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
    0x12, 0x2e, 0x0a, 0x0d, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x41, 0x64, 0x64, 0x72, 0x65, 0x73,
    0x73, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x08, 0x2e, 0x41, 0x64, 0x64, 0x72, 0x65, 0x73,
    0x73, 0x52, 0x0d, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x41, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73,
    0x12, 0x18, 0x0a, 0x07, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x09, 0x52, 0x07, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x19, 0x0a, 0x04, 0x72, 0x6f,
    0x6f, 0x6d, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x05, 0x2e, 0x52, 0x6f, 0x6f, 0x6d, 0x52,
    0x04, 0x72, 0x6f, 0x6f, 0x6d, 0x22, 0x34, 0x0a, 0x0c, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65,
    0x52, 0x65, 0x70, 0x6c, 0x79, 0x12, 0x24, 0x0a, 0x08, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65,
    0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x08, 0x2e, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67,
    0x65, 0x52, 0x08, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x22, 0x3f, 0x0a, 0x07, 0x41,
    0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x12, 0x18, 0x0a, 0x07, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73,
    0x73, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x07, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73,
    0x12, 0x1a, 0x0a, 0x08, 0x75, 0x73, 0x65, 0x72, 0x4e, 0x61, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x09, 0x52, 0x08, 0x75, 0x73, 0x65, 0x72, 0x4e, 0x61, 0x6d, 0x65, 0x22, 0x1a, 0x0a, 0x04,
    0x52, 0x6f, 0x6f, 0x6d, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x22, 0x41, 0x0a, 0x07, 0x4d, 0x65, 0x73, 0x73,
    0x61, 0x67, 0x65, 0x12, 0x1c, 0x0a, 0x04, 0x75, 0x73, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x08, 0x2e, 0x41, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x52, 0x04, 0x75, 0x73, 0x65,
    0x72, 0x12, 0x18, 0x0a, 0x07, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x09, 0x52, 0x07, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x32, 0xb8, 0x01, 0x0a, 0x0c,
    0x43, 0x6f, 0x6d, 0x6d, 0x75, 0x6e, 0x69, 0x63, 0x61, 0x74, 0x6f, 0x72, 0x12, 0x38, 0x0a, 0x14,
    0x49, 0x6e, 0x69, 0x74, 0x69, 0x61, 0x74, 0x65, 0x43, 0x6f, 0x6e, 0x76, 0x65, 0x72, 0x73, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x12, 0x10, 0x2e, 0x49, 0x6e, 0x69, 0x74, 0x69, 0x61, 0x74, 0x65, 0x52,
    0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x0e, 0x2e, 0x49, 0x6e, 0x69, 0x74, 0x69, 0x61, 0x74,
    0x65, 0x52, 0x65, 0x70, 0x6c, 0x79, 0x12, 0x3b, 0x0a, 0x15, 0x54, 0x65, 0x72, 0x6d, 0x69, 0x6e,
    0x61, 0x74, 0x65, 0x43, 0x6f, 0x6e, 0x76, 0x65, 0x72, 0x73, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x12,
    0x11, 0x2e, 0x54, 0x65, 0x72, 0x6d, 0x69, 0x6e, 0x61, 0x74, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65,
    0x73, 0x74, 0x1a, 0x0f, 0x2e, 0x54, 0x65, 0x72, 0x6d, 0x69, 0x6e, 0x61, 0x74, 0x65, 0x52, 0x65,
    0x70, 0x6c, 0x79, 0x12, 0x31, 0x0a, 0x0b, 0x53, 0x65, 0x6e, 0x64, 0x4d, 0x65, 0x73, 0x73, 0x61,
    0x67, 0x65, 0x12, 0x0f, 0x2e, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x52, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x1a, 0x0d, 0x2e, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x52, 0x65, 0x70,
    0x6c, 0x79, 0x28, 0x01, 0x30, 0x01, 0x4a, 0xb9, 0x12, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x46,
    0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x0a, 0x0a, 0x02, 0x06,
    0x00, 0x12, 0x04, 0x02, 0x00, 0x0a, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x06, 0x00, 0x01, 0x12, 0x03,
    0x02, 0x08, 0x14, 0x0a, 0x2e, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x00, 0x12, 0x03, 0x04, 0x02, 0x45,
    0x1a, 0x21, 0x20, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x74, 0x6f, 0x20, 0x73, 0x74,
    0x61, 0x72, 0x74, 0x20, 0x61, 0x20, 0x63, 0x6f, 0x6e, 0x76, 0x65, 0x72, 0x73, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x04, 0x06,
    0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x04, 0x1c, 0x2b, 0x0a,
    0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x04, 0x36, 0x43, 0x0a, 0x2c, 0x0a,
    0x04, 0x06, 0x00, 0x02, 0x01, 0x12, 0x03, 0x06, 0x02, 0x48, 0x1a, 0x1f, 0x20, 0x52, 0x65, 0x71,
    0x75, 0x65, 0x73, 0x74, 0x20, 0x74, 0x6f, 0x20, 0x65, 0x6e, 0x64, 0x20, 0x61, 0x20, 0x63, 0x6f,
    0x6e, 0x76, 0x65, 0x72, 0x73, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x06,
    0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x06, 0x06, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02,
    0x01, 0x02, 0x12, 0x03, 0x06, 0x1d, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x06, 0x38, 0x46, 0x0a, 0x7d, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x02, 0x12, 0x03, 0x09,
    0x02, 0x48, 0x1a, 0x70, 0x20, 0x42, 0x69, 0x2d, 0x64, 0x69, 0x72, 0x65, 0x63, 0x74, 0x69, 0x6f,
    0x6e, 0x61, 0x6c, 0x20, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x20, 0x6f, 0x70, 0x65, 0x6e, 0x65,
    0x64, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x20, 0x74, 0x72,
    0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x0a, 0x20, 0x54, 0x68, 0x69, 0x6e,
    0x6b, 0x69, 0x6e, 0x67, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x6a, 0x75, 0x73, 0x74, 0x20, 0x68,
    0x61, 0x76, 0x65, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x6f, 0x70, 0x65, 0x6e, 0x20, 0x66, 0x6f,
    0x72, 0x20, 0x65, 0x61, 0x63, 0x68, 0x20, 0x63, 0x6f, 0x6e, 0x76, 0x65, 0x72, 0x73, 0x61, 0x74,
    0x69, 0x6f, 0x6e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x09,
    0x06, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x09, 0x13, 0x19,
    0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x09, 0x1a, 0x28, 0x0a, 0x0c,
    0x0a, 0x05, 0x06, 0x00, 0x02, 0x02, 0x06, 0x12, 0x03, 0x09, 0x33, 0x39, 0x0a, 0x0c, 0x0a, 0x05,
    0x06, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x09, 0x3a, 0x46, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00,
    0x12, 0x04, 0x0c, 0x00, 0x11, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x0c,
    0x08, 0x17, 0x0a, 0x2f, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x0e, 0x02, 0x1c, 0x1a,
    0x22, 0x20, 0x41, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x69, 0x6e, 0x67, 0x20, 0x63, 0x6c, 0x69, 0x65,
    0x6e, 0x74, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x04, 0x0e, 0x02,
    0x0c, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x0e, 0x02, 0x09,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0e, 0x0a, 0x17, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0e, 0x1a, 0x1b, 0x0a, 0x3c, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x10, 0x02, 0x10, 0x1a, 0x2f, 0x20, 0x52, 0x6f, 0x6f, 0x6d,
    0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x20, 0x77, 0x69, 0x73, 0x68, 0x65, 0x73, 0x20, 0x74,
    0x6f, 0x20, 0x73, 0x74, 0x61, 0x72, 0x74, 0x20, 0x63, 0x6f, 0x6e, 0x76, 0x65, 0x72, 0x73, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x20, 0x77, 0x69, 0x74, 0x68, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x01, 0x04, 0x12, 0x04, 0x10, 0x02, 0x0e, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x06, 0x12, 0x03, 0x10, 0x02, 0x06, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x10, 0x07, 0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x10, 0x0e, 0x0f, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x13, 0x00, 0x16, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x13, 0x08, 0x15, 0x0a, 0x38, 0x0a, 0x04, 0x04,
    0x01, 0x02, 0x00, 0x12, 0x03, 0x15, 0x02, 0x13, 0x1a, 0x2b, 0x20, 0x53, 0x75, 0x63, 0x63, 0x65,
    0x73, 0x73, 0x20, 0x6f, 0x66, 0x20, 0x69, 0x6e, 0x69, 0x74, 0x69, 0x61, 0x6c, 0x69, 0x7a, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6f, 0x66, 0x20, 0x63, 0x6f, 0x6e, 0x76, 0x65, 0x72, 0x73, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x04,
    0x15, 0x02, 0x13, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x15,
    0x02, 0x06, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x15, 0x07, 0x0e,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x15, 0x11, 0x12, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x18, 0x00, 0x1d, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02,
    0x01, 0x12, 0x03, 0x18, 0x08, 0x18, 0x0a, 0x2f, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03,
    0x1a, 0x02, 0x1c, 0x1a, 0x22, 0x20, 0x41, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x20, 0x6f, 0x66,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x69, 0x6e, 0x67, 0x20,
    0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04,
    0x12, 0x04, 0x1a, 0x02, 0x18, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x06, 0x12,
    0x03, 0x1a, 0x02, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1a,
    0x0a, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x1a, 0x1a, 0x1b,
    0x0a, 0x1c, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x1c, 0x02, 0x10, 0x1a, 0x0f, 0x20,
    0x52, 0x6f, 0x6f, 0x6d, 0x20, 0x74, 0x6f, 0x20, 0x6c, 0x65, 0x61, 0x76, 0x65, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x04, 0x12, 0x04, 0x1c, 0x02, 0x1a, 0x1c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x01, 0x06, 0x12, 0x03, 0x1c, 0x02, 0x06, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x1c, 0x07, 0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x1c, 0x0e, 0x0f, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x1f,
    0x00, 0x22, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x1f, 0x08, 0x16, 0x0a,
    0x25, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x21, 0x02, 0x13, 0x1a, 0x18, 0x20, 0x53,
    0x75, 0x63, 0x63, 0x65, 0x73, 0x73, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x65, 0x72, 0x6d, 0x69, 0x6e,
    0x61, 0x74, 0x69, 0x6f, 0x6e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x04, 0x12,
    0x04, 0x21, 0x02, 0x1f, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x05, 0x12, 0x03,
    0x21, 0x02, 0x06, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x21, 0x07,
    0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x21, 0x11, 0x12, 0x0a,
    0x0a, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x24, 0x00, 0x2b, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x04, 0x01, 0x12, 0x03, 0x24, 0x08, 0x16, 0x0a, 0x2f, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12,
    0x03, 0x26, 0x02, 0x1c, 0x1a, 0x22, 0x20, 0x41, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x20, 0x6f,
    0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x69, 0x6e, 0x67,
    0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00,
    0x04, 0x12, 0x04, 0x26, 0x02, 0x24, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x06,
    0x12, 0x03, 0x26, 0x02, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x26, 0x0a, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x26, 0x1a,
    0x1b, 0x0a, 0x31, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x01, 0x12, 0x03, 0x28, 0x02, 0x15, 0x1a, 0x24,
    0x20, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x20, 0x74, 0x6f, 0x20, 0x61, 0x64, 0x64, 0x20,
    0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6f, 0x6e, 0x76, 0x65, 0x72, 0x73, 0x61, 0x74,
    0x69, 0x6f, 0x6e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x04, 0x12, 0x04, 0x28,
    0x02, 0x26, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x05, 0x12, 0x03, 0x28, 0x02,
    0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x01, 0x12, 0x03, 0x28, 0x09, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x03, 0x12, 0x03, 0x28, 0x13, 0x14, 0x0a, 0x25, 0x0a,
    0x04, 0x04, 0x04, 0x02, 0x02, 0x12, 0x03, 0x2a, 0x02, 0x10, 0x1a, 0x18, 0x20, 0x52, 0x6f, 0x6f,
    0x6d, 0x20, 0x74, 0x6f, 0x20, 0x61, 0x64, 0x64, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65,
    0x20, 0x74, 0x6f, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x04, 0x12, 0x04, 0x2a,
    0x02, 0x28, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x06, 0x12, 0x03, 0x2a, 0x02,
    0x06, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x2a, 0x07, 0x0b, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x03, 0x12, 0x03, 0x2a, 0x0e, 0x0f, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x05, 0x12, 0x04, 0x2d, 0x00, 0x30, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01,
    0x12, 0x03, 0x2d, 0x08, 0x14, 0x0a, 0x37, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x00, 0x12, 0x03, 0x2f,
    0x02, 0x20, 0x1a, 0x2a, 0x20, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x20, 0x6f, 0x66, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x20, 0x77, 0x72, 0x69, 0x74,
    0x74, 0x65, 0x6e, 0x20, 0x62, 0x79, 0x20, 0x6f, 0x74, 0x68, 0x65, 0x72, 0x73, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x04, 0x12, 0x03, 0x2f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x00, 0x06, 0x12, 0x03, 0x2f, 0x0b, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x2f, 0x13, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x2f, 0x1e, 0x1f, 0x0a, 0x8e, 0x01, 0x0a, 0x02, 0x04, 0x06, 0x12, 0x04, 0x36,
    0x00, 0x3b, 0x01, 0x1a, 0x1a, 0x20, 0x41, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x20, 0x6f, 0x66,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x6f, 0x72, 0x0a, 0x32,
    0x66, 0x20, 0x4e, 0x4f, 0x54, 0x45, 0x20, 0x74, 0x68, 0x65, 0x20, 0x62, 0x65, 0x6c, 0x6f, 0x77,
    0x20, 0x61, 0x72, 0x65, 0x20, 0x62, 0x61, 0x73, 0x69, 0x63, 0x61, 0x6c, 0x6c, 0x79, 0x20, 0x74,
    0x79, 0x70, 0x65, 0x20, 0x61, 0x6c, 0x69, 0x61, 0x73, 0x69, 0x6e, 0x67, 0x20, 0x62, 0x75, 0x74,
    0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x61, 0x6c, 0x6c, 0x6f, 0x77, 0x20, 0x66, 0x6f, 0x72, 0x20,
    0x65, 0x78, 0x74, 0x72, 0x61, 0x20, 0x64, 0x61, 0x74, 0x61, 0x20, 0x74, 0x6f, 0x0a, 0x20, 0x61,
    0x64, 0x64, 0x65, 0x64, 0x20, 0x65, 0x61, 0x73, 0x69, 0x6c, 0x79, 0x20, 0x69, 0x66, 0x20, 0x6e,
    0x65, 0x65, 0x64, 0x65, 0x64, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x06, 0x01, 0x12, 0x03,
    0x36, 0x08, 0x0f, 0x0a, 0x2a, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x00, 0x12, 0x03, 0x38, 0x02, 0x15,
    0x1a, 0x1d, 0x20, 0x4a, 0x75, 0x73, 0x74, 0x20, 0x70, 0x65, 0x72, 0x68, 0x61, 0x70, 0x73, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x69, 0x70, 0x20, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x04, 0x12, 0x04, 0x38, 0x02, 0x36, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x05, 0x12, 0x03, 0x38, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x00, 0x01, 0x12, 0x03, 0x38, 0x09, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x38, 0x13, 0x14, 0x0a, 0x2f, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x01,
    0x12, 0x03, 0x3a, 0x02, 0x16, 0x1a, 0x22, 0x20, 0x55, 0x73, 0x65, 0x72, 0x20, 0x6e, 0x61, 0x6d,
    0x65, 0x20, 0x74, 0x6f, 0x20, 0x62, 0x65, 0x20, 0x75, 0x73, 0x65, 0x64, 0x20, 0x69, 0x6e, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x72, 0x6f, 0x6f, 0x6d, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x01, 0x04, 0x12, 0x04, 0x3a, 0x02, 0x38, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01,
    0x05, 0x12, 0x03, 0x3a, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x3a, 0x09, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x03, 0x12, 0x03, 0x3a,
    0x14, 0x15, 0x0a, 0x3d, 0x0a, 0x02, 0x04, 0x07, 0x12, 0x04, 0x3e, 0x00, 0x40, 0x01, 0x1a, 0x31,
    0x20, 0x43, 0x6f, 0x6e, 0x73, 0x74, 0x72, 0x75, 0x63, 0x74, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x64,
    0x65, 0x66, 0x69, 0x6e, 0x69, 0x6e, 0x67, 0x20, 0x61, 0x20, 0x72, 0x6f, 0x6f, 0x6d, 0x20, 0x66,
    0x6f, 0x72, 0x20, 0x63, 0x6f, 0x6d, 0x6d, 0x75, 0x6e, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e,
    0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x07, 0x01, 0x12, 0x03, 0x3e, 0x08, 0x0c, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x07, 0x02, 0x00, 0x12, 0x03, 0x3f, 0x02, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07,
    0x02, 0x00, 0x04, 0x12, 0x04, 0x3f, 0x02, 0x3e, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x3f, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x3f, 0x09, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x3f, 0x10, 0x11, 0x0a, 0x55, 0x0a, 0x02, 0x04, 0x08, 0x12, 0x04, 0x43, 0x00, 0x46, 0x01, 0x1a,
    0x49, 0x20, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x20, 0x63, 0x6f, 0x6e, 0x73, 0x74, 0x72,
    0x75, 0x63, 0x74, 0x20, 0x74, 0x6f, 0x20, 0x64, 0x65, 0x74, 0x65, 0x72, 0x6d, 0x69, 0x6e, 0x65,
    0x20, 0x77, 0x68, 0x6f, 0x20, 0x68, 0x61, 0x73, 0x20, 0x73, 0x65, 0x6e, 0x74, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x69, 0x74,
    0x73, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x08,
    0x01, 0x12, 0x03, 0x43, 0x08, 0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x00, 0x12, 0x03,
    0x44, 0x02, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x04, 0x12, 0x04, 0x44, 0x02,
    0x43, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x06, 0x12, 0x03, 0x44, 0x02, 0x09,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x01, 0x12, 0x03, 0x44, 0x0a, 0x0e, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x03, 0x12, 0x03, 0x44, 0x11, 0x12, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x08, 0x02, 0x01, 0x12, 0x03, 0x45, 0x02, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02,
    0x01, 0x04, 0x12, 0x04, 0x45, 0x02, 0x44, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01,
    0x05, 0x12, 0x03, 0x45, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x45, 0x09, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x03, 0x12, 0x03, 0x45,
    0x13, 0x14, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
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
