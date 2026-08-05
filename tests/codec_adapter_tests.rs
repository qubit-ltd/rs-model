// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use std::str::FromStr;

use bigdecimal::BigDecimal;
use qubit_model::{
    commons::{
        CredentialInfoCodec,
        CredentialType,
    },
    contact::{
        CoordinateSystem,
        LocationCodec,
        LocationCoordinateCodec,
        LocationCoordinateDeserializer,
        LocationCoordinateSerializer,
        LocationCoordinateXmlAdapter,
        Phone,
        PhoneCodec,
        PhoneJsonDeserializer,
        PhoneJsonKeyDeserializer,
        PhoneJsonSerializer,
        PhoneTypeRegister,
        PhoneXmlAdapter,
    },
    privilege::PrivilegesCodec,
};

#[test]
fn test_credential_info_codec_round_trips_source_format() {
    let credential =
        CredentialInfoCodec::decode(Some(" IDENTITY_CARD-320103198807625364 "))
            .expect("decode credential")
            .expect("credential value");
    assert_eq!(credential.r#type, CredentialType::IdentityCard);
    assert_eq!(credential.number, "320103198807625364");
    assert_eq!(
        CredentialInfoCodec::encode(Some(&credential)),
        Some("IDENTITY_CARD-320103198807625364".into())
    );
    assert_eq!(CredentialInfoCodec::decode(Some("  ")).unwrap(), None);
    assert!(CredentialInfoCodec::decode(Some("INVALID")).is_err());
}

#[test]
fn test_location_coordinate_codec_normalizes_and_adapters_delegate() {
    let normalized = LocationCoordinateCodec::normalize(Some(
        BigDecimal::from_str("540.1").expect("decimal"),
    ))
    .expect("coordinate");
    assert_eq!(normalized.to_string(), "-179.900000");

    let serialized = LocationCoordinateSerializer::serialize(&normalized);
    assert_eq!(serialized, "-179.900000");
    assert_eq!(
        LocationCoordinateDeserializer::deserialize(&serialized).unwrap(),
        normalized
    );
    assert_eq!(
        LocationCoordinateXmlAdapter::unmarshal(Some(&serialized)).unwrap(),
        Some(normalized.clone())
    );
    assert_eq!(
        LocationCoordinateXmlAdapter::marshal(Some(&normalized)),
        Some(serialized)
    );
}

#[test]
fn test_location_codec_round_trips_coordinates_and_coordinate_system() {
    let codec = LocationCodec::new(Some(CoordinateSystem::Gcj02));
    let location = codec
        .decode(Some("116.482086,39.990496"))
        .expect("decode location")
        .expect("location value");
    assert_eq!(location.coordinate_system, Some(CoordinateSystem::Gcj02));
    assert_eq!(
        codec.encode(Some(&location)),
        Some("116.482086,39.990496".into())
    );
    assert!(codec.decode(Some("116.482086")).is_err());
}

#[test]
fn test_phone_codec_and_wire_adapters_preserve_string_shape() {
    let phone = PhoneCodec::decode(Some(" +86-025-84507781 "))
        .expect("decode phone")
        .expect("phone value");
    assert_eq!(phone.country_area.as_deref(), Some("86"));
    assert_eq!(phone.city_area.as_deref(), Some("025"));
    assert_eq!(phone.number, "84507781");
    assert_eq!(
        PhoneCodec::encode(Some(&phone)),
        Some("+86-025-84507781".into())
    );

    let json =
        PhoneJsonSerializer::serialize(Some(&phone)).expect("JSON phone");
    assert_eq!(json, "\"+86-025-84507781\"");
    assert_eq!(
        PhoneJsonDeserializer::deserialize(&json).unwrap(),
        Some(phone.clone())
    );
    assert_eq!(
        PhoneJsonKeyDeserializer::deserialize_key("+86-025-84507781").unwrap(),
        phone
    );
    assert_eq!(
        PhoneXmlAdapter::marshal(Some(&phone)),
        Some("+86-025-84507781".into())
    );
    assert_eq!(PhoneXmlAdapter::unmarshal(None).unwrap(), None);
}

#[test]
fn test_phone_type_register_exposes_all_wire_components() {
    let register = PhoneTypeRegister;
    assert_eq!(register.type_name(), core::any::type_name::<Phone>());
    assert_eq!(register.serializer(), PhoneJsonSerializer);
    assert_eq!(register.deserializer(), PhoneJsonDeserializer);
    assert_eq!(register.key_serializer(), PhoneJsonSerializer);
    assert_eq!(register.key_deserializer(), PhoneJsonKeyDeserializer);
}

#[test]
fn test_privileges_codec_is_a_public_source_compatible_adapter() {
    let privileges = PrivilegesCodec::decode(Some(" read, ,write "))
        .expect("privileges value");
    assert_eq!(privileges.0, ["read", "write"]);
    assert_eq!(
        PrivilegesCodec::encode(Some(&privileges)),
        Some("read,write".into())
    );
    assert_eq!(PrivilegesCodec::decode(None), None);
}
