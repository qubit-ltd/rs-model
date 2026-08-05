// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use chrono::{
    TimeZone,
    Utc,
};
use qubit_mixin::Normalizable;
use qubit_model::{
    person::UserInfo,
    system::{
        Action,
        Environment,
        Expired,
        ExpiredReason,
        Host,
        Log,
        OperationLog,
        OperationLogInfo,
        Platform,
        Session,
        Setting,
        VerifyCode,
        VerifyScene,
    },
};
use qubit_model_metadata::metadata_of;
use qubit_redact::Redact;

fn assert_redact<T: Redact>() {}

#[test]
fn system_public_models_preserve_source_shapes_and_traits() {
    assert_redact::<Environment>();
    assert_redact::<Expired>();
    assert_redact::<Host>();
    assert_redact::<Log>();
    assert_redact::<OperationLog>();
    assert_redact::<OperationLogInfo>();
    assert_redact::<Session>();
    assert_redact::<Setting>();
    assert_redact::<VerifyCode>();

    assert_eq!(metadata_of::<Environment>().struct_fields().len(), 5);
    assert_eq!(metadata_of::<Expired>().struct_fields().len(), 2);
    assert_eq!(metadata_of::<Host>().struct_fields().len(), 3);
    assert_eq!(metadata_of::<Log>().struct_fields().len(), 9);
    assert_eq!(metadata_of::<OperationLog>().struct_fields().len(), 51);
    assert_eq!(metadata_of::<OperationLogInfo>().struct_fields().len(), 11);
    assert_eq!(metadata_of::<Session>().struct_fields().len(), 11);
}

#[test]
fn system_model_metadata_preserves_source_constraints() {
    let environment = metadata_of::<Environment>();
    for field in ["ip", "location", "platform", "udid", "push_token"] {
        assert!(environment.indexes().any(|index| index.contains(field)));
    }

    let host = metadata_of::<Host>();
    assert_eq!(host.primary_key().unwrap().fields()[0].name(), "id");
    assert_eq!(
        host.unique_constraints()
            .next()
            .unwrap()
            .comparison_of("udid"),
        Some(qubit_model_metadata::UniqueComparison::Exact)
    );

    let operation = metadata_of::<OperationLog>();
    assert_eq!(operation.primary_key().unwrap().fields()[0].name(), "id");
    for field in [
        "action",
        "request_time",
        "client_ip",
        "request_id",
        "service",
    ] {
        assert!(operation.indexes().any(|index| index.contains(field)));
    }
}

#[test]
fn system_enums_and_session_helpers_preserve_source_behavior() {
    assert_eq!(ExpiredReason::SingleSession.id(), "single_session");
    assert_eq!(Action::PurgeAll.display_name(), "Purge all");
    assert_eq!(Action::from_name("BATCH_UPDATE"), Some(Action::BatchUpdate));
    assert_eq!(Action::from_name("batch_update"), None);
    assert_eq!(
        serde_json::to_string(&Platform::IpadOs).unwrap(),
        "\"IPAD_OS\""
    );
    assert_eq!(
        serde_json::to_string(&VerifyScene::Login).unwrap(),
        "\"LOGIN\""
    );

    let session = Session {
        roles: vec!["ADMIN".into()],
        user: Some(UserInfo {
            username: "alice".into(),
            ..UserInfo::default()
        }),
        ..Session::default()
    };
    assert!(session.has_role("ADMIN"));
    assert_eq!(session.username(), Some("alice"));
    assert_eq!(serde_json::to_value(&session).unwrap()["username"], "alice");

    Session::reset();
    Session::set_current_user(session.user.clone());
    assert_eq!(
        Session::current_user().map(|user| user.username),
        Some("alice".into())
    );
    Session::reset();
}

#[test]
fn system_normalization_preserves_source_empty_and_text_behavior() {
    let mut environment = Environment {
        ip: Some("  ".into()),
        udid: Some(" device ".into()),
        ..Environment::default()
    };
    environment.normalize();
    assert_eq!(environment.ip, None);
    assert_eq!(environment.udid.as_deref(), Some("device"));
    assert!(!environment.is_empty());
}

#[test]
fn operation_log_projects_and_assigns_compact_info() {
    let request_time = Utc.timestamp_opt(1_700_000_000, 0).unwrap();
    let mut log = OperationLog {
        id: Some(5),
        action: Action::Update,
        resource: Some("USER".into()),
        user: Some(UserInfo {
            username: "alice".into(),
            ..UserInfo::default()
        }),
        client_ip: "127.0.0.1".into(),
        success: Some(false),
        request_time: Some(request_time),
        ..OperationLog::default()
    };
    let info = log.info();
    assert_eq!(info.username.as_deref(), Some("alice"));
    assert_eq!(info.timestamp, Some(request_time));

    let replacement = OperationLogInfo {
        action: Action::Get,
        username: Some("bob".into()),
        error_code: Some("DENIED".into()),
        ..OperationLogInfo::default()
    };
    log.assign_info(&replacement);
    assert_eq!(log.action, Action::Get);
    assert_eq!(
        log.user.as_ref().map(|user| user.username.as_str()),
        Some("bob")
    );
    assert_eq!(
        log.error.as_ref().map(|error| error.code.as_str()),
        Some("DENIED")
    );
}

#[test]
fn system_redaction_hides_tokens_request_material_and_device_ids() {
    let environment = Environment {
        udid: Some("private-device".into()),
        push_token: Some("private-push-token".into()),
        ..Environment::default()
    };
    let rendered = format!("{:?}", environment.redacted());
    assert!(!rendered.contains("private-device"));
    assert!(!rendered.contains("private-push-token"));

    let log = OperationLog {
        user_token_hash: Some("private-user-hash".into()),
        request_body: Some("private-request-body".into()),
        response_body: Some("private-response-body".into()),
        ..OperationLog::default()
    };
    let rendered = format!("{:?}", log.redacted());
    assert!(!rendered.contains("private-user-hash"));
    assert!(!rendered.contains("private-request-body"));
    assert!(!rendered.contains("private-response-body"));
}

#[test]
fn logs_order_by_source_timestamp() {
    let earlier = Log {
        timestamp: Utc.timestamp_opt(1, 0).single(),
        ..Log::default()
    };
    let later = Log {
        timestamp: Utc.timestamp_opt(2, 0).single(),
        ..Log::default()
    };
    assert!(earlier < later);

    let expired = Expired {
        time: Utc.timestamp_opt(3, 0).single(),
        reason: ExpiredReason::Timeout,
    };
    assert_eq!(expired.reason, ExpiredReason::Timeout);
}
