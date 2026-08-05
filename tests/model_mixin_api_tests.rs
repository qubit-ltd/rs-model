// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_mixin::{Info, InfoWithEntity};
use qubit_model::{
    commons::Token,
    mixin::{
        InfoWithAppEntity, InfoWithToken, WithAttachment, WithAttachments, WithCreator,
        WithDeleter, WithModifier, WithStatefulInfoWithToken,
    },
};
use qubit_model_metadata::metadata_of;
use qubit_redact::Redact;

/// Requires diagnostic redaction for migrated composite values.
fn assert_redact<T: Redact>() {}

/// Requires the single-attachment source interface.
fn assert_with_attachment<T: WithAttachment + ?Sized>() {}

/// Requires the attachment-list source interface.
fn assert_with_attachments<T: WithAttachments + ?Sized>() {}

/// Requires the creator source interface.
fn assert_with_creator<T: WithCreator + ?Sized>() {}

/// Requires the modifier source interface.
fn assert_with_modifier<T: WithModifier + ?Sized>() {}

/// Requires the deleter source interface.
fn assert_with_deleter<T: WithDeleter + ?Sized>() {}

/// Requires the stateful-info-with-token source interface.
fn assert_with_stateful_info_with_token<T: WithStatefulInfoWithToken + ?Sized>() {}

#[test]
fn test_model_mixin_public_types_are_exported() {
    assert_redact::<InfoWithAppEntity>();
    assert_redact::<InfoWithToken>();
    assert_with_attachment::<dyn WithAttachment>();
    assert_with_attachments::<dyn WithAttachments>();
    assert_with_creator::<dyn WithCreator>();
    assert_with_modifier::<dyn WithModifier>();
    assert_with_deleter::<dyn WithDeleter>();
    assert_with_stateful_info_with_token::<dyn WithStatefulInfoWithToken>();

    assert_eq!(metadata_of::<InfoWithAppEntity>().struct_fields().len(), 2);
    assert_eq!(metadata_of::<InfoWithToken>().struct_fields().len(), 2);
}

#[test]
fn test_info_with_app_entity_preserves_composed_source_shape() {
    let value = InfoWithAppEntity::new(
        InfoWithEntity::new(
            Some(7),
            "SOURCE".into(),
            "Source".into(),
            None,
            Some("document".into()),
        ),
        Some(qubit_model::mixin::StatefulInfo::default()),
    );

    assert!(value.is_complete());
    let serialized = serde_json::to_value(&value).expect("serialize app info");
    assert_eq!(serialized["id"], 7);
    assert_eq!(serialized["entity"], "document");
    assert!(serialized.get("app").is_some());
}

#[test]
fn test_info_with_token_preserves_composed_source_shape_and_redaction() {
    let value = InfoWithToken::new(
        Info::new(Some(9), "USER".into(), "User".into(), None),
        Some(Token {
            value: "access-secret".into(),
            ..Token::default()
        }),
    );

    assert!(value.is_complete());
    assert!(!format!("{:?}", value.redacted()).contains("access-secret"));
    let serialized = serde_json::to_value(&value).expect("serialize token info");
    assert_eq!(serialized["id"], 9);
    assert_eq!(serialized["token"]["value"], "access-secret");
}
