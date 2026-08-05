// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Integration coverage for shared domain-model exports.

use chrono::DateTime;
use qubit_model::{
    commons::{CredentialType, Currency, RequestStatus, State, Token, VerifyState},
    mixin::{StatefulInfo, StatefulInfoWithToken},
    util::{MessageFormatter, Result},
    Entity,
    Module,
    Operation,
};
use qubit_model_metadata::metadata_of;

/// Verifies shared model values preserve serde and static metadata support.
#[test]
fn test_shared_models_serialize_and_expose_metadata() {
    let token = Token {
        value: "access-token".to_owned(),
        create_time: DateTime::from_timestamp(0, 0)
            .expect("the Unix epoch should be representable"),
        max_age: Some(60),
        previous_value: None,
    };
    let info = StatefulInfo::new(
        Some(7),
        "app".to_owned(),
        "Application".to_owned(),
        Some(State::Normal),
        None,
    );
    let value = StatefulInfoWithToken::new(info, Some(token.clone()));
    let response = Result::new(value);

    let encoded = serde_json::to_string(&response)
        .expect("shared models should serialize as JSON");
    assert!(encoded.contains("access-token"));
    assert_eq!(metadata_of::<Token>().struct_fields().len(), 4);
    assert_eq!(
        metadata_of::<StatefulInfoWithToken>().struct_fields().len(),
        2
    );
}

/// Verifies the Java utility export retains indexed-template formatting.
#[test]
fn test_message_formatter_replaces_indexed_parameters() {
    assert_eq!(
        MessageFormatter::format("Hello, {0}!", &["Qubit"]),
        "Hello, Qubit!"
    );
}

/// Verifies the root package enums remain available to downstream models.
#[test]
fn test_root_model_enums_preserve_domain_relationships() {
    assert_eq!(Operation::Login.module(), Module::BasicOperation);
    assert_eq!(Entity::App.as_str(), "app");
}

/// Verifies common enumeration values serialize with their Java wire names.
#[test]
fn test_common_enums_preserve_wire_values() {
    assert_eq!(
        serde_json::to_string(&Currency::Cny)
            .expect("currency should serialize"),
        "\"CNY\""
    );
    assert_eq!(CredentialType::IdentityCard.code(), "01");
    assert_eq!(RequestStatus::Completed.as_str(), "COMPLETED");
    assert_eq!(VerifyState::Valid.as_str(), "VALID");
}
