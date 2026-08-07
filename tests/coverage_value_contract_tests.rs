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
use qubit_model::Entity;
use qubit_model::Module;
use qubit_model::Operation;
use qubit_model::commons::Code;
use qubit_model::commons::CodeMap;
use qubit_model::commons::CredentialType;
use qubit_model::commons::RequestStatus;
use qubit_model::commons::VerifyState;
use qubit_model::contact::CoordinateSystem;
use qubit_model::contact::Phone;
use qubit_model::contact::PhoneCodec;
use qubit_model::contact::Region;
use qubit_model::controller::NullSortOption;
use qubit_model::controller::SortOrder;
use qubit_model::error::ErrorType;
use qubit_model::file::AttachmentType;
use qubit_model::notification::NotificationErrorCode;
use qubit_model::notification::VerifyScene;
use qubit_model::privilege::Privileges;
use qubit_model::privilege::PrivilegesCodec;
use qubit_model::security::KeyFormat;
use qubit_model::security::Signature;
use qubit_model::security::SignatureAlgorithm;
use qubit_model::util::Result;
use qubit_model::util::ResultValue;

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
        (
            CoordinateSystem::Wgs84,
            "WGS-84",
            "World Geodetic System 1984",
        ),
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
    let _ =
        NullSortOption::NullFirst.compare_none(false, false, SortOrder::Asc);
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

    let mut blank_phone = Phone::default();
    blank_phone.normalize();
    assert!(blank_phone.is_normalized_empty());
}

/// Exercises attachment classification identifiers and all MIME-type branches.
#[test]
fn test_attachment_types_preserve_ids_and_classify_content_types() {
    let types = [
        (AttachmentType::Image, "image"),
        (AttachmentType::Document, "document"),
        (AttachmentType::Audio, "audio"),
        (AttachmentType::Video, "video"),
        (AttachmentType::Vcard, "vcard"),
        (AttachmentType::Location, "location"),
        (AttachmentType::ExternalImage, "external_image"),
        (AttachmentType::ExternalAudio, "external_audio"),
        (AttachmentType::ExternalVideo, "external_video"),
    ];
    for (value, identifier) in types {
        assert_eq!(value.id(), identifier);
    }
    assert_eq!(
        AttachmentType::for_content_type("image/png"),
        AttachmentType::Image
    );
    assert_eq!(
        AttachmentType::for_content_type("audio/mpeg"),
        AttachmentType::Audio
    );
    assert_eq!(
        AttachmentType::for_content_type("video/mp4"),
        AttachmentType::Video
    );
    assert_eq!(
        AttachmentType::for_content_type("text/x-vcard"),
        AttachmentType::Vcard
    );
    assert_eq!(
        AttachmentType::for_content_type("application/pdf"),
        AttachmentType::Document
    );
}

/// Exercises all notification scene parsing branches and the stable SMS error.
#[test]
fn test_notification_values_preserve_source_contracts() {
    let scenes = [
        ("REGISTER", VerifyScene::Register),
        ("RESET_PASSWORD", VerifyScene::ResetPassword),
        ("PAY", VerifyScene::Pay),
        ("REFUND", VerifyScene::Refund),
        ("VERIFY_MOBILE", VerifyScene::VerifyMobile),
        ("VERIFY_EMAIL", VerifyScene::VerifyEmail),
        ("VERIFY_REALNAME", VerifyScene::VerifyRealname),
        ("RECEIVE_DRUG", VerifyScene::ReceiveDrug),
        ("MODIFY", VerifyScene::Modify),
        ("LOGIN", VerifyScene::Login),
        ("BIND_EMPLOYEE", VerifyScene::BindEmployee),
        ("BIND_PERSON", VerifyScene::BindPerson),
        ("OTHER", VerifyScene::Other),
    ];
    for (wire_value, expected) in scenes {
        assert_eq!(VerifyScene::from_wire_name(wire_value), Some(expected));
    }
    assert_eq!(VerifyScene::from_wire_name("register"), None);
    assert_eq!(
        NotificationErrorCode::SendSmsFailed.message_template_zh_cn(),
        "发送短信失败：{reason}"
    );
    assert_eq!(
        NotificationErrorCode::SendSmsFailed.error_type(),
        ErrorType::ThirdPartyError
    );
}

/// Exercises construction, alias compatibility, and consumption of responses.
#[test]
fn test_result_value_owns_and_returns_the_response_value() {
    let value = ResultValue::new(String::from("result"));
    assert_eq!(value.clone().into_inner(), "result");
    let alias: Result<i32> = ResultValue::new(7);
    assert_eq!(alias.value, 7);
}

/// Exercises every signature algorithm and key-format parser branch.
#[test]
fn test_security_value_codes_and_payload_updates_are_stable() {
    let algorithms = [
        (SignatureAlgorithm::Md2WithRsa, "MD2withRSA"),
        (SignatureAlgorithm::Md5WithRsa, "MD5withRSA"),
        (SignatureAlgorithm::Sha1WithRsa, "SHA1withRSA"),
        (SignatureAlgorithm::Sha224WithRsa, "SHA224withRSA"),
        (SignatureAlgorithm::Sha256WithRsa, "SHA256withRSA"),
        (SignatureAlgorithm::Sha384WithRsa, "SHA384withRSA"),
        (SignatureAlgorithm::Sha512WithRsa, "SHA512withRSA"),
        (SignatureAlgorithm::Sha1WithDsa, "SHA1withDSA"),
        (SignatureAlgorithm::Sha224WithDsa, "SHA224withDSA"),
        (SignatureAlgorithm::Sha256WithDsa, "SHA256withDSA"),
        (SignatureAlgorithm::Sha1WithEcdsa, "SHA1withECDSA"),
        (SignatureAlgorithm::Sha224WithEcdsa, "SHA224withECDSA"),
        (SignatureAlgorithm::Sha256WithEcdsa, "SHA256withECDSA"),
        (SignatureAlgorithm::Sha384WithEcdsa, "SHA384withECDSA"),
        (SignatureAlgorithm::Sha512WithEcdsa, "SHA512withECDSA"),
    ];
    for (algorithm, code) in algorithms {
        assert_eq!(algorithm.code(), code);
    }
    assert_eq!(KeyFormat::Pkcs8.code(), "PKCS#8");
    assert_eq!(KeyFormat::X509.code(), "X.509");
    for name in ["pkcs8", "PKCS#8"] {
        assert_eq!(KeyFormat::for_name(name), Some(KeyFormat::Pkcs8));
    }
    for name in ["x509", "X.509"] {
        assert_eq!(KeyFormat::for_name(name), Some(KeyFormat::X509));
    }
    assert_eq!(KeyFormat::for_name("pem"), None);

    let mut signature = Signature::default();
    signature.set_payload("id", "one").set_payload("id", "two");
    signature.set_message("message");
    assert_eq!(signature.signed_info.message, "message");
    assert_eq!(signature.signed_info.payload.len(), 1);
    assert_eq!(
        signature.signed_info.payload[0].value.as_deref(),
        Some("two")
    );
}

/// Exercises valid and invalid privilege codec branches without losing values.
#[test]
fn test_privilege_codecs_validate_and_preserve_delimited_values() {
    assert_eq!(Privileges::decode(None).expect("none is valid"), None);
    assert_eq!(
        Privileges::decode(Some("")).expect("empty is valid"),
        Some(Privileges::default())
    );
    let privileges = Privileges::decode(Some("  read, , write  "))
        .expect("valid values must decode")
        .expect("present input must produce a value");
    assert_eq!(privileges.0, ["read", "write"]);
    assert_eq!(
        privileges.encode().expect("valid values encode"),
        "read,write"
    );
    assert!(Privileges(vec![String::new()]).encode().is_err());
    assert!(Privileges(vec!["read,write".into()]).encode().is_err());
    assert_eq!(PrivilegesCodec::decode(None), None);
    assert_eq!(
        PrivilegesCodec::decode(Some("")),
        Some(Privileges::default())
    );
    assert_eq!(
        PrivilegesCodec::encode(Some(&privileges)).as_deref(),
        Some("read,write")
    );
}
