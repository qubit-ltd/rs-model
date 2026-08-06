// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! External behavior coverage for value-object codes and ordering policies.

use std::cmp::Ordering;

use qubit_mixin::Normalizable;
use qubit_model::{
    commons::{
        Code,
        CodeMap,
        CredentialType,
        RequestStatus,
        VerifyState,
    },
    contact::{
        CoordinateSystem,
        Phone,
        PhoneCodec,
        Region,
    },
    controller::{
        NullSortOption,
        SortOrder,
    },
    Entity,
    Module,
    Operation,
};

/// Exercises every stable code that is exposed by shared enumeration values.
#[test]
fn test_shared_value_codes_and_wire_names_are_stable() {
    let credential_types = [
        (CredentialType::IdentityCard, "01"),
        (CredentialType::ResidenceBooklet, "02"),
        (CredentialType::Passport, "03"),
        (CredentialType::OfficerCard, "04"),
        (CredentialType::DrivingCard, "05"),
        (CredentialType::HongkongMacaoReturnPermit, "06"),
        (CredentialType::TaiwanReturnPermit, "07"),
        (CredentialType::PoliceCard, "14"),
        (CredentialType::HongkongPassport, "16"),
        (CredentialType::MacaoPassport, "17"),
        (CredentialType::TaiwanPassport, "18"),
        (CredentialType::ForeignerPermanentResidencePermit, "19"),
        (CredentialType::HongkongMacaoTaiwanResidencePermit, "20"),
        (CredentialType::BirthCertificate, "21"),
        (CredentialType::SocialSecurityCard, "31"),
        (CredentialType::MedicareCard, "32"),
        (CredentialType::EmployeeCard, "50"),
        (CredentialType::PractisingCertificate, "51"),
        (CredentialType::TitleCertificate, "52"),
        (CredentialType::BusinessLicense, "70"),
        (CredentialType::OrganizationCode, "71"),
        (CredentialType::Other, "99"),
    ];
    for (value, code) in credential_types {
        assert_eq!(value.code(), code);
    }

    let statuses = [
        (RequestStatus::Created, "CREATED"),
        (RequestStatus::Submitted, "SUBMITTED"),
        (RequestStatus::Pending, "PENDING"),
        (RequestStatus::Processing, "PROCESSING"),
        (RequestStatus::Failed, "FAILED"),
        (RequestStatus::Cancelled, "CANCELLED"),
        (RequestStatus::Completed, "COMPLETED"),
    ];
    for (value, name) in statuses {
        assert_eq!(value.as_str(), name);
    }

    let states = [
        (VerifyState::None, "NONE"),
        (VerifyState::Verifying, "VERIFYING"),
        (VerifyState::Valid, "VALID"),
        (VerifyState::Invalid, "INVALID"),
    ];
    for (value, name) in states {
        assert_eq!(value.as_str(), name);
    }
}

/// Exercises coordinate descriptions and the complete administrative hierarchy.
#[test]
fn test_geographic_values_report_source_contracts() {
    let systems = [
        (CoordinateSystem::Wgs84, "WGS-84", "World Geodetic System 1984"),
        (CoordinateSystem::Gcj02, "GCJ-02", "Mars Coordinate System"),
        (CoordinateSystem::Bd09, "BD-09", "Baidu Coordinate System"),
    ];
    for (value, code, description) in systems {
        assert_eq!(value.code(), code);
        assert_eq!(value.description(), description);
    }

    assert_eq!(Region::Country.parent(), None);
    assert_eq!(Region::Province.parent(), Some(Region::Country));
    assert_eq!(Region::City.parent(), Some(Region::Province));
    assert_eq!(Region::District.parent(), Some(Region::City));
    assert_eq!(Region::Street.parent(), Some(Region::District));
}

/// Exercises all entity and operation ownership mappings.
#[test]
fn test_entity_and_operation_mappings_are_complete() {
    let entities = [
        (Entity::App, "app"),
        (Entity::Category, "category"),
        (Entity::Credential, "credential"),
        (Entity::Dict, "dict"),
        (Entity::DictEntry, "dict_entry"),
        (Entity::Source, "source"),
        (Entity::Payload, "payload"),
        (Entity::Session, "session"),
        (Entity::VerifyCode, "verify_code"),
    ];
    for (value, name) in entities {
        assert_eq!(value.as_str(), name);
    }

    let operations = [
        (Operation::Register, Module::BasicOperation),
        (Operation::Login, Module::BasicOperation),
        (Operation::Logout, Module::BasicOperation),
        (Operation::ListSetting, Module::SystemManagement),
        (Operation::ListUser, Module::UserManagement),
        (Operation::ListProduct, Module::ProductManagement),
        (Operation::ListOrder, Module::OrderManagement),
        (Operation::ListPrescription, Module::PrescriptionManagement),
        (Operation::ListAppointment, Module::AppointmentManagement),
        (Operation::ListWorkSchedule, Module::WorkScheduleManagement),
    ];
    for (operation, module) in operations {
        assert_eq!(operation.module(), module);
    }
}

/// Exercises each null ordering policy in both sort directions.
#[test]
fn test_null_sort_options_cover_all_null_comparisons() {
    for option in [
        NullSortOption::NullFirst,
        NullSortOption::NullLast,
        NullSortOption::NullSmallest,
        NullSortOption::NullLargest,
    ] {
        assert_eq!(
            option.compare_none(true, true, SortOrder::Asc),
            Ordering::Equal
        );
        assert_eq!(
            option.compare_none(false, true, SortOrder::Asc),
            option.compare_none(true, false, SortOrder::Asc).reverse()
        );
        assert_eq!(
            option.compare_none(false, true, SortOrder::Desc),
            option.compare_none(true, false, SortOrder::Desc).reverse()
        );
    }
}

/// Rejects comparisons that do not contain a null operand.
#[test]
#[should_panic(expected = "either operand must be null")]
fn test_null_sort_options_reject_two_present_operands() {
    let _ = NullSortOption::NullFirst.compare_none(false, false, SortOrder::Asc);
}

/// Exercises code normalization, emptiness, and every supported telephone form.
#[test]
fn test_code_models_and_phone_codec_preserve_source_forms() {
    let mut code = Code {
        standard: Some("  HL7  ".into()),
        code: "  A1  ".into(),
        ..Code::default()
    };
    code.normalize();
    assert_eq!(code.standard.as_deref(), Some("HL7"));
    assert_eq!(code.code, "A1");
    assert!(!code.is_empty());
    assert!(Code::default().is_empty());
    assert!(
        !Code {
            app: Some(Default::default()),
            ..Code::default()
        }
        .is_empty()
    );
    assert!(
        !Code {
            standard: Some("standard".into()),
            ..Code::default()
        }
        .is_empty()
    );
    assert!(
        !Code {
            code: "code".into(),
            ..Code::default()
        }
        .is_empty()
    );

    let mut code_map = CodeMap {
        entity: "  patient  ".into(),
        source: Some(code),
        platform_code: "  P1  ".into(),
        ..CodeMap::default()
    };
    code_map.normalize();
    assert_eq!(code_map.entity, "patient");
    assert_eq!(code_map.platform_code, "P1");
    assert!(!code_map.is_empty());
    assert!(CodeMap::default().is_empty());
    assert!(
        !CodeMap {
            id: Some(1),
            ..CodeMap::default()
        }
        .is_empty()
    );
    assert!(
        !CodeMap {
            entity: "patient".into(),
            ..CodeMap::default()
        }
        .is_empty()
    );
    assert!(
        !CodeMap {
            source: Some(Code {
                code: "source".into(),
                ..Code::default()
            }),
            ..CodeMap::default()
        }
        .is_empty()
    );
    assert!(
        !CodeMap {
            platform_code: "target".into(),
            ..CodeMap::default()
        }
        .is_empty()
    );

    let decoded = [
        ("123456", Phone::from("123456")),
        (
            "+86-123456",
            Phone {
                country_area: Some("86".into()),
                city_area: None,
                number: "123456".into(),
            },
        ),
        (
            "010-123456",
            Phone {
                country_area: None,
                city_area: Some("010".into()),
                number: "123456".into(),
            },
        ),
        (
            "+86-010-123456",
            Phone {
                country_area: Some("86".into()),
                city_area: Some("010".into()),
                number: "123456".into(),
            },
        ),
    ];
    for (source, expected) in decoded {
        let phone = PhoneCodec::decode(Some(source))
            .expect("supported source phone must decode")
            .expect("nonblank source phone must be present");
        assert_eq!(phone, expected);
        assert_eq!(PhoneCodec::encode(Some(&phone)).as_deref(), Some(source));
    }
    assert_eq!(PhoneCodec::decode(None).expect("none must decode"), None);
    assert!(PhoneCodec::decode(Some("+86-010-1-2")).is_err());
    assert_eq!(PhoneCodec::decode_without_error(Some("bad-1-2-3")), None);

    let mut phone = Phone {
        country_area: Some(" 86 ".into()),
        city_area: Some(" 010 ".into()),
        number: " 123456 ".into(),
    };
    phone.normalize();
    assert_eq!(phone.to_string(), "+86-010-123456");
    assert_eq!(Phone::from(String::from("123456")), Phone::from("123456"));
}
