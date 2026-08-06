// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Integration tests for migrated contact domain models.

use qubit_mixin::Normalizable;
use qubit_model::contact::{
    ContactCodecError, CoordinateSystem, LocationCodec, LocationCoordinateDeserializer, Phone,
    PhoneJsonKeyDeserializer,
};

#[test]
fn test_phone_preserves_all_source_number_components() {
    let phone = Phone {
        country_area: Some("86".to_owned()),
        city_area: Some("025".to_owned()),
        number: "88273847".to_owned(),
    };

    assert_eq!(phone.country_area.as_deref(), Some("86"));
    assert_eq!(phone.city_area.as_deref(), Some("025"));
    assert_eq!(phone.number, "88273847");
}

#[test]
fn test_phone_display_covers_optional_area_combinations() {
    assert_eq!(Phone::from("13800138000").to_string(), "13800138000");
    assert_eq!(
        Phone {
            country_area: Some("86".into()),
            city_area: None,
            number: "1".into()
        }
        .to_string(),
        "+86-1"
    );
    assert_eq!(
        Phone {
            country_area: None,
            city_area: Some("25".into()),
            number: "1".into()
        }
        .to_string(),
        "25-1"
    );
}

/// Verifies phone conversion, normalization, and JSON-key decoding behavior.
#[test]
fn test_phone_conversion_normalization_and_json_key_decoding() {
    let from_str = Phone::from("13800138000");
    let from_string = Phone::from(String::from("13800138000"));
    assert_eq!(from_str, from_string);
    assert_eq!(from_str.country_area, None);
    assert_eq!(from_str.city_area, None);

    let mut normalized = Phone {
        country_area: Some(" 86 ".to_owned()),
        city_area: Some(" 025 ".to_owned()),
        number: " 13800138000 ".to_owned(),
    };
    normalized.normalize();
    assert_eq!(normalized.country_area.as_deref(), Some("86"));
    assert_eq!(normalized.city_area.as_deref(), Some("025"));
    assert_eq!(normalized.number, "13800138000");
    assert!(!normalized.is_normalized_empty());

    let mut empty = Phone::default();
    empty.normalize();
    assert!(empty.is_normalized_empty());

    assert_eq!(
        PhoneJsonKeyDeserializer::deserialize_key("+86-025-84507781")
            .expect("a structured phone key should decode"),
        Phone {
            country_area: Some("86".to_owned()),
            city_area: Some("025".to_owned()),
            number: "84507781".to_owned(),
        }
    );
    assert!(matches!(
        PhoneJsonKeyDeserializer::deserialize_key(""),
        Err(ContactCodecError::InvalidPhone)
    ));
    assert_eq!(
        PhoneJsonKeyDeserializer::default(),
        PhoneJsonKeyDeserializer
    );
}

/// Verifies location codecs cover valid, absent, and invalid wire values.
#[test]
fn test_location_codecs_preserve_wire_contracts() {
    let codec = LocationCodec::new(Some(CoordinateSystem::Gcj02));
    let location = codec
        .decode(Some("116.482086,39.990496"))
        .expect("the coordinate pair should decode")
        .expect("a nonempty coordinate pair should produce a location");
    assert_eq!(location.coordinate_system, Some(CoordinateSystem::Gcj02));
    assert_eq!(
        codec.encode(Some(&location)),
        Some("116.482086,39.990496".to_owned())
    );

    let overridden = codec
        .decode_with_coordinate_system(Some("1,2"), Some(CoordinateSystem::Bd09))
        .expect("the coordinate pair should decode")
        .expect("a nonempty coordinate pair should produce a location");
    assert_eq!(overridden.coordinate_system, Some(CoordinateSystem::Bd09));

    let default_codec = LocationCodec::default();
    assert_eq!(default_codec.decode(None).expect("none is absent"), None);
    assert_eq!(
        default_codec.decode(Some("")).expect("empty is absent"),
        None
    );
    assert_eq!(
        default_codec
            .decode_with_coordinate_system(None, Some(CoordinateSystem::Wgs84))
            .expect("none is absent"),
        None
    );
    assert_eq!(default_codec.encode(None), None);
    for source in ["116.482086", "1,2,3"] {
        assert!(
            matches!(
                default_codec.decode(Some(source)),
                Err(ContactCodecError::InvalidLocation)
            ),
            "{source} must not be accepted as a coordinate pair"
        );
    }
    for source in [",39.9", "116.4,"] {
        assert!(
            matches!(
                default_codec.decode(Some(source)),
                Err(ContactCodecError::InvalidCoordinate)
            ),
            "{source} must not be accepted as a coordinate pair"
        );
    }
    assert!(matches!(
        default_codec.decode(Some("invalid,2")),
        Err(ContactCodecError::InvalidCoordinate)
    ));
    assert!(matches!(
        default_codec.decode(Some("1,invalid")),
        Err(ContactCodecError::InvalidCoordinate)
    ));

    assert_eq!(
        LocationCoordinateDeserializer::deserialize("540.1")
            .expect("a decimal coordinate should deserialize")
            .to_string(),
        "-179.900000"
    );
    assert!(matches!(
        LocationCoordinateDeserializer::deserialize("not-a-coordinate"),
        Err(ContactCodecError::InvalidCoordinate)
    ));
}
