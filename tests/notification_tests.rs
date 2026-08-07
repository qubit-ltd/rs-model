// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use chrono::DateTime;
use chrono::Utc;

use qubit_model::contact::Phone;
use qubit_model::error::ErrorType;
use qubit_model::mixin::StatefulInfo;
use qubit_model::notification::NotificationErrorCode;
use qubit_model::notification::SendSmsException;
use qubit_model::notification::VerifyCode;
use qubit_model::notification::VerifyScene;
use qubit_model_metadata::metadata_of;
use qubit_redact::Redact;

fn assert_redact<T: Redact>() {}

fn verify_code(code: &str) -> VerifyCode {
    VerifyCode {
        id: Some(11),
        tenant: StatefulInfo::default(),
        app: StatefulInfo::default(),
        mobile: None,
        email: Some("ada@example.test".to_owned()),
        scene: VerifyScene::Login,
        code: code.to_owned(),
        message: "Your code is ready".to_owned(),
        verified: false,
        create_time: DateTime::<Utc>::from_timestamp(0, 0)
            .expect("the Unix epoch should be representable"),
    }
}

#[test]
fn test_notification_public_types_expose_model_and_redact_contracts() {
    assert_eq!(metadata_of::<VerifyCode>().struct_fields().len(), 10);
    assert_redact::<VerifyCode>();
    assert_redact::<VerifyScene>();
    assert_redact::<NotificationErrorCode>();
    assert_redact::<SendSmsException>();
}

#[test]
fn test_verify_scene_preserves_java_wire_names() {
    assert_eq!(
        serde_json::to_string(&VerifyScene::ResetPassword)
            .expect("verify scene should serialize"),
        "\"RESET_PASSWORD\""
    );
    assert_eq!(
        VerifyScene::from_wire_name("BIND_EMPLOYEE"),
        Some(VerifyScene::BindEmployee)
    );
    assert_eq!(VerifyScene::from_wire_name("bind_employee"), None);
}

#[test]
fn test_verify_code_desensitizes_token_and_redacts_diagnostics() {
    let mut long = verify_code("1234567890");
    long.desensitize();
    assert_eq!(long.code, "1234...7890");

    let mut short = verify_code("123456");
    short.desensitize();
    assert_eq!(short.code, "******");

    let original = verify_code("raw-token");
    let redacted = format!("{:?}", original.redacted());
    assert!(!redacted.contains("raw-token"));
    assert!(redacted.contains("ada@example.test"));
}

#[test]
fn test_notification_error_preserves_code_type_and_parameters() {
    let phone = Phone {
        country_area: Some("86".to_owned()),
        city_area: Some("025".to_owned()),
        number: "88273847".to_owned(),
    };
    let error = SendSmsException::for_phone(
        phone,
        "SMS_ERROR_001".to_owned(),
        "insufficient balance".to_owned(),
    );

    assert_eq!(error.code(), NotificationErrorCode::SendSmsFailed);
    assert_eq!(error.error_type(), ErrorType::ThirdPartyError);
    assert_eq!(error.reason(), "insufficient balance");
    assert_eq!(
        error.parameters().get("phone"),
        Some(&Some("+86-025-88273847".to_owned()))
    );
    assert_eq!(
        error.parameters().get("third_party_code"),
        Some(&Some("SMS_ERROR_001".to_owned()))
    );
    assert_eq!(
        error.parameters().get("reason"),
        Some(&Some("insufficient balance".to_owned()))
    );
}

#[test]
fn test_send_sms_exception_supports_multiple_phone_targets() {
    let error = SendSmsException::for_phones(
        vec![
            Phone {
                country_area: None,
                city_area: None,
                number: "13800138000".to_owned(),
            },
            Phone {
                country_area: None,
                city_area: None,
                number: "13900139000".to_owned(),
            },
        ],
        "SMS_ERROR_002".to_owned(),
        "batch failed".to_owned(),
    );

    assert_eq!(error.phone, None);
    assert_eq!(error.phones.as_ref().map(Vec::len), Some(2));
    assert_eq!(
        error.parameters().get("phone"),
        Some(&Some("[13800138000, 13900139000]".to_owned()))
    );
}
