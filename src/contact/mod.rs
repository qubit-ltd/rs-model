// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Contact and geographic domain models.

mod address;
mod city;
#[allow(clippy::module_inception)]
mod contact;
mod contact_codec_error;
mod coordinate_system;
mod country;
mod district;
mod location;
mod location_codec;
mod location_coordinate_codec;
mod location_coordinate_deserializer;
mod location_coordinate_serializer;
mod location_coordinate_xml_adapter;
mod phone;
mod phone_codec;
mod phone_json_deserializer;
mod phone_json_key_deserializer;
mod phone_json_serializer;
mod phone_type_register;
mod phone_xml_adapter;
mod province;
mod region;
mod street;

pub use address::Address;
pub use city::City;
pub use contact::Contact;
pub use contact_codec_error::ContactCodecError;
pub use coordinate_system::CoordinateSystem;
pub use country::Country;
pub use district::District;
pub use location::Location;
pub use location_codec::LocationCodec;
pub use location_coordinate_codec::LocationCoordinateCodec;
pub use location_coordinate_deserializer::LocationCoordinateDeserializer;
pub use location_coordinate_serializer::LocationCoordinateSerializer;
pub use location_coordinate_xml_adapter::LocationCoordinateXmlAdapter;
pub use phone::Phone;
pub use phone_codec::PhoneCodec;
pub use phone_json_deserializer::PhoneJsonDeserializer;
pub use phone_json_key_deserializer::PhoneJsonKeyDeserializer;
pub use phone_json_serializer::PhoneJsonSerializer;
pub use phone_type_register::PhoneTypeRegister;
pub use phone_xml_adapter::PhoneXmlAdapter;
pub use province::Province;
pub use region::Region;
pub use street::Street;

pub use crate::address::AddressBuilder;
