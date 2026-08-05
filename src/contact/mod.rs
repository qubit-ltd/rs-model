// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Contact and geographic domain models.

mod address;
mod administrative_regions;
mod contact_codec_error;
mod contact_value;
mod coordinate_system;
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
mod region;

pub use address::Address;
pub use administrative_regions::{City, Country, District, Province, Street};
pub use contact_codec_error::ContactCodecError;
pub use contact_value::Contact;
pub use coordinate_system::CoordinateSystem;
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
pub use region::Region;

pub use crate::address::AddressBuilder;
