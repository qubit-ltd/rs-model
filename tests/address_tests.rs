// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use std::str::FromStr;

use bigdecimal::BigDecimal;
use qubit_mixin::{
    Info,
    Normalizable,
};
use qubit_model::{
    address::{
        Address,
        AddressBuilder,
        AddressErrorCode,
        MismatchMobileException,
        Region,
    },
    commons::VerifyState,
    contact::{
        Contact,
        Phone,
    },
    error::ErrorType,
};
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
        .longitude(
            BigDecimal::from_str("118.796877").expect("longitude should parse"),
        )
        .latitude(
            BigDecimal::from_str("32.060255").expect("latitude should parse"),
        )
        .build();

    assert_eq!(address.country.id, Some(1));
    assert_eq!(address.city.code, "NJ");
    assert_eq!(address.street.name, "Street");
    assert_eq!(address.detail, "No. 1");
    assert_eq!(
        address.location.as_ref().map(|value| &value.longitude),
        Some(
            &BigDecimal::from_str("118.796877")
                .expect("longitude should parse")
        )
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
