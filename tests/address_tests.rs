// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use bigdecimal::BigDecimal;
use std::str::FromStr;

use qubit_mixin::Info;
use qubit_mixin::Normalizable;
use qubit_model::address::Address;
use qubit_model::address::AddressBuilder;
use qubit_model::address::AddressErrorCode;
use qubit_model::address::MismatchMobileException;
use qubit_model::address::Region;
use qubit_model::commons::VerifyState;
use qubit_model::contact::City;
use qubit_model::contact::Contact;
use qubit_model::contact::Country;
use qubit_model::contact::District;
use qubit_model::contact::Phone;
use qubit_model::contact::Province;
use qubit_model::contact::Street;
use qubit_model::error::ErrorType;
use qubit_model_metadata::metadata_of;
use qubit_redact::Redact;

fn assert_redact<T: Redact>() {}

#[test]
fn test_address_public_types_expose_redact_contracts() {
    assert_redact::<Address>();
    assert_redact::<AddressBuilder>();
    assert_redact::<AddressErrorCode>();
    assert_redact::<MismatchMobileException>();
    assert_redact::<Region>();
}

#[test]
fn test_address_metadata_preserves_nested_references_and_indexes() {
    let address = metadata_of::<Address>();
    for (field, target) in [
        ("country", std::any::type_name::<Country>()),
        ("province", std::any::type_name::<Province>()),
        ("city", std::any::type_name::<City>()),
        ("district", std::any::type_name::<District>()),
        ("street", std::any::type_name::<Street>()),
    ] {
        let reference = address
            .field(field)
            .expect("address reference field")
            .reference()
            .expect("address nested reference");
        assert_eq!(reference.target().identity().type_name(), target);
        assert!(address.indexes().any(|index| index.contains(field)));
    }
    for field in ["detail", "postalcode", "location"] {
        assert!(address.indexes().any(|index| index.contains(field)));
    }
}

#[test]
fn test_address_builder_preserves_hierarchy_and_coordinates() {
    let address = AddressBuilder::new()
        .country_id(Some(1))
        .country_code("CN")
        .country_name("China")
        .province_id(Some(2))
        .province_code("JS")
        .province_name("Jiangsu")
        .city_id(Some(3))
        .city_code("NJ")
        .city_name("Nanjing")
        .district_id(Some(4))
        .district_code("XW")
        .district_name("Xuanwu")
        .street_id(Some(5))
        .street_code("ST")
        .street_name("Street")
        .detail("No. 1")
        .postalcode("210000")
        .longitude(BigDecimal::from_str("118.796877").expect("longitude should parse"))
        .latitude(BigDecimal::from_str("32.060255").expect("latitude should parse"))
        .build();

    assert_eq!(address.country.id, Some(1));
    assert_eq!(address.city.code, "NJ");
    assert_eq!(address.street.name, "Street");
    assert_eq!(address.detail, "No. 1");
    assert_eq!(
        address.location.as_ref().map(|value| &value.longitude),
        Some(&BigDecimal::from_str("118.796877").expect("longitude should parse"))
    );
}

#[test]
fn test_address_is_same_uses_detail_postal_location_and_street_id() {
    let mut first = AddressBuilder::new()
        .street_id(Some(7))
        .detail("No. 1")
        .postalcode("210000")
        .build();
    let second = first.clone();
    assert!(first.is_same(&second));

    first.street = Info {
        id: Some(8),
        ..Info::default()
    };
    assert!(!first.is_same(&second));
}

#[test]
fn test_address_error_preserves_logic_type_and_parameters() {
    let expected = Phone {
        country_area: None,
        city_area: None,
        number: "13800138000".to_owned(),
    };
    let actual = Phone {
        country_area: None,
        city_area: None,
        number: "13900139000".to_owned(),
    };
    let error = MismatchMobileException::new("applicant", expected, actual);

    assert_eq!(error.code(), AddressErrorCode::MismatchMobile);
    assert_eq!(error.error_type(), ErrorType::LogicError);
    assert_eq!(error.parameters()["name"], "applicant");
    assert_eq!(error.parameters()["expected"], "13800138000");
    assert_eq!(error.parameters()["actual"], "13900139000");
}

#[test]
fn test_contact_address_normalization_and_changed_address_verification() {
    let mut address = Address {
        country: Info {
            code: " CN ".into(),
            ..Info::default()
        },
        province: Info {
            code: " JS ".into(),
            ..Info::default()
        },
        city: Info {
            code: " NJ ".into(),
            ..Info::default()
        },
        district: Info {
            code: " XW ".into(),
            ..Info::default()
        },
        street: Info {
            code: " ST ".into(),
            ..Info::default()
        },
        detail: "  No. 1  ".into(),
        postalcode: Some(" 210000 ".into()),
        location: None,
    };
    address.normalize();
    assert_eq!(address.detail, "No. 1");
    assert_eq!(address.postalcode.as_deref(), Some("210000"));
    assert!(Address::default().is_normalized_empty());
    assert!(!address.is_normalized_empty());

    let other = Contact {
        address: Some(Address::default()),
        address_verified: Some(VerifyState::Valid),
        ..Contact::default()
    };
    let mut current = Contact {
        address: Some(address),
        ..Contact::default()
    };
    current.copy_verify_state(&other);
    assert_eq!(current.address_verified, Some(VerifyState::None));
    assert!(Contact::default().is_normalized_empty());
}
